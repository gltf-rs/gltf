use std::slice;
use {accessor, json, scene, Document};

pub use json::animation::{InterpolationAlgorithm, TrsProperty};

/// A keyframe animation.
#[derive(Clone, Debug)]
pub struct Animation<'a> {
    /// The parent `Gltf` struct.
    doc: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::animation::Animation,
}

/// An `Iterator` that visits the channels of an animation.
#[derive(Clone, Debug)]
pub struct Channels<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The internal channel iterator.
    iter: slice::Iter<'a, json::animation::Channel>,
}

/// An `Iterator` that visits the samplers of an animation.
#[derive(Clone, Debug)]
pub struct Samplers<'a> {
    /// The parent `Channel` struct.
    anim: Animation<'a>,

    /// The internal channel iterator.
    iter: slice::Iter<'a, json::animation::Sampler>,
}

impl<'a> Animation<'a> {
    /// Constructs an `Animation`.
    pub(crate) fn new(doc: &'a Document, index: usize, json: &'a json::animation::Animation) -> Self {
        Self {
            doc,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Returns an `Iterator` over the animation channels.
    ///
    /// Each channel targets an animation's sampler at a node's property.
    pub fn channels(&self) -> Channels<'a> {
        Channels {
            anim: self.clone(),
            iter: self.json.channels.iter(),
        }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns an `Iterator` over the animation samplers.
    ///
    /// Each sampler combines input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub fn samplers(&self) -> Samplers<'a> {
        Samplers {
            anim: self.clone(),
            iter: self.json.samplers.iter(),
        }
    }
}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug)]
pub struct Channel<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Channel,
}

impl<'a> Channel<'a> {
    /// Constructs a `Channel`.
    pub(crate) fn new(anim: Animation<'a>, json: &'a json::animation::Channel) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation<'a> {
        self.anim.clone()
    }

    /// Returns the sampler in this animation used to compute the value for the
    /// target.
    pub fn sampler(&self) -> Sampler<'a> {
        self.anim.samplers().nth(self.json.sampler.value()).unwrap()
    }

    /// Returns the node and TRS property to target.
    pub fn target(&self) -> Target<'a> {
        Target::new(self.anim.clone(), &self.json.target)
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
    pub(crate) fn new(anim: Animation<'a>, json: &'a json::animation::Target) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation<'a> {
        self.anim.clone()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Returns the target node.
    pub fn node(&self) -> scene::Node {
        self.anim.doc.nodes().nth(self.json.node.value()).unwrap()
    }

    /// Returns the node's TRS property to modify or the 'weights' of the morph
    /// targets it instantiates.
    pub fn path(&self) -> TrsProperty {
        self.json.path.unwrap()
    }
}

/// Defines a keyframe graph (but not its target).
#[derive(Clone, Debug)]
pub struct Sampler<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Sampler,
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub(crate) fn new(anim: Animation<'a>, json: &'a json::animation::Sampler) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation<'a> {
        self.anim.clone()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Returns the accessor containing the keyframe input values (e.g. time).
    pub fn input(&self) -> accessor::Accessor<'a> {
        self.anim
            .doc
            .accessors()
            .nth(self.json.input.value())
            .unwrap()
    }

    /// Returns the keyframe interpolation algorithm.
    pub fn interpolation(&self) -> InterpolationAlgorithm {
        self.json.interpolation.unwrap()
    }

    /// Returns the accessor containing the keyframe output values.
    pub fn output(&self) -> accessor::Accessor<'a> {
        self.anim
            .doc
            .accessors()
            .nth(self.json.output.value())
            .unwrap()
    }
}

impl<'a> Iterator for Channels<'a> {
    type Item = Channel<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|json| Channel::new(self.anim.clone(), json))
    }
}

impl<'a> Iterator for Samplers<'a> {
    type Item = Sampler<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|json| Sampler::new(self.anim.clone(), json))
    }
}
