
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
mod macros;

pub mod v100;
pub mod v200;

/// Error encountered when loading a glTF asset
#[derive(Debug)]
pub enum LoadError {
    /// Failure when deserializing a .gltf metadata file
    De(serde_json::error::Error),
    /// An index was found to be out of range
    InvalidIndices,
    /// Standard input / output error
    Io(std::io::Error),
    /// glTF version is not supported by the library
    VersionUnsupported(String),
}

/// Error encountered when converting a glTF asset from one version to another
#[derive(Clone, Debug)]
pub enum ConversionError {
    /// *Unimplemented*
    Incomplete,
}

/// Return value of `load()`
#[allow(dead_code)]
pub enum Dynamic {
    /// glTF version 1.0
    Version100(v100::Root),
    /// glTF version 2.0
    Version200(v200::Root),
}

/// Return value of `detect_version()`
#[derive(Debug, Eq, PartialEq)]
enum GltfVersion {
    /// glTF version 1.0
    V100,
    /// glTF version 2.0
    V200,
}

/// Attempts to extract the `asset.version` field of the .gltf file
fn detect_gltf_version(json: &str) -> Result<GltfVersion, String> {
    #[derive(Deserialize)]
    struct Asset {
        version: String,
    }
    
    #[derive(Deserialize)]
    struct Root {
        asset: Asset,
    }

    let root: Root = serde_json::from_str(&json)
        .map_err(|_| "asset.version field missing".to_string())?;
    match root.asset.version.as_str() {
        "1.0" | "1.0.0" => Ok(GltfVersion::V100),
        "2.0" | "2.0.0" => Ok(GltfVersion::V200),
        unsupported_version => Err(unsupported_version.to_string()),
    }
}

/// Loads a glTF asset
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let gltf = gltf::load("glTF-Sample-Models/1.0/Box/glTF/Box.gltf")
///     .expect("Error loading glTF asset");
/// ```
pub fn load<P>(path: P) -> Result<Dynamic, LoadError>
    where P: AsRef<std::path::Path>
{
    use std::io::Read;
    let mut file = std::fs::File::open(path).map_err(LoadError::Io)?;
    let mut json = String::new();
    let _ = file.read_to_string(&mut json).map_err(LoadError::Io)?;
    match detect_gltf_version(&json) {
        Ok(GltfVersion::V100) => {
            let root = v100::Root::load_from_str(&json)?;
            Ok(Dynamic::Version100(root))
        },
        Ok(GltfVersion::V200) => {
            let root = v200::Root::load_from_str(&json)?;
            Ok(Dynamic::Version200(root))
        },
        Err(version) => Err(LoadError::VersionUnsupported(version.clone())),
    }
}

impl Dynamic {
    /// Converts a loaded asset to a 1.0 conforming version
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let gltf = gltf::load("glTF-Sample-Models/1.0/Box/glTF/Box.gltf")
    ///                .expect("Error loading glTF asset")
    ///                .to_version_100()
    ///                .expect("Error converting asset to glTF version 1.0");
    /// ```
    pub fn to_version_100(self) -> Result<v100::Root, ConversionError> {
        match self {
            Dynamic::Version100(root) => Ok(root),
            Dynamic::Version200(_) => unimplemented!(),
        }
    }

    /// Converts a loaded asset to a 2.0 conforming version
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let gltf = gltf::load("glTF-Sample-Models/2.0/BoomBox/glTF/BoomBox.gltf")
    ///                .expect("Error loading glTF asset")
    ///                .to_version_200()
    ///                .expect("Error converting asset to glTF version 2.0");
    /// ```
    pub fn to_version_200(self) -> Result<v200::Root, ConversionError> {
        match self {
            Dynamic::Version100(_) => unimplemented!(),
            Dynamic::Version200(root) => Ok(root),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn detect_version_100() {
        let json = "{\"asset\":{\"version\":\"1.0.0\"}}";
        assert_eq!(detect_gltf_version(&json), Ok(GltfVersion::V100));
    }

    #[test]
    fn detect_version_200() {
        let json = "{\"asset\":{\"version\":\"2.0.0\"}}";
        assert_eq!(detect_gltf_version(&json), Ok(GltfVersion::V200));
    }

    #[test]
    fn detect_unsupported_version() {
        let json = "{\"asset\":{\"version\":\"3.1.4\"}}";
        assert_eq!(detect_gltf_version(&json), Err("3.1.4".to_string()));
    }
    
    #[test]
    fn detect_missing_version() {
        let json = "{}";
        assert_eq!(detect_gltf_version(&json),
                   Err("asset.version field missing".to_string()));
    }

    #[test]
    fn allow_extra_fields() {
        let json = "{\"asset\":{\"version\":\"1.0.0\",\"foo\":{}},\"bar\":{}}";
        assert_eq!(detect_gltf_version(&json), Ok(GltfVersion::V100));
    }
}

