
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_json;
use std;
use v1::{Extras, Root};

/// Error encountered when loading a glTF asset.
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

/// Imports a binary glTF 1.0 asset.
#[cfg(feature = "KHR_binary_glTF")]
fn import_binary_gltf<S, E>(mut stream: S) -> Result<Root<E>, ImportError>
    where S: std::io::Read, E: Extras
{
    use v1::root::extensions::{KhrBinaryGltf, KhrBinaryGltfHeader};
    
    let header: KhrBinaryGltfHeader = unsafe {
        let mut buffer = [0u8; 16];
        stream.read_exact(&mut buffer[..])?;
        std::mem::transmute_copy(&buffer)
    };

    if header.version != 1 {
        let message = format!("KHR_binary_glTF version: {}", header.version);
        return Err(ImportError::IncompatibleVersion(message));
    }

    if header.content_format != 0 {
        let message = format!("KHR_binary_glTF contentFormat: {}",
                              header.content_format);
        return Err(ImportError::Invalid(message));
    }
    
    let mut content = Vec::with_capacity(header.content_length as usize);
    unsafe {
        content.set_len(header.content_length as usize);
    }
    stream.read_exact(&mut content[..])?;

    let body_length = header.length - header.content_length - 20;
    let mut body = Vec::with_capacity(body_length as usize);
    unsafe {
        body.set_len(body_length as usize);
    }
    stream.read_exact(&mut body[..])?;

    let mut root: Root<E> = serde_json::from_slice(&content)?;
    root.extensions.khr_binary_gltf = Some(KhrBinaryGltf {
        body: body,
        content: content,
        header: header,
    });
    
    Ok(root)
}

/// Imports a binary glTF 1.0 asset.
#[cfg(not(feature = "KHR_binary_glTF"))]
fn import_binary_gltf<S, E>(_stream: S) -> Result<Root<E>, ImportError>
    where S: std::io::Read, E: Extras
{  
    return Err(ImportError::ExtensionDisabled("KHR_binary_glTF".to_string()));
}

/// Imports a standard (plain text JSON) glTF 1.0 asset.
fn import_standard_gltf<E>(data: Vec<u8>) -> Result<Root<E>, ImportError>
    where E: Extras
{
    let root: Root<E> = serde_json::from_slice(&data)?;

    Ok(root)
}

/// Imports a glTF 1.0 asset.
pub fn import<P, E>(path: P) -> Result<Root<E>, ImportError>
    where P: AsRef<std::path::Path>, E: Extras
{
    use std::io::Read;
    
    let mut file = std::fs::File::open(path)?;
    
    let mut buffer = Vec::with_capacity(4);
    unsafe {
        buffer.set_len(4);
    }
    file.read_exact(&mut buffer)?;

    if buffer.starts_with(b"glTF") {
        import_binary_gltf(file)
    } else {
        file.read_to_end(&mut buffer)?;
        import_standard_gltf(buffer)
    }
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
