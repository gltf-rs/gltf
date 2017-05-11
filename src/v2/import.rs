
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
use v2::Root;

/// Error encountered when importing a glTF 2.0 asset.
#[derive(Debug)]
pub enum ImportError {
    /// Failure when deserializing a .gltf metadata file.
    Deserialize(serde_json::error::Error),
    
    /// A glTF extension required by the asset has not been enabled by the user.
    ExtensionDisabled(String),
    
    /// A glTF extension required by the asset is not supported by the library.
    ExtensionUnsupported(String),
    
    /// The .gltf data is invalid.
    Invalid(String),
    
    /// Standard input / output error.
    Io(std::io::Error),
    
    /// The glTF version of the asset is incompatible with this function.
    IncompatibleVersion(String),
}

/// Imports a standard (plain text JSON) glTF 2.0 asset.
fn import_standard_gltf(data: Vec<u8>) -> Result<Root, ImportError> {
    let root: Root = serde_json::from_slice(&data)?;
    Ok(root)
}

fn import_impl(path: &Path) -> Result<Root, ImportError> {
    use std::io::Read;
    use self::ImportError::*;
    
    let mut file = std::fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let root: Root = if buffer.starts_with(b"glTF") {
        return Err(ExtensionUnsupported("KHR_binary_glTF (2.0)".to_string()));
    } else {
        file.read_to_end(&mut buffer)?;
        import_standard_gltf(buffer)?
    };

    if root.range_check().is_ok() {
        Ok(root)
    } else {
        Err(Invalid("index out of range".to_string()))
    }
}

/// Imports a glTF 2.0 asset.
pub fn import<P: AsRef<Path>>(path: P) -> Result<Root, ImportError> {
    import_impl(path.as_ref())
}

impl From<serde_json::Error> for ImportError {
    fn from(err: serde_json::Error) -> ImportError {
        ImportError::Deserialize(err)
    }
}

impl From<std::io::Error> for ImportError {
    fn from(err: std::io::Error) -> ImportError {
        ImportError::Io(err)
    }
}

