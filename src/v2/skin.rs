
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Extensions;
use traits::Extras;
use v2::{accessor, scene, Index, Root};

/// [Joints and matrices defining a skin](https://github.com/KhronosGroup/glTF/blob/d63b796e6b7f6b084c710b97b048d59d749cb04a/specification/2.0/schema/skin.schema.json)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Skin<E: Extras> {
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: Extensions,
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Skin,
    /// The index of the accessor containing the 4x4 inverse-bind matrices
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<Index<accessor::Accessor<E>>>,
    /// Indices of skeleton nodes used as joints in this skin
    pub joints: Vec<Index<scene::Node<E>>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The index of the node used as a skeleton root
    pub skeleton: Option<Index<scene::Node<E>>>,
}

impl<E: Extras> Skin<E> {
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
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

