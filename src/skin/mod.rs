use crate::{Accessor, Document, Node};

#[cfg(feature = "utils")]
use crate::Buffer;

/// Iterators.
pub mod iter;

/// Utility functions.
#[cfg(feature = "utils")]
#[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
pub mod util;

#[cfg(feature = "utils")]
#[doc(inline)]
pub use self::util::Reader;

/// Joints and matrices defining a skin.
#[derive(Debug)]
pub struct Skin<'a, E: json::CustomExtensions> {
    /// The parent `Document` struct.
    document: &'a Document<E>,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::skin::Skin,
}

impl<'a, E: json::CustomExtensions> Clone for Skin<'a, E> {
    fn clone(&self) -> Self {
        Self {
            document: self.document,
            index: self.index,
            json: self.json,
        }
    }
}

impl<'a, E: json::CustomExtensions> Skin<'a, E> {
    /// Constructs a `Skin`.
    pub(crate) fn new(document: &'a Document<E>, index: usize, json: &'a json::skin::Skin) -> Self {
        Self {
            document,
            index,
            json,
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

    /// Returns the accessor containing the 4x4 inverse-bind matrices.
    ///
    /// When `None`, each matrix is assumed to be the 4x4 identity matrix which
    /// implies that the inverse-bind matrices were pre-applied.
    pub fn inverse_bind_matrices(&self) -> Option<Accessor<'a, E>> {
        self.json
            .inverse_bind_matrices
            .as_ref()
            .map(|index| self.document.accessors().nth(index.value()).unwrap())
    }

    /// Constructs a skin reader.
    #[cfg(feature = "utils")]
    #[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
    pub fn reader<'s, F>(&'a self, get_buffer_data: F) -> Reader<'a, 's, F, E>
    where
        F: Clone + Fn(Buffer<'a, E>) -> Option<&'s [u8]>,
    {
        Reader {
            skin: self.clone(),
            get_buffer_data,
        }
    }

    /// Returns an `Iterator` that visits the skeleton nodes used as joints in
    /// this skin.
    pub fn joints(&self) -> iter::Joints<'a, E> {
        iter::Joints {
            document: self.document,
            iter: self.json.joints.iter(),
        }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(docsrs, doc(cfg(feature = "names")))]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns the node used as the skeleton root. When `None`, joints
    /// transforms resolve to scene root.
    pub fn skeleton(&self) -> Option<Node<'a, E>> {
        self.json
            .skeleton
            .as_ref()
            .map(|index| self.document.nodes().nth(index.value()).unwrap())
    }
}
