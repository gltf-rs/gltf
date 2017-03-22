
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_json;
use v2::{buffer, Extensions, Extras, Index};

/// [Defines a method for retrieving data from within a `BufferView`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#accessors)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Accessor {
    /// The index of the parent `BufferView` this accessor reads from.
    #[serde(rename = "bufferView")]
    pub buffer_view: Index<buffer::View>,
    /// The offset relative to the start of the parent `BufferView` in bytes
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    /// The number of elements within the `BufferView` (N.B. not number of bytes)
    pub count: u32,
    /// The data type of each element
    #[serde(rename = "componentType")]
    pub component_type: ComponentType,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The multiplicity of each element
    #[serde(rename = "type")]
    pub kind: Kind,
    /// Minimum value of each element in this attribute
    // TODO: Implement me properly
    #[serde(default)]
    pub min: serde_json::Value,
    /// Maximum value of each element in this attribute
    // TODO: Implement me properly
    #[serde(default)]
    pub max: serde_json::Value,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Specifies whether integer data values should be normalized
    #[serde(default)]
    pub normalized: bool,
    /// Sparse storage of attributes that deviate from their initialization value
    pub sparse: Option<SparseStorage>,
}

// TODO: Complete documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SparseIndices {
    /// The index of the parent `BufferView` containing the sparse indices
    #[serde(rename = "byteOffset")]
    pub buffer_view: Index<buffer::Buffer>,
    /// The offset relative to the start of the parent `BufferView` in bytes
    #[serde(default, rename = "byteOffset")]
    pub byte_offset: u32,
    /// The indices data type
    // N.B. Not all values are valid but it would be pedantic to have more than
    // one `DataType` enum and would also create inconsistency with the regular
    // `Accessor` struct.
    pub component_type: ComponentType,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
}

/// Sparse storage of attributes that deviate from their initialization value
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SparseStorage {
    /// Number of entries stored in the sparse array
    pub count: u32,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    // TODO: Complete documentation
    pub indices: SparseIndices,
    // TODO: Complete documentation
    pub values: SparseValues,
}

// TODO: Complete documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SparseValues {
    /// The index of the parent `BufferView` containing the sparse values
    #[serde(rename = "byteOffset")]
    pub buffer_view: Index<buffer::View>,
    /// The offset relative to the start of the parent `BufferView` in bytes
    #[serde(default, rename = "byteOffset")]
    pub byte_offset: u32,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
}

enum_number! {
    ComponentType {
        I8 = 5120,
        U8 = 5121,
        I16 = 5122,
        U16 = 5123,
        U32 = 5125,
        F32 = 5126,
    }
}

enum_string! {
    Kind {
        Scalar = "SCALAR",
        Vec2 = "VEC2",
        Vec3 = "VEC3",
        Vec4 = "VEC4",
        Mat2 = "MAT2",
        Mat3 = "MAT3",
        Mat4 = "MAT4",
    }
}
