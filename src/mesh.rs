
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::hash_map;
use std::slice;
use {accessor, extensions, json, material, Gltf};

use self::accessor::{Accessor, DataType, Dimensions, Iter};

pub use json::mesh::{Mode, Semantic};

/// XYZ vertex normals of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct Normals<'a>(Iter<'a, [f32; 3]>);

/// XYZ vertex normal displacements of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct NormalDisplacements<'a>(Iter<'a, [f32; 3]>);

/// XYZ vertex positions of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct Positions<'a>(Iter<'a, [f32; 3]>);

/// XYZ vertex position displacements of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct PositionDisplacements<'a>(Iter<'a, [f32; 3]>);

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
#[derive(Clone, Debug)]
pub struct Tangents<'a>(Iter<'a, [f32; 4]>);

/// XYZ vertex tangent displacements of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct TangentDisplacements<'a>(Iter<'a, [f32; 3]>);

/// Vertex colors.
#[derive(Clone, Debug)]
pub enum Colors<'a> {
    /// RGB vertex color of type `[u8; 3]>`.
    RgbU8(Iter<'a, [u8; 3]>),

    /// RGBA vertex color of type `[u8; 4]>`.
    RgbaU8(Iter<'a, [u8; 4]>),

    /// RGB vertex color of type `[u16; 3]>`.
    RgbU16(Iter<'a, [u16; 3]>),

    /// RGBA vertex color of type `[u16; 4]>`.
    RgbaU16(Iter<'a, [u16; 4]>),

    /// RGB vertex color of type `[f32; 3]`.
    RgbF32(Iter<'a, [f32; 3]>),

    /// RGBA vertex color of type `[f32; 4]`.
    RgbaF32(Iter<'a, [f32; 4]>),
}

/// Index data.
#[derive(Clone, Debug)]
pub enum Indices<'a> {
    /// Index data of type U8
    U8(Iter<'a, u8>),
    /// Index data of type U16
    U16(Iter<'a, u16>),
    /// Index data of type U32
    U32(Iter<'a, u32>),
}

/// Vertex joints.
/// Refer to the documentation on morph targets and skins for more
/// information.
#[derive(Clone, Debug)]
pub enum Joints<'a> {
    /// Joints of type `[u8; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U8(Iter<'a, [u8; 4]>),
    
    /// Joints of type `[u16; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U16(Iter<'a, [u16; 4]>),
}

/// UV texture co-ordinates.
#[derive(Clone, Debug)]
pub enum TexCoords<'a> {
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(Iter<'a, [f32; 2]>),

    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(Iter<'a, [u8; 2]>),

    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(Iter<'a, [u16; 2]>),
}

/// Weights,
/// Refer to the documentation on morph targets for more information.
#[derive(Clone, Debug)]
pub enum Weights<'a> {
    /// Weights of type `[f32; 4]`.
    F32(Iter<'a, [f32; 4]>),

    /// Weights of type `[u8; 4]`.
    U8(Iter<'a, [u8; 4]>),

    /// Weights of type `[u16; 4]`.
    U16(Iter<'a, [u16; 4]>),
}

/// Vertex attribute data.
#[derive(Clone, Debug)]
pub enum Attribute<'a> {
    /// Vertex colors.
    Colors(u32, Colors<'a>),

    /// Untyped user-defined vertex attributes.
    #[cfg(feature = "extras")]
    Extras(&'a str, accessor::Accessor<'a>),

    /// Vertex joints.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    Joints(u32, Joints<'a>),

    /// XYZ vertex positions of type `[f32; 3]`.
    Positions(Positions<'a>),

    /// XYZ vertex normals of type `[f32; 3]`.
    Normals(Normals<'a>),

    /// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
    /// sign value (-1 or +1) indicating the handedness of the tangent basis.
    Tangents(Tangents<'a>),

    /// UV texture co-ordinates.
    TexCoords(u32, TexCoords<'a>),

    /// Weights.
    /// Refer to the documentation on morph targets for more information.
    Weights(u32, Weights<'a>),
}

/// Morph targets.
#[derive(Clone, Debug)]
pub struct MorphTarget<'a> {
    /// XYZ vertex position displacements.
    positions: Option<PositionDisplacements<'a>>,

    /// XYZ vertex normal displacements.
    normals: Option<NormalDisplacements<'a>>,

    /// XYZ vertex tangent displacements.
    tangents: Option<TangentDisplacements<'a>>,
}

/// A set of primitives to be rendered.  A node can contain one or more meshes and
/// its transform places the meshes in the scene.
#[derive(Clone, Debug)]
pub struct Mesh<'a>  {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Mesh,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug)]
pub struct Primitive<'a>  {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Primitive,
}

/// An `Iterator` that visits the attributes of a `Primitive`.
#[derive(Clone, Debug)]
pub struct Attributes<'a> {
    /// The parent `Primitive` struct.
    prim: Primitive<'a>,

    /// The internal attribute iterIterator.
    iter: hash_map::Iter<'a, json::mesh::Semantic, json::Index<json::accessor::Accessor>>,
}

/// An `Iterator` that visits the primitives of a `Mesh`.
#[derive(Clone, Debug)]
pub struct Primitives<'a>  {
    /// The parent `Mesh` struct.
    mesh: Mesh<'a>,

    /// The internal JSON primitive iterIterator.
    iter: slice::Iter<'a, json::mesh::Primitive>,
}

/// An `Iterator` that visits the Morph targets of a `Primitive`.
#[derive(Clone, Debug)]
pub struct MorphTargets<'a>  {
    /// The parent `Primitive` struct.
    prim: &'a Primitive<'a>,

    /// The internal Morph target iterIterator.
    iter: slice::Iter<'a, json::mesh::MorphTargets>,
}

impl<'a>  Mesh<'a>  {
    /// Constructs a `Mesh`.
    pub fn new(gltf: &'a Gltf, json: &'a json::mesh::Mesh) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
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
    pub fn primitives(&self) -> Primitives {
        Primitives {
            mesh: self.clone(),
            iter: self.json.primitives.iter(),
        }
    }

    /// Defines the weights to be applied to the morph targets.
    pub fn weights(&self) -> Option<&[f32]> {
        self.json.weights.as_ref().map(Vec::as_slice)
    }
}

impl<'a> MorphTarget<'a> {
    /// Returns the XYZ position displacements.
    pub fn positions(&self) -> Option<PositionDisplacements<'a>> {
        self.positions.clone()
    }

    /// Returns the XYZ normal displacements.
    pub fn normals(&self) -> Option<NormalDisplacements<'a>> {
        self.normals.clone()
    }

    /// Returns the XYZ tangent displacements.
    pub fn tangents(&self) -> Option<TangentDisplacements<'a>> {
        self.tangents.clone()
    }
}

impl<'a> Colors<'a> {
    fn from_accessor(accessor: Accessor<'a>) -> Self {
        match (accessor.dimensions(), accessor.data_type()) {
            (Dimensions::Vec3, DataType::U8) => {
                Colors::RgbU8(unsafe {
                    accessor.iter()
                })
            },
            (Dimensions::Vec4, DataType::U8) => {
                Colors::RgbaU8(unsafe {
                    accessor.iter()
                })
            },
            (Dimensions::Vec3, DataType::U16) => {
                Colors::RgbU16(unsafe {
                    accessor.iter()
                })
            },
            (Dimensions::Vec4, DataType::U16) => {
                Colors::RgbaU16(unsafe {
                    accessor.iter()
                })
            },
            (Dimensions::Vec3, DataType::F32) => {
                Colors::RgbF32(unsafe {
                    accessor.iter()
                })
            },
            (Dimensions::Vec4, DataType::F32) => {
                Colors::RgbaF32(unsafe {
                    accessor.iter()
                })
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> TexCoords<'a> {
    fn from_accessor(accessor: Accessor<'a>) -> TexCoords<'a> {
        match accessor.data_type() {
            DataType::U8 => {
                TexCoords::U8(unsafe {
                    accessor.iter()
                })
            },
            DataType::U16 => {
                TexCoords::U16(unsafe {
                    accessor.iter()
                })
            },
            DataType::F32 => {
                TexCoords::F32(unsafe {
                    accessor.iter()
                })
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> Indices<'a> {
    fn from_accessor(accessor: Accessor<'a>) -> Indices<'a> {
        match accessor.data_type() {
            DataType::U8 => {
                Indices::U8(unsafe {
                    accessor.iter()
                })
            },
            DataType::U16 => {
                Indices::U16(unsafe {
                    accessor.iter()
                })
            },
            DataType::U32 => {
                Indices::U32(unsafe {
                    accessor.iter()
                })
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> Joints<'a> {
    fn from_accessor(accessor: Accessor<'a>) -> Joints<'a> {
        match accessor.data_type() {
            DataType::U8 => {
                Joints::U8(unsafe {
                    accessor.iter()
                })
            },
            DataType::U16 => {
                Joints::U16(unsafe {
                    accessor.iter()
                })
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> Weights<'a> {
    fn from_accessor(accessor: Accessor<'a>) -> Weights<'a> {
        match accessor.data_type() {
            DataType::U8 => {
                Weights::U8(unsafe {
                    accessor.iter()
                })
            },
            DataType::U16 => {
                Weights::U16(unsafe {
                    accessor.iter()
                })
            },
            DataType::F32 => {
                Weights::F32(unsafe {
                    accessor.iter()
                })
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> Primitive<'a> {
    /// Constructs a `Primitive`.
    pub fn new(gltf: &'a Gltf, json: &'a json::mesh::Primitive) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::mesh::Primitive {
        self.json
    }

    /// Returns the vertex colors of the given set.
    pub fn colors(&'a self, set: u32) -> Option<Colors<'a>> {
        self.find_accessor_with_semantic(Semantic::Colors(set))
            .map(|accessor| {
                Colors::from_accessor(accessor)
            })
    }

    /// Returns the vertex texture co-ordinates of the given set.
    pub fn tex_coords(&'a self, set: u32) -> Option<TexCoords<'a>> {
        self.find_accessor_with_semantic(Semantic::TexCoords(set))
            .map(|accessor| {
                TexCoords::from_accessor(accessor)
            })
    }

    /// Returns the joint indices of the given set.
    pub fn joints(&'a self, set: u32) -> Option<Joints<'a>> {
        self.find_accessor_with_semantic(Semantic::Joints(set))
            .map(|accessor| {
                Joints::from_accessor(accessor)
            })
    }
    
    /// Returns the joint weights of the given set.
    pub fn weights(&'a self, set: u32) -> Option<Weights<'a>> {
        self.find_accessor_with_semantic(Semantic::Weights(set))
            .map(|accessor| {
                Weights::from_accessor(accessor)
            })
    }

    /// Returns the primitive indices.
    pub fn indices(&'a self) -> Option<Indices<'a>> {
        self.json.indices.as_ref().map(|index| {
            let accessor = self.gltf.accessors().nth(index.value()).unwrap();
            Indices::from_accessor(accessor)
        })
    }
    
    /// Returns the primitive positions.
    pub fn positions(&'a self) -> Option<Positions<'a>> {
        self.find_accessor_with_semantic(Semantic::Positions)
            .map(|accessor| {
                Positions(unsafe {
                    accessor.iter()
                })
            })
    }

    /// Returns the primitive normals.
    pub fn normals(&'a self) -> Option<Normals<'a>> {
        self.find_accessor_with_semantic(Semantic::Normals)
            .map(|accessor| {
                Normals(unsafe {
                    accessor.iter()
                })
            })
    }

    /// Returns the primitive tangents.
    pub fn tangents(&'a self) -> Option<Tangents<'a>> {
        self.find_accessor_with_semantic(Semantic::Tangents)
            .map(|accessor| {
                Tangents(unsafe {
                    accessor.iter()
                })
            })
    }

    /// Returns the attribute with the given semantic value.
    fn find_accessor_with_semantic(
        &self,
        _semantic: Semantic,
    ) -> Option<accessor::Accessor> {
        unimplemented!()
    }
    
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub fn attributes(&self) -> Attributes<'a> {
        unimplemented!()
    }

    /// Extension specific data.
    pub fn extensions(&self) -> extensions::mesh::Primitive<'a> {
        extensions::mesh::Primitive::new(
            self.gltf,
            &self.json.extensions,
        )
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// The material to apply to this primitive when rendering
    pub fn material(&self) -> Option<material::Material<'a>> {
        self.json.material.as_ref().map(|index| {
            material::Material::new(self.gltf, self.gltf.as_json().get(index))
        })
    }

    /// The type of primitives to render.
    pub fn mode(&self) -> Mode {
        self.json.mode.unwrap()
    }

    /// Returns an iterIterator over the primitive Morph Targets.
    pub fn morph_targets(&'a self) -> Option<MorphTargets<'a>> {
        self.json.targets.as_ref().map(|targets| {
            MorphTargets {
                prim: self,
                iter: targets.iter(),
            }
        })
    }
}

impl<'a> Iterator for Attributes<'a> {
    type Item = (&'a Semantic, Attribute<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(semantic, index)| {
            use self::*;
            let accessor = self.prim.gltf
                .accessors()
                .nth(index.value())
                .unwrap();
            let attribute = match semantic {
                &Semantic::Positions => {
                    Attribute::Positions(unsafe {
                        Positions(accessor.iter())
                    })
                },
                &Semantic::Normals => {
                    Attribute::Normals(unsafe {
                        Normals(accessor.iter())
                    })
                },
                &Semantic::Tangents => {
                    Attribute::Tangents(unsafe {
                        Tangents(accessor.iter())
                    })
                },
                &Semantic::Colors(set) => {
                    Attribute::Colors(set, Colors::from_accessor(accessor))
                },
                &Semantic::TexCoords(set) => {
                    Attribute::TexCoords(set, TexCoords::from_accessor(accessor))
                },
                &Semantic::Joints(set) => {
                    Attribute::Joints(set, Joints::from_accessor(accessor))
                },
                &Semantic::Weights(set) => {
                    Attribute::Weights(set, Weights::from_accessor(accessor))
                },
                #[cfg(feature = "extras")]
                &Semantic::Extras(ref name) => {
                    Attribute::Extras(name, accessor)
                },
            };
            (semantic, attribute)
        })
    }
}

impl<'a> Iterator for Positions<'a> {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for PositionDisplacements<'a> {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for Normals<'a> {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
 
impl<'a> Iterator for NormalDisplacements<'a> {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for Tangents<'a> {
    type Item = [f32; 4];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for TangentDisplacements<'a> {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for Primitives<'a> {
    type Item = Primitive<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Primitive::new(self.mesh.gltf, json))
    }
}

impl<'a> Iterator for MorphTargets<'a> {
    type Item = MorphTarget<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
