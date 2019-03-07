use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Buffer {}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct View {}
