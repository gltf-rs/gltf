
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::slice;
use {camera, json, mesh, skin, Gltf};

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
    /// The parent `Gltf<'a>` struct.
    gltf: &'a Gltf<'a>,

    /// The corresponding JSON struct.
    json: &'a json::scene::Node<'a>,
}

/// The root `Node`s of a scene.
#[derive(Clone, Debug)]
pub struct Scene<'a> {
    /// The parent `Gltf<'a>` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf<'a>,

    /// The corresponding JSON struct.
    json: &'a json::scene::Scene<'a>,
}

/// An `Iterator` that visits the nodes in a scene.
#[derive(Clone, Debug)]
pub struct Nodes<'a> {
    /// The parent `Gltf<'a>` struct.
    gltf: &'a Gltf<'a>,

    /// The internal node index iterIterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node<'a>>>,
}

/// An `Iterator` that visits the children of a node.
#[derive(Clone, Debug)]
pub struct Children<'a> {
    /// The parent `Node` struct.
    parent: &'a Node<'a>,

    /// The internal node index iterIterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node<'a>>>,
}

impl<'a> Node<'a> {
    /// Constructs a `Node`.
    pub fn new(gltf: &'a Gltf<'a>, json: &'a json::scene::Node<'a>) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::scene::Node<'a> {
        self.json
    }

    /// The index of the camera referenced by this node.
    pub fn camera(&self) -> Option<camera::Camera<'a>> {
        self.json.camera.as_ref().map(|index| {
            camera::Camera::new(self.gltf, self.gltf.as_json().get(index))
        })
    }

    /// The indices of this node's children.
    pub fn children(&'a self) -> Children<'a> {
        Children {
            parent: self,
            iter: self.json.children.as_ref().map_or([].iter(), |x| x.iter()),
        }
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::scene::NodeExtensions<'a> {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras<'a> {
        &self.json.extras
    }

    /// 4x4 column-major transformation matrix.
    pub fn matrix(&self) -> [f32; 16] {
        self.json.matrix
    }

    /// The index of the mesh in this node.
    pub fn mesh(&self) -> Option<mesh::Mesh<'a>> {
        self.json.mesh.as_ref().map(|index| {
            mesh::Mesh::new(self.gltf, self.gltf.as_json().get(index))
        })
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
    }

    /// The node's unit quaternion rotation in the order (x, y, z, w), where w is
    /// the scalar.
    pub fn rotation(&self) -> [f32; 4] {
        self.json.rotation.0
    }

    /// The node's non-uniform scale.
    pub fn scale(&self) -> [f32; 3] {
        self.json.scale
    }

    /// The node's translation.
    pub fn translation(&self) -> [f32; 3] {
        self.json.translation
    }

    /// The index of the skin referenced by this node.
    pub fn skin(&self) -> Option<skin::Skin<'a>> {
        self.json.skin.as_ref().map(|index| {
            skin::Skin::new(self.gltf, self.gltf.as_json().get(index))
        })
    }

    /// The weights of the instantiated Morph Target. Number of elements must match
    /// number of Morph Targets of used mesh.
    pub fn weights(&self) -> Option<&[f32]> {
        self.json.weights.as_ref().map(Vec::as_slice)
    }
}

impl<'a> Scene<'a> {
    /// Constructs a `Scene`.
    pub fn new(gltf: &'a Gltf<'a>, json: &'a json::scene::Scene<'a>) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::scene::Scene<'a> {
        self.json
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::scene::SceneExtensions<'a> {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras<'a>{
        &self.json.extras
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
    }

    /// The indices of each root node.
    pub fn nodes(&self) -> Nodes<'a>  {
        Nodes {
            gltf: self.gltf,
            iter: self.json.nodes.iter(),
        }
    }
}

impl<'a> Iterator for Nodes<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.gltf.nodes().nth(index.value()).unwrap())
    }
}

impl<'a> Iterator for Children<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.parent.gltf.nodes().nth(index.value()).unwrap())
    }
}


