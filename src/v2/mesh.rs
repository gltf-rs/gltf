
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;
use v2::{accessor, json, material};

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

///  A set of primitives to be rendered.  A node can contain one or more meshes and its transform places the meshes in the scene.
pub struct Mesh<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf<'a>,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Mesh,
}

impl<'a> Mesh<'a> {
    /// Constructs a `Mesh`.
    pub fn new(gltf: &'a Gltf<'a>, json: &'a json::mesh::Mesh) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::mesh::Mesh {
        self.json
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::mesh::MeshExtensions {
        &self.json.extensions
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    ///  Defines the geometry to be renderered with a material.
    pub fn primitives(&self) -> ! {
        unimplemented!()
    }

    ///  Defines the weights to be applied to the morph targets.
    pub fn weights(&self) -> Option<&[f32]> {
        self.json.weights.as_ref().map(Vec::as_slice)
    }
}

///  Geometry to be rendered with the given material.
pub struct Primitive<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf<'a>,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Primitive,
}

impl<'a> Primitive<'a> {
    /// Constructs a `Primitive`.
    pub fn new(gltf: &'a Gltf<'a>, json: &'a json::mesh::Primitive) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::mesh::Primitive {
        self.json
    }

    ///  Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub fn attributes(&self) -> ! {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::mesh::PrimitiveExtensions {
        &self.json.extensions
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    ///  The index of the accessor that contains the indices.
    pub fn indices(&self) -> Option<accessor::Accessor<'a>> {
        self.json.indices.as_ref().map(|index| {
            accessor::Accessor::new(self.gltf, self.gltf.as_json().get(index))
        })
    }

    ///  The index of the material to apply to this primitive when rendering
    pub fn material(&self) -> Option<material::Material<'a>> {
        self.json.material.as_ref().map(|index| {
            material::Material::new(self.gltf, self.gltf.as_json().get(index))
        })
    }

    ///  The type of primitives to render.
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

    ///  An array of Morph Targets, each Morph Target is a dictionary mapping
    /// attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their
    /// deviations in the Morph Target.
    pub fn targets(&self) -> ! {
        unimplemented!()
    }
}
