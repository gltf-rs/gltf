
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_json;
use std;
use std::boxed::Box;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use json;
use gltf::{BufferData, ImageData, Gltf};
use root::Root;
use validation;

/// A trait for representing sources of glTF data that may be read by an importer.
pub trait DataSource {
    /// Read the contents of a .gltf or .glb file.
    fn gltf(&mut self) -> io::Result<Box<Read>>;
    
    /// Read the contents of a glTF buffer.
    fn buffer(&mut self, buffer: &json::buffer::Buffer) -> io::Result<Box<Read>>;

    /// Read the contents of a glTF image.
    fn image(&mut self, image: &json::image::Image) -> io::Result<Box<Read>>;
}

/// Error encountered when importing a glTF 2.0 asset.
#[derive(Debug)]
pub enum Error {
    /// Failure when deserializing a .gltf metadata file.
    Deserialize(serde_json::error::Error),
    
    /// A glTF extension required by the asset has not been enabled by the user.
    ExtensionDisabled(String),
    
    /// A glTF extension required by the asset is not supported by the library.
    ExtensionUnsupported(String),
    
    /// The glTF version of the asset is incompatible with the importer.
    IncompatibleVersion(String),
    
    /// Data source error.
    Io(io::Error),

    /// The .gltf data is invalid.
    Validation(Vec<validation::Error>),
}

/// A simple synchronous data source that can read from the file system.
#[derive(Clone, Debug)]
pub struct SimpleDataSource<'a> {
    /// The path to the glTF directory.
    path: &'a Path,
}

/// Imports glTF 2.0.
#[derive(Clone)]
pub struct Importer<S: DataSource> {
    /// The data source.
    source: S,
}

/// Convenience function for importing glTF under the default configuration.
pub fn import<P: AsRef<Path>>(path: &P) -> Result<Gltf, Error> {
    Importer::new(SimpleDataSource::new(path)).import()
}

impl<S: DataSource> Importer<S> {
    /// Constructs an `Importer`.
    pub fn new(source: S) -> Self {
        Self {
            source: source,
        }
    }
    
    /// Import some glTF 2.0.
    pub fn import(mut self) -> Result<Gltf, Error> {
        use std::io::Read;
        use self::Error::*;
        use self::validation::{JsonPath, Validate};

        // Read .gltf / .glb file
        let mut buffer = vec![];
        let _ = self.source.gltf()?.read_to_end(&mut buffer)?;
        if buffer.starts_with(b"glTF") {
            return Err(ExtensionUnsupported("KHR_binary_glTF".to_string()));
        }

        // Parse and validate the .gltf JSON data
        let json: json::Root = serde_json::from_slice(&buffer)?;
        let mut errs = Vec::new();
        json.validate(&json, || JsonPath::new(), &mut |err| errs.push(err));
        if !errs.is_empty() {
            return Err(Validation(errs));
        }

        // Read the glTF buffer data
        let mut buffers = vec![];
        for entry in &json.buffers {
            let mut data = vec![];
            let _ = self.source.buffer(entry)?.read_to_end(&mut data)?;
            buffers.push(BufferData(data));
        }

        // Read the glTF image data
        let mut images = vec![];
        for entry in &json.images {
            let image = if let Some(buffer_view) = entry.buffer_view.as_ref() {
                ImageData::Borrowed(buffer_view.value())
            } else {
                let mut buffer = vec![];
                let _ = self.source.image(entry)?.read_to_end(&mut buffer)?;
                ImageData::Owned(buffer)
            };
            images.push(image);
        }

        Ok(Gltf::new(Root::new(json), buffers, images))
    }
}

impl<'a> SimpleDataSource<'a> {
    /// Constructs a simple synchronous data source that can read from the file
    /// system with the given path as its base directory.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use gltf::v2::import::{import, SimpleDataSource};
    /// let gltf = import(SimpleDataSource("~/Test.gltf"))?;
    /// println!("{:#?}", gltf);
    /// ```
    pub fn new<P: AsRef<Path>>(path: &'a P) -> Self {
        Self {
            path: path.as_ref(),
        }
    }
}

impl<'a> DataSource for SimpleDataSource<'a> {
    /// Read the contents of a .gltf or .glb file.
    fn gltf(&mut self) -> io::Result<Box<Read>> {
        let file = File::open(self.path)?;
        Ok(Box::new(BufReader::new(file)))
    }

    /// Read the contents of a glTF buffer.
    fn buffer(&mut self, buffer: &json::buffer::Buffer) -> io::Result<Box<Read>> {
        let path = self.path.join(buffer.uri.as_ref().unwrap());
        let file = File::open(path)?;
        Ok(Box::new(BufReader::new(file)))
    }

    /// Read the contents of a glTF image.
    fn image(&mut self, image: &json::image::Image) -> io::Result<Box<Read>> {
        let path = self.path.join(image.uri.as_ref().unwrap());
        let file = File::open(path)?;
        Ok(Box::new(BufReader::new(file)))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Deserialize(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<Vec<validation::Error>> for Error {
    fn from(errs: Vec<validation::Error>) -> Error {
        Error::Validation(errs)
    }
}
