use crate::{accessor, scene, Extras, Index, UnrecognizedExtensions};

/// Joints and matrices defining a skin.
#[derive(
    Clone,
    Debug,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Stub,
    gltf_derive::Validate,
)]
pub struct Skin {
    /// The index of the accessor containing the 4x4 inverse-bind matrices.
    ///
    /// When `None`,each matrix is assumed to be the 4x4 identity matrix
    /// which implies that the inverse-bind matrices were pre-applied.
    pub inverse_bind_matrices: Option<Index<accessor::Accessor>>,

    /// Indices of skeleton nodes used as joints in this skin.
    ///
    /// The array length must be the same as the `count` property of the
    /// `inverse_bind_matrices` `Accessor` (when defined).
    pub joints: Vec<Index<scene::Node>>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The index of the node used as a skeleton root.
    ///
    /// When `None`, joints transforms resolve to scene root.
    pub skeleton: Option<Index<scene::Node>>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}
