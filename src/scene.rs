
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cgmath;
use cgmath::prelude::*;
use json;
use std::{mem, slice};

use {Camera, Gltf, Mesh, Skin};

type Matrix3 = cgmath::Matrix3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;
type Quaternion = cgmath::Quaternion<f32>;
type Vector3 = cgmath::Vector3<f32>;

/// 4x4 identity matrix.
const IDENTITY: [f32; 16] = {
    [1.0, 0.0, 0.0, 0.0, 
     0.0, 1.0, 0.0, 0.0, 
     0.0, 0.0, 1.0, 0.0, 
     0.0, 0.0, 0.0, 1.0]
};

/// The transform for a `Node`.
#[derive(Clone, Debug)]
pub enum Transform {
    /// 4x4 transformation matrix in column-major order.
    Matrix {
        /// 4x4 matrix.
        matrix: [[f32; 4]; 4],
    },

    /// Decomposed TRS properties.
    Decomposed {
        /// `[x, y, z]` vector.
        translation: [f32; 3],

        /// `[x, y, z, w]` quaternion, where `w` is the scalar.
        rotation: [f32; 4],

        /// `[x, y, z]` vector.
        scale: [f32; 3],
    },
}

impl Transform {
    /// Returns the matrix representation of this transform.
    ///
    /// If the transform is `Decomposed`, then the matrix is generated with the
    /// equation `matrix = translation * rotation * scale`.
    pub fn matrix(self) -> [[f32; 4]; 4] {
        match self {
            Transform::Matrix { matrix } => matrix,
            Transform::Decomposed { translation, rotation, scale } => {
                let t = Matrix4::from_translation(translation.into());
                let r = Matrix4::from(Quaternion::new(rotation[3], rotation[0], rotation[1], rotation[2]));
                let s = Matrix4::from_nonuniform_scale(scale[0], scale[1], scale[2]);
                (t * r * s).into()
            },
        }
    }

    /// Returns the decomposed representation of this transform.
    ///
    /// If the transform is `Matrix`, then the decomposition is extracted from the
    /// matrix.
    pub fn decomposed(self) -> ([f32; 3], [f32; 4], [f32; 3]) {
        match self {
            Transform::Matrix { matrix: mut m } => {
                let translation = [
                    mem::replace(&mut m[3][0], 0.0),
                    mem::replace(&mut m[3][1], 0.0),
                    mem::replace(&mut m[3][2], 0.0),
                ];
                let sx = Vector3::new(m[0][0], m[0][1], m[0][2]).magnitude();
                m[0][0] /= sx;
                m[0][1] /= sx;
                m[0][2] /= sx;
                let sy = Vector3::new(m[1][0], m[1][1], m[1][2]).magnitude();
                m[1][0] /= sy;
                m[1][1] /= sy;
                m[1][2] /= sy;
                let sz = Vector3::new(m[2][0], m[2][1], m[2][2]).magnitude();
                m[2][0] /= sz;
                m[2][1] /= sz;
                m[2][2] /= sz;
                let scale = [sx, sy, sz];
                let r = Quaternion::from(
                    Matrix3::new(
                        m[0][0], m[0][1], m[0][2],
                        m[1][0], m[1][1], m[1][2],
                        m[2][0], m[2][1], m[2][2],
                    ),
                );
                let rotation = [r.v.x, r.v.y, r.v.z, r.s];
                (translation, rotation, scale)
            },
            Transform::Decomposed { translation, rotation, scale } => {
                (translation, rotation, scale)
            },
        }
    }
}

/// A node in the node hierarchy.
///
/// When a node contains a skin, all its meshes contain `JOINTS_0` and `WEIGHTS_0`
/// attributes.
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

    /// The internal node index iterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

/// An `Iterator` that visits the children of a node.
#[derive(Clone, Debug)]
pub struct Children<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The internal node index iterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

impl<'a> Node<'a> {
    /// Constructs a `Node`.
    pub(crate) fn new(
        gltf: &'a Gltf,
        index: usize,
        json: &'a json::scene::Node,
    ) -> Self {
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
    #[doc(hidden)]
    pub fn as_json(&self) ->  &json::scene::Node {
        self.json
    }

    /// Returns the camera referenced by this node.
    pub fn camera(&self) -> Option<Camera> {
        self.json.camera.as_ref().map(|index| {
            self.gltf.cameras().nth(index.value()).unwrap()
        })
    }

    /// Returns an `Iterator` that visits the node's children.
    pub fn children(&self) -> Children {
        Children {
            gltf: self.gltf,
            iter: self.json.children.as_ref().map_or([].iter(), |x| x.iter()),
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Returns the 4x4 column-major transformation matrix.
    #[deprecated(since = "0.9.1", note = "Use `transform().matrix()` instead")]
    pub fn matrix(&self) -> [f32; 16] {
        self.json.matrix.unwrap_or(IDENTITY)
    }

    /// Returns the mesh referenced by this node.
    pub fn mesh(&self) -> Option<Mesh> {
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
    #[deprecated(since = "0.9.1", note = "Use `transform().decomposed()` instead.")]
    pub fn rotation(&self) -> [f32; 4] {
        self.json.rotation.0
    }

    /// Returns the node's non-uniform scale.
    #[deprecated(since = "0.9.1", note = "Use `transform().decomposed()` instead.")]
    pub fn scale(&self) -> [f32; 3] {
        self.json.scale
    }

    /// Returns the node's translation.
    #[deprecated(since = "0.9.1", note = "Use `transform().decomposed()` instead.")]
    pub fn translation(&self) -> [f32; 3] {
        self.json.translation
    }

    /// Returns the node's transform.
    pub fn transform(&self) -> Transform {
        if let Some(matrix) = self.json.matrix.clone() {
            unsafe {
                Transform::Matrix {
                    matrix: mem::transmute(matrix),
                }
            }
        } else {
            Transform::Decomposed {
                translation: self.json.translation,
                rotation: self.json.rotation.0,
                scale: self.json.scale,
            }
        }
    }

    /// Returns the skin referenced by this node.
    pub fn skin(&self) -> Option<Skin> {
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
    pub(crate) fn new(
        gltf: &'a Gltf,
        index: usize,
        json: &'a json::scene::Scene,
    ) -> Self {
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
    #[doc(hidden)]
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

impl<'a> ExactSizeIterator for Children<'a> {}
impl<'a> Iterator for Children<'a> {
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

