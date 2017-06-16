
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
use std::path::{Path, PathBuf};
use json;
use gltf::Gltf;
use root::Root;
use validation;

/// Return type of `Importer::import_*`.
pub type Result<'a> = std::result::Result<Gltf<'a>, Error>;

/// A trait for representing sources of glTF data that may be read by an importer.
pub trait Source {
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
struct FromPath {
    /// The path to the glTF directory.
    path: PathBuf,
}

/// Describes image data required to render a single glTF asset.
#[derive(Clone, Debug)]
pub enum ImageData {
    /// The image data is borrowed from a buffer.
    Borrowed(usize),

    /// The image data is owned.
    Owned(Vec<u8>),
}

/// Imports glTF 2.0.
#[derive(Clone)]
pub struct Importer {
    /// The glTF JSON data.
    gltf: Vec<u8>,

    /// The imported glTF buffers.
    buffers: Vec<Vec<u8>>,

    /// The imported glTF images.
    images: Vec<ImageData>,
}

impl Importer {
    /// Constructs an `Importer`.
    pub fn new() -> Self {
        Self {
            buffers: vec![],
            images: vec![],
            gltf: vec![],
        }
    }

    /// Clears any data held by the importer.
    /// Must be called at the beginning of each import call.
    fn clear(&mut self) {
        self.buffers.clear();
        self.images.clear();
        self.gltf.clear();
    }

    /// Imports some glTF from the given custom source.
    pub fn import_from_source<'a, S>(&'a mut self, mut source: S) -> Result<'a>
        where S: Source
    {
        use std::io::Read;
        use self::Error::*;
        use self::validation::{JsonPath, Validate};

        // Cleanup from the last import call
        self.clear();
        
        // Read .gltf / .glb file
        let _ = source.gltf()?.read_to_end(&mut self.gltf)?;
        if self.gltf.starts_with(b"glTF") {
            return Err(ExtensionUnsupported("KHR_binary_glTF".to_string()));
        }

        // Parse and validate the .gltf JSON data
        let json: json::Root = serde_json::from_slice(&self.gltf)?;
        let mut errs = Vec::new();
        json.validate(&json, || JsonPath::new(), &mut |err| errs.push(err));
        if !errs.is_empty() {
            return Err(Validation(errs));
        }

        // Read the glTF buffer data
        for entry in &json.buffers {
            let mut data = vec![];
            let _ = source.buffer(entry)?.read_to_end(&mut data)?;
            self.buffers.push(data);
        }

        // Read the glTF image data
        for entry in &json.images {
            let image = if let Some(buffer_view) = entry.buffer_view.as_ref() {
                ImageData::Borrowed(buffer_view.value())
            } else {
                let mut buffer = vec![];
                let _ = source.image(entry)?.read_to_end(&mut buffer)?;
                ImageData::Owned(buffer)
            };
            self.images.push(image);
        }

        let buffer_data: Vec<_>  = self.buffers.iter().map(Vec::as_slice).collect();
        let mut image_data = vec![];
        for entry in &self.images {
            let slice = match entry {
                &ImageData::Borrowed(index) => self.buffers[index].as_slice(),
                &ImageData::Owned(ref data) => data.as_slice(),
            };
            image_data.push(slice);
        }
        Ok(Gltf::new(Root::new(json), buffer_data, image_data))

    }
    
    /// Import some glTF 2.0 from the file system.
    pub fn import_from_path<'a, P>(&'a mut self, path: P) -> Result<'a>
        where P: AsRef<Path>
    {
        self.import_from_source(FromPath::new(path))
    }
}

impl FromPath {
    /// Constructs a simple synchronous data source that can read from the file
    /// system.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Source for FromPath {
    fn gltf(&mut self) -> io::Result<Box<Read>> {
        let file = File::open(&self.path)?;
        Ok(Box::new(BufReader::new(file)))
    }

    fn buffer(&mut self, buffer: &json::buffer::Buffer) -> io::Result<Box<Read>> {
        let uri = buffer.uri.as_ref().unwrap().as_ref();
        let path = self.path.parent().unwrap().join(uri);
        let file = File::open(path)?;
        Ok(Box::new(BufReader::new(file)))
    }

    fn image(&mut self, image: &json::image::Image) -> io::Result<Box<Read>> {
        let uri = image.uri.as_ref().unwrap().as_ref();
        let path = self.path.parent().unwrap().join(uri);
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
