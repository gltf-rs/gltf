
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;
use v2::{buffer, json};

///  A typed view into a buffer view.
pub struct Accessor<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::accessor::Accessor,
}

impl<'a> Accessor<'a> {
    /// Constructs an `Accessor`.
    pub fn new(gltf: &'a Gltf, json: &'a json::accessor::Accessor) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::accessor::Accessor {
        self.json
    }

    ///  The parent buffer view this accessor reads from.
    pub fn view(&self) -> buffer::View<'a> {
        self.gltf.iter_views().nth(self.json.buffer_view.value() as usize).unwrap()
    }

    ///  The offset relative to the start of the parent buffer view in bytes.
    pub fn offset(&self) -> u32 {
        self.json.byte_offset
    }

    ///  The number of components within the buffer view - not to be confused with
    /// the number of bytes in the buffer view.
    pub fn count(&self) -> u32 {
        self.json.count
    }

    ///  The data type of components in the attribute.
    pub fn component_type(&self) -> ! {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::accessor::AccessorExtensions {
        &self.json.extensions
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    ///  Specifies if the attribute is a scalar, vector, or matrix.
    pub fn type_(&self) -> &json::accessor::Type {
        unimplemented!()
    }

    ///  Minimum value of each component in this attribute.
    pub fn min(&self) -> &[f32] {
        &self.json.min
    }

    ///  Maximum value of each component in this attribute.
    pub fn max(&self) -> &[f32] {
        &self.json.max
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    ///  Specifies whether integer data values should be normalized.
    pub fn normalized(&self) -> bool {
        self.json.normalized
    }

    ///  Sparse storage of attributes that deviate from their initialization value.
    pub fn sparse(&self) -> Option<sparse::Sparse<'a>> {
        self.json.sparse.as_ref().map(|json| {
            sparse::Sparse::new(self.gltf, json)
        })
    }
}

/// Contains data structures for sparse storage.
pub mod sparse {
    use v2::Gltf;
    use v2::{buffer, json};
    
    ///  Indices of those attributes that deviate from their initialization value.
    pub struct Indices<'a> {
        /// The parent `Gltf` struct.
        gltf: &'a Gltf,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Indices,
    }

    impl<'a> Indices<'a> {
        /// Constructs a `Indices`.
        pub fn new(
            gltf: &'a Gltf,
            json: &'a json::accessor::sparse::Indices,
        ) -> Self {
            Self {
                gltf: gltf,
                json: json,
            }
        }

        /// Returns the internal JSON item.
        pub fn as_json(&self) ->  &json::accessor::sparse::Indices {
            self.json
        }

        /// The parent buffer view containing the sparse indices.  The referenced
        /// buffer view must not have `ARRAY_BUFFER` nor `ELEMENT_ARRAY_BUFFER` as
        /// its target.
        pub fn view(&self) -> buffer::View<'a> {
            self.gltf.iter_views().nth(self.json.buffer_view.value() as usize).unwrap()
        }

        /// The offset relative to the start of the parent buffer view in bytes.
        pub fn offset(&self) -> u32 {
            self.json.byte_offset
        }

        /// The data type of each index.
        pub fn component_type(&self) -> ! {
            unimplemented!()
        }

        /// Extension specific data.
        pub fn extensions(&self) -> &json::accessor::sparse::IndicesExtensions {
            &self.json.extensions
        }

        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
        }
    }
    ///Sparse storage of attributes that deviate from their initialization value.
    pub struct Sparse<'a> {
        /// The parent `Gltf` struct.
        gltf: &'a Gltf,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Sparse,
    }

    impl<'a> Sparse<'a> {
        /// Constructs a `Sparse`.
        pub fn new(
            gltf: &'a Gltf,
            json: &'a json::accessor::sparse::Sparse,
        ) -> Self {
            Self {
                gltf: gltf,
                json: json,
            }
        }

        /// Returns the internal JSON item.
        pub fn as_json(&self) -> &json::accessor::sparse::Sparse {
            self.json
        }

        ///The number of attributes encoded in this sparse accessor.
        pub fn count(&self) -> u32 {
            self.json.count
        }

        /// Index array of size `count` that points to those accessor attributes
        /// that deviate from their initialization value.  Indices must strictly
        /// increase.
        pub fn indices(&self) -> Indices<'a> {
            Indices::new(self.gltf, &self.json.indices)
        }

        /// Array of size `count * number_of_components` storing the displaced
        /// accessor attributes pointed by `indices`.  Substituted values must have
        /// the same `component_type` and number of components as the base
        /// `Accessor`.
        pub fn values(&self) -> Values<'a> {
            Values::new(self.gltf, &self.json.values)
        }

        ///  Extension specific data.
        pub fn extensions(&self) -> &json::accessor::sparse::StorageExtensions {
            &self.json.extensions
        }

        ///  Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
        }
    }

    ///  Array of size `count * number_of_components` storing the displaced accessor
    /// attributes pointed by `accessor::sparse::Indices`.
    pub struct Values<'a> {
        /// The parent `Gltf` struct.
        gltf: &'a Gltf,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Values,
    }

    impl<'a> Values<'a> {
        /// Constructs a `Values`.
        pub fn new(
            gltf: &'a Gltf,
            json: &'a json::accessor::sparse::Values,
        ) -> Self {
            Self {
                gltf: gltf,
                json: json,
            }
        }

        /// Returns the internal JSON item.
        pub fn as_json(&self) ->  &json::accessor::sparse::Values {
            self.json
        }

        /// The parent buffer view containing the sparse indices.  The referenced
        /// buffer view must not have `ARRAY_BUFFER` nor `ELEMENT_ARRAY_BUFFER` as
        /// its target.
        pub fn view(&self) -> buffer::View<'a> {
            self.gltf.iter_views().nth(self.json.buffer_view.value() as usize).unwrap()
        }

        /// The offset relative to the start of the parent buffer view in bytes.
        pub fn offset(&self) -> u32 {
            self.json.byte_offset
        }

        /// Extension specific data.
        pub fn extensions(&self) -> &json::accessor::sparse::ValuesExtensions {
            &self.json.extensions
        }

        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
        }
    }
}
