
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{accessor, scene, Extras, Index, Root};

enum_string! {
    Interpolation {
        Linear = "LINEAR",
        Step = "STEP",
    }
}

enum_string! {
    Path {
        Rotation = "rotation",
        Scale = "scale",
        Translation = "translation",
        Weights = "weights",
    }
}

/// A keyframe animation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Animation<E: Extras> {
    /// Extension specific data
    #[serde(default)]
    pub extensions: AnimationExtensions,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Animation,
    
    /// An array of channels, each of which targets an animation's sampler at a
    /// node's property
    ///
    /// Different channels of the same animation must not have equal targets
    pub channels: Vec<Channel<E>>,
    
    /// Optional user-defined name for this object
    pub name: Option<String>,
    
    /// An array of samplers that combine input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target)
    pub samplers: Vec<Sampler<E>>,
}

/// Extension specific data for `Animation`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AnimationExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Targets an animation's sampler at a node's property
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Channel<E: Extras> {
    /// The index of the sampler used to compute the value for the target
    pub sampler: Index<Sampler<E>>,
    
    /// The index of the node and TRS property to target
    pub target: Target<E>,
    
    /// Extension specific data
    #[serde(default)]
    pub extensions: ChannelExtensions,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::AnimationChannel,
}

/// Extension specific data for `Channel`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChannelExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// The index of the node and TRS property that an animation channel targets
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Target<E: Extras> {
    /// Extension specific data
    #[serde(default)]
    pub extensions: TargetExtensions,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::AnimationTarget,
    
    /// The index of the node to target
    pub node: Index<scene::Node<E>>,
    
    /// The name of the node's TRS property to modify or the 'weights' of the
    /// morph targets it instantiates
    pub path: Path,
}

/// Extension specific data for `Target`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TargetExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Defines a keyframe graph but not its target
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sampler<E: Extras> {
    /// Extension specific data
    #[serde(default)]
    pub extensions: SamplerExtensions,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::AnimationSampler,
    
    /// The index of the accessor containing keyframe input values (e.g. time)
    pub input: Index<accessor::Accessor<E>>,
    
    /// The interpolation algorithm
    pub interpolation: Interpolation,
    
    /// The index of an accessor containing keyframe output values
    pub output: Index<accessor::Accessor<E>>,
}

/// Extension specific data for `Sampler`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SamplerExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl<E: Extras> Animation<E> {
    #[doc(hidden)]
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
        for sampler in &self.samplers {
            let _ = root.try_get(&sampler.input)?;
            let _ = root.try_get(&sampler.output)?;
        }
        for channel in &self.channels {
            let _ = root.try_get(&channel.target.node)?;
            if channel.sampler.value() as usize >= self.samplers.len() {
                return Err(());
            }
        }
        Ok(())
    }
}
