use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// Image data used to create a texture.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Image {}
