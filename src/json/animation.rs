
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use json::{accessor, scene, Extras, Index, Root};
use validation::{Action, Error, JsonPath, Validate};

/// All valid interpolation algorithms.
pub const VALID_INTERPOLATION_ALGORITHMS: &'static [&'static str] = &[
    "LINEAR",
    "STEP",
    "CATMULLROMSPLINE",
    "CUBICSPLINE",
];

/// All valid TRS property names.
pub const VALID_TRS_PROPERTIES: &'static [&'static str] = &[
    "translation",
    "rotation",
    "scale",
    "weights",
];

/// A keyframe animation.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Animation {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: AnimationExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// An array of channels, each of which targets an animation's sampler at a
    /// node's property.
    ///
    /// Different channels of the same animation must not have equal targets.
    pub channels: Vec<Channel>,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,
    
    /// An array of samplers that combine input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub samplers: Vec<Sampler>,
}

/// Extension specific data for `Animation`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct AnimationExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Channel {
    /// The index of a sampler in this animation used to compute the value for the target.
    pub sampler: Index<Sampler>,
    
    /// The index of the node and TRS property to target.
    pub target: Target,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: ChannelExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Channel`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct ChannelExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// The index of the node and TRS property that an animation channel targets.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Target {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: TargetExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// The index of the node to target.
    pub node: Index<scene::Node>,
    
    /// The name of the node's TRS property to modify or the 'weights' of the
    /// morph targets it instantiates.
    pub path: TrsProperty,
}

/// Extension specific data for `Target`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct TargetExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Defines a keyframe graph but not its target.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Sampler {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: SamplerExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// The index of an accessor containing keyframe input values, e.g., time.
    pub input: Index<accessor::Accessor>,
    
    /// The interpolation algorithm.
    #[serde(default)]
    pub interpolation: InterpolationAlgorithm,
    
    /// The index of an accessor containing keyframe output values.
    pub output: Index<accessor::Accessor>,
}

/// Extension specific data for `Sampler`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct SamplerExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Specifies an interpolation algorithm.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InterpolationAlgorithm(pub String);

/// Specifies a TRS property.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrsProperty(pub String);

impl Validate for Animation {
    fn validate<P, R>(&self, root: &Root, path: P, mut report: &mut R) -> Action
        where P: Fn() -> JsonPath, R: FnMut(Error) -> Action
    {
        try_validate!(self.samplers, root, || path().field("samplers"), report);
        try_validate!(self.channels, root, || path().field("channels"), report);
        for (index, channel) in self.channels.iter().enumerate() {
            if channel.sampler.value() as usize >= self.samplers.len() {
                let field = format!("channels[{}].sampler", index);
                match report(Error::index_out_of_bounds(path().field(&field))) {
                    Action::Stop => return Action::Stop,
                    _ => {},
                }   
            }
        }
        Action::Continue
    }
}

impl Validate for Channel {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R) -> Action
        where P: Fn() -> JsonPath, R: FnMut(Error) -> Action
    {
        Action::Continue
    }
}

impl Default for InterpolationAlgorithm {
    fn default() -> Self {
        InterpolationAlgorithm("LINEAR".to_string())
    }
}

impl Validate for InterpolationAlgorithm {
    fn validate<P, R>(&self, _: &Root, path: P, report: &mut R) -> Action
        where P: Fn() -> JsonPath, R: FnMut(Error) -> Action
    {
        if !VALID_INTERPOLATION_ALGORITHMS.contains(&self.0.as_ref()) {
            report(Error::invalid_enum(path(), self.0.clone()))
        } else {
            Action::Continue
        }
    }
}

impl Validate for TrsProperty {
    fn validate<P, R>(&self, _: &Root, path: P, report: &mut R) -> Action
        where P: Fn() -> JsonPath, R: FnMut(Error) -> Action
    {
        if !VALID_TRS_PROPERTIES.contains(&self.0.as_ref()) {
            report(Error::invalid_enum(path(), self.0.clone()))
        } else {
            Action::Continue
        }
    }
}
