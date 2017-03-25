// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use traits::{Extensions, Extras};

enum_number! {
    ComponentType {
        I8 = 5120,
        U8 = 5121,
        I16 = 5122,
        U16 = 5123,
        I32 = 5124,
        U32 = 5125,
        F32 = 5126,
        F64 = 5127,
    }
}

impl Default for ComponentType {
    fn default() -> ComponentType {
        ComponentType::I8
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

impl Default for Kind {
    fn default() -> Kind {
        Kind::Scalar
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Accessor<E: Extensions, X: Extras> {
    /// The ID of the bufferView
    #[serde(rename = "bufferView")]
    pub buffer_view: String,

    /// The offset relative to the start of the bufferView in bytes.
    ///
    /// This must be a multiple of the size of the data type.
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,

    /// The stride, in bytes, between attributes referenced by this accessor.
    ///
    /// When this is zero, the attributes are tightly packed.
    #[serde(rename = "byteStride")]
    #[serde(default)]
    pub byte_stride: u32,

    /// The datatype of components in the attribute.
    #[serde(rename = "componentType")]
    pub component_type: ComponentType,

    /// The number of attributes referenced by this accessor, not to be confused
    /// with the number of bytes or number of components.
    pub count: u32,

    /// Specifies if the attribute is a scalar, vector, or matrix, and the
    /// number of elements in the vector or matrix.
    ///
    /// TODO: Coerce string into enum and back
    #[serde(rename = "type")]
    #[serde(default)]
    pub kind: Kind,

    /// Maximum value of each component in this attribute.
    ///
    /// When both min and max arrays are defined, they have the same length. The
    /// length is determined by the value of the type property; it can be 1, 2,
    /// 3, 4, 9, or 16.
    pub max: Option<Vec<f32>>,

    /// Minimum value of each component in this attribute.
    ///
    /// When both min and max arrays are defined, they have the same length. The
    /// length is determined by the value of the type property; it can be 1, 2,
    /// 3, 4, 9, or 16.
    pub min: Option<Vec<f32>>,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., an accessor and a buffer could
    /// have the same name, or two accessors could even have the same name.
    pub name: Option<String>,

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: <E as Extensions>::Accessor,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <X as Extras>::Accessor,
}

#[cfg(test)]
mod test {
    extern crate serde_json;
    use super::*;

    #[test]
    fn invalid_component_type() {
        let data = r#"{
    "bufferView": "bufferViewWithVertices_id",
    "byteOffset": 0,
    "byteStride": 3,
    "componentType": 5128,
    "count": 1024,
    "type": "SCALAR"
}"#;

        let accessor = serde_json::from_str::<Accessor>(data);
        assert!(accessor.is_err());
    }

    #[test]
    fn it_deserializes_an_accessor() {
        let data = r#"{
    "bufferView": "bufferViewWithVertices_id",
    "byteOffset": 0,
    "byteStride": 3,
    "componentType": 5126,
    "count": 1024,
    "type": "SCALAR",
    "name": "user-defined accessor name",
    "max": [
        -1.0,
        -1.0,
        -1.0
    ],
    "min": [
        1.0,
        1.0,
        1.0
    ],
    "extensions": {
        "extension_name": {
            "extension specific": "value"
        }
    },
    "extras": {
        "Application specific": "The extra object can contain any properties."
    }
}"#;

        let accessor: Accessor = serde_json::from_str(data).unwrap();

        assert_eq!("bufferViewWithVertices_id", accessor.buffer_view);
        assert_eq!(0, accessor.byte_offset);
        assert_eq!(3, accessor.byte_stride);
        assert_eq!(ComponentType::F32, accessor.component_type);
        assert_eq!(1024, accessor.count);
        assert_eq!(Kind::Scalar, accessor.kind);
        assert_eq!(3, accessor.max.unwrap().len());
    }
}
