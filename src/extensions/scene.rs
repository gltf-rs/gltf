
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {json, Gltf};

///  A node in the node hierarchy. When the node contains `skin`, all
/// `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes. A node can
/// have either a `matrix` or any combination of `translation`/`rotation`/`scale`
/// (TRS) properties. TRS properties are converted to matrices and postmultiplied in
/// the `T * R * S` order to compose the transformation matrix; first the scale is
/// applied to the vertices, then the rotation, and then the translation. If none are
/// provided, the transform is the identity. When a node is targeted for animation
/// (referenced by an animation.channel.target), only TRS properties may be present;
/// `matrix` will not be present.
#[derive(Clone, Debug)]
pub struct Node<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::scene::Node,
}

/// The root `Node`s of a scene.
#[derive(Clone, Debug)]
pub struct Scene<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::scene::Scene,
}

impl<'a> Node<'a> {
    /// Constructs a `Node`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::scene::Node) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::scene::Node {
        self.json
    }
}

impl<'a> Scene<'a> {
    /// Constructs a `Scene`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::scene::Scene) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::scene::Scene {
        self.json
    }
}
