
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
use std::fmt::Debug;
use std::path::Path;

/// Contains the implementation of the binary glTF importer.
mod binary;

/// Contains the implementation of the standard glTF importer.
mod standard;

/// Contains data structures for import configuration.
pub mod config;

/// Contains import data.
pub mod data;

/// Contains the reference `Source` implementation, namely `FromPath`.
pub mod from_path;

pub use self::config::Config;
pub use self::data::Data;
pub use self::from_path::FromPath;

/// A trait for representing sources of glTF data that may be read by an importer.
pub trait Source: Debug + Send + Sync + 'static {
    /// User error type.
    type Error: std::error::Error + Send + Sync;

    /// Read the contents of a .gltf or .glb file.
    fn source_gltf(&self) -> BoxFuture<Box<[u8]>, Self::Error>;

    /// Read the contents of external data.
    fn source_external_data(&self, uri: &str) -> BoxFuture<Box<[u8]>, Self::Error>;
}

/// Error encountered when importing a glTF 2.0 asset.
#[derive(Debug)]
pub enum Error<S: Source> {
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
    Shared(future::SharedError<Error<S>>),
    
    /// Data source error.
    Source(S::Error),

    /// The .gltf data is invalid.
    Validation(Vec<(json::Path, validation::Error)>),
}

/// A `Future` that drives the importation of glTF.
pub struct Import<S: Source>(BoxFuture<Gltf, Error<S>>);

impl<S: Source> Import<S> {
    /// Constructs an `Import` from a custom `Source` and `Config` arguments.
    pub fn custom(source: S, config: Config) -> Self {
        let future = source
            .source_gltf()
            .map_err(Error::Source)
            .and_then(move |data| {
                if data.starts_with(b"glTF") {
                    binary::import(data, source, config)
                } else {
                    standard::import(data, source, config)
                }
            })
            .boxed();
        Import(future)
    }

    /// Drives the import process to completion. Blocks the current thread until
    /// the process is complete.
    pub fn sync(self) -> Result<Gltf, Error<S>> {
        self.wait()
    }
}

impl Import<FromPath> {
    /// Constructs an `Import` with `FromPath` as its data source and default
    /// configuration parameters. 
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Import::custom(FromPath::new(path), Default::default())
    }
}

impl<S: Source> Future for Import<S> {
    type Item = Gltf;
    type Error = Error<S>;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

impl<S: Source> From<ImageError> for Error<S> {
    fn from(err: ImageError) -> Error<S> {
        Error::Decode(err)
    }
}

impl<S: Source> From<serde_json::Error> for Error<S> {
    fn from(err: serde_json::Error) -> Error<S> {
        Error::MalformedJson(err)
    }
}

impl<S: Source> From<std::io::Error> for Error<S> {
    fn from(err: std::io::Error) -> Error<S> {
        Error::Io(err)
    }
}

impl<S: Source> From<future::SharedError<Error<S>>> for Error<S> {
    fn from(err: future::SharedError<Error<S>>) -> Error<S> {
        Error::Shared(err)
    }
}

impl<S: Source> From<Vec<(json::Path, validation::Error)>> for Error<S> {
    fn from(errs: Vec<(json::Path, validation::Error)>) -> Error<S> {
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
            &Decode(_) => "Image decoding failed",
            &ExtensionDisabled(_) => "Asset requires a disabled extension",
            &ExtensionUnsupported(_) => "Assets requires an unsupported extension",
            &IncompatibleVersion(_) => "Asset is not glTF version 2.0",
            &Io(_) => "I/O error",
            &MalformedGlb(_) => "Malformed .glb file",
            &MalformedJson(_) => "Malformed .gltf / .glb JSON",
            &Source(_) => "Data source error",
            &Shared(_) => "Shared error",
            &Validation(_) => "Asset failed validation tests",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        use self::Error::*;
        match self {
            &MalformedJson(ref err) => Some(err),
            &Io(ref err) => Some(err),
            &Source(ref err) => Some(err),
            _ => None,
        }
    }
}
