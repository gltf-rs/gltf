
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{marker, mem};
use {buffer, extensions, import, json};

use futures::future::SharedError;
use futures::{BoxFuture, Future};
use {Data, Gltf};

pub use json::accessor::ComponentType as DataType;
pub use json::accessor::Type as Dimensions;

/// A typed view into a buffer view.
#[derive(Clone, Debug)]
pub struct Accessor<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::accessor::Accessor,
}

/// An `Iterator` that iterates over the members of an accessor.
#[derive(Clone, Debug)]
pub struct Iter<T> {
    /// Number of iterations left.
    count: usize,

    /// The address of the value yielded from the last iteration.
    ptr: *const u8,

    /// The number of bytes to advance `ptr` by per iteration.
    stride: usize,

    /// The buffer view data we're iterating over.
    data: Data,

    /// Consumes the data type we're returning at each iteration.
    _mk: marker::PhantomData<T>,
}

impl<'a> Accessor<'a> {
    /// Constructs an `Accessor`.
    pub fn new(gltf: &'a Gltf, json: &'a json::accessor::Accessor) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the size of each component that this accessor describes.
    #[allow(dead_code)]
    fn size(&self) -> usize {
        self.data_type().size() * self.dimensions().multiplicity()
    }

    /// Returns an `Iterator` that interprets the data pointed to by the accessor
    /// as the given type.
    /// 
    /// The data referenced by the accessor is guaranteed to be appropriately
    /// aligned for any standard Rust type.
    ///
    /// # Panics
    ///
    /// If the size of an individual `T` does not match the accessor component size.
    pub unsafe fn iter<T>(&self) -> BoxFuture<Iter<T>, SharedError<import::Error>> {
        let count = self.count();
        let offset = self.offset() as isize;
        let stride = self.view().stride().unwrap_or(mem::size_of::<T>());
        self.view()
            .data()
            .map(move |data| {
                let ptr = data.as_ptr().offset(offset);
                Iter {
                    count: count,
                    ptr: ptr,
                    stride: stride,
                    data: data,
                    _mk: marker::PhantomData,
                }
            })
            .boxed()
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::accessor::Accessor {
        self.json
    }

    /// The parent buffer view this accessor reads from.
    pub fn view(&self) -> buffer::View<'a> {
        self.gltf.views().nth(self.json.buffer_view.value()).unwrap()
    }

    /// The offset relative to the start of the parent buffer view in bytes.
    pub fn offset(&self) -> usize {
        self.json.byte_offset as usize
    }

    /// The number of components within the buffer view - not to be confused with
    /// the number of bytes in the buffer view.
    pub fn count(&self) -> usize {
        self.json.count as usize
    }

    /// The data type of components in the attribute.
    pub fn data_type(&self) -> DataType {
        self.json.component_type.unwrap().0
    }

    /// Extension specific data.
    pub fn extensions(&self) -> extensions::accessor::Accessor<'a> {
        extensions::accessor::Accessor::new(
            self.gltf,
            &self.json.extensions,
        )
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Specifies if the attribute is a scalar, vector, or matrix.
    pub fn dimensions(&self) -> Dimensions {
        self.json.type_.unwrap()
    }

    /// Minimum value of each component in this attribute.
    pub fn min(&self) -> &[f32] {
        &self.json.min
    }

    /// Maximum value of each component in this attribute.
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

    /// Sparse storage of attributes that deviate from their initialization value.
    pub fn sparse(&self) -> Option<sparse::Sparse> {
        self.json.sparse.as_ref().map(|json| {
            sparse::Sparse::new(self.gltf, json)
        })
    }
}

impl<T> ExactSizeIterator for Iter<T> {}
impl<T> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            let value: T = unsafe {
                mem::transmute_copy(&*self.ptr)
            };
            self.count -= 1;
            unsafe {
                self.ptr = self.ptr.offset(self.stride as isize);
            }
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count as usize, Some(self.count as usize))
    }
}

/// Contains data structures for sparse storage.
pub mod sparse {
    use Gltf;
    use {buffer, extensions, json};

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

        /// Extension specific data.
        pub fn extensions(&self) -> extensions::accessor::sparse::Indices<'a> {
            extensions::accessor::sparse::Indices::new(
                self.gltf,
                &self.json.extensions,
            )
        }

        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
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

        /// The number of attributes encoded in this sparse accessor.
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
        pub fn extensions(&self) -> extensions::accessor::sparse::Sparse<'a> {
            extensions::accessor::sparse::Sparse::new(
                self.gltf,
                &self.json.extensions,
            )
        }

        ///  Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
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
        pub fn view(&self) -> buffer::View {
            self.gltf.views().nth(self.json.buffer_view.value()).unwrap()
        }

        /// The offset relative to the start of the parent buffer view in bytes.
        pub fn offset(&self) -> u32 {
            self.json.byte_offset
        }

        /// Extension specific data.
        pub fn extensions(&self) -> extensions::accessor::sparse::Values<'a> {
            extensions::accessor::sparse::Values::new(
                self.gltf,
                &self.json.extensions,
            )
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
