
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de;
use std::fmt;
use json::{extensions, Extras, Index, Root, Path};
use validation::{Checked, Error, Validate};

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

/// Specifies the target a GPU buffer should be bound to. 
#[derive(Clone, Copy, Debug)]
pub enum Target {
    /// Corresponds to `GL_ARRAY_BUFFER`.
    ArrayBuffer = 1,

    /// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
    ElementArrayBuffer,
}

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct Buffer {
    /// The length of the buffer in bytes.
    #[serde(default, rename = "byteLength")]
    pub byte_length: u32,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub name: Option<String>,

    /// The uri of the buffer.  Relative paths are relative to the .gltf file.
    /// Instead of referencing an external file, the uri can also be a data-uri.
    pub uri: Option<String>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::buffer::Buffer,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct View {
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
    #[serde(rename = "byteStride")]
    pub byte_stride: Option<ByteStride>,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub name: Option<String>,

    /// Optional target the buffer should be bound to.
    pub target: Option<Checked<Target>>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::buffer::View,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// The stride, in bytes, between vertex attributes.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct ByteStride(pub u32);

impl Validate for ByteStride {
    fn validate_completely<P, R>(&self, _: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&Fn() -> Path, Error),
    {
        if self.0 % 4 != 0 {
            // Not a multiple of 4
            report(&path, Error::Invalid);
        }

        if self.0 < MIN_BYTE_STRIDE || self.0 > MAX_BYTE_STRIDE {
            report(&path, Error::Invalid);
        }
    }
}

impl<'de> de::Deserialize<'de> for Checked<Target> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Target>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_TARGETS)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::Target::*;
                use validation::Checked::*;
                Ok(match value as u32 {
                    ARRAY_BUFFER => Valid(ArrayBuffer),
                    ELEMENT_ARRAY_BUFFER => Valid(ElementArrayBuffer),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_u64(Visitor)
    }
}

