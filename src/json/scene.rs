
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::marker::PhantomData;

use json::{camera, mesh, scene, skin, Extras, Index, Root};
use validation::{Error, JsonPath, Validate};

/// A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node can have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), only TRS properties may be present; `matrix` will not be present..
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Node<'a> {
    /// The index of the camera referenced by this node.
    pub camera: Option<Index<camera::Camera<'a>>>,
    
    /// The indices of this node's children.
    pub children: Option<Vec<Index<scene::Node<'a>>>>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: NodeExtensions<'a>,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
    
    /// 4x4 column-major transformation matrix.
    #[serde(default = "node_matrix_default")]
    pub matrix: [f32; 16],

    /// The index of the mesh in this node.
    pub mesh: Option<Index<mesh::Mesh<'a>>>,
    
    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,
    
    /// The node's unit quaternion rotation in the order (x, y, z, w), where w is
    /// the scalar.
    #[serde(default)]
    pub rotation: UnitQuaternion,

    /// The node's non-uniform scale.
    #[serde(default = "node_scale_default")]
    pub scale: [f32; 3],

    /// The node's translation.
    #[serde(default)]
    pub translation: [f32; 3],
    
    /// The index of the skin referenced by this node.
    pub skin: Option<Index<skin::Skin<'a>>>,
    
    /// The weights of the instantiated Morph Target. Number of elements must match
    /// number of Morph Targets of used mesh.
    pub weights: Option<Vec<f32>>,
}

/// Extension specific data for `Node`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct NodeExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

fn node_matrix_default() -> [f32; 16] {
    [1.0, 0.0, 0.0, 0.0, 
     0.0, 1.0, 0.0, 0.0, 
     0.0, 0.0, 1.0, 0.0, 
     0.0, 0.0, 0.0, 1.0]
}

fn node_scale_default() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

/// The root `Node`s of a scene.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Scene<'a> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: SceneExtensions<'a>,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
    
    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,

    /// The indices of each root node.
    pub nodes: Vec<Index<Node<'a>>>,
}

/// Extension specific data for `Scene`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct SceneExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// Unit quaternion rotation in the order (x, y, z, w), where w is the scalar.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct UnitQuaternion(pub [f32; 4]);

impl Default for UnitQuaternion {
    fn default() -> Self {
        UnitQuaternion([0.0, 0.0, 0.0, 1.0])
    }
}

impl<'a> Validate<'a> for UnitQuaternion {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        for x in &self.0 {
            if *x < -1.0 || *x > 1.0 {
                let reason = format!("outside of permitted range [-1.0, 1.0]");
                report(Error::invalid_value(path(), self.0.to_vec(), reason));
                // Only report once
                break;
            }
        }
    }
}
