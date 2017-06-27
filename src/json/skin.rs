
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::marker::PhantomData;

use json::{accessor, scene, Extras, Index};

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Skin<'a> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: SkinExtensions<'a>,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
    
    /// The index of the accessor containing the 4x4 inverse-bind matrices.
    ///
    /// When `None`,each matrix is assumed to be the 4x4 identity matrix
    /// which implies that the inverse-bind matrices were pre-applied.
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<Index<accessor::Accessor<'a>>>,
    
    /// Indices of skeleton nodes used as joints in this skin.
    ///
    /// The array length must be the same as the `count` property of the
    /// `inverse_bind_matrices` `Accessor` (when defined).
    pub joints: Vec<Index<scene::Node<'a>>>,
    
    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,
    
    /// The index of the node used as a skeleton root.
    ///
    /// When `None`, joints transforms resolve to scene root.
    pub skeleton: Option<Index<scene::Node<'a>>>,
}

/// Extension specific data for `Skin`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct SkinExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}
