
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de;
use std::fmt;
use json::{accessor, scene, Extras, Index, Root};
use validation::{Checked, Error, JsonPath, Validate};

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

/// Specifies an interpolation algorithm.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum InterpolationAlgorithm {
    Linear,
    Step,
    CatmullRomSpline,
    CubicSpline,
}

/// Specifies a TRS property.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum TrsProperty {
    Translation,
    Rotation,
    Scale,
    Weights,
}

/// A keyframe animation.
#[derive(Clone, Debug, Deserialize)]
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
    #[cfg(feature = "names")]
    pub name: Option<String>,
    
    /// An array of samplers that combine input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub samplers: Vec<Sampler>,
}

/// Extension specific data for `Animation`.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct AnimationExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug, Deserialize)]
pub struct Channel {
    /// The index of a sampler in this animation used to compute the value for the
    /// target.
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
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct ChannelExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// The index of the node and TRS property that an animation channel targets.
#[derive(Clone, Debug, Deserialize, Validate)]
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
    pub path: Checked<TrsProperty>,
}

/// Extension specific data for `Target`.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct TargetExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Defines a keyframe graph but not its target.
#[derive(Clone, Debug, Deserialize, Validate)]
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
    pub interpolation: Checked<InterpolationAlgorithm>,
    
    /// The index of an accessor containing keyframe output values.
    pub output: Index<accessor::Accessor>,
}

/// Extension specific data for `Sampler`.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct SamplerExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

impl Validate for Animation {
    fn validate_minimally<P, R>(&self, root: &Root, path: P, mut report: &mut R)
    where
        P: Fn() -> JsonPath,
        R: FnMut(&Fn() -> JsonPath, Error),
    {
        self.samplers.validate_minimally(root, || path().field("samplers"), report);
        for (index, channel) in self.channels.iter().enumerate() {
            if channel.sampler.value() as usize >= self.samplers.len() {
                let path = || path().field("channels").index(index).field("sampler");
                report(&path, Error::IndexOutOfBounds);
            }
        }
    }
}

impl Default for InterpolationAlgorithm {
    fn default() -> Self {
        InterpolationAlgorithm::Linear
    }
}

impl<'de> de::Deserialize<'de> for Checked<InterpolationAlgorithm> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<InterpolationAlgorithm>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_INTERPOLATION_ALGORITHMS)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::InterpolationAlgorithm::*;
                use validation::Checked::*;
                Ok(match value {
                    "LINEAR" => Valid(Linear),
                    "STEP" => Valid(Step),
                    "CATMULLROMSPLINE" => Valid(CatmullRomSpline),
                    "CUBICSPLINE" => Valid(CubicSpline),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl<'de> de::Deserialize<'de> for Checked<TrsProperty> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<TrsProperty>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_TRS_PROPERTIES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::TrsProperty::*;
                use validation::Checked::*;
                Ok(match value {
                    "translation" => Valid(Translation),
                    "rotation" => Valid(Rotation),
                    "scale" => Valid(Scale),
                    "weights" => Valid(Weights),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
