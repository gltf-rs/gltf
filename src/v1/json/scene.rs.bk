// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::json::Extras;

/// A node in the node hierarchy.
///
/// A node can have either the `camera`, `meshes`, or `skeletons`/`skin`/`meshes`
/// properties defined.
///
/// * In the latter case, all `primitives` in the referenced `meshes` contain
///   `JOINT` and `WEIGHT` attributes and the referenced `material`/`technique`
///   from each `primitive` has parameters with `JOINT` and `WEIGHT` semantics.
/// * A node can have either a `matrix` or any combination of
///   `translation`/`rotation`/`scale` (TRS) properties. TRS properties are
///   converted to matrices and postmultiplied in the `T * R * S` order to
///   compose the transformation matrix; first the scale is applied to the
///   vertices, then the rotation, and then the translation. If none are provided,
///   the transform is the identity. When a node is targeted for animation
///   (referenced by an animation.channel.target), only TRS properties may be
///   present; `matrix` will not be present.
#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
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

    /// A 4x4 transformation matrix stored in column-major order.
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

    /// Optional user-defined name for this node.
    pub name: Option<String>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: NodeExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

fn node_matrix_default() -> [f32; 16] {
    [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
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

/// Extension specific data for `Node`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NodeExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// The root nodes of a scene.
#[derive(Debug, Deserialize, Serialize)]
pub struct Scene {
    /// The IDs of each root node.
    #[serde(default)]
    pub nodes: Vec<String>,

    /// The user-defined name of this object.
    pub name: Option<String>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: SceneExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Scene`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SceneExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

