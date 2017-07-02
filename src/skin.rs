
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::slice;
use {accessor, extensions, json, scene, Gltf};

/// Joints and matrices defining a skin.
#[derive(Clone, Debug)]
pub struct Skin<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::skin::Skin,
}

/// An `Iterator` that visits the inverse bind matrices of a `Skin`.
#[derive(Clone, Debug)]
pub struct InverseBindMatrices<'a>(accessor::Iter<'a, [[f32; 4]; 4]>);

/// An `Iterator` that visits the joints of a `Skin`.
#[derive(Clone, Debug)]
pub struct Joints<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The internal node index iterIterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

impl<'a> Skin<'a> {
    /// Constructs a `Skin`.
    pub fn new(gltf: &'a Gltf, json: &'a json::skin::Skin) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::skin::Skin {
        self.json
    }

    /// Extension specific data.
    pub fn extensions(&self) -> extensions::skin::Skin {
        extensions::skin::Skin::new(
            self.gltf,
            &self.json.extensions,
        )
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// The index of the accessor containing the 4x4 inverse-bind matrices.  When
    /// `None`,each matrix is assumed to be the 4x4 identity matrix which implies
    /// that the inverse-bind matrices were pre-applied.
    pub fn inverse_bind_matrices(&self) -> Option<InverseBindMatrices<'a>> {
        self.json.inverse_bind_matrices.as_ref().map(|index| {
            let accessor = self.gltf.accessors().nth(index.value()).unwrap();
            InverseBindMatrices(unsafe {
                accessor.iter()
            })
        })
    }

    /// Indices of skeleton nodes used as joints in this skin.  The array length
    /// must be the same as the `count` property of the `inverse_bind_matrices`
    /// `Accessor` (when defined).
    pub fn joints(&self) -> Joints<'a> {
        Joints {
            gltf: self.gltf,
            iter: self.json.joints.iter(),
        }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// The index of the node used as a skeleton root.  When `None`, joints
    /// transforms resolve to scene root.
    pub fn skeleton(&self) -> Option<scene::Node<'a>> {
        self.json.skeleton.as_ref().map(|index| {
            scene::Node::new(self.gltf, self.gltf.as_json().get(index))
        })
    }
}

impl<'a> Iterator for InverseBindMatrices<'a>  {
    type Item = [[f32; 4]; 4];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for Joints<'a>  {
    type Item = scene::Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.gltf.nodes().nth(index.value()).unwrap())
    }
}

