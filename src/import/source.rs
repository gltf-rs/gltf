
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures::future;
use futures::BoxFuture;

use import;

use std;
use std::boxed::Box;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::marker::Send;

pub use self::from_path::FromPath;

/// A trait for representing sources of glTF data that may be read by an importer.
pub trait Source: Send + 'static {
    /// Read the contents of a .gltf or .glb file.
    fn source_gltf(&self) -> BoxFuture<Box<[u8]>, import::Error>;

    /// Read the contents of external data.
    fn source_external_data(&self, uri: &str) -> BoxFuture<Box<[u8]>, import::Error>;
}

/// Data source error.
#[derive(Debug)]
pub enum Error {
    /// Standard I/O error.
    Io(std::io::Error),
}

/// Contains the reference `Source` implementation.
pub mod from_path {
    use super::*;
    use std::path::{Path, PathBuf};

    /// A simple synchronous data source that can read from the file system.
    #[derive(Clone, Debug)]
    pub struct FromPath {
        /// The path to the glTF directory.
        path: PathBuf,
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

    fn read_to_end(path: PathBuf) -> BoxFuture<Box<[u8]>, import::Error> {
        let future = future::lazy(move || {
            use std::io::Read;
            // TODO: Actually make this async.
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            let mut data = vec![];
            let _ = reader.read_to_end(&mut data)?;
            Ok(data.into_boxed_slice())
        });
        Box::new(future)
    }

    impl Source for FromPath {
        fn source_gltf(&self) -> BoxFuture<Box<[u8]>, import::Error> {
            read_to_end(self.path.to_path_buf())
        }

        fn source_external_data(&self, uri: &str) -> BoxFuture<Box<[u8]>, import::Error> {
            let path = self.path.parent().unwrap_or(Path::new("./")).join(uri);
            read_to_end(path)
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Io(_) => "I/O error",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            &Error::Io(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

