use json;
use std::slice;

use {Accessor, Document, Node};

/// Joints and matrices defining a skin.
#[derive(Clone, Debug)]
pub struct Skin<'a> {
    /// The parent `Gltf` struct.
    doc: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::skin::Skin,
}

/// An `Iterator` that visits the joints of a `Skin`.
#[derive(Clone, Debug)]
pub struct Joints<'a> {
    /// The parent `Gltf` struct.
    doc: &'a Document,

    /// The internal node index iterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

impl<'a> Skin<'a> {
    /// Constructs a `Skin`.
    pub(crate) fn new(doc: &'a Document, index: usize, json: &'a json::skin::Skin) -> Self {
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

    /// Returns the accessor containing the 4x4 inverse-bind matrices.
    ///
    /// When `None`, each matrix is assumed to be the 4x4 identity matrix which
    /// implies that the inverse-bind matrices were pre-applied.
    pub fn inverse_bind_matrices(&self) -> Option<Accessor<'a>> {
        self.json
            .inverse_bind_matrices
            .as_ref()
            .map(|index| self.doc.accessors().nth(index.value()).unwrap())
    }

    /// Returns an `Iterator` that visits the skeleton nodes used as joints in
    /// this skin.
    pub fn joints(&self) -> Joints<'a> {
        Joints {
            doc: self.doc,
            iter: self.json.joints.iter(),
        }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns the node used as the skeleton root. When `None`, joints
    /// transforms resolve to scene root.
    pub fn skeleton(&self) -> Option<Node<'a>> {
        self.json
            .skeleton
            .as_ref()
            .map(|index| self.doc.nodes().nth(index.value()).unwrap())
    }
}

impl<'a> Iterator for Joints<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.doc.nodes().nth(index.value()).unwrap())
    }
}
