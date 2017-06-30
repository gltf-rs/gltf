
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use json::{buffer, Extras, Index, Root};
use validation::{Action, Error, JsonPath, Validate};

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

/// All valid index component types.
pub const VALID_INDEX_COMPONENT_TYPES: &'static [u32] = &[
    UNSIGNED_BYTE,
    UNSIGNED_SHORT,
    UNSIGNED_INT,
];

/// All valid generic vertex attribute component types.
pub const VALID_GENERIC_ATTRIBUTE_COMPONENT_TYPES: &'static [u32] = &[
    BYTE,
    UNSIGNED_BYTE,
    SHORT,
    UNSIGNED_SHORT,
    UNSIGNED_INT,
    FLOAT,
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
    #[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
    pub struct IndicesExtensions {
        #[serde(default)]
        _allow_unknown_fields: (),
    }

    /// Indices of those attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
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
        pub component_type: IndexComponentType,

        /// Extension specific data.
        pub extensions: IndicesExtensions,

        /// Optional application specific data.
        pub extras: Extras,
    }

    /// Extension specific data for `Storage`.
    #[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
    pub struct StorageExtensions {
        #[serde(default)]
        _allow_unknown_fields: (),
    }

    /// Sparse storage of attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
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
    #[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
    pub struct ValuesExtensions {
        #[serde(default)]
        _allow_unknown_fields: (),
    }

    /// Array of size `count * number_of_components` storing the displaced
    /// accessor attributes pointed by `accessor::sparse::Indices`.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
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
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct AccessorExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// A typed view into a buffer view.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
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
    pub component_type: GenericComponentType,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: AccessorExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// Specifies if the attribute is a scalar, vector, or matrix.
    #[serde(rename = "type")]
    pub type_: Type,
    
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
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct IndexComponentType(pub u32);

/// The data type of a generic vertex attribute.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct GenericComponentType(pub u32);

/// Specifies if an attribute is a scalar, vector, or matrix.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Type(pub String);

impl Validate for IndexComponentType {
    fn validate<P, R>(&self, _: &Root, path: P, report: &mut R) -> Action
        where P: Fn() -> JsonPath, R: FnMut(Error) -> Action
    {
        if !VALID_INDEX_COMPONENT_TYPES.contains(&self.0) {
            report(Error::invalid_enum(path(), self.0))
        } else {
            Action::Continue
        }
    }
}

impl Validate for GenericComponentType {
    fn validate<P, R>(&self, _: &Root, path: P, report: &mut R) -> Action
        where P: Fn() -> JsonPath, R: FnMut(Error) -> Action
    {
        if !VALID_GENERIC_ATTRIBUTE_COMPONENT_TYPES.contains(&self.0) {
            report(Error::invalid_enum(path(), self.0))
        } else {
            Action::Continue
        }
    }
}

impl Validate for Type {
    fn validate<P, R>(&self, _: &Root, path: P, report: &mut R) -> Action
        where P: Fn() -> JsonPath, R: FnMut(Error) -> Action
    {
        if !VALID_ACCESSOR_TYPES.contains(&self.0.as_str()) {
            report(Error::invalid_enum(path(), self.0.clone()))
        } else {
            Action::Continue
        }
    }
}
