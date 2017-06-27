
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::marker::PhantomData;

use json::{accessor, scene, Extras, Index, Root};
use validation::{Error, JsonPath, Validate};

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
pub struct Animation<'a> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: AnimationExtensions<'a>,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
    
    /// An array of channels, each of which targets an animation's sampler at a
    /// node's property.
    ///
    /// Different channels of the same animation must not have equal targets.
    pub channels: Vec<Channel<'a>>,
    
    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,
    
    /// An array of samplers that combine input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub samplers: Vec<Sampler<'a>>,
}

/// Extension specific data for `Animation`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct AnimationExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Channel<'a> {
    /// The index of a sampler in this animation used to compute the value for the target.
    pub sampler: Index<Sampler<'a>>,
    
    /// The index of the node and TRS property to target.
    pub target: Target<'a>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: ChannelExtensions<'a>,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
}

/// Extension specific data for `Channel`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct ChannelExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// The index of the node and TRS property that an animation channel targets.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Target<'a> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: TargetExtensions<'a>,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
    
    /// The index of the node to target.
    pub node: Index<scene::Node<'a>>,
    
    /// The name of the node's TRS property to modify or the 'weights' of the
    /// morph targets it instantiates.
    pub path: TrsProperty<'a>,
}

/// Extension specific data for `Target`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct TargetExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// Defines a keyframe graph but not its target.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Sampler<'a> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: SamplerExtensions<'a>,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
    
    /// The index of an accessor containing keyframe input values, e.g., time.
    pub input: Index<accessor::Accessor<'a>>,
    
    /// The interpolation algorithm.
    #[serde(default)]
    pub interpolation: InterpolationAlgorithm<'a>,
    
    /// The index of an accessor containing keyframe output values.
    pub output: Index<accessor::Accessor<'a>>,
}

/// Extension specific data for `Sampler`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct SamplerExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// Specifies an interpolation algorithm.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InterpolationAlgorithm<'a>(pub Cow<'a, str>);

/// Specifies a TRS property.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrsProperty<'a>(pub Cow<'a, str>);

impl<'a> Validate<'a> for Animation<'a> {
    fn validate<P, R>(&self, root: &Root<'a>, path: P, mut report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        self.samplers.validate(root, || path().field("samplers"), report);
        self.channels.validate(root, || path().field("channels"), report);
        for (index, channel) in self.channels.iter().enumerate() {
            if channel.sampler.value() as usize >= self.samplers.len() {
                let field = format!("channels[{}].sampler", index);
                report(Error::index_out_of_bounds(path().field(&field)));
            }
        }
    }
}

impl<'a> Validate<'a> for Channel<'a> {
    fn validate<P, R>(&self, _root: &Root<'a>, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

impl<'a> Default for InterpolationAlgorithm<'a> {
    fn default() -> Self {
        InterpolationAlgorithm(Cow::from("LINEAR"))
    }
}

impl<'a> Validate<'a> for InterpolationAlgorithm<'a> {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if !VALID_INTERPOLATION_ALGORITHMS.contains(&self.0.as_ref()) {
            report(Error::invalid_enum(path(), self.0.clone()));
        }
    }
}

impl<'a> Validate<'a> for TrsProperty<'a> {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if !VALID_TRS_PROPERTIES.contains(&self.0.as_ref()) {
            report(Error::invalid_enum(path(), self.0.clone()));
        }
    }
}
