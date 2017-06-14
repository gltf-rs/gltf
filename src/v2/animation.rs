
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;
use v2::{accessor, json, scene};

pub enum InterpolationAlgorithm {}
pub enum TrsProperty {}

///  A keyframe animation.
pub struct Animation<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::animation::Animation,
}

impl<'a> Animation<'a> {
    /// Constructs an `Animation`.
    pub fn new(gltf: &'a Gltf, json: &'a json::animation::Animation) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::animation::Animation {
        self.json
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::animation::AnimationExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }

    ///  An array of channels, each of which targets an animation's sampler at a node's property.  Different channels of the same animation must not have equal targets.
    pub fn channels(&self) -> &Vec<Channel> {
        unimplemented!()
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> &Option<String> {
        unimplemented!()
    }

    ///  An array of samplers that combine input and output accessors with an interpolation algorithm to define a keyframe graph (but not its target).
    pub fn samplers(&self) -> ! {
        unimplemented!()
    }
}

///  Targets an animation's sampler at a node's property.
pub struct Channel<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::animation::Channel,
}

impl<'a> Channel<'a> {
    /// Constructs a `Channel`.
    pub fn new(gltf: &'a Gltf, json: &'a json::animation::Channel) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::animation::Channel {
        self.json
    }

    ///  The index of a sampler in this animation used to compute the value for the target.
    pub fn sampler(&self) -> Sampler<'a> {
        unimplemented!()
    }

    ///  The index of the node and TRS property to target.
    pub fn target(&self) -> ! {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::animation::ChannelExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }
}
///  The index of the node and TRS property that an animation channel targets.
pub struct Target<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::animation::Target,
}

impl<'a> Target<'a> {
    /// Constructs a `Target`.
    pub fn new(gltf: &'a Gltf, json: &'a json::animation::Target) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::animation::Target {
        self.json
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::animation::TargetExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }

    ///  The index of the node to target.
    pub fn node(&self) -> &scene::Node<'a> {
        unimplemented!()
    }

    ///  The name of the node's TRS property to modify or the 'weights' of the morph targets it instantiates.
    pub fn path(&self) -> TrsProperty {
        unimplemented!()
    }
}
///  Defines a keyframe graph but not its target.
pub struct Sampler<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::animation::Sampler,
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub fn new(gltf: &'a Gltf, json: &'a json::animation::Sampler) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::animation::Sampler {
        self.json
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::animation::SamplerExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }

    ///  The index of an accessor containing keyframe input values, e.g., time.
    pub fn input(&self) -> accessor::Accessor<'a> {
        unimplemented!()
    }

    ///  The interpolation algorithm.
    pub fn interpolation(&self) -> InterpolationAlgorithm {
        unimplemented!()
    }

    ///  The index of an accessor containing keyframe output values.
    pub fn output(&self) -> accessor::Accessor<'a> {
        unimplemented!()
    }
}
