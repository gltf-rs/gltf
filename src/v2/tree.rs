
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{self, Extras};

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Debug)]
pub struct Mesh<'a, E: 'a + Extras> {
    root: &'a v2::root::Root<E>,
}

/// A node in the node hierarchy.  When the node contains `skin`, all
/// `mesh.primitives` must contain `JOINT` and `WEIGHT` attributes.
///
/// A node can have either a `matrix` or any combination of
/// `translation`/`rotation`/`scale` (TRS) properties. TRS properties are
/// converted to matrices and postmultiplied in the `T * R * S` order to compose
/// the transformation matrix; first the scale is applied to the vertices, then
/// the rotation, and then the translation. If none are provided, the transform
/// is the identity. When a node is targeted for animation (referenced by an
/// animation.channel.target), only TRS properties may be present; `matrix` will
/// not be present.
#[derive(Debug)]
pub struct Node<'a, E: 'a + Extras> {
    parent: Option<&'a Node<'a, E>>,
    root: &'a v2::root::Root<E>,
    node: &'a v2::scene::Node<E>,
}

/// The root object for a glTF asset.
#[derive(Debug)]
pub struct Root<'a, E: 'a + Extras> {
    root: &'a v2::root::Root<E>,
}

/// The root nodes of a scene.
#[derive(Debug)]
pub struct Scene<'a, E: 'a + Extras> {
    root: &'a v2::root::Root<E>,
    scene: &'a v2::scene::Scene<E>,
}

/// An `Iterator` that visits the children of a node.
#[derive(Debug)]
pub struct WalkChildNodes<'a, E: 'a + Extras> {
    index: usize,
    parent: &'a Node<'a, E>,
    root: &'a v2::root::Root<E>,
}

/// An `Iterator` that visits every node in a scene.
#[derive(Debug)]
pub struct WalkNodes<'a, E: 'a + Extras> {
    index: usize,
    root: &'a v2::root::Root<E>,
    scene: &'a v2::scene::Scene<E>,
}

/// An `Iterator` that visits every scene in a glTF asset.
#[derive(Debug)]
pub struct WalkScenes<'a, E: 'a + Extras> {
    index: usize,
    root: &'a v2::root::Root<E>,
}

impl<'a, E: 'a + Extras> Node<'a, E> {
    /// Returns the camera referenced by this node.
    pub fn camera(&'a self) -> Option<&'a v2::camera::Camera<E>> {
        self.node.camera.as_ref().map(|index| self.root.get(index))
    }

    /// Returns the internal glTF object data.
    pub fn data(&'a self) -> &'a v2::scene::Node<E> {
        &self.node
    }
    
    /// Returns the mesh referenced by this node.
    pub fn mesh(&'a self) -> Option<&'a v2::mesh::Mesh<E>> {
        self.node.mesh.as_ref().map(|index| self.root.get(index))
    }

    /// Returns this node's parent node.
    pub fn parent(&'a self) -> Option<&'a Node<E>> {
        self.parent
    }
    
    /// Returns the skin referenced by this node.
    pub fn skin(&'a self) -> Option<&'a v2::skin::Skin<E>> {
        self.node.skin.as_ref().map(|index| self.root.get(index))
    }

    /// Returns an `Iterator` that visits every child node.
    pub fn walk_child_nodes(&'a self) -> WalkChildNodes<'a, E> {
        WalkChildNodes {
            index: 0,
            parent: self,
            root: self.root,            
        }
    }
}

impl<'a, E: 'a + Extras> Root<'a, E> {
    /// Returns the internal glTF object data.
    pub fn data(&'a self) -> &'a v2::root::Root<E> {
        &self.root
    }

    /// Returns a reference to the glTF root object that can be used to perform
    /// tree traversal operations.
    pub fn new(root: &'a v2::root::Root<E>) -> Self {
        Self {
            root: root,
        }
    }

    /// Returns an `Iterator` that walks the scenes of the glTF asset.
    pub fn walk_scenes(&'a self) -> WalkScenes<'a, E> {
        WalkScenes {            
            index: 0,
            root: self.root,
        }
    }
}

impl<'a, E: 'a + Extras> Scene<'a, E> {
    /// Returns the internal glTF object data.
    pub fn data(&'a self) -> &'a v2::scene::Scene<E> {
        &self.scene
    }

    /// Returns an `Iterator` that walks the root nodes in a scene.
    pub fn walk_nodes(&'a self) -> WalkNodes<'a, E> {
        WalkNodes {
            index: 0,
            root: self.root,
            scene: self.scene,
        }
    }
}

impl<'a, E: 'a + Extras> Iterator for WalkChildNodes<'a, E> {
    type Item = Node<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.parent.node.children.len() {
            self.index += 1;
            Some(Node {
                node: self.root.get(&self.parent.node.children[self.index - 1]),
                parent: Some(self.parent),
                root: self.root,
            })
        } else {
            None
        }
    }
}

impl<'a, E: 'a + Extras> Iterator for WalkNodes<'a, E> {
    type Item = Node<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.scene.nodes.len() {
            self.index += 1;
            Some(Node {
                node: self.root.get(&self.scene.nodes[self.index - 1]),
                parent: None,
                root: self.root,
            })
        } else {
            None
        }
    }
}

impl<'a, E: 'a + Extras> Iterator for WalkScenes<'a, E> {
    type Item = Scene<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.root.scenes().len() {
            self.index += 1;
            Some(Scene {
                root: self.root,
                scene: &self.root.scenes()[self.index - 1],
            })
        } else {
            None
        }
    }
}

