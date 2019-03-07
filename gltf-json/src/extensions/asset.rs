use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Asset {}
