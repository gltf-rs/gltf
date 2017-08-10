
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{marker, mem};
use {buffer, json};

use {Gltf, Loaded, Source};

pub use json::accessor::ComponentType as DataType;
pub use json::accessor::Type as Dimensions;

/// A typed view into a buffer view.
#[derive(Clone, Debug)]
pub struct Accessor<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::accessor::Accessor,

    /// The buffer view this accessor reads from.
    view: buffer::View<'a>,
}

/// An `Iterator` that iterates over the members of an accessor.
#[derive(Clone, Debug)]
pub struct Iter<'a, T: Copy> {
    /// The total number of iterations left.
    count: usize,

    /// The index of the next iteration.
    index: usize,

    /// The number of bytes between each item.
    stride: usize,

    /// Byte offset into the buffer view where the items begin.
    offset: usize,
    
    /// The accessor we're iterating over.
    accessor: Loaded<'a, Accessor<'a>>,

    /// Consumes the data type we're returning at each iteration.
    _consume_data_type: marker::PhantomData<T>,
}

impl<'a> Accessor<'a> {
    /// Constructs an `Accessor`.
    pub fn new(
        gltf: &'a Gltf,
        index: usize,
        json: &'a json::accessor::Accessor,
    ) -> Self {
        let view = gltf.views().nth(index).unwrap();
        Self {
            gltf,
            index,
            json,
            view,
        }
    }

    /// Converts an `Accessor` into a `Loaded<Accessor>`.
    pub fn loaded(self, source: &'a Source) -> Loaded<'a, Accessor<'a>> {
        Loaded {
            item: self,
            source,
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

    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::accessor::Accessor {
        self.json
    }

    /// Returns the buffer view this accessor reads from.
    pub fn view(&self) -> buffer::View<'a> {
        self.gltf.views().nth(self.json.buffer_view.value()).unwrap()
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
    pub fn min(&self) -> &[f32] {
        &self.json.min
    }

    /// Returns the maximum value of each component in this attribute.
    pub fn max(&self) -> &[f32] {
        &self.json.max
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Specifies whether integer data values should be normalized.
    pub fn normalized(&self) -> bool {
        self.json.normalized
    }

    /// Returns sparse storage of attributes that deviate from their initialization
    /// value.
    pub fn sparse(&self) -> Option<sparse::Sparse> {
        self.json.sparse.as_ref().map(|json| {
            sparse::Sparse::new(self.gltf, json)
        })
    }
}

impl<'a> Loaded<'a, Accessor<'a>> {
    /// Returns an `Iterator` that interprets the data pointed to by the accessor
    /// as the given type.
    /// 
    /// The data referenced by the accessor is guaranteed to be appropriately
    /// aligned for any standard Rust type.
    ///
    /// # Panics
    ///
    /// If the size of an individual `T` does not match the accessor component size.
    pub unsafe fn iter<T: Copy>(&self) -> Iter<'a, T> {
        assert_eq!(self.size(), mem::size_of::<T>());
        let count = self.count();
        let offset = self.offset();
        let stride = self.view.stride().unwrap_or(mem::size_of::<T>());
        Iter {
            accessor: self.clone(),
            count,
            offset,
            stride,
            index: 0,
            _consume_data_type: marker::PhantomData,
        }
    }

    /// Returns sparse storage of attributes that deviate from their initialization
    /// value.
    pub fn sparse(&'a self) -> Option<Loaded<'a, sparse::Sparse<'a>>> {
        self.item
            .sparse()
            .map(|item| {
                Loaded {
                    item,
                    source: self.source,
                }
            })
    }

    /// Returns the buffer view this accessor reads from.
    pub fn view(&'a self) -> Loaded<'a, buffer::View<'a>> {
        Loaded {
            item: self.item.view(),
            source: self.source,
        }
    }
}

impl<'a, T: Copy> ExactSizeIterator for Iter<'a, T> {}
impl<'a, T: Copy> Iterator for Iter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let ptr_offset = self.offset + self.index * self.stride;
            let view = Loaded {
                item: self.accessor.view(),
                source: self.accessor.source,
            };
            let data = view.data();
            let ptr = unsafe { data.as_ptr().offset(ptr_offset as isize) };
            let value: T = unsafe { mem::transmute_copy(&*ptr) };
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.count - self.index;
        (hint, Some(hint))
    }
}

/// Contains data structures for sparse storage.
pub mod sparse {
    use {Gltf, Loaded};
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
        gltf: &'a Gltf,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Indices,
    }

    impl<'a> Indices<'a> {
        /// Constructs `sparse::Indices`.
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

        /// Returns the buffer view containing the sparse indices.
        pub fn view(&self) -> buffer::View<'a> {
            self.gltf.views().nth(self.json.buffer_view.value()).unwrap()
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
    
    impl<'a> Loaded<'a, Indices<'a>> {
        /// Returns the buffer view containing the sparse indices.
        pub fn view(&self) -> Loaded<'a, buffer::View<'a>> {
            Loaded {
                item: self.item.view(),
                source: self.source,
            }
        }
    }
        
    /// Sparse storage of attributes that deviate from their initialization value.
    pub struct Sparse<'a> {
        /// The parent `Gltf` struct.
        gltf: &'a Gltf,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Sparse,
    }

    impl<'a> Sparse<'a> {
        /// Constructs `Sparse`.
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

        /// Returns the number of attributes encoded in this sparse accessor.
        pub fn count(&self) -> u32 {
            self.json.count
        }

        /// Returns an index array of size `count` that points to those accessor
        /// attributes that deviate from their initialization value.
        pub fn indices(&self) -> Indices<'a> {
            Indices::new(self.gltf, &self.json.indices)
        }

        /// Returns an array of size `count * number_of_components`, storing the
        /// displaced accessor attributes pointed by `indices`.
        pub fn values(&self) -> Values<'a> {
            Values::new(self.gltf, &self.json.values)
        }

        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
        }
    }

    impl<'a> Loaded<'a, Sparse<'a>> {
        /// Returns an index array of size `count` that points to those accessor
        /// attributes that deviate from their initialization value.
        pub fn indices(&'a self) -> Loaded<'a, Indices<'a>> {
            Loaded {
                item: self.item.indices(),
                source: self.source,
            }
        }

        /// Returns an array of size `count * number_of_components`, storing the
        /// displaced accessor attributes pointed by `indices`.
        pub fn values(&'a self) -> Loaded<'a, Values<'a>> {
            Loaded {
                item: self.item.values(),
                source: self.source,
            }
        }
    }
    
    /// Array of size `count * number_of_components` storing the displaced accessor
    /// attributes pointed by `accessor::sparse::Indices`.
    pub struct Values<'a> {
        /// The parent `Gltf` struct.
        gltf: &'a Gltf,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Values,
    }

    impl<'a> Values<'a> {
        /// Constructs `sparse::Values`.
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

        /// Returns the buffer view containing the sparse values.
        pub fn view(&self) -> buffer::View {
            self.gltf.views().nth(self.json.buffer_view.value()).unwrap()
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

    
    impl<'a> Loaded<'a, Values<'a>> {
        /// Returns the buffer view containing the sparse values.
        pub fn view(&'a self) -> Loaded<'a, buffer::View> {
            Loaded {
                item: self.item.view(),
                source: self.source,
            }
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
