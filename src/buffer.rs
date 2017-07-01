
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use {json, Gltf};

/// Specifies the target a GPU buffer should be bound to. 
pub enum Target {
    /// Corresponds to `GL_ARRAY_BUFFER`.
    ArrayBuffer = 34962,

    /// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
    ElementArrayBuffer = 34963,
}

///  A buffer points to binary data representing geometry, animations, or skins.
pub struct Buffer {
    /// The corresponding buffer data.
    data: &'a [u8],
    
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::buffer::Buffer,
}

impl Buffer {
    /// Constructs a `Buffer`.
    pub fn new(gltf: &'a Gltf, json: &'a json::buffer::Buffer, data: &'a [u8]) -> Self {
        Self {
            data: data,
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::buffer::Buffer {
        self.json
    }

    /// The length of the buffer in bytes.
    pub fn length(&self) -> u32 {
        self.json.byte_length
    }

    /// The buffer data.
    pub fn data(&self) -> &[u8] {
        self.data
    }
    
    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::buffer::BufferExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
///  A view into a buffer generally representing a subset of the buffer.
pub struct View {
    /// The corresponding buffer view data.
    data: &'a [u8],
    
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::buffer::View,
}

impl View {
    /// Constructs a `View`.
    pub fn new(
        gltf: &'a Gltf,
        json: &'a json::buffer::View,
        data: &'a [u8],
    ) -> Self {
        Self {
            data: data,
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::buffer::View {
        self.json
    }

    /// Returns the parent `Buffer`.
    pub fn buffer(&self) -> Buffer {
        self.gltf.buffers().nth(self.json.buffer.value()).unwrap()
    }

    /// Returns the length of the buffer view in bytes.
    pub fn length(&self) -> u32 {
        self.json.byte_length
    }

    /// Returns the offset into the parent buffer in bytes.
    pub fn offset(&self) -> u32 {
        self.json.byte_offset
    }

    /// Returns the stride in bytes between vertex attributes or other interleavable
    /// data. When `None`, data is assumed to be tightly packed.
    pub fn stride(&self) -> Option<u32> {
        self.json.byte_stride.map(|x| x.0)
    }

    /// Returns the buffer view data.
    pub fn data(&self) -> &[u8] {
        self.data
    }
    
    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
    }

    /// Optional target the buffer should be bound to.
    pub fn target(&self) -> Option<Target> {
        self.json.target.map(|x| match x.0 {
            json::buffer::ARRAY_BUFFER => Target::ArrayBuffer,
            json::buffer::ELEMENT_ARRAY_BUFFER => Target::ElementArrayBuffer,
            _ => unreachable!(),
        })
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::buffer::ViewExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
