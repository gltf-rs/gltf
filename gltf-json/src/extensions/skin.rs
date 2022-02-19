use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Skin {}
