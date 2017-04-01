
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use v1::Extras;

enum_number! {
    Mode {
        Points = 0,
        Line = 1,
        LineLoop = 2,
        Triangles = 4,
        TriangleStrip = 5,
        TriangleFan = 6,
    }
}

/// A set of primitives to be rendered
///
/// A node can contain one or more meshes and the node's transform places the
/// meshes in the scene
#[derive(Debug, Deserialize, Serialize)]
pub struct Mesh<E: Extras> {
    /// An array of primitives, each defining geometry to be rendered with a
    /// material
    #[serde(default)]
    pub primitives: Vec<Primitive<E>>,

    /// The user-defined name of this object
    pub name: Option<String>,

    /// Extension specific data
    #[serde(default)]
    pub extensions: MeshExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Mesh,
}

/// Extension specific data for `Mesh`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MeshExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Geometry to be rendered with the given material
#[derive(Debug, Deserialize, Serialize)]
pub struct Primitive<E: Extras> {
    /// A dictionary object of strings, where each string is the ID of the
    /// accessor containing an attribute
    #[serde(default)]
    pub attributes: HashMap<String, String>,

    /// The ID of the accessor that contains the indices
    ///
    /// When defined, the accessor must contain indices: the bufferView
    /// referenced by the accessor must have a target equal to 34963
    /// (ELEMENT_ARRAY_BUFFER); a byteStride that is tightly packed, i.e., 0 or
    /// the byte size of componentType in bytes; componentType must be 5121
    /// (UNSIGNED_BYTE) or 5123 (UNSIGNED_SHORT); and type must be "SCALAR"
    pub indices: Option<String>,

    /// The ID of the material to apply to this primitive when rendering
    pub material: String,

    /// The type of primitives to render
    #[serde(default)]
    pub mode: Mode,

    /// Extension specific data
    #[serde(default)]
    pub extensions: PrimitiveExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::MeshPrimitive,
}

/// Extension specific data for `Primitive`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PrimitiveExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Triangles
    }
}

