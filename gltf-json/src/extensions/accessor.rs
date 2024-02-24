use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

/// Contains data structures for sparse storage.
pub mod sparse {
    use super::*;

    /// Indices of those attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
    pub struct Indices {}

    /// Sparse storage of attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
    pub struct Sparse {}

    /// Array of size `count * number_of_components` storing the displaced
    /// accessor attributes pointed by `accessor::sparse::Indices`.
    #[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
    pub struct Values {}
}

/// A typed view into a buffer view.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Accessor {
    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}
