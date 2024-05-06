use std::{iter, slice};

use crate::animation::{Animation, Channel, Sampler};

/// An `Iterator` that visits the channels of an animation.
#[derive(Clone, Debug)]
pub struct Channels<'a> {
    /// The parent `Animation` struct.
    pub(crate) anim: Animation<'a>,

    /// The internal channel iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::animation::Channel>>,
}

/// An `Iterator` that visits the samplers of an animation.
#[derive(Clone, Debug)]
pub struct Samplers<'a> {
    /// The parent `Channel` struct.
    pub(crate) anim: Animation<'a>,

    /// The internal channel iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::animation::Sampler>>,
}

impl<'a> Iterator for Channels<'a> {
    type Item = Channel<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Channel::new(self.anim.clone(), json, index))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let anim = self.anim;
        self.iter
            .last()
            .map(|(index, json)| Channel::new(anim, json, index))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Channel::new(self.anim.clone(), json, index))
    }
}

impl<'a> Iterator for Samplers<'a> {
    type Item = Sampler<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Sampler::new(self.anim.clone(), json, index))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let anim = self.anim;
        self.iter
            .last()
            .map(|(index, json)| Sampler::new(anim, json, index))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Sampler::new(self.anim.clone(), json, index))
    }
}
