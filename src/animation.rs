
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::slice::Iter as SliceIter;
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
pub struct Animation<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::animation::Animation,
}

/// An `Iterator` that visits the channels of an animation.
#[derive(Clone, Debug)]
pub struct IterChannels<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The internal channel iterator.
    iter: SliceIter<'a, json::animation::Channel>,
}

/// An `Iterator` that visits the samplers of an animation.
#[derive(Clone, Debug)]
pub struct IterSamplers<'a> {
    /// The parent `Channel` struct.
    anim: Animation<'a>,

    /// The internal channel iterator.
    iter: SliceIter<'a, json::animation::Sampler>,
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
    pub fn iter_channels(&self) -> IterChannels<'a> {
        IterChannels {
            anim: self.clone(),
            iter: self.json.channels.iter(),
        }
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// An array of samplers that combine input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub fn iter_samplers(&self) -> IterSamplers<'a> {
        IterSamplers {
            anim: self.clone(),
            iter: self.json.samplers.iter(),
        }
    }
}

///  Targets an animation's sampler at a node's property.
#[derive(Clone, Debug)]
pub struct Channel<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Channel,
}

impl<'a> Channel<'a> {
    /// Constructs a `Channel`.
    pub fn new(anim: Animation<'a>, json: &'a json::animation::Channel) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation<'a> {
        self.anim.clone()
    }
    
    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::animation::Channel {
        self.json
    }

    /// Returns the sampler in this animation used to compute the value for the
    /// target.
    pub fn sampler(&self) -> Sampler<'a> {
        self.anim.iter_samplers().nth(self.json.sampler.value() as usize).unwrap()
    }

    /// Returns the node and TRS property to target.
    pub fn target(&self) -> Target<'a> {
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
pub struct Target<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Target,
}

impl<'a> Target<'a> {
    /// Constructs a `Target`.
    pub fn new(anim: Animation<'a>, json: &'a json::animation::Target) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation<'a> {
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
    pub fn node(&self) -> scene::Node<'a> {
        self.anim.gltf.iter_nodes().nth(self.json.node.value() as usize).unwrap()
    }

    /// The name of the node's TRS property to modify or the 'weights' of the morph
    /// targets it instantiates.
    pub fn path(&self) -> TrsProperty {
        // TODO: Not sure how to implement this.
        unimplemented!()
    }
}

///  Defines a keyframe graph but not its target.
#[derive(Clone, Debug)]
pub struct Sampler<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Sampler,
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub fn new(anim: Animation<'a>, json: &'a json::animation::Sampler) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation<'a> {
        self.anim.clone()
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::animation::Sampler {
        self.json
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::animation::SamplerExtensions {
        &self.json.extensions
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    ///  The index of an accessor containing keyframe input values, e.g., time.
    pub fn input(&self) -> accessor::Accessor<'a> {
        self.anim.gltf.iter_accessors().nth(self.json.input.value() as usize).unwrap()
    }

    ///  The interpolation algorithm.
    pub fn interpolation(&self) -> InterpolationAlgorithm {
        use self::InterpolationAlgorithm::*;
        match self.json.interpolation.0.as_str() {
            "CATMULLROMSPLINE" => CatmullRomSpline,
            "CUBICSPLINE" => CubicSpline,
            "LINEAR" => Linear,
            "STEP" => Step,
            _ => unreachable!(),
        }
    }

    ///  The index of an accessor containing keyframe output values.
    pub fn output(&self) -> accessor::Accessor<'a> {
        self.anim.gltf.iter_accessors().nth(self.json.output.value() as usize).unwrap()
    }
}

impl<'a> Iterator for IterChannels<'a> {
    type Item = Channel<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Channel::new(self.anim.clone(), json))
    }
}

impl<'a> Iterator for IterSamplers<'a> {
    type Item = Sampler<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Sampler::new(self.anim.clone(), json))
    }
}
