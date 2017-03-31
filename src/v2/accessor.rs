
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{buffer, Extras, Index, Root};

pub mod sparse {
    use super::*;

    enum_number! {
        ComponentType {
            U8 = 5121,
            U16 = 5123,
            U32 = 5125,
        }
    }
    
    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct IndicesExtensions {
        #[serde(default)]
        _allow_extra_fields: (),
    }

    // TODO: Complete documentation
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Indices<E: Extras> {
        /// The index of the parent `BufferView` containing the sparse indices
        #[serde(rename = "bufferView")]
        pub buffer_view: Index<buffer::BufferView<E>>,
        /// The offset relative to the start of the parent `BufferView` in bytes
        #[serde(default, rename = "byteOffset")]
        pub byte_offset: u32,
        /// The indices data type
        pub component_type: ComponentType,

        pub extensions: IndicesExtensions,
        pub extras: <E as Extras>::AccessorSparseIndices,
    }
    
    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct StorageExtensions {
        #[serde(default)]
        _allow_extra_fields: (),
    }
    
    /// Sparse storage of attributes that deviate from their initialization value
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Storage<E: Extras> {
        /// Number of entries stored in the sparse array
        pub count: u32,
        // TODO: Complete documentation
        pub indices: Indices<E>,
        // TODO: Complete documentation
        pub values: Values<E>,

        pub extensions: StorageExtensions,
        pub extras: <E as Extras>::AccessorSparseStorage,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct ValuesExtensions {
        #[serde(default)]
        _allow_extra_fields: (),
    }
    
    // TODO: Complete documentation
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Values<E: Extras> {
        /// The index of the parent `BufferView` containing the sparse values
        #[serde(rename = "bufferView")]
        pub buffer_view: Index<buffer::BufferView<E>>,
        /// The offset relative to the start of the parent `BufferView` in bytes
        #[serde(default, rename = "byteOffset")]
        pub byte_offset: u32,

        pub extensions: ValuesExtensions,
        pub extras: <E as Extras>::AccessorSparseValues,
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccessorExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// [Defines a method for retrieving data from within a `BufferView`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#accessors)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Accessor<E: Extras> {
    /// The index of the parent `BufferView` this accessor reads from.
    #[serde(rename = "bufferView")]
    pub buffer_view: Index<buffer::BufferView<E>>,
    /// The offset relative to the start of the parent `BufferView` in bytes
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    /// The number of elements within the `BufferView` (N.B. not number of bytes)
    pub count: u32,
    /// The data type of each element
    #[serde(rename = "componentType")]
    pub component_type: ComponentType,
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: AccessorExtensions,
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Accessor,
    /// The multiplicity of each element
    #[serde(rename = "type")]
    pub kind: Kind,
    /// Minimum value of each element in this attribute
    #[serde(default)]
    pub min: Vec<f32>,
    /// Maximum value of each element in this attribute
    #[serde(default)]
    pub max: Vec<f32>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Specifies whether integer data values should be normalized
    #[serde(default)]
    pub normalized: bool,
    /// Sparse storage of attributes that deviate from their initialization value
    pub sparse: Option<sparse::Storage<E>>,
}

impl<E: Extras> Accessor<E> {
    /// Returns `Ok(())` if all indices are in range of the maximums
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
        if let Some(ref sparse) = self.sparse {
            let _ = root.try_get(&sparse.indices.buffer_view)?;
            let _ = root.try_get(&sparse.values.buffer_view)?;
        }
        let _ = root.try_get(&self.buffer_view)?;
        Ok(())
    }
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
