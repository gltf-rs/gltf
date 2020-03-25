use crate::{accessor, scene, Document};

#[cfg(feature = "utils")]
use crate::Buffer;

pub use json::animation::{Interpolation, Property};

/// Iterators.
pub mod iter;

/// Utility functions.
#[cfg(feature = "utils")]
#[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
pub mod util;

#[cfg(feature = "utils")]
#[doc(inline)]
pub use self::util::Reader;

/// A keyframe animation.
#[derive(Clone, Debug)]
pub struct Animation<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::animation::Animation,
}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug)]
pub struct Channel<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Channel,
}

/// Defines a keyframe graph (but not its target).
#[derive(Clone, Debug)]
pub struct Sampler<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Sampler,
}

/// The node and TRS property that an animation channel targets.
#[derive(Clone, Debug)]
pub struct Target<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Target,
}

impl<'a> Animation<'a> {
    /// Constructs an `Animation`.
    pub(crate) fn new(
        document: &'a Document, index: usize,
        json: &'a json::animation::Animation,
    ) -> Self {
        Self {
            document: document,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }

    /// Returns an `Iterator` over the animation channels.
    ///
    /// Each channel targets an animation's sampler at a node's property.
    pub fn channels(&self) -> iter::Channels<'a> {
        iter::Channels {
            anim: self.clone(),
            iter: self.json.channels.iter(),
        }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns an `Iterator` over the animation samplers.
    ///
    /// Each sampler combines input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub fn samplers(&self) -> iter::Samplers<'a> {
        iter::Samplers {
            anim: self.clone(),
            iter: self.json.samplers.iter(),
        }
    }
}

impl<'a> Channel<'a> {
    /// Constructs a `Channel`.
    pub(crate) fn new(
        anim: Animation<'a>,
        json: &'a json::animation::Channel,
    ) -> Self {
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

    /// Returns the node and property to target.
    pub fn target(&self) -> Target<'a> {
        Target::new(self.anim.clone(), &self.json.target)
    }

    /// Constructs an animation channel reader.
    #[cfg(feature = "utils")]
    #[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
    pub fn reader<'s, F>(&self, get_buffer_data: F) -> Reader<'a, 's, F>
    where
        F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
    {
        Reader {
            channel: self.clone(),
            get_buffer_data,
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

impl<'a> Target<'a> {
    /// Constructs a `Target`.
    pub(crate) fn new(
        anim: Animation<'a>,
        json: &'a json::animation::Target,
    ) -> Self {
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
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }

    /// Returns the target node.
    pub fn node(&self) -> scene::Node<'a> {
        self.anim.document.nodes().nth(self.json.node.value()).unwrap()
    }

    /// Returns the node's property to modify or the 'weights' of the morph
    /// targets it instantiates.
    pub fn property(&self) -> Property {
        self.json.path.unwrap()
    }
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub(crate) fn new(
        anim: Animation<'a>,
        json: &'a json::animation::Sampler,
    ) -> Self {
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
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }

    /// Returns the accessor containing the keyframe input values (e.g. time).
    pub fn input(&self) -> accessor::Accessor<'a> {
        self.anim.document.accessors().nth(self.json.input.value()).unwrap()
    }

    /// Returns the keyframe interpolation algorithm.
    pub fn interpolation(&self) -> Interpolation {
        self.json.interpolation.unwrap()
    }

    /// Returns the accessor containing the keyframe output values.
    pub fn output(&self) -> accessor::Accessor<'a> {
        self.anim.document.accessors().nth(self.json.output.value()).unwrap()
    }
}
