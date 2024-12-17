use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate, PartialEq)]
pub struct Asset {}
