
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {extensions, import, json};

use Gltf;

pub use json::buffer::Target;

///  A buffer points to binary data representing geometry, animations, or skins.
pub struct Buffer<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::buffer::Buffer,
}

///  A view into a buffer generally representing a subset of the buffer.
pub struct View<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::buffer::View,
}

impl<'a> Buffer<'a> {
    /// Constructs a `Buffer`.
    pub fn new(
        gltf: &'a Gltf,
        json: &'a json::buffer::Buffer,
    ) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::buffer::Buffer {
        self.json
    }

    /// The length of the buffer in bytes.
    pub fn length(&self) -> usize {
        self.json.byte_length as usize
    }

    /// The buffer data.
    pub fn data(&self) -> import::Data {
        unimplemented!()
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Extension specific data.
    pub fn extensions(&self) -> extensions::buffer::Buffer<'a> {
        extensions::buffer::Buffer::new(
            self.gltf,
            &self.json.extensions,
        )
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl<'a> View<'a> {
    /// Constructs a `View`.
    pub fn new(
        gltf: &'a Gltf,
        json: &'a json::buffer::View,
    ) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::buffer::View {
        self.json
    }

    /// Returns the parent `Buffer`.
    pub fn buffer(&self) -> Buffer<'a> {
        self.gltf.buffers().nth(self.json.buffer.value()).unwrap()
    }

    /// Returns the length of the buffer view in bytes.
    pub fn length(&self) -> usize {
        self.json.byte_length as usize
    }

    /// Returns the offset into the parent buffer in bytes.
    pub fn offset(&self) -> usize {
        self.json.byte_offset as usize
    }

    /// Returns the stride in bytes between vertex attributes or other interleavable
    /// data. When `None`, data is assumed to be tightly packed.
    pub fn stride(&self) -> Option<usize> {
        self.json.byte_stride.map(|x| x.0 as usize)
    }

    /// Returns the buffer view data.
    pub fn data(&self) -> import::Data {
        self.buffer().data().subview(self.offset(), self.length())
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Optional target the buffer should be bound to.
    pub fn target(&self) -> Option<Target> {
        self.json.target.map(|target| target.unwrap())
    }

    /// Extension specific data.
    pub fn extensions(&self) -> extensions::buffer::View<'a> {
        extensions::buffer::View::new(
            self.gltf,
            &self.json.extensions,
        )
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
