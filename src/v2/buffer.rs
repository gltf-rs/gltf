
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{Extras, Index, Root};

enum_number! {
    Target {
        ArrayBuffer = 34962,
        ElementArrayBuffer = 34963,
    }
}

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Buffer {
    /// The length of the buffer in bytes.
    #[serde(default, rename = "byteLength")]
    pub byte_length: u32,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// Uniform resource locator of the buffer.
    ///
    /// Relative paths are relative to the .gltf file.
    pub uri: String,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: BufferExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Buffer`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BufferExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BufferView {
    /// The parent `Buffer`.
    pub buffer: Index<Buffer>,

    /// The length of the `BufferView` in bytes.
    #[serde(rename = "byteLength")]
    pub byte_length: u32,

    /// Offset into the parent buffer in bytes.
    #[serde(default, rename = "byteOffset")]
    pub byte_offset: u32,

    /// The stride in bytes between vertex attributes or other interleavable data.
    ///
    /// When zero, data is assumed to be tightly packed.
    #[serde(default, rename = "byteStride")]
    pub byte_stride: u32,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// Optional target the buffer should be bound to.
    pub target: Option<Target>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: BufferViewExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `BufferView`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BufferViewExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl Buffer {
    #[doc(hidden)]
    pub fn range_check(&self, _root: &Root) -> Result<(), ()> {
        Ok(())
    }
}

impl BufferView {
    #[doc(hidden)]
    pub fn range_check(&self, root: &Root) -> Result<(), ()> {
        let _ = root.try_get(&self.buffer)?;
        Ok(())
    }
}
