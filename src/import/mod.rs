
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//!
//! glTF JSON, buffers, and images often come from a wide range of external sources,
//! so customization is an important design goal of the import module. The `Source`
//! trait is provided to facilitate the user customization of the importer data
//! loading process.
//!
//! For convenience, the library contains one implementation of the `Source` trait
//! which allows for reading from the file system. This implemenation may be used as
//! reference for other schemes.
//!
//! ```
//! use std::path::Path;
//! use std::io::{self, BufReader, Read};
//! use std::fs::File;
//!
//! /// A simple synchronous data source that can read from the file system.
//! #[derive(Debug)]
//! struct SimpleSource<'a>(&'a Path);
//!
//! impl<'a> gltf::import::Source for SimpleSource<'a> {
//!     type Err = io::Error;
//!
//!     fn read_gltf(&self) -> Result<Box<Read>, Self::Err> {
//!         let file = File::open(&self.0)?;
//!         Ok(Box::new(BufReader::new(file)))
//!     }
//!
//!     fn read_external_data(&self, uri: &str) -> Result<Box<Read>, Self::Err> {
//!         let path = self.0.parent().unwrap_or(Path::new("./")).join(uri);
//!         let file = File::open(path)?;
//!         Ok(Box::new(BufReader::new(file)))
//!     }
//! }
//!
//! fn main() {
//!     let mut importer = gltf::Importer::new();
//!     let path = Path::new("glTF-Sample-Models/2.0/Box/glTF/Box.gltf");
//!     let source = SimpleSource(&path);
//!     match importer.import_from_source(source) {
//!         Ok(gltf) => println!("{:#?}", gltf.as_json()),
//!         Err(err) => println!("error: {:?}", err),
//!     }
//! }
//! ```

use gltf::Gltf;
use validation;
use serde_json;
use std;
use std::fmt;
use std::path::Path;

mod binary;
mod standard;

pub mod source;

pub use self::source::Source;

use self::source::FromPath;

/// Error encountered when importing a glTF 2.0 asset.
#[derive(Debug)]
pub enum Error<S: Source> {
    /// Failure when deserializing a .gltf metadata file.
    Deserialize(serde_json::error::Error),
    
    /// A glTF extension required by the asset has not been enabled by the user.
    ExtensionDisabled(String),
    
    /// A glTF extension required by the asset is not supported by the library.
    ExtensionUnsupported(String),
    
    /// The glTF version of the asset is incompatible with the importer.
    IncompatibleVersion(String),

    /// I/O error.
    Io(std::io::Error),
    
    /// Data source error.
    Source(S::Err),

    /// The .gltf data is invalid.
    Validation(Vec<validation::Error>),
}

/// Describes image data required to render a single glTF asset.
#[derive(Clone, Debug)]
pub enum Image {
    /// The image data is borrowed from a buffer.
    Borrowed(usize),

    /// The image data is owned.
    Owned(Vec<u8>),
}

/// Imports glTF 2.0.
#[derive(Clone)]
pub struct Importer {
    /// The binary glTF importer.
    binary: binary::Importer,

    /// The standard glTF importer.
    standard: standard::Importer,
}

impl Importer {
    /// Constructs an `Importer`.
    pub fn new() -> Self {
        Self {
            binary: binary::Importer::new(),
            standard: standard::Importer::new(),
        }
    }

    /// Imports some glTF from the given custom source.
    pub fn import_from_source<'a, S>(
        &'a mut self,
        source: S,
    ) -> Result<Gltf<'a>, Error<S>>
        where S: Source
    {
        use std::io::Read;

        let mut stream = source.read_gltf().map_err(Error::Source)?;
        let mut buffer = [0u8; 4];
        let _ = stream.read_exact(&mut buffer)?;
        if &buffer == b"glTF" {
            self.binary.import(buffer.chain(stream), source)
        } else {
            self.standard.import(buffer.chain(stream), source)
        }
    }
    
    /// Import some glTF 2.0 from the file system.
    pub fn import_from_path<'a, P>(
        &'a mut self,
        path: P,
    ) -> Result<Gltf<'a>, Error<FromPath>>
        where P: AsRef<Path>
    {
        self.import_from_source(FromPath::new(path))
    }
}

impl<S: Source> From<serde_json::Error> for Error<S> {
    fn from(err: serde_json::Error) -> Error<S> {
        Error::Deserialize(err)
    }
}

impl<S: Source> From<std::io::Error> for Error<S> {
    fn from(err: std::io::Error) -> Error<S> {
        Error::Io(err)
    }
}

impl<S: Source> From<Vec<validation::Error>> for Error<S> {
    fn from(errs: Vec<validation::Error>) -> Error<S> {
        Error::Validation(errs)
    }
}

impl<S: Source> fmt::Display for Error<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl<S: Source> std::error::Error for Error<S> {
    fn description(&self) -> &str {
        use self::Error::*;
        match self {
            &Deserialize(_) => "Malformed .gltf / .glb file",
            &ExtensionDisabled(_) => "Asset requires a disabled extension",
            &ExtensionUnsupported(_) => "Assets requires an unsupported extension",
            &IncompatibleVersion(_) => "Asset is not glTF version 2.0",
            &Io(_) => "I/O error",
            &Source(_) => "Data source error",
            &Validation(_) => "Asset failed validation tests",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        use self::Error::*;
        match self {
            &Deserialize(ref err) => Some(err),
            &Io(ref err) => Some(err),
            &Source(ref err) => Some(err),
            _ => None,
        }
    }
}
