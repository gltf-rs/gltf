
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::collections::hash_map;
use std::slice;
use {accessor, json, material, Gltf};

use self::accessor::{Accessor, DataType, Dimensions, Iter};

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
pub enum Attribute {
    /// Vertex colors.
    Colors(u32, Colors),

    /// Untyped user-defined vertex attributes.
    #[cfg(feature = "extras")]
    Extras(String, accessor::Accessor),

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
    prim: Primitive,

    /// The internal attribute iterIterator.
    iter: hash_map::Iter<'a, json::mesh::Semantic, json::Index<json::accessor::Accessor>>,
}

/// An `Iterator` that visits the primitives of a `Mesh`.
#[derive(Clone, Debug)]
pub struct Primitives<'a>  {
    /// The parent `Mesh` struct.
    mesh: Mesh,

    /// The internal JSON primitive iterIterator.
    iter: slice::Iter<'a, json::mesh::Primitive>,
}

/// An `Iterator` that visits the Morph targets of a `Primitive`.
#[derive(Clone, Debug)]
pub struct MorphTargets<'a>  {
    /// The parent `Primitive` struct.
    prim: &'a Primitive,

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
    pub fn extensions(&self) -> &json::mesh::MeshExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
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

impl MorphTarget {
    /// Returns the XYZ position displacements.
    pub fn positions(&self) -> Option<PositionDisplacements> {
        self.positions.clone()
    }

    /// Returns the XYZ normal displacements.
    pub fn normals(&self) -> Option<NormalDisplacements> {
        self.normals.clone()
    }

    /// Returns the XYZ tangent displacements.
    pub fn tangents(&self) -> Option<TangentDisplacements> {
        self.tangents.clone()
    }
}

impl Colors {
    fn from_accessor(accessor: Accessor) -> Self {
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

impl TexCoords {
    fn from_accessor(accessor: Accessor) -> TexCoords {
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

impl Indices {
    fn from_accessor(accessor: Accessor) -> Indices {
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

impl Joints {
    fn from_accessor(accessor: Accessor) -> Joints {
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

impl Weights {
    fn from_accessor(accessor: Accessor) -> Weights {
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

impl Primitive {
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
    pub fn colors(&'a self, set: u32) -> Option<Colors> {
        self.find_accessor_with_semantic(Semantic::Colors(set))
            .map(|accessor| {
                Colors::from_accessor(accessor)
            })
    }

    /// Returns the vertex texture co-ordinates of the given set.
    pub fn tex_coords(&'a self, set: u32) -> Option<TexCoords> {
        self.find_accessor_with_semantic(Semantic::TexCoords(set))
            .map(|accessor| {
                TexCoords::from_accessor(accessor)
            })
    }

    /// Returns the joint indices of the given set.
    pub fn joints(&'a self, set: u32) -> Option<Joints> {
        self.find_accessor_with_semantic(Semantic::Joints(set))
            .map(|accessor| {
                Joints::from_accessor(accessor)
            })
    }
    
    /// Returns the joint weights of the given set.
    pub fn weights(&'a self, set: u32) -> Option<Weights> {
        self.find_accessor_with_semantic(Semantic::Weights(set))
            .map(|accessor| {
                Weights::from_accessor(accessor)
            })
    }

    /// Returns the primitive indices.
    pub fn indices(&'a self) -> Option<Indices> {
        self.json.indices.as_ref().map(|index| {
            let accessor = self.gltf.accessors().nth(index.value()).unwrap();
            Indices::from_accessor(accessor)
        })
    }
    
    /// Returns the primitive positions.
    pub fn positions(&'a self) -> Option<Positions> {
        self.find_accessor_with_semantic(Semantic::Positions)
            .map(|accessor| {
                Positions(unsafe {
                    accessor.iter()
                })
            })
    }

    /// Returns the primitive normals.
    pub fn normals(&'a self) -> Option<Normals> {
        self.find_accessor_with_semantic(Semantic::Normals)
            .map(|accessor| {
                Normals(unsafe {
                    accessor.iter()
                })
            })
    }

    /// Returns the primitive tangents.
    pub fn tangents(&'a self) -> Option<Tangents> {
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
        semantic: Semantic,
    ) -> Option<accessor::Accessor> {
        for (json, index) in self.json.attributes.0.iter() {
            if Semantic::from_str(json.as_str()) == semantic {
                return Some(self.gltf.accessors().nth(index.value()).unwrap());
            }
        }
        None
    }
    
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub fn attributes(&self) -> Attributes {
        Attributes {
            prim: self.clone(),
            iter: self.json.attributes.0.iter(),
        }
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::mesh::PrimitiveExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// The material to apply to this primitive when rendering
    pub fn material(&self) -> Option<material::Material> {
        self.json.material.as_ref().map(|index| {
            material::Material::new(self.gltf, self.gltf.as_json().get(index))
        })
    }

    /// The type of primitives to render.
    pub fn mode(&self) -> Mode {
        use self::Mode::*;
        match self.json.mode.0 {
            json::mesh::POINTS => Points,
            json::mesh::LINES => Lines,
            json::mesh::LINE_LOOP => LineLoop,
            json::mesh::LINE_STRIP => LineStrip,
            json::mesh::TRIANGLES => Triangles,
            json::mesh::TRIANGLE_STRIP => TriangleStrip,
            json::mesh::TRIANGLE_FAN => TriangleFan,
            _ => unreachable!(),
        }
    }

    /// Returns an iterIterator over the primitive Morph Targets.
    pub fn morph_targets(&'a self) -> Option<MorphTargets> {
        self.json.targets.as_ref().map(|targets| {
            MorphTargets {
                prim: self,
                iter: targets.iter(),
            }
        })
    }
}

impl Iterator for Attributes {
    type Item = (Semantic, Attribute);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(json, index)| {
            use self::*;
            let semantic = Semantic::from_str(json.as_str());
            let accessor = self.prim.gltf
                .accessors()
                .nth(index.value())
                .unwrap();
            let attribute = match semantic {
                Semantic::Positions => {
                    Attribute::Positions(unsafe {
                        Positions(accessor.iter())
                    })
                },
                Semantic::Normals => {
                    Attribute::Normals(unsafe {
                        Normals(accessor.iter())
                    })
                },
                Semantic::Tangents => {
                    Attribute::Tangents(unsafe {
                        Tangents(accessor.iter())
                    })
                },
                Semantic::Colors(set) => {
                    Attribute::Colors(set, Colors::from_accessor(accessor))
                },
                Semantic::TexCoords(set) => {
                    Attribute::TexCoords(set, TexCoords::from_accessor(accessor))
                },
                Semantic::Joints(set) => {
                    Attribute::Joints(set, Joints::from_accessor(accessor))
                },
                Semantic::Weights(set) => {
                    Attribute::Weights(set, Weights::from_accessor(accessor))
                },
            };
            (semantic, attribute)
        })
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

impl Iterator for Primitives {
    type Item = Primitive;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Primitive::new(self.mesh.gltf, json))
    }
}

impl Iterator for MorphTargets {
    type Item = MorphTarget;
    fn next(&mut self) -> Option<Self::Item> {
        let get_accessor = |gltf, idx| gltf.accessors().nth(idx.value()).unwrap();
        self.iter.next().map(|targets| {
            let semantic = |name| json::mesh::Semantic(Cow::from(name));
            let positions = targets.position_displacements.map(|index| {
                let accessor = get_accessor(self.prim.gltf, index);
                unsafe {
                    PositionDisplacements(accessor.iter())
                }
            });
            let normals = targets.0.get(&semantic("NORMAL")).map(|index| {
                let accessor = self.prim.gltf
                    .accessors()
                    .nth(index.value())
                    .unwrap();
                unsafe {
                    NormalDisplacements(accessor.iter())
                }
            });
            let tangents = targets.0.get(&semantic("TANGENT")).map(|index| {
                let accessor = self.prim.gltf
                    .accessors()
                    .nth(index.value())
                    .unwrap();
                unsafe {
                    TangentDisplacements(accessor.iter())
                }
            });
            MorphTarget {
                positions: positions,
                normals: normals,
                tangents: tangents,
            }
        })
    }
}
