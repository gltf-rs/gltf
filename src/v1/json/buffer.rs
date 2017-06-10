// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::json::Extras;

enum_number! {
    Target {
        ArrayBuffer = 34962,
        ElementArrayBuffer = 34963,
    }
}

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Debug, Deserialize, Serialize)]
pub struct Buffer {
    /// The uri of the buffer.
    ///
    /// Relative paths are relative to the .gltf file.
    pub uri: String,

    /// The length of the buffer in bytes.
    #[serde(rename = "byteLength")]
    #[serde(default)]
    pub byte_length: usize,

    /// XMLHttpRequest responseType.
    #[serde(rename = "type")]
    pub kind: Option<String>,

    /// The user-defined name of this object.
    pub name: Option<String>,

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
#[derive(Debug, Deserialize, Serialize)]
pub struct BufferView {
    /// The ID of the parent buffer.
    pub buffer: String,

    /// The offset into the parent buffer in bytes.
    #[serde(rename = "byteOffset")]
    #[serde(default)]
    pub byte_offset: usize,

    /// The length of the `BufferView` in bytes.
    #[serde(rename = "byteLength")]
    #[serde(default)]
    pub byte_length: usize,

    /// The target that the buffer should be bound to.
    ///
    /// When this is not provided, the `BufferView` contains animation or skin
    /// data.
    pub target: Option<Target>,

    /// The user-defined name of this object.
    pub name: Option<String>,

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
