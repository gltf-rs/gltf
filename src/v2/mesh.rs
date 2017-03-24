
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use v2::{accessor, material, traits, Extensions, Extras, Index};

/// [A set of primitives to be rendered]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#mesh)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mesh<E: traits::Extensions, X: traits::Extras> {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Defines the geometry of this mesh to be renderered with a material
    pub primitives: Vec<Primitive<E, X>>,
    /// Defines the weights to be applied to the morph targets
    #[serde(default)]
    pub weights: Vec<f32>,
}

/// [Geometry to be rendered with the given material]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#meshprimitive)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Primitive<E: traits::Extensions, X: traits::Extras> {
    /// Maps attribute semantic names to the `Accessor`s containing their data
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, Index<accessor::Accessor<E, X>>>,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Index of the `Accessor` containing mesh indices
    pub indices: Option<Index<accessor::Accessor<E, X>>>,
    /// The index of the material to apply to this primitive when rendering
    pub material: Index<material::Material>,
    /// The type of primitives to render
    #[serde(default)]
    pub mode: Mode,
    #[serde(default)]
    /// Morph targets
    // TODO: Confirm that this the correct implementation and update
    // `Root::indices_are_valid()` as required
    pub targets: Vec<std::collections::HashMap<String, Index<accessor::Accessor<E, X>>>>,
}

enum_number! {
    Mode {
        Points = 0,
        Lines = 1,
        LineLoop = 2,
        LineStrip = 3,
        Triangles = 4,
        TriangleStrip = 5,
        TriangleFan = 6,
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Triangles
    }
}
