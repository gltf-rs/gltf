
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_json;
use std;
use std::path::Path;
use json::Root;
use validation;

/// Error encountered when importing a glTF 2.0 asset.
#[derive(Debug)]
pub enum Error {
    /// Failure when deserializing a .gltf metadata file.
    Deserialize(serde_json::error::Error),
    
    /// A glTF extension required by the asset has not been enabled by the user.
    ExtensionDisabled(String),
    
    /// A glTF extension required by the asset is not supported by the library.
    ExtensionUnsupported(String),
    
    /// The .gltf data is invalid.
    Validation(Vec<validation::Error>),
    
    /// Standard input / output error.
    Io(std::io::Error),
    
    /// The glTF version of the asset is incompatible with this function.
    IncompatibleVersion(String),
}

/// Imports a standard (plain text JSON) glTF 2.0 asset.
fn import_standard_gltf(data: Vec<u8>) -> Result<Root, Error> {
    let root: Root = serde_json::from_slice(&data)?;
    Ok(root)
}

fn import_impl(path: &Path) -> Result<Root, Error> {
    use std::io::Read;
    use self::Error::*;
    use self::validation::{JsonPath, Validate};
    
    let mut file = std::fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let root: Root = if buffer.starts_with(b"glTF") {
        return Err(ExtensionUnsupported("KHR_binary_glTF".to_string()));
    } else {
        file.read_to_end(&mut buffer)?;
        import_standard_gltf(buffer)?
    };

    let mut errs = Vec::new();
    root.validate(&root, || JsonPath::new(), &mut |err| errs.push(err));
    if errs.is_empty() {
        Ok(root)
    } else {
        Err(Validation(errs))
    }
}

/// Imports a glTF 2.0 asset.
pub fn import<P: AsRef<Path>>(path: P) -> Result<Root, Error> {
    import_impl(path.as_ref())
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
