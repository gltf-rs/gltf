
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::slice;
use {accessor, json, scene, Gltf};

/// The interpolation algorithm.
#[derive(Clone, Debug)]
pub enum InterpolationAlgorithm {
    CatmullRomSpline,
    CubicSpline,
    Linear,
    Step,
}

/// The name of the node's TRS property to modify or the 'weights' of the
/// morph targets it instantiates.
#[derive(Clone, Debug)]
pub enum TrsProperty {
    Rotation,
    Scale,
    Translation,
    Weights,
}

/// A keyframe animation.
#[derive(Clone, Debug)]
pub struct Animation {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::animation::Animation,
}

/// An `Iterator` that visits the channels of an animation.
#[derive(Clone, Debug)]
pub struct Channels {
    /// The parent `Animation` struct.
    anim: Animation,

    /// The internal channel iterIterator.
    iter: slice::Iter<'a, json::animation::Channel>,
}

/// An `Iterator` that visits the samplers of an animation.
#[derive(Clone, Debug)]
pub struct Samplers {
    /// The parent `Channel` struct.
    anim: Animation,

    /// The internal channel iterIterator.
    iter: slice::Iter<'a, json::animation::Sampler>,
}

impl Animation {
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

    /// Extension specific data.
    pub fn extensions(&self) -> &json::animation::AnimationExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// An array of channels, each of which targets an animation's sampler at a
    /// node's property.  Different channels of the same animation must not have
    /// equal targets.
    pub fn channels(&self) -> Channels {
        Channels {
            anim: self.clone(),
            iter: self.json.channels.iter(),
        }
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
    }

    /// An array of samplers that combine input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub fn samplers(&self) -> Samplers {
        Samplers {
            anim: self.clone(),
            iter: self.json.samplers.iter(),
        }
    }
}

///  Targets an animation's sampler at a node's property.
#[derive(Clone, Debug)]
pub struct Channel {
    /// The parent `Animation` struct.
    anim: Animation,

    /// The corresponding JSON struct.
    json: &'a json::animation::Channel,
}

impl Channel {
    /// Constructs a `Channel`.
    pub fn new(anim: Animation, json: &'a json::animation::Channel) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation {
        self.anim.clone()
    }
    
    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::animation::Channel {
        self.json
    }

    /// Returns the sampler in this animation used to compute the value for the
    /// target.
    pub fn sampler(&self) -> Sampler {
        self.anim.samplers().nth(self.json.sampler.value()).unwrap()
    }

    /// Returns the node and TRS property to target.
    pub fn target(&self) -> Target {
        Target::new(self.anim.clone(), &self.json.target)
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::animation::ChannelExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

/// The node and TRS property that an animation channel targets.
#[derive(Clone, Debug)]
pub struct Target {
    /// The parent `Animation` struct.
    anim: Animation,

    /// The corresponding JSON struct.
    json: &'a json::animation::Target,
}

impl Target {
    /// Constructs a `Target`.
    pub fn new(anim: Animation, json: &'a json::animation::Target) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation {
        self.anim.clone()
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::animation::Target {
        self.json
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::animation::TargetExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// The node to target.
    pub fn node(&self) -> scene::Node {
        self.anim.gltf.nodes().nth(self.json.node.value()).unwrap()
    }

    /// The name of the node's TRS property to modify or the 'weights' of the morph
    /// targets it instantiates.
    pub fn path(&self) -> TrsProperty {
        use self::TrsProperty::*;
        match self.json.path.0.as_ref() {
            "translation" => Translation,
            "rotation" => Rotation,
            "scale" => Scale,
            "weights" => Weights,
            _ => unreachable!(),
        }
    }
}

/// Defines a keyframe graph but not its target.
#[derive(Clone, Debug)]
pub struct Sampler {
    /// The parent `Animation` struct.
    anim: Animation,

    /// The corresponding JSON struct.
    json: &'a json::animation::Sampler,
}

impl Sampler {
    /// Constructs a `Sampler`.
    pub fn new(anim: Animation, json: &'a json::animation::Sampler) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation {
        self.anim.clone()
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::animation::Sampler {
        self.json
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::animation::SamplerExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// The index of an accessor containing keyframe input values, e.g., time.
    pub fn input(&self) -> accessor::Accessor {
        self.anim.gltf.accessors().nth(self.json.input.value()).unwrap()
    }

    /// The interpolation algorithm.
    pub fn interpolation(&self) -> InterpolationAlgorithm {
        use self::InterpolationAlgorithm::*;
        match self.json.interpolation.0.as_ref() {
            "CATMULLROMSPLINE" => CatmullRomSpline,
            "CUBICSPLINE" => CubicSpline,
            "LINEAR" => Linear,
            "STEP" => Step,
            _ => unreachable!(),
        }
    }

    /// The index of an accessor containing keyframe output values.
    pub fn output(&self) -> accessor::Accessor {
        self.anim.gltf.accessors().nth(self.json.output.value()).unwrap()
    }
}

impl Iterator for Channels {
    type Item = Channel;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Channel::new(self.anim.clone(), json))
    }
}

impl Iterator for Samplers {
    type Item = Sampler;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Sampler::new(self.anim.clone(), json))
    }
}
