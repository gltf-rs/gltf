use std::{collections, iter, slice};

use super::{Attribute, Mesh, MorphTarget, Primitive};
use crate::Document;

/// An `Iterator` that visits the morph targets of a `Primitive`.
#[derive(Clone, Debug)]
pub struct MorphTargets<'a, E: json::CustomExtensions> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document<E>,

    /// The internal JSON iterator.
    pub(crate) iter: slice::Iter<'a, json::mesh::MorphTarget>,
}

/// An `Iterator` that visits the attributes of a `Primitive`.
#[derive(Clone, Debug)]
pub struct Attributes<'a, E: json::CustomExtensions> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document<E>,

    /// The parent `Primitive` struct.
    #[allow(dead_code)]
    pub(crate) prim: Primitive<'a, E>,

    /// The internal attribute iterator.
    pub(crate) iter: collections::hash_map::Iter<
        'a,
        json::validation::Checked<json::mesh::Semantic>,
        json::Index<json::accessor::Accessor>,
    >,
}

/// An `Iterator` that visits the primitives of a `Mesh`.
#[derive(Clone, Debug)]
pub struct Primitives<'a, E: json::CustomExtensions> {
    /// The parent `Mesh` struct.
    pub(crate) mesh: Mesh<'a, E>,

    /// The internal JSON primitive iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::mesh::Primitive>>,
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Attributes<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Attributes<'a, E> {
    type Item = Attribute<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, index)| {
            let semantic = key.as_ref().unwrap().clone();
            let accessor = self.document.accessors().nth(index.value()).unwrap();
            (semantic, accessor)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Primitives<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Primitives<'a, E> {
    type Item = Primitive<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Primitive::new(self.mesh.clone(), index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let mesh = self.mesh;
        self.iter
            .last()
            .map(|(index, json)| Primitive::new(mesh, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Primitive::new(self.mesh.clone(), index, json))
    }
}

fn map_morph_target<'a, E: json::CustomExtensions>(
    document: &'a crate::Document<E>,
    json: &json::mesh::MorphTarget,
) -> MorphTarget<'a, E> {
    let positions = json
        .positions
        .as_ref()
        .map(|index| document.accessors().nth(index.value()).unwrap());
    let normals = json
        .normals
        .as_ref()
        .map(|index| document.accessors().nth(index.value()).unwrap());
    let tangents = json
        .tangents
        .as_ref()
        .map(|index| document.accessors().nth(index.value()).unwrap());
    MorphTarget {
        positions,
        normals,
        tangents,
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for MorphTargets<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for MorphTargets<'a, E> {
    type Item = MorphTarget<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|json| map_morph_target(self.document, json))
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
            .map(|json| map_morph_target(document, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|json| map_morph_target(self.document, json))
    }
}

/// An `Iterator` that visits the variant mappings of a `Mesh`.
#[cfg(feature = "KHR_materials_variants")]
#[derive(Clone, Debug)]
pub struct Mappings<'a> {
    /// Internal mapping iterator.
    pub(crate) iter: slice::Iter<'a, json::extensions::mesh::Mapping>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

#[cfg(feature = "KHR_materials_variants")]
impl<'a> ExactSizeIterator for Mappings<'a> {}
#[cfg(feature = "KHR_materials_variants")]
impl<'a> Iterator for Mappings<'a> {
    type Item = crate::khr_materials_variants::Mapping<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .next()
            .map(|json| crate::khr_materials_variants::Mapping::new(document, json))
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
            .map(|json| crate::khr_materials_variants::Mapping::new(document, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .nth(n)
            .map(|json| crate::khr_materials_variants::Mapping::new(document, json))
    }
}
