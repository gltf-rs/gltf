
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
    /// Standard input / output error
    Io(std::io::Error),
    /// Failure when deserializing a .gltf metadata file
    De(serde_json::error::Error),
}

/// Error encountered when converting a glTF asset from one version to another
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

/// Loads a glTF asset
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let gltf = gltf::load("./examples/box/Box.gltf")
///     .expect("Error loading glTF asset");
/// ```
pub fn load<P>(path: P) -> Result<Dynamic, LoadError>
    where P: AsRef<std::path::Path>
{
    // TODO: Check version and return version 1.0 or 2.0 accordingly
    let root = v100::Root::load(path)?;
    Ok(Dynamic::Version100(root))
}

impl Dynamic {
    pub fn to_version_100(self) -> Result<v100::Root, ConversionError> {
        match self {
            Dynamic::Version100(root) => Ok(root),
            Dynamic::Version200(_) => unimplemented!(),
        }
    }

    pub fn to_version_200(self) -> Result<v200::Root, ConversionError> {
        match self {
            Dynamic::Version100(_) => unimplemented!(),
            Dynamic::Version200(root) => Ok(root),
        }
    }
}

