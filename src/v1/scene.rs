// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::Extras;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NodeExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SceneExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node<E: Extras> {
    /// The ID of the camera referenced by this node.
    pub camera: Option<String>,

    /// The IDs of this node's children.
    #[serde(default)]
    pub children: Vec<String>,

    /// The ID of skeleton nodes.
    ///
    /// Each node defines a subtree, which has a jointName of the corresponding
    /// element in the referenced skin.jointNames.
    #[serde(default)]
    pub skeletons: Vec<String>,

    /// The ID of the skin referenced by this node.
    pub skin: Option<String>,

    /// Name used when this node is a joint in a skin.
    #[serde(rename = "jointName")]
    pub joint_name: Option<String>,

    /// A floating-point 4x4 transformation matrix stored in column-major order.
    #[serde(default = "node_matrix_default")]
    pub matrix: [f32; 16],

    /// The IDs of the meshes in this node.
    ///
    /// Multiple meshes are allowed so each can share the same transform matrix.
    #[serde(default)]
    pub meshes: Vec<String>,

    /// The node's unit quaternion rotation in the order (x, y, z, w),
    /// where w is the scalar.
    #[serde(default = "node_rotation_default")]
    pub rotation: [f32; 4],

    /// The node's non-uniform scale.
    #[serde(default = "node_scale_default")]
    pub scale: [f32; 3],

    /// The node's translation.
    #[serde(default = "node_translation_default")]
    pub translation: [f32; 3],

    pub name: Option<String>,
    
    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: NodeExtensions,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Node,
}

fn node_matrix_default() -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

fn node_rotation_default() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn node_scale_default() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

fn node_translation_default() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scene<E: Extras> {
    /// The IDs of each root node.
    #[serde(default)]
    pub nodes: Vec<String>,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a scene and a buffer could have
    /// the same name, or two scenes could even have the same name.
    pub name: Option<String>,

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: SceneExtensions,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Scene,
}
