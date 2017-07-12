
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base64;
use futures::future;
use import;
use std::{self, fmt};

use futures::BoxFuture;
use std::boxed::Box;
use std::fs::File;
use std::io::BufReader;
use std::marker::Send;

pub use self::from_path::FromPath;

/// Data source error.
#[derive(Debug)]
pub enum Error {
    /// Standard I/O error.
    Io(std::io::Error),

    /// Base64 decoding error.
    Base64(base64::DecodeError),
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
            let _ = reader
                .read_to_end(&mut data)
                .map_err(|err| import::Error::Source(Error::Io(err)))?;
            Ok(data.into_boxed_slice())
        });
        Box::new(future)
    }

    fn decode_base64(stream: Vec<u8>) -> BoxFuture<Box<[u8]>, import::Error> {
        let future = future::lazy(move || {
            let data = base64::decode(&stream)
                .map_err(|err| import::Error::Source(Error::Base64(err)))?
                .into_boxed_slice();
            Ok(data)
        });
        Box::new(future)
    }
    
    impl Source for FromPath {
        fn source_gltf(&self) -> BoxFuture<Box<[u8]>, import::Error> {
            read_to_end(self.path.to_path_buf())
        }

        fn source_external_data(&self, uri: &str) -> BoxFuture<Box<[u8]>, import::Error> {
            let data_scheme = "data:application/octet-stream;base64,";
            if uri.starts_with(data_scheme) {
                let stream = uri[data_scheme.len()..].as_bytes().to_vec();
                decode_base64(stream)
            } else {
                let path = self.path.parent().unwrap_or(Path::new("./")).join(uri);
                read_to_end(path)
            }
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Base64(_) => "base64 decoding",
            &Error::Io(_) => "I/O error",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            &Error::Base64(ref err) => Some(err),
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
