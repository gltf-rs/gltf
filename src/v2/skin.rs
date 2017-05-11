
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{accessor, scene, Extras, Index, Root};

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Skin {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: SkinExtensions,
    
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
    pub name: Option<String>,
    
    /// The index of the node used as a skeleton root.
    ///
    /// When `None`, joints transforms resolve to scene root.
    pub skeleton: Option<Index<scene::Node>>,
}

/// Extension specific data for `Skin`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SkinExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl Skin {
    #[doc(hidden)]
    pub fn range_check(&self, root: &Root) -> Result<(), ()> {
        if let Some(ref accessor) = self.inverse_bind_matrices {
            let _ = root.try_get(accessor)?;
        }
        for joint in &self.joints {
            let _ = root.try_get(joint)?;
        }
        if let Some(ref node) = self.skeleton {
            let _ = root.try_get(node)?;
        }
        Ok(())
    }
}
