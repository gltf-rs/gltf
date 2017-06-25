
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{error, fmt};

use std::boxed::Box;
use std::fs::File;
use std::io::{BufReader, Read};

pub use self::from_path::FromPath;

/// A trait for representing sources of glTF data that may be read by an importer.
pub trait Source: fmt::Debug {
    /// Error type.
    type Err: error::Error + fmt::Debug;
    
    /// Read the contents of a .gltf or .glb file.
    fn read_gltf(&self) -> Result<Box<Read>, Self::Err>;

    /// Read the contents of external data.
    fn read_external_data(&self, uri: &str) -> Result<Box<Read>, Self::Err>;
}

pub mod from_path {
    use super::*;
    use std::{self, fmt};

    use std::path::{Path, PathBuf};

    /// Possible runtime error for the `FromPath` data source.
    #[derive(Debug)]
    pub enum Error {
        /// I/O error.
        Io(std::io::Error),

        /// The URI scheme for a buffer or image is not supported by this `Source`.
        UnsupportedScheme,
    }

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

    impl std::error::Error for Error {
        fn description(&self) -> &str {
            match self {
                &Error::Io(_) => "I/O error",
                &Error::UnsupportedScheme => "Unsupported scheme",
            }
        }

        fn cause(&self) -> Option<&std::error::Error> {
            match self {
                &Error::Io(ref err) => Some(err),
                _ => None,
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
    
    impl Source for FromPath {
        type Err = Error;

        fn read_gltf(&self) -> Result<Box<Read>, Self::Err> {
            let file = File::open(&self.path)?;
            Ok(Box::new(BufReader::new(file)))
        }

        fn read_external_data(&self, uri: &str) -> Result<Box<Read>, Self::Err> {
            let path = self.path.parent().unwrap_or(Path::new("./")).join(uri);
            let file = File::open(path)?;
            Ok(Box::new(BufReader::new(file)))
        }
    }
}
