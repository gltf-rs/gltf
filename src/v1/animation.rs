// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde_json;

use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Target {
    /// The ID of the node to target.
    pub id: String,

    /// The name of the node's TRS property to modify.
    pub path: String,

    // TODO: extension
    // TODO: extras
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Channel {
    /// The ID of a sampler in this animation used to compute the value for the
    /// target, e.g., a node's translation, rotation, or scale (TRS).
    pub sampler: String,

    /// The ID of the node and TRS property to target.
    pub target: Target,

    // TODO: extension
    // TODO: extras
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Sampler {
    /// The ID of a parameter in this animation to use as keyframe input.
    ///
    /// This parameter must have type FLOAT. The values represent time in
    /// seconds with `time[0] >= 0.0`, and monotonically increasing values,
    /// i.e., time[n + 1] >= time[n]
    pub input: String,

    /// Interpolation algorithm.
    ///
    /// When an animation targets a node's rotation, and the animation's
    /// interpolation is "LINEAR", spherical linear interpolation (slerp) should
    /// be used to interpolate quaternions.
    #[serde(default = "sampler_interpolation_default")]
    pub interpolation: String,

    /// The ID of a parameter in this animation to use as keyframe output.
    pub output: String,

    // TODO: extension
    // TODO: extras
}

fn sampler_interpolation_default() -> String {
    "LINEAR".to_string()
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Animation {
    /// An array of channels, each of which targets an animation's sampler at a
    /// node's property.
    #[serde(default)]
    pub channels: Vec<Channel>,

    /// A dictionary object of strings whose values are IDs of accessors with
    /// keyframe data, e.g., time, translation, rotation, etc.
    #[serde(default)]
    pub parameters: HashMap<String, String>,

    /// A dictionary object of animation.sampler objects that combines input and
    /// output parameters with an interpolation algorithm to define a keyframe
    /// graph (but not its target).
    #[serde(default)]
    pub samplers: HashMap<String, Sampler>,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., an animation and a buffer could
    /// have the same name, or two animations could even have the same name.
    pub name: Option<String>,

    // TODO: extension
    // TODO: extras
}
