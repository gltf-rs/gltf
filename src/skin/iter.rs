use std::slice;

use crate::{Document, Node};

/// An `Iterator` that visits the joints of a `Skin`.
#[derive(Clone, Debug)]
pub struct Joints<'a, E: json::CustomExtensions> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document<E>,

    /// The internal node index iterator.
    pub(crate) iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Joints<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Joints<'a, E> {
    type Item = Node<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.document.nodes().nth(index.value()).unwrap())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|index| document.nodes().nth(index.value()).unwrap())
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|index| self.document.nodes().nth(index.value()).unwrap())
    }
}
