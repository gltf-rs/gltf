// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;

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

impl Default for Mode {
    fn default() -> Mode {
        Mode::Triangles
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Primitive {
    /// A dictionary object of strings, where each string is the ID of the
    /// accessor containing an attribute.
    #[serde(default)]
    pub attributes: HashMap<String, String>,

    /// The ID of the accessor that contains the indices.
    ///
    /// When this is not defined, the primitives should be rendered without
    /// indices using drawArrays().
    ///
    /// When defined, the accessor must contain indices: the bufferView
    /// referenced by the accessor must have a target equal to 34963
    /// (ELEMENT_ARRAY_BUFFER); a byteStride that is tightly packed, i.e., 0 or
    /// the byte size of componentType in bytes; componentType must be 5121
    /// (UNSIGNED_BYTE) or 5123 (UNSIGNED_SHORT); and type must be "SCALAR".
    pub indices: Option<String>,

    /// The ID of the material to apply to this primitive when rendering.
    pub material: String,

    /// The type of primitives to render.
    #[serde(default)]
    pub mode: Mode,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Mesh {
    /// An array of primitives, each defining geometry to be rendered with a
    /// material.
    #[serde(default)]
    pub primitives: Vec<Primitive>,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a mesh and a buffer could have the
    /// same name, or two meshes could even have the same name.
    pub name: Option<String>,
}
