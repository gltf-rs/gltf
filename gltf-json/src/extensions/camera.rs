use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// A camera's projection.
///
/// A node can reference a camera to apply a transform to place the camera in the
/// scene.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Camera {}

/// Values for an orthographic camera.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Orthographic {}

/// Values for a perspective camera.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Perspective {}
