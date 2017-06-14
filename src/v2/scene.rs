
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;
use v2::{camera, json, mesh, skin};

///  A node in the node hierarchy. When the node contains `skin`, all
/// `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes. A node can
/// have either a `matrix` or any combination of `translation`/`rotation`/`scale`
/// (TRS) properties. TRS properties are converted to matrices and postmultiplied in
/// the `T * R * S` order to compose the transformation matrix; first the scale is
/// applied to the vertices, then the rotation, and then the translation. If none are
/// provided, the transform is the identity. When a node is targeted for animation
/// (referenced by an animation.channel.target), only TRS properties may be present;
/// `matrix` will not be present..
pub struct Node<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::scene::Node,
}

impl<'a> Node<'a> {
    /// Constructs a `Node`.
    pub fn new(gltf: &'a Gltf, json: &'a json::scene::Node) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::scene::Node {
        self.json
    }

    ///  The index of the camera referenced by this node.
    pub fn camera(&self) -> Option<camera::Camera<'a>> {
        unimplemented!()
    }

    ///  The indices of this node's children.
    pub fn children(&self) -> ! {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::scene::NodeExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }

    ///  4x4 column-major transformation matrix.
    pub fn matrix(&self) -> &[f32; 16] {
        unimplemented!()
    }

    ///  The index of the mesh in this node.
    pub fn mesh(&self) -> Option<mesh::Mesh<'a>> {
        unimplemented!()
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> &Option<String> {
        unimplemented!()
    }

    ///  The node's unit quaternion rotation in the order (x, y, z, w), where w is
    /// the scalar.
    pub fn rotation(&self) -> [f32; 4] {
        unimplemented!()
    }

    ///  The node's non-uniform scale.
    pub fn scale(&self) -> [f32; 3] {
        unimplemented!()
    }

    ///  The node's translation.
    pub fn translation(&self) -> [f32; 3] {
        unimplemented!()
    }

    ///  The index of the skin referenced by this node.
    pub fn skin(&self) -> &Option<skin::Skin<'a>> {
        unimplemented!()
    }

    ///  The weights of the instantiated Morph Target. Number of elements must match number of Morph Targets of used mesh.
    pub fn weights(&self) -> Option<&[f32]> {
        unimplemented!()
    }
}

///  The root `Node`s of a scene.
pub struct Scene<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::scene::Scene,
}

impl<'a> Scene<'a> {
    /// Constructs a `Scene`.
    pub fn new(gltf: &'a Gltf, json: &'a json::scene::Scene) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::scene::Scene {
        self.json
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::scene::SceneExtensions {
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

    ///  The indices of each root node.
    pub fn nodes(&self) -> ! {
        unimplemented!()
    }
}
