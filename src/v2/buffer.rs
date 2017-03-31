
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Extensions;
use traits::Extras;
use v2::{Index, Root};

/// [The identifier of the `BufferView` this accessor reads from.
/// Describes the location, type, and size of a binary blob included with the asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#buffer)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Buffer<E: Extras> {
    /// The length of the buffer in bytes
    #[serde(default, rename = "byteLength")]
    pub byte_length: u32,
    
    /// Optional user-defined name for this object
    pub name: Option<String>,
    
    /// Uniform resource locator for the buffer data relative to the .gltf file
    // N.B. the spec says this is not required but I think that is incorrect
    pub uri: String,
    
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: Extensions,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Buffer,
}

/// [Represents a subset of a `Buffer`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#buffers-and-buffer-views)  
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BufferView<E: Extras> {
    /// The index of the parent `Buffer`
    pub buffer: Index<Buffer<E>>,
    
    /// The length of the buffer view data in bytes
    #[serde(rename = "byteLength")]
    pub byte_length: u32,
    
    /// Offset into the parent buffer in bytes
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    
    /// The stride in bytes between vertex attributes in this buffer view
    #[serde(default)]
    pub byte_stride: u32,

    /// Optional user-defined name for this object
    pub name: Option<String>,
    
    /// Optional target the buffer should be bound to
    pub target: Option<Target>,

    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: Extensions,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::BufferView,
}

enum_number! {
    Target {
        ArrayBuffer = 34962,
        ElementArrayBuffer = 34963,
    }
}

impl<E: Extras> Buffer<E> {
    pub fn range_check(&self, _root: &Root<E>) -> Result<(), ()> {
        Ok(())
    }
}

impl<E: Extras> BufferView<E> {
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
        let _ = root.try_get(&self.buffer)?;
        Ok(())
    }
}

