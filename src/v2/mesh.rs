
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;
use v2::{accessor, json, material};

pub enum Mode {}

///  A set of primitives to be rendered.  A node can contain one or more meshes and its transform places the meshes in the scene.
pub struct Mesh<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Mesh,
}

impl<'a> Mesh<'a> {
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

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::mesh::MeshExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> &Option<String> {
        unimplemented!()
    }

    ///  Defines the geometry to be renderered with a material.
    pub fn primitives(&self) -> &Vec<json::mesh::Primitive> {
        unimplemented!()
    }

    ///  Defines the weights to be applied to the morph targets.
    pub fn weights(&self) -> Option<&[f32]> {
        unimplemented!()
    }
}
///  Geometry to be rendered with the given material.
pub struct Primitive<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Primitive,
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

    ///  Maps attribute semantic names to the `Accessor`s containing the corresponding attribute data.
    pub fn attributes(&self) -> ! {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::mesh::PrimitiveExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }

    ///  The index of the accessor that contains the indices.
    pub fn indices(&self) -> Option<accessor::Accessor<'a>> {
        unimplemented!()
    }

    ///  The index of the material to apply to this primitive when rendering
    pub fn material(&self) -> Option<material::Material<'a>> {
        unimplemented!()
    }

    ///  The type of primitives to render.
    pub fn mode(&self) -> Mode {
        unimplemented!()
    }

    ///  An array of Morph Targets, each Morph Target is a dictionary mapping attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their deviations in the Morph Target.
    pub fn targets(&self) -> ! {
        unimplemented!()
    }
}
