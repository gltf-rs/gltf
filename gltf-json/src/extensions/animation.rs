
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// A keyframe animation.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Animation {}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Channel {}

/// The index of the node and TRS property that an animation channel targets.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct Target {}

/// Defines a keyframe graph but not its target.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct Sampler {}
