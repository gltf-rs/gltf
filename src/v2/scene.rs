
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{camera, mesh, scene, skin, Extras, Index, Root};

/// A node in the node hierarchy.
///
/// * When the node contains `skin`, all `mesh::Primitive`s must contain `"JOINT"`
///   and `"WEIGHT"` attributes.
/// * A node can have either a `matrix` or any combination of
///   `translation`/`rotation`/`scale` (TRS) properties.
/// * TRS properties are converted to matrices and postmultiplied in the
///   `T * R * S` order to compose the transformation matrix; first the scale is
///   applied to the vertices, then the rotation, and then the translation.
/// * If none are provided, the transform is the identity.
/// * When a node is targeted for animation (referenced by an
///   `animation::ChannelTarget`), only TRS properties may be present and `matrix`
///   will not be present.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Node<E: Extras> {
    /// The camera referenced by this node.
    pub camera: Option<Index<camera::Camera<E>>>,
    
    /// The child nodes belonging to this node in the node heirarchy.
    #[serde(default)]
    pub children: Vec<Index<scene::Node<E>>>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: NodeExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Node,
    
    /// 4x4 column-major transformation matrix.
    #[serde(default = "node_matrix_default")]
    pub matrix: [[f32; 4]; 4],
    
    /// The `Mesh` in this node.
    pub mesh: Option<Index<mesh::Mesh<E>>>,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,
    
    /// The node's unit quaternion rotation `[x, y, z, w]`.
    #[serde(default = "node_rotation_default")]
    pub rotation: [f32; 4],
    
    #[serde(default = "node_scale_default")]
    /// The node's non-uniform scale.
    pub scale: [f32; 3],
    
    #[serde(default)]
    /// The node's translation.
    pub translation: [f32; 3],
    
    /// The index of the skin referenced by this node.
    pub skin: Option<Index<skin::Skin<E>>>,
    
    /// The weights of the instantiated morph target.
    ///
    /// The number of elements must match the number of morph targets used by the
    /// mesh.
    pub weights: Option<Vec<f32>>,
}

/// Extension specific data for `Node`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NodeExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

fn node_matrix_default() -> [[f32; 4]; 4] {
    [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
}

fn node_rotation_default() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn node_scale_default() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

/// The root `Node`s of a scene.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scene<E: Extras> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: SceneExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Scene,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,
    
    /// The indices of each root `Node`.
    #[serde(default)]
    pub nodes: Vec<Index<Node<E>>>,
}

/// Extension specific data for `Scene`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SceneExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl<E: Extras> Node<E> {
    #[doc(hidden)]
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
        if let Some(ref camera) = self.camera {
            let _ = root.try_get(&camera)?;
            for node in &self.children {
                let _ = root.try_get(node)?;
            }
        }
        if let Some(ref mesh) = self.mesh {
            let _ = root.try_get(mesh)?;
        }
        if let Some(ref skin) = self.skin {
            let _ = root.try_get(skin)?;
        }
        Ok(())
    }
}

impl<E: Extras> Scene<E> {
    #[doc(hidden)]
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
        for node in &self.nodes {
            let _ = root.try_get(node)?;
        }
        Ok(())
    }
}
