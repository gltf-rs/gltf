
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {json, Gltf};

///  A buffer points to binary data representing geometry, animations, or skins.
pub struct Buffer<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::buffer::Buffer,
}

///  A view into a buffer generally representing a subset of the buffer.
pub struct View<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::buffer::View,
}

impl<'a> Buffer<'a> {
    /// Constructs a `Buffer`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::buffer::Buffer) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::buffer::Buffer {
        self.json
    }
}

impl<'a> View<'a> {
    /// Constructs a `View`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::buffer::View) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::buffer::View {
        self.json
    }
}
