
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Deref;
use v2::{self, Extras};

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
    root: &'a v2::root::Root<E>,
    /// Deref
    node: &'a v2::scene::Node<E>,
}

/// The root object for a glTF asset.
#[derive(Debug)]
pub struct Root<'a, E: 'a + Extras> {
    /// Deref
    root: &'a v2::root::Root<E>,
}

/// The root nodes of a scene.
#[derive(Debug)]
pub struct Scene<'a, E: 'a + Extras> {
    root: &'a v2::root::Root<E>,
    /// Deref
    scene: &'a v2::scene::Scene<E>,
}

/// An `Iterator` that visits the children of a `Node`
#[derive(Debug)]
pub struct WalkChildNodes<'a, E: 'a + Extras> {
    index: usize,
    parent: &'a v2::scene::Node<E>,
    root: &'a v2::root::Root<E>,
}

/// An `Iterator` that visits every node in a `Scene`
#[derive(Debug)]
pub struct WalkNodes<'a, E: 'a + Extras> {
    index: usize,
    root: &'a v2::root::Root<E>,
    scene: &'a v2::scene::Scene<E>,
}

/// An `Iterator` that visits every scene in a glTF asset
#[derive(Debug)]
pub struct WalkScenes<'a, E: 'a + Extras> {
    index: usize,
    root: &'a v2::root::Root<E>,
}

impl<'a, E: 'a + Extras> Node<'a, E> {
    /// Returns an `Iterator` that visits every child node
    pub fn walk_child_nodes(&'a self) -> WalkChildNodes<'a, E> {
        WalkChildNodes {
            index: 0,
            parent: self,
            root: self.root,            
        }
    }
}

impl<'a, E: 'a + Extras> Deref for Node<'a, E> {
    type Target = v2::scene::Node<E>;
    fn deref(&self) -> &Self::Target {
        self.node
    }
}

impl<'a, E: 'a + Extras> Root<'a, E> {
    /// Returns a reference to the glTF root object that can be used to perform
    /// tree traversal operations.
    pub fn new(root: &'a v2::root::Root<E>) -> Self {
        Self {
            root: root,
        }
    }

    /// Returns an `Iterator` that walks the scenes of the glTF asset
    pub fn walk_scenes(&'a self) -> WalkScenes<'a, E> {
        WalkScenes {            
            index: 0,
            root: self,
        }
    }
}

impl<'a, E: 'a + Extras> Deref for Root<'a, E> {
    type Target = v2::root::Root<E>;
    fn deref(&self) -> &Self::Target {
        self.root
    }
}

impl<'a, E: 'a + Extras> Scene<'a, E> {
    /// Returns an `Iterator` that walks the root nodes in a scene
    pub fn walk_nodes(&'a self) -> WalkNodes<'a, E> {
        WalkNodes {
            index: 0,
            root: self.root,
            scene: self.scene,
        }
    }
}

impl<'a, E: 'a + Extras> Deref for Scene<'a, E> {
    type Target = v2::scene::Scene<E>;
    fn deref(&self) -> &Self::Target {
        self.scene
    }
}


impl<'a, E: 'a + Extras> Iterator for WalkChildNodes<'a, E> {
    type Item = Node<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.parent.children.len() {
            self.index += 1;
            Some(Node {
                node: self.root.get(&self.parent.children[self.index - 1]),
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

