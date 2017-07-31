
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::hash_map;
use std::{iter, slice};
use {accessor, extensions, json, material};

use accessor::{Accessor, DataType, Dimensions, Iter};
use Gltf;

pub use json::mesh::{Mode, Semantic};

/// XYZ vertex normals of type `[f32; 3]`.
#[derive(Debug)]
pub struct Normals(Iter<[f32; 3]>);

/// XYZ vertex normal displacements of type `[f32; 3]`.
#[derive(Debug)]
pub struct NormalDisplacements(Iter<[f32; 3]>);

/// XYZ vertex positions of type `[f32; 3]`.
#[derive(Debug)]
pub struct Positions(Iter<[f32; 3]>);

/// XYZ vertex position displacements of type `[f32; 3]`.
#[derive(Debug)]
pub struct PositionDisplacements(Iter<[f32; 3]>);

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
#[derive(Debug)]
pub struct Tangents(Iter<[f32; 4]>);

/// XYZ vertex tangent displacements of type `[f32; 3]`.
#[derive(Debug)]
pub struct TangentDisplacements(Iter<[f32; 3]>);

/// Vertex colors.
#[derive(Debug)]
pub enum Colors {
    /// RGB vertex color of type `[u8; 3]>`.
    RgbU8(Iter<[u8; 3]>),

    /// RGBA vertex color of type `[u8; 4]>`.
    RgbaU8(Iter<[u8; 4]>),

    /// RGB vertex color of type `[u16; 3]>`.
    RgbU16(Iter<[u16; 3]>),

    /// RGBA vertex color of type `[u16; 4]>`.
    RgbaU16(Iter<[u16; 4]>),

    /// RGB vertex color of type `[f32; 3]`.
    RgbF32(Iter<[f32; 3]>),

    /// RGBA vertex color of type `[f32; 4]`.
    RgbaF32(Iter<[f32; 4]>),
}

/// Index data.
#[derive(Debug)]
pub enum Indices {
    /// Index data of type U8
    U8(Iter<u8>),
    /// Index data of type U16
    U16(Iter<u16>),
    /// Index data of type U32
    U32(Iter<u32>),
}

/// Vertex joints.
/// Refer to the documentation on morph targets and skins for more
/// information.
#[derive(Debug)]
pub enum Joints {
    /// Joints of type `[u8; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U8(Iter<[u8; 4]>),
    
    /// Joints of type `[u16; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U16(Iter<[u16; 4]>),
}

/// UV texture co-ordinates.
#[derive(Debug)]
pub enum TexCoords {
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(Iter<[f32; 2]>),

    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(Iter<[u8; 2]>),

    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(Iter<[u16; 2]>),
}

/// Weights,
/// Refer to the documentation on morph targets for more information.
#[derive(Debug)]
pub enum Weights {
    /// Weights of type `[f32; 4]`.
    F32(Iter<[f32; 4]>),

    /// Weights of type `[u8; 4]`.
    U8(Iter<[u8; 4]>),

    /// Weights of type `[u16; 4]`.
    U16(Iter<[u16; 4]>),
}

/// Vertex attribute data.
#[derive(Debug)]
pub enum Attribute {
    /// Vertex colors.
    Colors(u32, Colors),

    // TODO: Handle extras (needs to be handled elsewhere to avoid taking lifetime)
    // #[cfg(feature = "extras")]
    // Extras(&'a str, accessor::Accessor),

    /// Vertex joints.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    Joints(u32, Joints),

    /// XYZ vertex positions of type `[f32; 3]`.
    Positions(Positions),

    /// XYZ vertex normals of type `[f32; 3]`.
    Normals(Normals),

    /// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
    /// sign value (-1 or +1) indicating the handedness of the tangent basis.
    Tangents(Tangents),

    /// UV texture co-ordinates.
    TexCoords(u32, TexCoords),

    /// Weights.
    /// Refer to the documentation on morph targets for more information.
    Weights(u32, Weights),
}

/// Morph targets.
#[derive(Debug)]
pub struct MorphTarget {
    /// XYZ vertex position displacements.
    positions: Option<PositionDisplacements>,

    /// XYZ vertex normal displacements.
    normals: Option<NormalDisplacements>,

    /// XYZ vertex tangent displacements.
    tangents: Option<TangentDisplacements>,
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
    pub fn new(gltf: &'a Gltf, index: usize, json: &'a json::mesh::Mesh) -> Self {
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

    /// Extension specific data.
    pub fn extensions(&self) -> extensions::mesh::Mesh<'a> {
        extensions::mesh::Mesh::new(
            self.gltf,
            &self.json.extensions,
        )
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

impl Colors {
    fn from_accessor<'a>(accessor: Accessor<'a>) -> Colors {
        unsafe {
            match (accessor.dimensions(), accessor.data_type()) {
                (Dimensions::Vec3, DataType::U8) => {
                    Colors::RgbU8(accessor.iter())
                },
                (Dimensions::Vec4, DataType::U8) => {
                    Colors::RgbaU8(accessor.iter())
                },
                (Dimensions::Vec3, DataType::U16) => {
                    Colors::RgbU16(accessor.iter())
                },
                (Dimensions::Vec4, DataType::U16) => {
                    Colors::RgbaU16(accessor.iter())
                },
                (Dimensions::Vec3, DataType::F32) => {
                    Colors::RgbF32(accessor.iter())
                },
                (Dimensions::Vec4, DataType::F32) => {
                    Colors::RgbaF32(accessor.iter())
                },
                _ => unreachable!(),
            }
        }
    }
}

impl TexCoords {
    fn from_accessor<'a>(accessor: Accessor<'a>) -> TexCoords {
        unsafe {
            match accessor.data_type() {
                DataType::U8 => TexCoords::U8(accessor.iter()),
                DataType::U16 => TexCoords::U16(accessor.iter()),
                DataType::F32 => TexCoords::F32(accessor.iter()),
                _ => unreachable!(),
            }
        }
    }
}

impl Indices {
    fn from_accessor<'a>(accessor: Accessor) -> Indices {
        unsafe {
            match accessor.data_type() {
                DataType::U8 => Indices::U8(accessor.iter()),
                DataType::U16 => Indices::U16(accessor.iter()),
                DataType::U32 => Indices::U32(accessor.iter()),
                _ => unreachable!(),
            }
        }
    }
}

impl Joints {
    fn from_accessor<'a>(accessor: Accessor<'a>) -> Joints {
        unsafe {
            match accessor.data_type() {
                DataType::U8 => Joints::U8(accessor.iter()),
                DataType::U16 => Joints::U16(accessor.iter()),
                _ => unreachable!(),
            }
        }
    }
}

impl Weights {
    fn from_accessor<'a>(accessor: Accessor<'a>) -> Weights {
        unsafe {
            match accessor.data_type() {
                DataType::U8 => Weights::U8(accessor.iter()),
                DataType::U16 => Weights::U16(accessor.iter()),
                DataType::F32 => Weights::F32(accessor.iter()),
                _ => unreachable!(),
            }
        }
    }
}

impl<'a> Primitive<'a> {
    /// Constructs a `Primitive`.
    pub fn new(mesh: &'a Mesh<'a>, index: usize, json: &'a json::mesh::Primitive) -> Self {
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

    /// Returns the vertex colors of the given set.
    pub fn colors(&self, set: u32) -> Option<Colors> {
        self.find_accessor_with_semantic(Semantic::Colors(set))
            .map(|accessor| Colors::from_accessor(accessor))
    }

    /// Returns the vertex texture co-ordinates of the given set.
    pub fn tex_coords(&self, set: u32) -> Option<TexCoords> {
        self.find_accessor_with_semantic(Semantic::TexCoords(set))
            .map(|accessor| TexCoords::from_accessor(accessor))
    }

    /// Returns the joint indices of the given set.
    pub fn joints(&self, set: u32) -> Option<Joints> {
        self.find_accessor_with_semantic(Semantic::Joints(set))
            .map(|accessor| Joints::from_accessor(accessor))
    }
    
    /// Returns the joint weights of the given set.
    pub fn weights(&self, set: u32) -> Option<Weights> {
        self.find_accessor_with_semantic(Semantic::Weights(set))
            .map(|accessor| Weights::from_accessor(accessor))
    }

    /// Returns the primitive indices.
    pub fn indices(&self) -> Option<Indices> {
        self.json.indices.as_ref().map(|index| {
            let accessor = self.mesh.gltf.accessors().nth(index.value()).unwrap();
            Indices::from_accessor(accessor)
        })
    }
    
    /// Returns the primitive positions.
    pub fn positions(&self) -> Option<Positions> {
        self.find_accessor_with_semantic(Semantic::Positions)
            .map(|accessor| unsafe {
                Positions(accessor.iter())
            })
    }

    /// Returns the primitive normals.
    pub fn normals(&self) -> Option<Normals> {
        self.find_accessor_with_semantic(Semantic::Normals)
            .map(|accessor| unsafe {
                Normals(accessor.iter())
            })
    }

    /// Returns the primitive tangents.
    pub fn tangents(&self) -> Option<Tangents> {
        self.find_accessor_with_semantic(Semantic::Tangents)
            .map(|accessor| unsafe {
                Tangents(accessor.iter())
            })
    }

    /// Returns the attribute with the given semantic value.
    fn find_accessor_with_semantic(
        &self,
        semantic: Semantic,
    ) -> Option<accessor::Accessor<'a>> {
        self.json.attributes
            .iter()
            .find(|&(ref key, _)| key.as_ref().unwrap() == &semantic)
            .map(|(_, index)| self.mesh.gltf.accessors().nth(index.value()).unwrap())
    }

    /// Extension specific data.
    pub fn extensions(&self) -> extensions::mesh::Primitive<'a> {
        extensions::mesh::Primitive::new(
            self.mesh,
            &self.json.extensions,
        )
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// The material to apply to this primitive when rendering
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

impl Iterator for Positions {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Iterator for PositionDisplacements {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Iterator for Normals {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
 
impl Iterator for NormalDisplacements {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Iterator for Tangents {
    type Item = [f32; 4];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Iterator for TangentDisplacements {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for Primitives<'a> {
    type Item = Primitive<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, json)| Primitive::new(self.mesh, index, json))
    }
}
