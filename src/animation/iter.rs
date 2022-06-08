use std::slice;

use crate::animation::{Animation, Channel, Sampler};

/// An `Iterator` that visits the channels of an animation.
#[derive(Clone, Debug)]
pub struct Channels<'a, E: json::ThirdPartyExtensions> {
    /// The parent `Animation` struct.
    pub(crate) anim: Animation<'a, E>,

    /// The internal channel iterator.
    pub(crate) iter: slice::Iter<'a, json::animation::Channel>,
}

/// An `Iterator` that visits the samplers of an animation.
#[derive(Clone, Debug)]
pub struct Samplers<'a, E: json::ThirdPartyExtensions> {
    /// The parent `Channel` struct.
    pub(crate) anim: Animation<'a, E>,

    /// The internal channel iterator.
    pub(crate) iter: slice::Iter<'a, json::animation::Sampler>,
}

impl<'a, E: json::ThirdPartyExtensions> Iterator for Channels<'a, E> {
    type Item = Channel<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|json| Channel::new(self.anim.clone(), json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let anim = self.anim;
        self.iter.last().map(|json| Channel::new(anim, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|json| Channel::new(self.anim.clone(), json))
    }
}

impl<'a, E: json::ThirdPartyExtensions> Iterator for Samplers<'a, E> {
    type Item = Sampler<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|json| Sampler::new(self.anim.clone(), json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let anim = self.anim;
        self.iter.last().map(|json| Sampler::new(anim, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|json| Sampler::new(self.anim.clone(), json))
    }
}
