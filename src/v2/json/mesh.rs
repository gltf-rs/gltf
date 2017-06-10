
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use v2::json::{accessor, material, Extras, Index};

/// Extension specific data for `Mesh`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MeshExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mesh {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: MeshExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,
    
    /// Defines the geometry to be renderered with a material.
    pub primitives: Vec<Primitive>,

    /// Defines the weights to be applied to the morph targets.
    pub weights: Option<Vec<f32>>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Primitive {
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub attributes: HashMap<String, Index<accessor::Accessor>>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: PrimitiveExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// The index of the accessor that contains the indices.
    pub indices: Option<Index<accessor::Accessor>>,
    
    /// The index of the material to apply to this primitive when rendering
    pub material: Option<Index<material::Material>>,
    
    /// The type of primitives to render.
    #[serde(default = "primitive_mode_default")]
    pub mode: u32,
    
    /// An array of Morph Targets, each  Morph Target is a dictionary mapping attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their deviations in the Morph Target.
    pub targets: Option<Vec<HashMap<String, Index<accessor::Accessor>>>>,
}

fn primitive_mode_default() -> u32 {
    4
}

/// Extension specific data for `Primitive`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PrimitiveExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}
