use crate::{Document, Material};

/// A variant.
pub struct Variant<'a, E: json::CustomExtensions> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document<E>,

    /// The corresponding JSON index.
    #[allow(dead_code)]
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::extensions::scene::khr_materials_variants::Variant,
}

impl<'a, E: json::CustomExtensions> Variant<'a, E> {
    /// Constructs a `Variant`.
    pub(crate) fn new(
        document: &'a Document<E>,
        index: usize,
        json: &'a json::extensions::scene::khr_materials_variants::Variant,
    ) -> Self {
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
pub struct Mapping<'a, E: json::CustomExtensions> {
    /// The parent `Document` struct.
    document: &'a Document<E>,

    /// The corresponding JSON struct.
    json: &'a json::extensions::mesh::Mapping,
}

impl<'a, E: json::CustomExtensions> Mapping<'a, E> {
    /// Constructs a `Mapping`.
    pub(crate) fn new(
        document: &'a Document<E>,
        json: &'a json::extensions::mesh::Mapping,
    ) -> Self {
        Self { document, json }
    }

    /// Get the variant indices that use this material.
    pub fn variants(&self) -> &'a [u32] {
        &self.json.variants
    }

    /// Get the corresponding material.
    pub fn material(&self) -> Material<'a, E> {
        self.document
            .materials()
            .nth(self.json.material as usize)
            .unwrap_or_else(|| Material::default(self.document))
    }
}
