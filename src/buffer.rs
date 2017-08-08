
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops;
use json;

use {Gltf, Loaded, Source};

pub use json::buffer::Target;

/// Buffer data.
#[derive(Clone, Debug)]
pub struct Data<'a> {
    /// Parent `Buffer`.
    parent: Loaded<'a, Buffer<'a>>,

    /// Buffer range.
    range: ops::Range<usize>,
}

impl<'a> ops::Deref for Data<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.parent.source.source_buffer(&self.parent.item)[self.range.clone()]
    }
}

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug)]
pub struct Buffer<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::buffer::Buffer,
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug)]
pub struct View<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::buffer::View,

    /// The parent `Buffer`.
    parent: Buffer<'a>,
}

impl<'a> Buffer<'a> {
    /// Constructs a `Buffer`.
    pub fn new(
        gltf: &'a Gltf,
        index: usize,
        json: &'a json::buffer::Buffer,
    ) -> Self {
        Self {
            gltf: gltf,
            index: index,
            json: json,
        }
    }

    /// Converts `View` into a `Loaded<View>`.
    pub fn loaded(self, source: &'a Source) -> Loaded<'a, Buffer<'a>> {
        Loaded {
            item: self,
            source,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::buffer::Buffer {
        self.json
    }

    /// The length of the buffer in bytes.
    pub fn length(&self) -> usize {
        self.json.byte_length as usize
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl<'a> Loaded<'a, Buffer<'a>> {
    /// Returns the buffer data.
    pub fn data(&self) -> Data {
        let parent = self.clone();
        let range = 0..self.length();
        Data {
            parent,
            range,
        }
    }
}

impl<'a> View<'a> {
    /// Constructs a `View`.
    pub fn new(
        gltf: &'a Gltf,
        index: usize,
        json: &'a json::buffer::View,
    ) -> Self {
        let parent = gltf.buffers().nth(json.buffer.value()).unwrap();
        Self {
            gltf,
            index,
            json,
            parent,
        }
    }

    /// Converts `View` into a `Loaded<View>`.
    pub fn loaded(self, source: &'a Source) -> Loaded<'a, View<'a>> {
        Loaded {
            item: self,
            source,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
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

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Optional target the buffer should be bound to.
    pub fn target(&self) -> Option<Target> {
        self.json.target.map(|target| target.unwrap())
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
impl<'a> Loaded<'a, View<'a>> {
    /// Returns the buffer view data.
    pub fn data(&self) -> Data {
        let begin = self.offset();
        let end = self.length();
        let range = begin..end;
        let parent = self.parent.clone().loaded(self.source);
        Data {
            parent,
            range,
        }
    }
}

