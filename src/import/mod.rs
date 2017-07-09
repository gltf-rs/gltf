
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
//! ```no-run
//! use std::path::Path;
//! use std::io::{self, BufReader, Read};
//! use std::fs::File;
//!
//! /// A simple synchronous data source that can read from the file system.
//! #[derive(Debug)]
//! struct SimpleSource<'a>(&'a Path);
//!
//! impl<'a> gltf::import::Source for SimpleSource<'a> {
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
//!     let path = Path::new("glTF-Sample-Models/2.0/Box/glTF/Box.gltf");
//!     let source = SimpleSource(&path);
//!     match gltf::import::from_source(source) {
//!         Ok(gltf) => println!("{:#?}", gltf.as_json()),
//!         Err(err) => println!("error: {:?}", err),
//!     }
//! }
//! ```

use futures::future;
use json;
use validation;
use serde_json;
use std::{self, fmt};

use image_crate::ImageError;
use futures::{BoxFuture, Future};
use gltf::Gltf;
use std::boxed::Box;
use std::path::Path;

mod binary;
mod standard;

/// Contains the `Source` trait and its reference implementation.
pub mod source;

pub use self::source::Source;

use self::source::FromPath;

/// Error encountered when importing a glTF 2.0 asset.
#[derive(Debug)]
pub enum Error {
    /// A glTF image could not be decoded.
    Decode(ImageError),

    /// A glTF extension required by the asset has not been enabled by the user.
    ExtensionDisabled(String),
    
    /// A glTF extension required by the asset is not supported by the library.
    ExtensionUnsupported(String),
    
    /// The glTF version of the asset is incompatible with the importer.
    IncompatibleVersion(String),

    /// Standard I/O error.
    Io(std::io::Error),
    
    /// Failure when parsing a .glb file.
    MalformedGlb(String),

    /// Failure when deserializing .gltf or .glb JSON.
    MalformedJson(serde_json::error::Error),
    
    /// Data source error.
    Source(source::Error),

    /// Error upon lazy loading data.
    LazyLoading(future::SharedError<Error>),
    
    /// The .gltf data is invalid.
    Validation(Vec<(json::Path, validation::Error)>),
}

/// Import some glTF 2.0 synchronously from the file system.
pub fn from_path_sync<P>(path: P) -> Result<Gltf, Error>
    where P: AsRef<Path>
{
    from_path(path).wait()
}

/// Imports some glTF from the given custom source.
pub fn from_source<S: Source>(source: S) -> BoxFuture<Gltf, Error> {
    let future = source
        .source_gltf()
        .and_then(move |data| {
            if data.starts_with(b"glTF") {
                binary::import(data, source)
            } else {
                standard::import(data, source)
            }
        });
    Box::new(future)
}

/// Import some glTF 2.0 from the file system.
pub fn from_path<P>(path: P) -> BoxFuture<Gltf, Error>
    where P: AsRef<Path>
{
    from_source(FromPath::new(path))
}

impl From<ImageError> for Error {
    fn from(err: ImageError) -> Error {
        Error::Decode(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::MalformedJson(err)
    }
}

impl From<future::SharedError<Error>> for Error {
    fn from(err: future::SharedError<Error>) -> Error {
        Error::LazyLoading(err)
    }
}

impl From<source::Error> for Error {
    fn from(err: source::Error) -> Error {
        Error::Source(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<Vec<(json::Path, validation::Error)>> for Error {
    fn from(errs: Vec<(json::Path, validation::Error)>) -> Error {
        Error::Validation(errs)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match self {
            &Decode(_) => "Image decoding failed",
            &ExtensionDisabled(_) => "Asset requires a disabled extension",
            &ExtensionUnsupported(_) => "Assets requires an unsupported extension",
            &IncompatibleVersion(_) => "Asset is not glTF version 2.0",
            &Io(_) => "I/O error",
            &LazyLoading(_) => "Lazy loading",
            &MalformedGlb(_) => "Malformed .glb file",
            &MalformedJson(_) => "Malformed .gltf / .glb JSON",
            &Source(_) => "Data source error",
            &Validation(_) => "Asset failed validation tests",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        use self::Error::*;
        match self {
            &MalformedJson(ref err) => Some(err),
            &Io(ref err) => Some(err),
            _ => None,
        }
    }
}
