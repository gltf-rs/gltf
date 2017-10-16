
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{collections, iter, slice};
use json;

use {Accessor, Gltf, Material};

pub use json::mesh::{Mode, Semantic};
use json::validation::Checked;

/// Vertex attribute data.
pub type Attribute<'a> = (Semantic, Accessor<'a>);

/// A single morph target for a mesh primitive.
#[derive(Clone, Debug)]
pub struct MorphTarget<'a> {
    /// XYZ vertex position displacements.
    positions: Option<Accessor<'a>>,

    /// XYZ vertex normal displacements.
    normals: Option<Accessor<'a>>,

    /// XYZ vertex tangent displacements.
    tangents: Option<Accessor<'a>>,
}

/// An `Iterator` that visits the morph targets of a `Primitive`.
#[derive(Clone, Debug)]
pub struct MorphTargets<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The internal JSON iterator.
    iter: slice::Iter<'a, json::mesh::MorphTarget>,
}

/// A set of primitives to be rendered.
#[derive(Clone, Debug)]
pub struct Mesh<'a>  {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Mesh,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug)]
pub struct Primitive<'a>  {
    /// The parent `Mesh` struct.
    mesh: &'a Mesh<'a>,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Primitive,
}

/// An `Iterator` that visits the attributes of a `Primitive`.
#[derive(Clone, Debug)]
pub struct Attributes<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The parent `Primitive` struct.
    prim: &'a Primitive<'a>,

    /// The internal attribute iterator.
    iter: collections::hash_map::Iter<
        'a,
        json::validation::Checked<json::mesh::Semantic>,
        json::Index<json::accessor::Accessor>,
    >,
}

/// An `Iterator` that visits the primitives of a `Mesh`.
#[derive(Clone, Debug)]
pub struct Primitives<'a>  {
    /// The parent `Mesh` struct.
    mesh: &'a Mesh<'a>,

    /// The internal JSON primitive iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::mesh::Primitive>>,
}

/// The minimum and maximum values for a generic accessor.
#[derive(Clone, Debug, PartialEq)]
pub struct Bounds<T> {
    /// Minimum value.
    pub min: T,

    /// Maximum value.
    pub max: T,
}

impl<'a> Mesh<'a>  {
    /// Constructs a `Mesh`.
    pub(crate) fn new(
        gltf: &'a Gltf,
        index: usize,
        json: &'a json::mesh::Mesh,
    ) -> Self {
        Self {
            gltf: gltf,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the internal JSON item.
    #[doc(hidden)]
    pub fn as_json(&self) ->  &json::mesh::Mesh {
        self.json
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Defines the geometry to be renderered with a material.
    pub fn primitives(&'a self) -> Primitives<'a> {
        Primitives {
            mesh: self,
            iter: self.json.primitives.iter().enumerate(),
        }
    }

    /// Defines the weights to be applied to the morph targets.
    pub fn weights(&self) -> Option<&[f32]> {
        self.json.weights.as_ref().map(Vec::as_slice)
    }
}

impl<'a> Primitive<'a> {
    /// Constructs a `Primitive`.
    pub(crate) fn new(
        mesh: &'a Mesh<'a>,
        index: usize,
        json: &'a json::mesh::Primitive,
    ) -> Self {
        Self {
            mesh: mesh,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    #[doc(hidden)]
    pub fn as_json(&self) ->  &json::mesh::Primitive {
        self.json
    }

    /// Returns the bounds of the `POSITION` vertex attribute when provided.
    ///
    /// # Panics
    ///
    /// Panics for `POSITION` accessors with missing bounds.
    ///
    /// Since `POSITION` accessors must include bounds information, one can
    /// call `Gltf::validate_minimally` to ensure this data exists.
    pub fn position_bounds(&self) -> Option<Bounds<[f32; 3]>> {
        if let Some(pos_accessor_index) = self.json.attributes.get(&Checked::Valid(Semantic::Positions)) {
            let pos_accessor = self.mesh.gltf.accessors().nth(pos_accessor_index.value()).unwrap();
            // NOTE: cannot panic if validated "minimally"
            let min: [f32; 3] = json::from_value(pos_accessor.min().unwrap()).unwrap();
            let max: [f32; 3] = json::from_value(pos_accessor.max().unwrap()).unwrap();
            Some(Bounds {
                min: [min[0], min[1], min[2]],
                max: [max[0], max[1], max[2]]
            })
        } else {
            None
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Return the accessor with the given semantic.
    pub fn get(&self, semantic: &Semantic) -> Option<Accessor> {
        self.json.attributes
            .get(&json::validation::Checked::Valid(semantic.clone()))
            .map(|index| self.mesh.gltf.accessors().nth(index.value()).unwrap())
    }

    /// Returns the accessor containing the primitive indices, if provided.
    pub fn indices(&self) -> Option<Accessor> {
        self.json.indices
            .as_ref()
            .map(|index| self.mesh.gltf.accessors().nth(index.value()).unwrap())
    }

    /// Returns an `Iterator` that visits the vertex attributes.
    pub fn attributes(&self) -> Attributes {
        Attributes {
            gltf: self.mesh.gltf,
            prim: self,
            iter: self.json.attributes.iter(),
        }
    }

    /// Returns the material to apply to this primitive when rendering
    pub fn material(&self) -> Material {
        self.json.material
            .as_ref()
            .map(|index| self.mesh.gltf.materials().nth(index.value()).unwrap())
            .unwrap_or_else(|| Material::default(self.mesh.gltf))
    }

    /// The type of primitives to render.
    pub fn mode(&self) -> Mode {
        self.json.mode.unwrap()
    }

    /// Returns an `Iterator` that visits the morph targets of the primitive.
    pub fn morph_targets(&self) -> MorphTargets {
        if let Some(slice) = self.json.targets.as_ref() {
            MorphTargets {
                gltf: self.mesh.gltf,
                iter: slice.iter(),
            }
        } else {
            MorphTargets {
                gltf: self.mesh.gltf,
                iter: (&[]).iter(),
            }
        }
    }
}

impl<'a> MorphTarget<'a> {
    /// Returns the XYZ vertex position displacements.
    pub fn positions(&self) -> Option<Accessor<'a>> {
        self.positions.clone()
    }

    /// Returns the XYZ vertex normal displacements.
    pub fn normals(&self) -> Option<Accessor<'a>> {
        self.normals.clone()
    }

    /// Returns the XYZ vertex tangent displacements.
    pub fn tangents(&self) -> Option<Accessor<'a>> {
        self.tangents.clone()
    }
}

impl<'a> ExactSizeIterator for Attributes<'a> {}
impl<'a> Iterator for Attributes<'a> {
    type Item = Attribute<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(key, index)| {
                let semantic = key.as_ref().unwrap().clone();
                let accessor = self.gltf.accessors().nth(index.value()).unwrap();
                (semantic, accessor)
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Primitives<'a> {}
impl<'a> Iterator for Primitives<'a> {
    type Item = Primitive<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, json)| Primitive::new(self.mesh, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for MorphTargets<'a> {}
impl<'a> Iterator for MorphTargets<'a> {
    type Item = MorphTarget<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|json| {
                let positions = json.positions
                    .as_ref()
                    .map(|index| self.gltf.accessors().nth(index.value()).unwrap());
                let normals = json.normals
                    .as_ref()
                    .map(|index| self.gltf.accessors().nth(index.value()).unwrap());
                let tangents = json.tangents
                    .as_ref()
                    .map(|index| self.gltf.accessors().nth(index.value()).unwrap());
                MorphTarget {
                    positions,
                    normals,
                    tangents,
                }
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
