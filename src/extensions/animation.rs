
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {animation, json, Gltf};

/// A keyframe animation.
#[derive(Clone, Debug)]
pub struct Animation<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::animation::Animation,
}

impl<'a> Animation<'a> {
    /// Constructs an `Animation`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::animation::Animation) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::animation::Animation {
        self.json
    }
}

///  Targets an animation's sampler at a node's property.
#[derive(Clone, Debug)]
pub struct Channel<'a> {
    /// The parent `Animation` struct.
    #[allow(dead_code)]
    anim: animation::Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::extensions::animation::Channel,
}

impl<'a> Channel<'a> {
    /// Constructs a `Channel`.
    pub fn new(anim: animation::Animation<'a>, json: &'a json::extensions::animation::Channel) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> animation::Animation<'a> {
        self.anim.clone()
    }
    
    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::animation::Channel {
        self.json
    }
}

/// The node and TRS property that an animation channel targets.
#[derive(Clone, Debug)]
pub struct Target<'a> {
    /// The parent `Animation` struct.
    #[allow(dead_code)]
    anim: animation::Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::extensions::animation::Target,
}

impl<'a> Target<'a> {
    /// Constructs a `Target`.
    pub fn new(anim: animation::Animation<'a>, json: &'a json::extensions::animation::Target) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> animation::Animation<'a> {
        self.anim.clone()
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::animation::Target {
        self.json
    }
}

/// Defines a keyframe graph but not its target.
#[derive(Clone, Debug)]
pub struct Sampler<'a> {
    /// The parent `Animation` struct.
    #[allow(dead_code)]
    anim: animation::Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::extensions::animation::Sampler,
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub fn new(anim: animation::Animation<'a>, json: &'a json::extensions::animation::Sampler) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> animation::Animation<'a> {
        self.anim.clone()
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::animation::Sampler {
        self.json
    }
}
