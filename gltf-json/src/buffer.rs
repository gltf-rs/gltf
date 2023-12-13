use crate::validation::{Checked, Error, USize64, Validate};
use crate::{extensions, Extras, Index, Path, Root};
use gltf_derive::Validate;
use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};
use std::fmt;

/// Corresponds to `GL_ARRAY_BUFFER`.
pub const ARRAY_BUFFER: u32 = 34_962;

/// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
pub const ELEMENT_ARRAY_BUFFER: u32 = 34_963;

/// The minimum byte stride.
pub const MIN_BYTE_STRIDE: usize = 4;

/// The maximum byte stride.
pub const MAX_BYTE_STRIDE: usize = 252;

/// All valid GPU buffer targets.
pub const VALID_TARGETS: &[u32] = &[ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER];

/// Specifies the target a GPU buffer should be bound to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Target {
    /// Corresponds to `GL_ARRAY_BUFFER`.
    ArrayBuffer = 1,

    /// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
    ElementArrayBuffer,
}

impl ser::Serialize for Target {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *self {
            Target::ArrayBuffer => serializer.serialize_u32(ARRAY_BUFFER),
            Target::ElementArrayBuffer => serializer.serialize_u32(ELEMENT_ARRAY_BUFFER),
        }
    }
}

/// Distance between individual items in a buffer view, measured in bytes.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Stride(pub usize);

impl Validate for Stride {
    fn validate<P, R>(&self, _root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        if self.0 < MIN_BYTE_STRIDE || self.0 > MAX_BYTE_STRIDE {
            report(&path, Error::Invalid);
        }
    }
}

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Buffer {
    /// The length of the buffer in bytes.
    #[serde(default, rename = "byteLength")]
    pub byte_length: USize64,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// The uri of the buffer.  Relative paths are relative to the .gltf file.
    /// Instead of referencing an external file, the uri can also be a data-uri.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::buffer::Buffer>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// A view into a buffer generally representing a subset of the buffer.
///
/// <https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#reference-bufferview>
///
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct View {
    /// The parent `Buffer`.
    pub buffer: Index<Buffer>,

    /// The length of the `BufferView` in bytes.
    #[serde(rename = "byteLength")]
    pub byte_length: USize64,

    /// Offset into the parent buffer in bytes.
    #[serde(
        default,
        rename = "byteOffset",
        skip_serializing_if = "Option::is_none"
    )]
    pub byte_offset: Option<USize64>,

    /// The stride in bytes between vertex attributes or other interleavable data.
    ///
    /// When zero, data is assumed to be tightly packed.
    #[serde(rename = "byteStride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_stride: Option<Stride>,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// Optional target the buffer should be bound to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Checked<Target>>,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::buffer::View>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

impl<'de> de::Deserialize<'de> for Checked<Target> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Target>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_TARGETS)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::Target::*;
                use crate::validation::Checked::*;
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
