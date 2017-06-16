
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::marker::PhantomData;

use json::{Extras, Index, Root};
use validation::{Error, JsonPath, Validate};

/// Corresponds to `GL_ARRAY_BUFFER`.
pub const ARRAY_BUFFER: u32 = 34962;

/// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
pub const ELEMENT_ARRAY_BUFFER: u32 = 34963;

/// The minimum byte stride.
pub const MIN_BYTE_STRIDE: u32 = 4;

/// The maximum byte stride.
pub const MAX_BYTE_STRIDE: u32 = 252;

/// All valid GPU buffer targets.
pub const VALID_TARGETS: &'static [u32] = &[
    ARRAY_BUFFER,
    ELEMENT_ARRAY_BUFFER,
];

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Buffer<'a> {
    /// The length of the buffer in bytes.
    #[serde(default, rename = "byteLength")]
    pub byte_length: u32,

    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,

    /// The uri of the buffer.  Relative paths are relative to the .gltf file.  Instead of referencing an external file, the uri can also be a data-uri.
    pub uri: Option<Cow<'a, str>>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: BufferExtensions<'a>,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
}

/// Extension specific data for `Buffer`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct BufferExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct View<'a> {
    /// The parent `Buffer`.
    pub buffer: Index<Buffer<'a>>,

    /// The length of the `BufferView` in bytes.
    #[serde(rename = "byteLength")]
    pub byte_length: u32,

    /// Offset into the parent buffer in bytes.
    #[serde(default, rename = "byteOffset")]
    pub byte_offset: u32,

    /// The stride in bytes between vertex attributes or other interleavable data.
    ///
    /// When zero, data is assumed to be tightly packed.
    #[serde(rename = "byteStride")]
    pub byte_stride: Option<ByteStride>,

    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,

    /// Optional target the buffer should be bound to.
    pub target: Option<Target>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: ViewExtensions<'a>,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
}

/// Extension specific data for `View`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct ViewExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// The stride, in bytes, between vertex attributes.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct ByteStride(pub u32);

/// Specifies the target a GPU buffer should be bound to. 
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Target(pub u32);

impl<'a> Validate<'a> for ByteStride {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if self.0 % 4 != 0 {
            // Not a multiple of 4
            report(Error::invalid_value(path(), self.0));
        }

        if self.0 < MIN_BYTE_STRIDE || self.0 > MAX_BYTE_STRIDE {
            report(Error::invalid_value(path(), self.0));
        }
    }
}

impl<'a> Validate<'a> for Target {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if !VALID_TARGETS.contains(&self.0) {
            report(Error::invalid_enum(path(), self.0));
        }
    }
}
