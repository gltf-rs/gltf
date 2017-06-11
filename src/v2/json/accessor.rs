
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::json::{buffer, Extras, Index};

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
    #[serde(deny_unknown_fields)]
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
        pub component_type: u32,

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
    pub component_type: u32,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: AccessorExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// Specifies if the attribute is a scalar, vector, or matrix.
    #[serde(rename = "type")]
    pub type_: String,
    
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

/*
impl Validate for Accessor {
    fn validate<F>(&self, root: &Root, path: JsonPath, report: &mut F)
        where F: FnMut(Error)
    {
        self.buffer_view.validate(root, path.field("bufferView"), report);
        self.sparse.validate(root, path.field("sparse"), report);
    }
}
*/
