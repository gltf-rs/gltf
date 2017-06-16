
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
pub type Normal<'a> = Iter<'a, [f32; 3]>;

/// XYZ vertex positions of type `[f32; 3]`.
pub type Position<'a> = Iter<'a, [f32; 3]>;

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
pub type Tangent<'a> = Iter<'a, [f32; 4]>;

/// Vertex colors.
#[derive(Clone, Debug)]
pub enum Color<'a> {
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
pub enum TexCoord<'a> {
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
pub enum Attribute<'a> {
    /// Vertex colors.
    Color(u32, Color<'a>),

    /// Untyped user-defined vertex attributes.
    Extra(&'a str, accessor::Accessor<'a>),

    /// Vertex joints.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    Joints(u32, Joints<'a>),

    /// XYZ vertex positions of type `[f32; 3]`.
    Position(Position<'a>),

    /// XYZ vertex normals of type `[f32; 3]`.
    Normal(Normal<'a>),

    /// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
    /// sign value (-1 or +1) indicating the handedness of the tangent basis.
    Tangent(Tangent<'a>),

    /// UV texture co-ordinates.
    TexCoord(u32, TexCoord<'a>),

    /// Weights.
    /// Refer to the documentation on morph targets for more information.
    Weights(u32, Weights<'a>),
}

/// The type of primitives to render.
pub enum Mode {
    /// Corresponds to `GL_POINTS`.
    Points = 0,

    /// Corresponds to `GL_LINES`.
    Lines = 1,

    /// Corresponds to `GL_LINE_LOOP`.
    LineLoop = 2,

    /// Corresponds to `GL_LINE_STRIP`.
    LineStrip = 3,

    /// Corresponds to `GL_TRIANGLES`.
    Triangles = 4,

    /// Corresponds to `GL_TRIANGLE_STRIP`.
    TriangleStrip = 5,

    /// Corresponds to `GL_TRIANGLE_FAN`.
    TriangleFan = 6,
}

/// Vertex attribute semantic name.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Semantic {
    /// XYZ vertex positions.
    Position,

    /// XYZ vertex normals.
    Normal,

    /// XYZW vertex tangents where the `w` component is a sign value indicating the
    /// handedness of the tangent basis.
    Tangent,

    /// RGB or RGBA vertex color.
    Color(u32),

    /// UV texture co-ordinates.
    TexCoord(u32),

    /// Joint indices.
    Joints(u32),

    /// Joint weights.
    Weights(u32),
}

/// A set of primitives to be rendered.  A node can contain one or more meshes and
/// its transform places the meshes in the scene.
#[derive(Clone, Debug)]
pub struct Mesh<'a> {
    /// The parent `Gltf<'a>` struct.
    gltf: &'a Gltf<'a>,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Mesh<'a>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug)]
pub struct Primitive<'a> {
    /// The parent `Gltf<'a>` struct.
    gltf: &'a Gltf<'a>,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Primitive<'a>,
}

/// An `Iterator` that visits the attributes of a `Primitive`.
#[derive(Clone, Debug)]
pub struct IterAttributes<'a> {
    /// The parent `Primitive` struct.
    prim: Primitive<'a>,

    /// The internal attribute iterator.
    iter: hash_map::Iter<'a, json::mesh::Semantic<'a>, json::Index<json::accessor::Accessor<'a>>>,
}

/// An `Iterator` that visits the primitives of a `Mesh`.
#[derive(Clone, Debug)]
pub struct IterPrimitives<'a> {
    /// The parent `Mesh` struct.
    mesh: Mesh<'a>,

    /// The internal JSON primitive iterator.
    iter: slice::Iter<'a, json::mesh::Primitive<'a>>,
}

impl<'a> Mesh<'a> {
    /// Constructs a `Mesh`.
    pub fn new(gltf: &'a Gltf<'a>, json: &'a json::mesh::Mesh<'a>) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::mesh::Mesh<'a> {
        self.json
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::mesh::MeshExtensions<'a> {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras<'a> {
        &self.json.extras
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
    }

    /// Defines the geometry to be renderered with a material.
    pub fn iter_primitives(&self) -> IterPrimitives<'a> {
        IterPrimitives {
            mesh: self.clone(),
            iter: self.json.primitives.iter(),
        }
    }

    /// Defines the weights to be applied to the morph targets.
    pub fn weights(&self) -> Option<&[f32]> {
        self.json.weights.as_ref().map(Vec::as_slice)
    }
}

impl<'a> Color<'a> {
    fn from_accessor(accessor: Accessor<'a>) -> Self {
        match (accessor.dimensions(), accessor.data_type()) {
            (Dimensions::Vec3, DataType::U8) => {
                Color::RgbU8(unsafe {
                    accessor.iter()
                })
            },
            (Dimensions::Vec4, DataType::U8) => {
                Color::RgbaU8(unsafe {
                    accessor.iter()
                })
            },
            (Dimensions::Vec3, DataType::U16) => {
                Color::RgbU16(unsafe {
                    accessor.iter()
                })
            },
            (Dimensions::Vec4, DataType::U16) => {
                Color::RgbaU16(unsafe {
                    accessor.iter()
                })
            },
            (Dimensions::Vec3, DataType::F32) => {
                Color::RgbF32(unsafe {
                    accessor.iter()
                })
            },
            (Dimensions::Vec4, DataType::F32) => {
                Color::RgbaF32(unsafe {
                    accessor.iter()
                })
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> TexCoord<'a> {
    fn from_accessor(accessor: Accessor<'a>) -> TexCoord<'a> {
        match accessor.data_type() {
            DataType::U8 => {
                TexCoord::U8(unsafe {
                    accessor.iter()
                })
            },
            DataType::U16 => {
                TexCoord::U16(unsafe {
                    accessor.iter()
                })
            },
            DataType::F32 => {
                TexCoord::F32(unsafe {
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
    pub fn new(gltf: &'a Gltf<'a>, json: &'a json::mesh::Primitive<'a>) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::mesh::Primitive<'a> {
        self.json
    }

    /// Returns the vertex colors of the given set.
    pub fn color(&'a self, set: u32) -> Option<Color<'a>> {
        self.find_accessor_with_semantic(Semantic::Color(set))
            .map(|accessor| {
                Color::from_accessor(accessor)
            })
    }

    /// Returns the vertex texture co-ordinates of the given set.
    pub fn tex_coord(&'a self, set: u32) -> Option<TexCoord<'a>> {
        self.find_accessor_with_semantic(Semantic::TexCoord(set))
            .map(|accessor| {
                TexCoord::from_accessor(accessor)
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
            let accessor = self.gltf.iter_accessors().nth(index.value()).unwrap();
            Indices::from_accessor(accessor)
        })
    }
    
    /// Returns the primitive positions.
    pub fn position(&'a self) -> Option<Position<'a>> {
        self.find_accessor_with_semantic(Semantic::Position)
            .map(|accessor| {
                unsafe {
                    accessor.iter()
                }
            })
    }

    /// Returns the primitive normals.
    pub fn normal(&'a self) -> Option<Normal<'a>> {
        self.find_accessor_with_semantic(Semantic::Normal)
            .map(|accessor| {
                unsafe {
                    accessor.iter()
                }
            })
    }

    /// Returns the primitive tangents.
    pub fn tangent(&'a self) -> Option<Tangent<'a>> {
        self.find_accessor_with_semantic(Semantic::Tangent)
            .map(|accessor| {
                unsafe {
                    accessor.iter()
                }
            })
    }

    /// Returns the attribute with the given semantic value.
    fn find_accessor_with_semantic(
        &self,
        semantic: Semantic,
    ) -> Option<accessor::Accessor<'a>> {
        for (json, index) in self.json.attributes.0.iter() {
            if Semantic::from_str(json.as_str()) == semantic {
                return Some(self.gltf.iter_accessors().nth(index.value()).unwrap());
            }
        }
        None
    }
    
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub fn iter_attributes(&self) -> IterAttributes<'a> {
        IterAttributes {
            prim: self.clone(),
            iter: self.json.attributes.0.iter(),
        }
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::mesh::PrimitiveExtensions<'a> {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras<'a> {
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

    /// An array of Morph Targets, each Morph Target is a dictionary mapping
    /// attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their
    /// deviations in the Morph Target.
    pub fn targets(&self) -> ! {
        unimplemented!()
    }
}

impl Semantic {
    fn from_str(name: &str) -> Self {
        use self::Semantic::*;
        match &name[..2] {
            "NO" => Normal,
            "PO" => Position,
            "TA" => Tangent,
            "CO" => Color(name["COLOR_".len()..].parse().unwrap()),
            "TE" => TexCoord(name["TEXCOORD_".len()..].parse().unwrap()),
            "JO" => Joints(name["JOINTS_".len()..].parse().unwrap()),
            "WE" => Weights(name["WEIGHTS_".len()..].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl<'a> Iterator for IterAttributes<'a> {
    type Item = (Semantic, Attribute<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(json, index)| {
            use self::*;
            let semantic = Semantic::from_str(json.as_str());
            let accessor = self.prim.gltf
                .iter_accessors()
                .nth(index.value())
                .unwrap();
            let attribute = match semantic {
                Semantic::Position => {
                    Attribute::Position(unsafe {
                        accessor.iter()
                    })
                },
                Semantic::Normal => {
                    Attribute::Normal(unsafe {
                        accessor.iter()
                    })
                },
                Semantic::Tangent => {
                    Attribute::Tangent(unsafe {
                        accessor.iter()
                    })
                },
                Semantic::Color(set) => {
                    Attribute::Color(set, Color::from_accessor(accessor))
                },
                Semantic::TexCoord(set) => {
                    Attribute::TexCoord(set, TexCoord::from_accessor(accessor))
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

impl<'a> Iterator for IterPrimitives<'a> {
    type Item = Primitive<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Primitive::new(self.mesh.gltf, json))
    }
}
