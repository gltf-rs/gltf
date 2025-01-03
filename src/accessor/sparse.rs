use crate::{buffer, Document};

/// The index data type.
#[derive(Clone, Debug)]
pub enum IndexType {
    /// Corresponds to `GL_UNSIGNED_BYTE`.
    U8 = 5121,

    /// Corresponds to `GL_UNSIGNED_SHORT`.
    U16 = 5123,

    /// Corresponds to `GL_UNSIGNED_INT`.
    U32 = 5125,
}

/// Indices of those attributes that deviate from their initialization value.
pub struct Indices<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::accessor::sparse::Indices,
}

impl<'a> Indices<'a> {
    /// Constructs `sparse::Indices`.
    pub(crate) fn new(document: &'a Document, json: &'a json::accessor::sparse::Indices) -> Self {
        Self { document, json }
    }

    /// Returns the buffer view containing the sparse indices.
    pub fn view(&self) -> buffer::View<'a> {
        self.document
            .views()
            .nth(self.json.buffer_view.value())
            .unwrap()
    }

    /// The offset relative to the start of the parent buffer view in bytes.
    pub fn offset(&self) -> usize {
        self.json.byte_offset.0 as usize
    }

    /// The data type of each index.
    pub fn index_type(&self) -> IndexType {
        match self.json.component_type.unwrap().0 {
            json::accessor::ComponentType::U8 => IndexType::U8,
            json::accessor::ComponentType::U16 => IndexType::U16,
            json::accessor::ComponentType::U32 => IndexType::U32,
            _ => unreachable!(),
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// Sparse storage of attributes that deviate from their initialization value.
pub struct Sparse<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::accessor::sparse::Sparse,
}

impl<'a> Sparse<'a> {
    /// Constructs `Sparse`.
    pub(crate) fn new(document: &'a Document, json: &'a json::accessor::sparse::Sparse) -> Self {
        Self { document, json }
    }

    /// Returns the number of attributes encoded in this sparse accessor.
    pub fn count(&self) -> usize {
        self.json.count.0 as usize
    }

    /// Returns an index array of size `count` that points to those accessor
    /// attributes that deviate from their initialization value.
    pub fn indices(&self) -> Indices<'a> {
        Indices::new(self.document, &self.json.indices)
    }

    /// Returns an array of size `count * number_of_components`, storing the
    /// displaced accessor attributes pointed by `indices`.
    pub fn values(&self) -> Values<'a> {
        Values::new(self.document, &self.json.values)
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// Array of size `count * number_of_components` storing the displaced accessor
/// attributes pointed by `accessor::sparse::Indices`.
pub struct Values<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::accessor::sparse::Values,
}

impl<'a> Values<'a> {
    /// Constructs `sparse::Values`.
    pub(crate) fn new(document: &'a Document, json: &'a json::accessor::sparse::Values) -> Self {
        Self { document, json }
    }

    /// Returns the buffer view containing the sparse values.
    pub fn view(&self) -> buffer::View<'a> {
        self.document
            .views()
            .nth(self.json.buffer_view.value())
            .unwrap()
    }

    /// The offset relative to the start of the parent buffer view in bytes.
    pub fn offset(&self) -> usize {
        self.json.byte_offset.0 as usize
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

impl IndexType {
    /// Returns the number of bytes this value represents.
    pub fn size(&self) -> usize {
        use self::IndexType::*;
        match *self {
            U8 => 1,
            U16 => 2,
            U32 => 4,
        }
    }
}
