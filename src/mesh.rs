
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
    Color(u8),

    /// UV texture co-ordinates.
    TexCoord(u8),

    /// Joint indices.
    Joints(u8),

    /// Joint weights.
    Weights(u8),
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

    /// Returns the attribute with the given semantic value.
    fn find_attribute_with_semantic(
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

    /// The index of the accessor that contains the indices.
    pub fn indices(&self) -> Option<accessor::Accessor<'a>> {
        self.json.indices.as_ref().map(|index| {
            accessor::Accessor::new(self.gltf, self.gltf.as_json().get(index))
        })
    }

    /// The index of the material to apply to this primitive when rendering
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
        match &name[..1] {
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
    type Item = (Semantic, accessor::Accessor<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(json, index)| {
            let accessor = self.prim.gltf
                .iter_accessors()
                .nth(index.value())
                .unwrap();
            (Semantic::from_str(json.as_str()), accessor)
        })
    }
}

impl<'a> Iterator for IterPrimitives<'a> {
    type Item = Primitive<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Primitive::new(self.mesh.gltf, json))
    }
}
