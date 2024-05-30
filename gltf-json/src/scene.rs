use crate::validation::Validate;
use crate::{camera, extensions, mesh, scene, skin, Extras, Index};
use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// A node in the node hierarchy.  When the node contains `skin`, all
/// `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.
/// A node can have either a `matrix` or any combination of
/// `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted
/// to matrices and postmultiplied in the `T * R * S` order to compose the
/// transformation matrix; first the scale is applied to the vertices, then the
/// rotation, and then the translation. If none are provided, the transform is the
/// identity. When a node is targeted for animation (referenced by an
/// animation.channel.target), only TRS properties may be present; `matrix` will not
/// be present.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Node {
    /// The index of the camera referenced by this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<Index<camera::Camera>>,

    /// The indices of this node's children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Index<scene::Node>>>,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::scene::Node>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,

    /// 4x4 column-major transformation matrix.
    ///
    /// glTF 2.0 specification:
    ///     When a node is targeted for animation (referenced by an
    ///     animation.channel.target), only TRS properties may be present;
    ///     matrix will not be present.
    ///
    /// TODO: Ensure that .matrix is set to None or otherwise skipped during
    ///       serialization, if the node is targeted for animation.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<[f32; 16]>,

    /// The index of the mesh in this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Index<mesh::Mesh>>,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// The node's unit quaternion rotation in the order (x, y, z, w), where w is
    /// the scalar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<UnitQuaternion>,

    /// The node's non-uniform scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f32; 3]>,

    /// The node's translation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<[f32; 3]>,

    /// The index of the skin referenced by this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<Index<skin::Skin>>,

    /// The weights of the instantiated Morph Target. Number of elements must match
    /// the number of Morph Targets of used mesh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,
}

/// The root `Node`s of a scene.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Scene {
    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::scene::Scene>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// The indices of each root node.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<Index<Node>>,
}

/// Unit quaternion rotation in the order (x, y, z, w), where w is the scalar.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct UnitQuaternion(pub [f32; 4]);

impl Default for UnitQuaternion {
    fn default() -> Self {
        UnitQuaternion([0.0, 0.0, 0.0, 1.0])
    }
}

impl Validate for UnitQuaternion {}
