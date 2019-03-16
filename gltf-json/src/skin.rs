use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};
use crate::{accessor, extensions, scene, Extras, Index};

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Skin {
    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::skin::Skin>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,

    /// The index of the accessor containing the 4x4 inverse-bind matrices.
    ///
    /// When `None`,each matrix is assumed to be the 4x4 identity matrix
    /// which implies that the inverse-bind matrices were pre-applied.
    #[serde(rename = "inverseBindMatrices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverse_bind_matrices: Option<Index<accessor::Accessor>>,

    /// Indices of skeleton nodes used as joints in this skin.
    ///
    /// The array length must be the same as the `count` property of the
    /// `inverse_bind_matrices` `Accessor` (when defined).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub joints: Vec<Index<scene::Node>>,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// The index of the node used as a skeleton root.
    ///
    /// When `None`, joints transforms resolve to scene root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skeleton: Option<Index<scene::Node>>,
}
