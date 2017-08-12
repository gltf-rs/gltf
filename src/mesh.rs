
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::hash_map;
use std::{iter, slice};
use {json, material};

use accessor::Accessor;
use Gltf;

pub use json::mesh::{Mode, Semantic};

/// Vertex attribute data.
#[derive(Clone, Debug)]
pub enum Attribute<'a> {
    /// Vertex colors.
    Colors(u32, Accessor<'a>),

    #[cfg(feature = "extras")]
    Extras(&'a str, Accessor<'a>),

    /// Vertex joints.
    Joints(u32, Accessor<'a>),

    /// XYZ vertex positions of type `[f32; 3]`.
    Positions(Accessor<'a>),

    /// XYZ vertex normals of type `[f32; 3]`.
    Normals(Accessor<'a>),

    /// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
    /// sign value (-1 or +1) indicating the handedness of the tangent basis.
    Tangents(Accessor<'a>),

    /// UV texture co-ordinates.
    TexCoords(u32, Accessor<'a>),

    /// Weights.
    Weights(u32, Accessor<'a>),
}

/// Morph targets.
#[derive(Clone, Debug)]
pub struct MorphTargets<'a> {
    /// XYZ vertex position displacements.
    positions: Option<Accessor<'a>>,

    /// XYZ vertex normal displacements.
    normals: Option<Accessor<'a>>,

    /// XYZ vertex tangent displacements.
    tangents: Option<Accessor<'a>>,
}

/// A set of primitives to be rendered.  A node can contain one or more meshes and
/// its transform places the meshes in the scene.
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
    /// The parent `Primitive` struct.
    prim: &'a Primitive<'a>,

    /// The internal attribute iterIterator.
    iter: hash_map::Iter<'a, json::mesh::Semantic, json::Index<json::accessor::Accessor>>,
}

/// An `Iterator` that visits the primitives of a `Mesh`.
#[derive(Clone, Debug)]
pub struct Primitives<'a>  {
    /// The parent `Mesh` struct.
    mesh: &'a Mesh<'a>,

    /// The internal JSON primitive iterIterator.
    iter: iter::Enumerate<slice::Iter<'a, json::mesh::Primitive>>,
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
    pub fn as_json(&self) ->  &json::mesh::Primitive {
        self.json
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Returns the material to apply to this primitive when rendering
    pub fn material(&self) -> material::Material<'a> {
        self.json.material
            .as_ref()
            .map(|index| self.mesh.gltf.materials().nth(index.value()).unwrap())
            .unwrap_or_else(|| material::Material::default(self.mesh.gltf))
    }

    /// The type of primitives to render.
    pub fn mode(&self) -> Mode {
        self.json.mode.unwrap()
    }
}

impl<'a> Iterator for Primitives<'a> {
    type Item = Primitive<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, json)| Primitive::new(self.mesh, index, json))
    }
}
