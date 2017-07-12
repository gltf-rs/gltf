
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures::future;
use import;
use std::{self, fmt};

use futures::{Future, Poll};
use std::boxed::Box;
use std::fs::File;
use std::io::BufReader;
use std::marker::Send;
use std::path::{Path, PathBuf};

/// Data source error.
#[derive(Debug)]
pub enum Error {
    /// Standard I/O error.
    Io(std::io::Error),
}

/// A simple data source that can read from the file system.
#[derive(Clone, Debug)]
pub struct FromPath {
    /// The path to the glTF directory.
    path: PathBuf,
}

impl FromPath {
    /// Constructs a simple data source that can read from the file system.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

/// A `Future` that drives the loading of external data on the file system.
struct ReadToEnd {
    /// The path of the file to be loaded.
    path: PathBuf,
}

impl Future for ReadToEnd {
    type Item = Box<[u8]>;
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // TODO: Actually make this async.
        use std::io::Read;
        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);
        let mut data = vec![];
        let _ = reader.read_to_end(&mut data)?;
        Ok(data.into_boxed_slice())
    }
}

impl import::Source for FromPath {
    type Error = Error;

    fn source_gltf(&self) -> Box<Future<Item = Box<[u8]>, Error = Self::Error>> {
        Box::new(ReadToEnd {
            path: self.path.clone(),
        })
    }

    fn source_external_data(&self, uri: &str) -> Box<Future<Item = Box<[u8]>, Error = Self::Error>> {
        Box::new(ReadToEnd {
            path: self.path.parent().unwrap_or(Path::new("./")).join(uri),
        })
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
