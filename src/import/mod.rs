
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
//! For convenience, the library contains one implementation of the `Source` trait,
//! namely `FromPath`, which allows for reading from the file system.
//!
//! This implementation may be used as reference.

use futures::future;
use json;
use validation;
use serde_json;
use std::{self, fmt};

use image_crate::ImageError;
use futures::{BoxFuture, Future, Poll};
use gltf::Gltf;
use std::boxed::Box;
use std::path::Path;

/// Contains the implementation of the binary glTF importer.
mod binary;

/// Contains the implementation of the standard glTF importer.
mod standard;

/// Contains data structures for import configuration.
pub mod config;

/// Contains the `Source` trait and its reference implementation.
pub mod source;

pub use self::config::Config;
pub use self::source::{FromPath, Source};

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

/// A `Future` that drives the importation of glTF.
pub struct Import {
    /// Internal `Future`.
    future: BoxFuture<Gltf, Error>,
}

/// Imports some glTF from the given custom source.
fn import<S: Source>(source: S, config: Config) -> BoxFuture<Gltf, Error> {
    let future = source
        .source_gltf()
        .and_then(move |data| {
            if data.starts_with(b"glTF") {
                binary::import(data, source, config)
            } else {
                standard::import(data, source, config)
            }
        });
    Box::new(future)
}

impl Import {
    /// Constructs an `Import` from a custom `Source` and `Config` arguments.
    pub fn custom<S: Source>(source: S, config: Config) -> Self {
        Import {
            future: import(source, config),
        }
    }

    /// Constructs an `Import` with `FromPath` as its data source and default
    /// configuration parameters. 
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Import::custom(FromPath::new(path), Default::default())
    }

    /// Drives the import process to completion. Blocks the current thread until
    /// the process is complete.
    pub fn sync(self) -> Result<Gltf, Error> {
        self.wait()
    }
}

impl Future for Import {
    type Item = Gltf;
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
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
