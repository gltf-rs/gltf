use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// Universally unique identifier.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Uuid {
    /// The identifier.
    pub uuid: String,
}
