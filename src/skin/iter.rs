use std::slice;

use crate::{Document, Node};

/// An `Iterator` that visits the joints of a `Skin`.
#[derive(Clone, Debug)]
pub struct Joints<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The internal node index iterator.
    pub(crate) iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

impl<'a> Iterator for Joints<'a>  {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.document.nodes().nth(index.value()).unwrap())
    }
}
