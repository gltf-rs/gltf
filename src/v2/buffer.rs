
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;
use v2::json;

/// Specifies the target a GPU buffer should be bound to. 
pub enum Target {
    /// Corresponds to `GL_ARRAY_BUFFER`.
    ArrayBuffer = 34962,

    /// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
    ElementArrayBuffer = 34963,
}

///  A buffer points to binary data representing geometry, animations, or skins.
pub struct Buffer<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf<'a>,

    /// The corresponding JSON struct.
    json: &'a json::buffer::Buffer,
}

impl<'a> Buffer<'a> {
    /// Constructs a `Buffer`.
    pub fn new(gltf: &'a Gltf<'a>, json: &'a json::buffer::Buffer) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::buffer::Buffer {
        self.json
    }

    ///  The length of the buffer in bytes.
    pub fn byte_length(&self) -> u32 {
        self.json.byte_length
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    ///  The uri of the buffer. Relative paths are relative to the .gltf file.
    /// Instead of referencing an external file, the uri can also be a data-uri.
    pub fn uri(&self) -> Option<&str> {
        self.json.uri.as_ref().map(String::as_str)
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::buffer::BufferExtensions {
        &self.json.extensions
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
///  A view into a buffer generally representing a subset of the buffer.
pub struct View<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf<'a>,

    /// The corresponding JSON struct.
    json: &'a json::buffer::View,
}

impl<'a> View<'a> {
    /// Constructs a `View`.
    pub fn new(gltf: &'a Gltf<'a>, json: &'a json::buffer::View) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::buffer::View {
        self.json
    }

    ///  The parent `Buffer`.
    pub fn buffer(&self) -> Buffer<'a> {
        Buffer::new(self.gltf, self.gltf.as_json().get(&self.json.buffer))
    }

    ///  The length of the `BufferView` in bytes.
    pub fn byte_length(&self) -> u32 {
        self.json.byte_length
    }

    ///  Offset into the parent buffer in bytes.
    pub fn byte_offset(&self) -> u32 {
        self.json.byte_offset
    }

    ///  The stride in bytes between vertex attributes or other interleavable data.
    /// When zero, data is assumed to be tightly packed.
    pub fn byte_stride(&self) -> Option<u32> {
        self.json.byte_stride.map(|x| x.0)
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    ///  Optional target the buffer should be bound to.
    pub fn target(&self) -> Option<Target> {
        self.json.target.map(|x| match x.0 {
            json::buffer::ARRAY_BUFFER => Target::ArrayBuffer,
            json::buffer::ELEMENT_ARRAY_BUFFER => Target::ElementArrayBuffer,
            _ => unreachable!(),
        })
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::buffer::ViewExtensions {
        &self.json.extensions
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
