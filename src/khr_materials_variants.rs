use gltf_json::Extras;
use crate::{Document, Material};

/// A variant.
pub struct Variant<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::extensions::scene::khr_materials_variants::Variant,
}

impl<'a> Variant<'a> {
    /// Constructs a `Variant`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a json::extensions::scene::khr_materials_variants::Variant) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Name of the variant.
    pub fn name(&self) -> &'a str {
        &self.json.name
    }
}

/// A mapping.
pub struct Mapping<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::extensions::mesh::Mapping,
}

impl<'a> Mapping<'a> {
    /// Constructs a `Mapping`.
    pub(crate) fn new(document: &'a Document, json: &'a json::extensions::mesh::Mapping) -> Self {
        Self {
            document,
            json,
        }
    }

    /// Get the variant indices that use this material.
    pub fn variants(&self) -> &'a [u32] {
        &self.json.variants
    }

    /// Get the corresponding material.
    pub fn material(&self) -> Material<'a> {
        self.document.materials().nth(self.json.material as usize)
        .unwrap_or_else(|| Material::default(self.document))
    }
}
