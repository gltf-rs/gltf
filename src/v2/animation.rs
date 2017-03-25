
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use traits::{Extensions, Extras};
use v2::{accessor, scene, Index};

/// [A keyframe animation]
/// (https://github.com/KhronosGroup/glTF/blob/d63b796e6b7f6b084c710b97b048d59d749cb04a/specification/2.0/schema/animation.schema.json)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Animation<E: Extensions, X: Extras> {
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: <E as Extensions>::Animation,
    /// Optional applcation specific data
    #[serde(default)]
    pub extras: <X as Extras>::Animation,
    /// Defines the channels of the animation
    pub channels: Vec<Channel<E, X>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Defines samplers that combine input and output accessors
    pub samplers: Vec<Sampler<E, X>>,
}

/// Targets an animation's sampler at a node's property
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Channel<E: Extensions, X: Extras> {
    /// The index of the sampler used to compute the value for the target
    pub sampler: Index<Sampler<E, X>>,
    /// The index of the node and TRS property to target
    pub target: Target<E, X>,
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: <E as Extensions>::AnimationChannel,
    /// Optional application specific data
    #[serde(default)]
    pub extras: <X as Extras>::AnimationChannel,
}

/// The index of the node and TRS property that an animation channel targets
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Target<E: Extensions, X: Extras> {
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: <E as Extensions>::AnimationTarget,
    /// Optional application specific data
    #[serde(default)]
    pub extras: <X as Extras>::AnimationTarget,
    /// The index of the node to target
    pub node: Index<scene::Node<E, X>>,
    /// The name of the node's TRS property to modify
    pub path: Path,
}

enum_string! {
    Path {
        Rotation = "rotation",
        Scale = "scale",
        Translation = "translation",
    }
}

/// Defines a keyframe graph but not its target
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sampler<E: Extensions, X: Extras> {
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: <E as Extensions>::AnimationSampler,
    /// Optional application specific data
    #[serde(default)]
    pub extras: <X as Extras>::AnimationSampler,
    /// The index of the accessor containing keyframe input values (e.g. time)
    pub input: Index<accessor::Accessor<E, X>>,
    /// The interpolation algorithm
    pub interpolation: Interpolation,
    /// The index of an accessor containing keyframe output values
    pub output: Index<accessor::Accessor<E, X>>,
}

enum_string! {
    Interpolation {
        Linear = "LINEAR",
        Step = "STEP",
    }
}
