use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Mesh {}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Primitive {}
