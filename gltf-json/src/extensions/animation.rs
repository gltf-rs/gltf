use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// A keyframe animation.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Animation {}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Channel {}

/// The index of the node and TRS property that an animation channel targets.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Target {}

/// Defines a keyframe graph but not its target.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Sampler {}
