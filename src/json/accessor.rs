
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use json::{buffer, Extras, Index};
use serde::de;
use std::fmt;
use validation::Checked;

/// The component data type.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ComponentType {
    /// Corresponds to `GL_BYTE`.
    I8,

    /// Corresponds to `GL_UNSIGNED_BYTE`.
    U8,

    /// Corresponds to `GL_SHORT`.
    I16,

    /// Corresponds to `GL_UNSIGNED_SHORT`.
    U16,

    /// Corresponds to `GL_UNSIGNED_INT`.
    U32,

    /// Corresponds to `GL_FLOAT`.
    F32,
}

/// Specifies whether an attribute, vector, or matrix.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Type {
    /// Scalar quantity.
    Scalar,

    /// 2D vector.
    Vec2,

    /// 3D vector.
    Vec3,

    /// 4D vector.
    Vec4,

    /// 2x2 matrix.
    Mat2,

    /// 3x3 matrix.
    Mat3,

    /// 4x4 matrix.
    Mat4,
}

/// Corresponds to `GL_BYTE`.
pub const BYTE: u32 = 5120;

/// Corresponds to `GL_UNSIGNED_BYTE`.
pub const UNSIGNED_BYTE: u32 = 5121;

/// Corresponds to `GL_SHORT`.
pub const SHORT: u32 = 5122;

/// Corresponds to `GL_UNSIGNED_SHORT`.
pub const UNSIGNED_SHORT: u32 = 5123;

/// Corresponds to `GL_UNSIGNED_INT`.
pub const UNSIGNED_INT: u32 = 5125;

/// Corresponds to `GL_FLOAT`.
pub const FLOAT: u32 = 5126;

/// All valid generic vertex attribute component types.
pub const VALID_COMPONENT_TYPES: &'static [u32] = &[
    BYTE,
    UNSIGNED_BYTE,
    SHORT,
    UNSIGNED_SHORT,
    UNSIGNED_INT,
    FLOAT,
];

/// All valid index component types.
pub const VALID_INDEX_TYPES: &'static [u32] = &[
    UNSIGNED_BYTE,
    UNSIGNED_SHORT,
    UNSIGNED_INT,
];

/// All valid accessor types.
pub const VALID_ACCESSOR_TYPES: &'static [&'static str] = &[
    "SCALAR",
    "VEC2",
    "VEC3",
    "VEC4",
    "MAT2",
    "MAT3",
    "MAT4",
];

/// Contains data structures for sparse storage.
pub mod sparse {
    use super::*;
    
    /// Extension specific data for `Indices`.
    #[derive(Clone, Debug, Default, Deserialize, Validate)]
    pub struct IndicesExtensions {
        #[serde(default)]
        _allow_unknown_fields: (),
    }

    /// Indices of those attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Deserialize, Validate)]
    pub struct Indices {
        /// The parent buffer view containing the sparse indices.
        ///
        /// The referenced buffer view must not have `ARRAY_BUFFER` nor
        /// `ELEMENT_ARRAY_BUFFER` as its target.
        #[serde(rename = "bufferView")]
        pub buffer_view: Index<buffer::View>,

        /// The offset relative to the start of the parent `BufferView` in bytes.
        #[serde(default, rename = "byteOffset")]
        pub byte_offset: u32,

        /// The data type of each index.
        #[serde(rename = "componentType")]
        pub component_type: Checked<IndexComponentType>,

        /// Extension specific data.
        pub extensions: IndicesExtensions,

        /// Optional application specific data.
        pub extras: Extras,
    }

    /// Extension specific data for `Storage`.
    #[derive(Clone, Debug, Default, Deserialize, Validate)]
    pub struct StorageExtensions {
        #[serde(default)]
        _allow_unknown_fields: (),
    }

    /// Sparse storage of attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Deserialize, Validate)]
    #[serde(deny_unknown_fields)]
    pub struct Sparse {
        /// The number of attributes encoded in this sparse accessor.
        pub count: u32,

        /// Index array of size `count` that points to those accessor attributes
        /// that deviate from their initialization value.
        ///
        /// Indices must strictly increase.
        pub indices: Indices,

        /// Array of size `count * number_of_components` storing the displaced
        /// accessor attributes pointed by `indices`.
        ///
        /// Substituted values must have the same `component_type` and number of
        /// components as the base `Accessor`.
        pub values: Values,

        /// Extension specific data.
        pub extensions: StorageExtensions,

        /// Optional application specific data.
        pub extras: Extras,
    }

    /// Extension specific data for `Values`.
    #[derive(Clone, Debug, Default, Deserialize, Validate)]
    pub struct ValuesExtensions {
        #[serde(default)]
        _allow_unknown_fields: (),
    }

    /// Array of size `count * number_of_components` storing the displaced
    /// accessor attributes pointed by `accessor::sparse::Indices`.
    #[derive(Clone, Debug, Deserialize, Validate)]
    #[serde(deny_unknown_fields)]
    pub struct Values {
        /// The parent buffer view containing the sparse indices.
        ///
        /// The referenced buffer view must not have `ARRAY_BUFFER` nor
        /// `ELEMENT_ARRAY_BUFFER` as its target.
        #[serde(rename = "bufferView")]
        pub buffer_view: Index<buffer::View>,

        /// The offset relative to the start of the parent buffer view in bytes.
        #[serde(default, rename = "byteOffset")]
        pub byte_offset: u32,

        /// Extension specific data.
        pub extensions: ValuesExtensions,

        /// Optional application specific data.
        pub extras: Extras,
    }
}

/// Extension specific data for an `Accessor`.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct AccessorExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// A typed view into a buffer view.
#[derive(Clone, Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Accessor {
    /// The parent buffer view this accessor reads from.
    #[serde(rename = "bufferView")]
    pub buffer_view: Index<buffer::View>,
    
    /// The offset relative to the start of the parent `BufferView` in bytes.
    #[serde(default, rename = "byteOffset")]
    pub byte_offset: u32,
    
    /// The number of components within the buffer view - not to be confused
    /// with the number of bytes in the buffer view.
    pub count: u32,
    
    /// The data type of components in the attribute.
    #[serde(rename = "componentType")]
    pub component_type: Checked<GenericComponentType>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: AccessorExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// Specifies if the attribute is a scalar, vector, or matrix.
    #[serde(rename = "type")]
    pub type_: Checked<Type>,

    /// Minimum value of each component in this attribute.
    pub min: Vec<f32>,

    /// Maximum value of each component in this attribute.
    pub max: Vec<f32>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// Specifies whether integer data values should be normalized.
    #[serde(default)]
    pub normalized: bool,
    
    /// Sparse storage of attributes that deviate from their initialization
    /// value.
    pub sparse: Option<sparse::Sparse>,
}

/// The data type of an index.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct IndexComponentType(pub ComponentType);

/// The data type of a generic vertex attribute.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct GenericComponentType(pub ComponentType);

impl<'de> de::Deserialize<'de> for Checked<GenericComponentType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<GenericComponentType>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_COMPONENT_TYPES)
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::ComponentType::*;
                use validation::Checked::*;
                Ok(match value {
                    BYTE => Valid(GenericComponentType(I8)),
                    UNSIGNED_BYTE => Valid(GenericComponentType(U8)),
                    SHORT => Valid(GenericComponentType(I16)),
                    UNSIGNED_SHORT => Valid(GenericComponentType(U16)),
                    UNSIGNED_INT => Valid(GenericComponentType(U32)),
                    FLOAT => Valid(GenericComponentType(F32)),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

impl<'de> de::Deserialize<'de> for Checked<IndexComponentType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<IndexComponentType>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_INDEX_TYPES)
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::ComponentType::*;
                use validation::Checked::*;
                Ok(match value {
                    UNSIGNED_BYTE => Valid(IndexComponentType(U8)),
                    UNSIGNED_SHORT => Valid(IndexComponentType(U16)),
                    UNSIGNED_INT => Valid(IndexComponentType(U32)),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

impl<'de> de::Deserialize<'de> for Checked<Type> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Type>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_ACCESSOR_TYPES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::Type::*;
                use validation::Checked::*;
                Ok(match value {
                    "SCALAR" => Valid(Scalar),
                    "VEC2" => Valid(Vec2),
                    "VEC3" => Valid(Vec3),
                    "VEC4" => Valid(Vec4),
                    "MAT2" => Valid(Mat2),
                    "MAT3" => Valid(Mat3),
                    "MAT4" => Valid(Mat4),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
