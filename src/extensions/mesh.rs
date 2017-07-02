
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {json, Gltf};

/// A set of primitives to be rendered.  A node can contain one or more meshes and
/// its transform places the meshes in the scene.
#[derive(Clone, Debug)]
pub struct Mesh<'a>  {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::mesh::Mesh,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug)]
pub struct Primitive<'a>  {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::mesh::Primitive,
}

impl<'a>  Mesh<'a>  {
    /// Constructs a `Mesh`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::mesh::Mesh) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::mesh::Mesh {
        self.json
    }
}

impl<'a> Primitive<'a> {
    /// Constructs a `Primitive`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::mesh::Primitive) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::mesh::Primitive {
        self.json
    }
}
