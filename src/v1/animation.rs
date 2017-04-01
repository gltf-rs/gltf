// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use v1::Extras;

enum_string! {
    Interpolation {
        Linear = "LINEAR",
        Step = "STEP",
    }
}

enum_string! {
    Path {
        Translation = "translation",
        Rotation = "rotation",
        Scale = "scale",
    }
}

/// A keyframe animation
#[derive(Debug, Deserialize, Serialize)]
pub struct Animation<E: Extras> {
    /// An array of channels, each of which targets an animation's sampler at a
    /// node's property
    #[serde(default)]
    pub channels: Vec<Channel<E>>,

    /// A dictionary object of strings whose values are IDs of accessors with
    /// keyframe data, e.g., time, translation, rotation, etc
    #[serde(default)]
    pub parameters: HashMap<String, String>,

    /// A dictionary object of animation.sampler objects that combines input and
    /// output parameters with an interpolation algorithm to define a keyframe
    /// graph (but not its target)
    #[serde(default)]
    pub samplers: HashMap<String, Sampler<E>>,

    /// Optional user-defined name of this object
    pub name: Option<String>,

    /// Extension specific data
    #[serde(default)]
    pub extensions: AnimationExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Animation,
}

/// Extension specific data for `Animation`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AnimationExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Targets an animation's sampler at a node's property
#[derive(Debug, Deserialize, Serialize)]
pub struct Channel<E: Extras> {
    /// The ID of a sampler in this animation used to compute the value for the
    /// target, e.g., a node's translation, rotation, or scale (TRS).
    pub sampler: String,

    /// The ID of the node and TRS property to target.
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

/// Combines input and output parameters with an interpolation algorithm to
/// define a keyframe graph (but not its target)
#[derive(Debug, Deserialize, Serialize)]
pub struct Sampler<E: Extras> {
    /// The ID of a parameter in this animation to use as keyframe input
    ///
    /// This parameter must have type `f32`. The values represent time in
    /// seconds with `time[0] >= 0.0`, and monotonically increasing values,
    /// i.e., time[n + 1] >= time[n]
    pub input: String,

    /// Interpolation algorithm
    ///
    /// When an animation targets a node's rotation, and the animation's
    /// interpolation is `Linear`, spherical linear interpolation (slerp) should
    /// be used to interpolate quaternions
    #[serde(default)]
    pub interpolation: Interpolation,

    /// The ID of a parameter in this animation to use as keyframe output
    pub output: String,

    /// Extension specific data
    #[serde(default)]
    pub extensions: SamplerExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::AnimationSampler,
}

/// Extension specific data for `Sampler`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SamplerExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Defines the ID of the node and TRS property that an animation channel targets
#[derive(Debug, Deserialize, Serialize)]
pub struct Target<E: Extras> {
    /// The ID of the node to target
    pub id: String,

    /// The name of the node's TRS property to modify
    pub path: Path,

    /// Extension specific data
    #[serde(default)]
    pub extensions: TargetExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::AnimationTarget,
}

/// Extension specific data for `Target`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TargetExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl Default for Interpolation {
    fn default() -> Interpolation {
        Interpolation::Linear
    }
}

#[cfg(test)]
mod test {
    extern crate serde_json;
    use super::*;
    use v1;

    #[test]
    fn it_deserializes_an_animation() {
        let data = r#"{
    "channels": [
        {
            "sampler": "a_sampler",
            "target": {
                "id": "node_id",
                "path": "rotation",
                "extensions": {
                    "extension_name": {
                        "extension specific": "value"
                    }
                },
                "extras": {
                    "Application specific": "The extra object can contain any properties."
                }
            },
            "extensions": {
                "extension_name": {
                    "extension specific": "value"
                }
            },
            "extras": {
                "Application specific": "The extra object can contain any properties."
            }
        }
    ],
    "name": "user-defined animation name",
    "parameters": {
        "TIME": "time_accessor",
        "rotation": "rotation_accessor"
    },
    "samplers": {
        "a_sampler": {
            "input": "TIME",
            "interpolation": "LINEAR",
            "output": "rotation",
            "extensions": {
                "extension_name": {
                    "extension specific": "value"
                }
            },
            "extras": {
                "Application specific": "The extra object can contain any properties."
            }
        }
    },
    "extensions": {
        "extension_name": {
            "extension specific": "value"
        }
    },
    "extras": {
        "Application specific": "The extra object can contain any properties."
    }
}"#;
        let animation: Animation<v1::extras::Any> = serde_json::from_str(data).unwrap();

        assert_eq!("user-defined animation name", animation.name.unwrap());
        assert_eq!(2, animation.parameters.len());
    }
}
