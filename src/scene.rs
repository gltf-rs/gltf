
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use json;
use std::slice;

use {Camera, Gltf, Loaded, Mesh, Skin};

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

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::scene::Node,
}

/// The root `Node`s of a scene.
#[derive(Clone, Debug)]
pub struct Scene<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::scene::Scene,
}

/// An `Iterator` that visits the nodes in a scene.
#[derive(Clone, Debug)]
pub struct Nodes<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The internal node index iterIterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

/// An `Iterator` that visits the children of a node.
#[derive(Clone, Debug)]
pub struct Children<'a> {
    /// The parent `Node` struct.
    parent: &'a Node<'a>,

    /// The internal node index iterIterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

impl<'a> Node<'a> {
    /// Constructs a `Node`.
    pub fn new(gltf: &'a Gltf, index: usize, json: &'a json::scene::Node) -> Self {
        Self {
            gltf: gltf,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::scene::Node {
        self.json
    }

    /// Returns the camera referenced by this node.
    pub fn camera(&self) -> Option<Camera<'a>> {
        self.json.camera.as_ref().map(|index| {
            self.gltf.cameras().nth(index.value()).unwrap()
        })
    }

    /// Returns an `Iterator` that visits the node's children.
    pub fn children(&'a self) -> Children<'a> {
        Children {
            parent: self,
            iter: self.json.children.as_ref().map_or([].iter(), |x| x.iter()),
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Returns the 4x4 column-major transformation matrix.
    pub fn matrix(&self) -> [f32; 16] {
        self.json.matrix
    }

    /// Returns the mesh referenced by this node.
    pub fn mesh(&self) -> Option<Mesh<'a>> {
        self.json.mesh.as_ref().map(|index| {
            self.gltf.meshes().nth(index.value()).unwrap()
        })
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns the node's unit quaternion rotation in the order `[x, y, z, w]`,
    /// where `w` is the scalar.
    pub fn rotation(&self) -> [f32; 4] {
        self.json.rotation.0
    }

    /// Returns the node's non-uniform scale.
    pub fn scale(&self) -> [f32; 3] {
        self.json.scale
    }

    /// Returns the node's translation.
    pub fn translation(&self) -> [f32; 3] {
        self.json.translation
    }

    /// Returns the skin referenced by this node.
    pub fn skin(&self) -> Option<Skin<'a>> {
        self.json.skin.as_ref().map(|index| {
            self.gltf.skins().nth(index.value()).unwrap()
        })
    }

    /// Returns the weights of the instantiated morph target.
    pub fn weights(&self) -> Option<&[f32]> {
        self.json.weights.as_ref().map(Vec::as_slice)
    }
}

impl<'a> Scene<'a> {
    /// Constructs a `Scene`.
    pub fn new(gltf: &'a Gltf, index: usize, json: &'a json::scene::Scene) -> Self {
        Self {
            gltf: gltf,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::scene::Scene {
        self.json
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras{
        &self.json.extras
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns an `Iterator` that visits each root node of the scene.
    pub fn nodes(&self) -> Nodes<'a> {
        Nodes {
            gltf: self.gltf,
            iter: self.json.nodes.iter(),
        }
    }
}

impl<'a> Loaded<'a, Node<'a>> {
    /// Returns the camera referenced by this node.
    pub fn camera(&'a self) -> Option<Loaded<'a, Camera<'a>>> {
        self.item.camera().map(|item| {
            Loaded {
                item,
                source: self.source,
            }
        })
    }

    /// Returns an `Iterator` that visits the node's children.
    pub fn children(&'a self) -> Loaded<'a, Children<'a>> {
        Loaded {
            item: self.item.children(),
            source: self.source,
        }
    }

    /// Returns the mesh referenced by this node.
    pub fn mesh(&'a self) -> Option<Loaded<'a, Mesh<'a>>> {
        self.item.mesh().map(|item| {
            Loaded {
                item,
                source: self.source,
            }
        })
    }

    /// Returns the skin referenced by this node.
    pub fn skin(&self) -> Option<Loaded<'a, Skin<'a>>> {
        self.item.skin().map(|item| {
            Loaded {
                item,
                source: self.source,
            }
        })
    }
}

impl<'a> Loaded<'a, Scene<'a>> {
    /// Returns an `Iterator` that visits each root node of the scene.
    pub fn nodes(&'a self) -> Loaded<'a, Nodes<'a>> {
        Loaded {
            item: self.item.nodes(),
            source: self.source,
        }
    }
}

impl<'a> ExactSizeIterator for Nodes<'a> {}
impl<'a> Iterator for Nodes<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.gltf.nodes().nth(index.value()).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Loaded<'a, Nodes<'a>> {}
impl<'a> Iterator for Loaded<'a, Nodes<'a>> {
    type Item = Loaded<'a, Node<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.item.next().map(|item| {
            Loaded {
                item,
                source: self.source,
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.item.size_hint()
    }
}

impl<'a> ExactSizeIterator for Children<'a> {}
impl<'a> Iterator for Children<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.parent.gltf.nodes().nth(index.value()).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Loaded<'a, Children<'a>> {}
impl<'a> Iterator for Loaded<'a, Children<'a>> {
    type Item = Loaded<'a, Node<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.item.next().map(|item| {
            Loaded {
                item,
                source: self.source,
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.item.size_hint()
    }
}

