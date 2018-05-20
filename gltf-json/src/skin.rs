use {accessor, extensions, scene, Extras, Index};

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Skin {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::skin::Skin,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// The index of the accessor containing the 4x4 inverse-bind matrices.
    ///
    /// When `None`,each matrix is assumed to be the 4x4 identity matrix
    /// which implies that the inverse-bind matrices were pre-applied.
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<Index<accessor::Accessor>>,
    
    /// Indices of skeleton nodes used as joints in this skin.
    ///
    /// The array length must be the same as the `count` property of the
    /// `inverse_bind_matrices` `Accessor` (when defined).
    pub joints: Vec<Index<scene::Node>>,
    
    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub name: Option<String>,
    
    /// The index of the node used as a skeleton root.
    ///
    /// When `None`, joints transforms resolve to scene root.
    pub skeleton: Option<Index<scene::Node>>,
}
