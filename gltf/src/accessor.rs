use {buffer, json};

use Document;

pub use json::accessor::ComponentType as DataType;
pub use json::accessor::Type as Dimensions;

/// A typed view into a buffer view.
#[derive(Clone, Debug)]
pub struct Accessor<'a> {
    /// The parent `Gltf` struct.
    doc: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::accessor::Accessor,

    /// The buffer view this accessor reads from.
    view: buffer::View<'a>,
}

impl<'a> Accessor<'a> {
    /// Constructs an `Accessor`.
    pub(crate) fn new(doc: &'a Document, index: usize, json: &'a json::accessor::Accessor) -> Self {
        let view = doc.views().nth(json.buffer_view.value()).unwrap();
        Self {
            doc,
            index,
            json,
            view,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the size of each component that this accessor describes.
    pub fn size(&self) -> usize {
        self.data_type().size() * self.dimensions().multiplicity()
    }

    /// Returns the buffer view this accessor reads from.
    pub fn view(&self) -> buffer::View<'a> {
        self.doc
            .views()
            .nth(self.json.buffer_view.value())
            .unwrap()
    }

    /// Returns the offset relative to the start of the parent buffer view in bytes.
    pub fn offset(&self) -> usize {
        self.json.byte_offset as usize
    }

    /// Returns the number of components within the buffer view - not to be confused
    /// with the number of bytes in the buffer view.
    pub fn count(&self) -> usize {
        self.json.count as usize
    }

    /// Returns the data type of components in the attribute.
    pub fn data_type(&self) -> DataType {
        self.json.component_type.unwrap().0
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Specifies if the attribute is a scalar, vector, or matrix.
    pub fn dimensions(&self) -> Dimensions {
        self.json.type_.unwrap()
    }

    /// Returns the minimum value of each component in this attribute.
    pub fn min(&self) -> Option<json::Value> {
        self.json.min.clone()
    }

    /// Returns the maximum value of each component in this attribute.
    pub fn max(&self) -> Option<json::Value> {
        self.json.max.clone()
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Specifies whether integer data values should be normalized.
    pub fn normalized(&self) -> bool {
        self.json.normalized
    }

    /// Returns sparse storage of attributes that deviate from their initialization
    /// value.
    pub fn sparse(&self) -> Option<sparse::Sparse> {
        self.json
            .sparse
            .as_ref()
            .map(|json| sparse::Sparse::new(self.doc, json))
    }
}

/// Contains data structures for sparse storage.
pub mod sparse {
    use Document;
    use {buffer, json};

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
        /// The parent `Gltf` struct.
        doc: &'a Document,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Indices,
    }

    impl<'a> Indices<'a> {
        /// Constructs `sparse::Indices`.
        pub(crate) fn new(doc: &'a Document, json: &'a json::accessor::sparse::Indices) -> Self {
            Self {
                doc,
                json,
            }
        }

        /// Returns the internal JSON item.
        #[doc(hidden)]
        pub fn as_json(&self) -> &json::accessor::sparse::Indices {
            self.json
        }

        /// Returns the buffer view containing the sparse indices.
        pub fn view(&self) -> buffer::View<'a> {
            self.doc
                .views()
                .nth(self.json.buffer_view.value())
                .unwrap()
        }

        /// The offset relative to the start of the parent buffer view in bytes.
        pub fn offset(&self) -> u32 {
            self.json.byte_offset
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
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
        }
    }

    /// Sparse storage of attributes that deviate from their initialization value.
    pub struct Sparse<'a> {
        /// The parent `Gltf` struct.
        doc: &'a Document,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Sparse,
    }

    impl<'a> Sparse<'a> {
        /// Constructs `Sparse`.
        pub(crate) fn new(doc: &'a Document, json: &'a json::accessor::sparse::Sparse) -> Self {
            Self {
                doc,
                json,
            }
        }

        /// Returns the internal JSON item.
        #[doc(hidden)]
        pub fn as_json(&self) -> &json::accessor::sparse::Sparse {
            self.json
        }

        /// Returns the number of attributes encoded in this sparse accessor.
        pub fn count(&self) -> u32 {
            self.json.count
        }

        /// Returns an index array of size `count` that points to those accessor
        /// attributes that deviate from their initialization value.
        pub fn indices(&self) -> Indices<'a> {
            Indices::new(self.doc, &self.json.indices)
        }

        /// Returns an array of size `count * number_of_components`, storing the
        /// displaced accessor attributes pointed by `indices`.
        pub fn values(&self) -> Values<'a> {
            Values::new(self.doc, &self.json.values)
        }

        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
        }
    }

    /// Array of size `count * number_of_components` storing the displaced accessor
    /// attributes pointed by `accessor::sparse::Indices`.
    pub struct Values<'a> {
        /// The parent `Gltf` struct.
        doc: &'a Document,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Values,
    }

    impl<'a> Values<'a> {
        /// Constructs `sparse::Values`.
        pub(crate) fn new(doc: &'a Document, json: &'a json::accessor::sparse::Values) -> Self {
            Self {
                doc,
                json,
            }
        }

        /// Returns the internal JSON item.
        #[doc(hidden)]
        pub fn as_json(&self) -> &json::accessor::sparse::Values {
            self.json
        }

        /// Returns the buffer view containing the sparse values.
        pub fn view(&self) -> buffer::View {
            self.doc
                .views()
                .nth(self.json.buffer_view.value())
                .unwrap()
        }

        /// The offset relative to the start of the parent buffer view in bytes.
        pub fn offset(&self) -> u32 {
            self.json.byte_offset
        }

        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
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
}
