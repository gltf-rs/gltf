
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Extensions;
use v2::Extras;
use v2::{camera, mesh, scene, skin, Index, Root};

/// [A single member of the glTF scene hierarchy]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#scenes)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Node<E: Extras> {
    /// The index of the camera referenced by this node
    // N.B. The spec says this is required but the sample models don't provide it
    // TODO: Remove `Option` as necessary
    pub camera: Option<Index<camera::Camera<E>>>,
    /// The indices of this node's children
    #[serde(default)]
    pub children: Vec<Index<scene::Node<E>>>,
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: Extensions,
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Node,
    /// 4x4 column-major transformation matrix
    #[serde(default = "node_matrix_default")]
    pub matrix: [[f32; 4]; 4],
    /// The index of the `Mesh` in this node
    // N.B. The spec says this is required but the sample models don't provide it
    // TODO: Remove `Option` as necessary
    pub mesh: Option<Index<mesh::Mesh<E>>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The node's unit quaternion rotation `[x, y, z, w]`
    #[serde(default = "node_rotation_default")]
    pub rotation: [f32; 4],
    #[serde(default = "node_scale_default")]
    /// The node's non-uniform scale
    pub scale: [f32; 3],
    #[serde(default)]
    /// The node's translation
    pub translation: [f32; 3],
    /// The index of the skin referenced by this node
    // N.B. The spec says this is required but the sample models don't provide it
    // TODO: Remove `Option` as necessary
    pub skin: Option<Index<skin::Skin<E>>>,
    /// The weights of the morph target
    // N.B. The spec says this is required but the sample models don't provide it
    // TODO: Remove `Option` as necessary
    pub weights: Option<Vec<f32>>,
}

fn node_matrix_default() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn node_rotation_default() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn node_scale_default() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}


/// [A set of visual objects to render](https://github.com/KhronosGroup/glTF/tree/2.0/specification/2.0#scenes)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scene<E: Extras> {
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: Extensions,
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Scene,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The indices of each root `Node` in this scene
    #[serde(default)]
    pub nodes: Vec<Index<Node<E>>>,
}

impl<E: Extras> Node<E> {
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
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
        for node in &self.nodes {
            let _ = root.try_get(node)?;
        }
        Ok(())
    }
}
