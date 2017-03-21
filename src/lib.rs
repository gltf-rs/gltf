
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

pub mod v1;
pub mod v2;

/// Error encountered when loading a glTF asset
#[derive(Debug)]
pub enum ImportError {
    /// Failure when deserializing a .gltf metadata file
    Deserialize(serde_json::error::Error),
    /// The .gltf data is invalid
    Invalid(String),
    /// Standard input / output error
    Io(std::io::Error),
    /// glTF version is not supported by the library
    Unsupported(Version),
}

/// Error encountered when converting a glTF asset from one version to another
#[derive(Clone, Debug)]
pub enum ConversionError {
    /// *Unimplemented*
    Unimplemented,
}

/// Return value of `load()`
#[allow(dead_code)]
pub enum Data {
    /// glTF version 1.0
    V1(v1::Root),
    /// glTF version 2.0
    V2(v2::Root),
}

/// An imported glTF asset
pub struct Gltf {
    /// The version of the glTF specification this asset conforms to
    pub version: Version,
    /// The asset data
    pub data: Data,
}

/// glTF version x.x.x
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Version(u32, u32, u32);

/// Attempts to parse the `asset.version` field of a .gltf file
fn detect_version(json: &str) -> Result<Version, String> {
    #[derive(Deserialize)]
    struct Asset {
        version: String,
    }

    #[derive(Deserialize)]
    struct Root {
        asset: Asset,
    }

    match serde_json::from_str::<Root>(&json) {
        Ok(root) => {
            let mut iter = root.asset
                .version
                .split(".")
                .filter_map(|s| s.parse().ok());
            let major = iter.next();
            let minor = iter.next().unwrap_or(0);
            let patch = iter.next().unwrap_or(0);
            match major {
                Some(n) => Ok(Version(n, minor, patch)),
                None => Err(format!("asset.version \"{}\" invalid",
                                    root.asset.version.to_owned())),
            }
        },
        Err(_) => Err("asset.version field missing".to_string()),
    }
}

impl Data {
    /// Converts an imported asset to a 1.0 conforming version
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let path = "glTF-Sample-Models/1.0/Box/glTF/Box.gltf";
    /// let gltf = gltf::Gltf::import(path)
    ///     .expect("Error importing glTF asset")
    ///     .data.to_v1()
    ///     .expect("Error converting asset to glTF version 1.0");
    /// ```
    pub fn to_v1(self) -> Result<v1::Root, ConversionError> {
        match self {
            Data::V1(root) => Ok(root),
            Data::V2(_) => unimplemented!(),
        }
    }

    /// Converts a loaded asset to a 2.0 conforming version
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let path = "glTF-Sample-Models/2.0/BoomBox/glTF/BoomBox.gltf";
    /// let gltf = gltf::Gltf::import(path)
    ///     .expect("Error loading glTF asset")
    ///     .data.to_v2()
    ///     .expect("Error converting asset to glTF version 2.0");
    /// ```
    pub fn to_v2(self) -> Result<v2::Root, ConversionError> {
        match self {
            Data::V1(_) => unimplemented!(),
            Data::V2(root) => Ok(root),
        }
    }
}

impl Gltf {
    /// Imports a glTF asset
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let path = "glTF-Sample-Models/1.0/Box/glTF/Box.gltf";
    /// let gltf = gltf::Gltf::import(path).expect("Error importing glTF asset");
    /// ```
    pub fn import<P>(path: P) -> Result<Self, ImportError>
        where P: AsRef<std::path::Path>
    {
        use std::io::Read;
        let mut file = std::fs::File::open(path).map_err(ImportError::Io)?;
        let mut json = String::new();
        let _ = file.read_to_string(&mut json).map_err(ImportError::Io)?;
        match detect_version(&json) {
            Ok(Version(1, 0, 0)) => {
                let root = v1::Root::import_from_str(&json)?;
                let gltf = Gltf {
                    version: Version(1, 0, 0),
                    data: Data::V1(root),
                };
                Ok(gltf)
            }
            Ok(Version(2, 0, 0)) => {
                let root = v2::Root::import_from_str(&json)?;
                let gltf = Gltf {
                    version: Version(2, 0, 0),
                    data: Data::V2(root),
                };
                Ok(gltf)
            }
            Ok(other) => Err(ImportError::Unsupported(other)),
            Err(err) => Err(ImportError::Invalid(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_version_100() {
        let json = "{\"asset\":{\"version\":\"1.0.0\"}}";
        assert_eq!(detect_version(&json), Ok(Version(1, 0, 0)));
    }

    #[test]
    fn detect_version_200() {
        let json = "{\"asset\":{\"version\":\"2.0.0\"}}";
        assert_eq!(detect_version(&json), Ok(Version(2, 0, 0)));
    }

    #[test]
    fn detect_missing_version() {
        let json = "{}";
        assert_eq!(detect_version(&json),
                   Err("asset.version field missing".to_string()));
    }

    #[test]
    fn allow_extra_fields() {
        let json = "{\"asset\":{\"version\":\"1.0.0\",\"foo\":{}},\"bar\":{}}";
        assert_eq!(detect_version(&json), Ok(Version(1, 0, 0)));
    }
}
