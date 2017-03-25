// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
mod macros;

pub mod extras;
pub mod traits;
pub mod v1;
pub mod v2;

pub use traits::Extras;

/// Error encountered when loading a glTF asset
#[derive(Debug)]
pub enum ImportError {
    /// Failure when deserializing a .gltf metadata file
    Deserialize(serde_json::error::Error),
    /// A glTF extension required by the asset has not been enabled by the user
    ExtensionDisabled(String),
    /// A glTF extension required by the asset is not supported by the library
    ExtensionUnsupported(String),
    /// The .gltf data is invalid
    Invalid(String),
    /// Standard input / output error
    Io(std::io::Error),
    /// The asset glTF version is not supported by the library
    VersionUnsupported(String),
}

/// Error encountered when converting a glTF asset from one version to another
#[derive(Clone, Debug)]
pub enum ConversionError {
    /// *Unimplemented*
    Unimplemented,
}

/// A imported glTF asset of generic version
pub enum Generic<E: Extras> {
    /// A 1.x.x conforming asset
    V1(v1::Root<E>),
    /// A 2.x.x conforming asset
    V2(v2::Root<E>),
}

/// glTF specification version x.x.x
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Version(u32, u32, u32);

/// Imports a glTF asset
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let path = "glTF-Sample-Models/1.0/Box/glTF/Box.gltf";
/// let gltf = gltf::import::<_, gltf::extras::None>(path)
///     .expect("Error importing glTF asset");
/// ```
pub fn import<P, E>(path: P) -> Result<Generic<E>, ImportError>
    where P: AsRef<std::path::Path>, E: Extras
{
    use std::io::Read;
    let mut file = std::fs::File::open(path).map_err(ImportError::Io)?;
    let mut json = String::new();
    let _ = file.read_to_string(&mut json).map_err(ImportError::Io)?;

    #[derive(Deserialize)]
    struct Asset {
        version: String,
    }

    #[derive(Deserialize)]
    struct Meta {
        asset: Asset,
        #[serde(default, rename = "extensionsRequired")]
        extensions_required: Vec<String>,
    }

    let meta = serde_json::from_str::<Meta>(&json)
        .map_err(ImportError::Deserialize)?; 
    let version =  {
        let mut iter = meta.asset.version
            .split(".")
            .filter_map(|s| s.parse().ok());
        if let Some(major) = iter.next() {
            let minor = iter.next().unwrap_or(0);
            let patch = iter.next().unwrap_or(0);
            Version(major, minor, patch)
        } else {
            return Err(ImportError::Invalid(format!("asset.version invalid")));
        }
    };
    match version {
        Version(1, 0, 0) | Version(1, 0, 1) => {
            for extension_name in &meta.extensions_required {
                if !v1::extensions::SUPPORTED_EXTENSION_NAMES.contains(&extension_name.as_str()) {
                    return Err(ImportError::ExtensionUnsupported(extension_name.clone()));
                } else if !v1::extensions::ENABLED_EXTENSION_NAMES.contains(&extension_name.as_str()) {
                    return Err(ImportError::ExtensionDisabled(extension_name.clone()));
                }
            }
            let root = v1::Root::import_from_str(&json)?;
            Ok(Generic::V1(root))
        }
        Version(2, 0, 0) => {
            for extension_name in &meta.extensions_required {
                if !v2::extensions::SUPPORTED_EXTENSION_NAMES.contains(&extension_name.as_str()) {
                    return Err(ImportError::ExtensionUnsupported(extension_name.clone()));
                } else if !v2::extensions::ENABLED_EXTENSION_NAMES.contains(&extension_name.as_str()) {
                    return Err(ImportError::ExtensionDisabled(extension_name.clone()));
                }
            }
            let root = v2::Root::import_from_str(&json)?;
            Ok(Generic::V2(root))
        }
        Version(major, minor, patch) => {
            let trio = format!("{}.{}.{}", major, minor, patch);
            Err(ImportError::VersionUnsupported(trio))
        }
    }
}

impl<E: Extras> Generic<E> {
    /// Converts an imported asset to a 1.0 conforming version
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let path = "glTF-Sample-Models/1.0/Box/glTF/Box.gltf";
    /// let gltf = gltf::import::<_, gltf::extras::None>(path)
    ///     .expect("Error importing glTF asset")
    ///     .to_v1()
    ///     .expect("Error converting asset to glTF version 1.0");
    /// ```
    pub fn to_v1(self) -> Result<v1::Root<E>, ConversionError> {
        match self {
            Generic::V1(root) => Ok(root),
            Generic::V2(_) => unimplemented!(),
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
    /// let gltf = gltf::import::<_, gltf::extras::None>(path)
    ///     .expect("Error loading glTF asset")
    ///     .to_v2()
    ///     .expect("Error converting asset to glTF version 2.0");
    /// ```
    pub fn to_v2(self) -> Result<v2::Root<E>, ConversionError> {
        match self {
            Generic::V1(_) => unimplemented!(),
            Generic::V2(root) => Ok(root),
        }
    }
}

