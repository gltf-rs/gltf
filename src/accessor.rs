
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::marker::PhantomData;
use std::mem::{size_of, transmute_copy};
use {buffer, json, Gltf};

/// The component data type.
#[derive(Clone, Copy, Debug)]
pub enum DataType {
    /// Corresponds to `GL_BYTE`.
    I8 = 5120,

    /// Corresponds to `GL_UNSIGNED_BYTE`.
    U8 = 5121,

    /// Corresponds to `GL_SHORT`.
    I16 = 5122,

    /// Corresponds to `GL_UNSIGNED_SHORT`.
    U16 = 5123,

    /// Corresponds to `GL_UNSIGNED_INT`.
    U32 = 5125,

    /// Corresponds to `GL_FLOAT`.
    F32 = 5126,
}

/// Specifies whether an attribute, vector, or matrix.
#[derive(Clone, Copy, Debug)]
pub enum Dimensions {
    /// Scalar quantity.
    Scalar,

    /// 2D vector.
    Vec2,

    /// 3D vector.
    Vec3,

    /// 4D vector.
    Vec4,

    /// 2x2 matrix.
    Mat2,

    /// 3x3 matrix.
    Mat3,

    /// 4x4 matrix.
    Mat4,
}

/// A typed view into a buffer view.
#[derive(Clone, Debug)]
pub struct Accessor<'a> {
    /// The parent `Gltf<'a>` struct.
    gltf: &'a Gltf<'a>,

    /// The corresponding JSON struct.
    json: &'a json::accessor::Accessor<'a>,
}

/// An `Iterator` that iterates over the members of an accessor.
#[derive(Clone, Debug)]
pub struct Iter<'a, T: 'a> {
    /// Number of iterations left.
    count: u32,

    /// The address of the value yielded from the last iteration.
    ptr: *const u8,

    /// The number of bytes to advance `ptr` by per iteration.
    stride: u32,

    /// Consumes the data type we're iterating over.
    _mk: PhantomData<&'a T>,
}

impl<'a> Accessor<'a> {
    /// Constructs an `Accessor`.
    pub fn new(gltf: &'a Gltf<'a>, json: &'a json::accessor::Accessor<'a>) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the size of each component that this accessor describes.
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
    pub unsafe fn iter<T>(&self) -> Iter<'a, T> {
        assert!(self.size() == size_of::<T>());
        let view = self.view();
        let data = view.data();
        let ptr = data.as_ptr().offset(self.json.byte_offset as isize);
        Iter {
            count: self.json.count,
            ptr: ptr,
            stride: view.stride().unwrap_or(size_of::<T>() as u32),
            _mk: PhantomData,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::accessor::Accessor<'a> {
        self.json
    }

    /// The parent buffer view this accessor reads from.
    pub fn view(&self) -> buffer::View<'a> {
        self.gltf.views().nth(self.json.buffer_view.value()).unwrap()
    }

    /// The offset relative to the start of the parent buffer view in bytes.
    pub fn offset(&self) -> u32 {
        self.json.byte_offset
    }

    /// The number of components within the buffer view - not to be confused with
    /// the number of bytes in the buffer view.
    pub fn count(&self) -> u32 {
        self.json.count
    }

    /// The data type of components in the attribute.
    pub fn data_type(&self) -> DataType {
        use self::DataType::*;
        use json::accessor::*;
        match self.json.component_type.0 {
            BYTE => I8,
            UNSIGNED_BYTE => U8,
            SHORT => I16,
            UNSIGNED_SHORT => U16,
            UNSIGNED_INT => U32,
            FLOAT => F32,
            _ => unreachable!(),
        }
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::accessor::AccessorExtensions<'a> {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras<'a> {
        &self.json.extras
    }

    /// Specifies if the attribute is a scalar, vector, or matrix.
    pub fn dimensions(&self) -> Dimensions {
        use self::Dimensions::*;
        match self.json.type_.0.as_ref() {
            "SCALAR" => Scalar,
            "VEC2" => Vec2,
            "VEC3" => Vec3,
            "VEC4" => Vec4,
            "MAT2" => Mat2,
            "MAT3" => Mat3,
            "MAT4" => Mat4,
            _ => unreachable!(),
        }
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
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_ref().map(Cow::as_ref)
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

impl DataType {
    /// Returns the number of bytes this value represents.
    pub fn size(&self) -> usize {
        use self::DataType::*;
        match *self {
            I8 | U8 => 1,
            I16 | U16 => 2,
            F32 | U32 => 4,
        }
    }
}

impl Dimensions {
    /// Returns the equivalent number of scalar quantities this dimension represents.
    pub fn multiplicity(&self) -> usize {
        use self::Dimensions::*;
        match *self {
            Scalar => 1,
            Vec2 => 2,
            Vec3 => 3,
            Vec4 => 4,
            Mat2 => 4,
            Mat3 => 9,
            Mat4 => 16,
        }
    }
}

impl<'a, T: 'a> ExactSizeIterator for Iter<'a, T> {}
impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            let value: T = unsafe {
                transmute_copy(&*self.ptr)
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
    
    ///  Indices of those attributes that deviate from their initialization value.
    pub struct Indices<'a> {
        /// The parent `Gltf<'a>` struct.
        gltf: &'a Gltf<'a>,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Indices<'a>,
    }

    impl<'a> Indices<'a> {
        /// Constructs a `Indices`.
        pub fn new(
            gltf: &'a Gltf<'a>,
            json: &'a json::accessor::sparse::Indices<'a>,
        ) -> Self {
            Self {
                gltf: gltf,
                json: json,
            }
        }

        /// Returns the internal JSON item.
        pub fn as_json(&self) ->  &json::accessor::sparse::Indices<'a> {
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
            use self::IndexType::*;
            use json::accessor::*;
            match self.json.component_type.0 {
                UNSIGNED_BYTE => U8,
                UNSIGNED_SHORT => U16,
                UNSIGNED_INT => U32,
                _ => unreachable!(),
            }
        }

        /// Extension specific data.
        pub fn extensions(&self) -> &json::accessor::sparse::IndicesExtensions<'a> {
            &self.json.extensions
        }

        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras<'a> {
            &self.json.extras
        }
    }
    ///Sparse storage of attributes that deviate from their initialization value.
    pub struct Sparse<'a> {
        /// The parent `Gltf<'a>` struct.
        gltf: &'a Gltf<'a>,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Sparse<'a>,
    }

    impl<'a> Sparse<'a> {
        /// Constructs a `Sparse`.
        pub fn new(
            gltf: &'a Gltf<'a>,
            json: &'a json::accessor::sparse::Sparse<'a>,
        ) -> Self {
            Self {
                gltf: gltf,
                json: json,
            }
        }

        /// Returns the internal JSON item.
        pub fn as_json(&self) -> &json::accessor::sparse::Sparse<'a> {
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
        pub fn extensions(&self) -> &json::accessor::sparse::StorageExtensions<'a> {
            &self.json.extensions
        }

        ///  Optional application specific data.
        pub fn extras(&self) -> &json::Extras<'a> {
            &self.json.extras
        }
    }

    ///  Array of size `count * number_of_components` storing the displaced accessor
    /// attributes pointed by `accessor::sparse::Indices`.
    pub struct Values<'a> {
        /// The parent `Gltf<'a>` struct.
        gltf: &'a Gltf<'a>,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Values<'a>,
    }

    impl<'a> Values<'a> {
        /// Constructs a `Values`.
        pub fn new(
            gltf: &'a Gltf<'a>,
            json: &'a json::accessor::sparse::Values<'a>,
        ) -> Self {
            Self {
                gltf: gltf,
                json: json,
            }
        }

        /// Returns the internal JSON item.
        pub fn as_json(&self) ->  &json::accessor::sparse::Values<'a> {
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

        /// Extension specific data.
        pub fn extensions(&self) -> &json::accessor::sparse::ValuesExtensions<'a> {
            &self.json.extensions
        }

        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras<'a> {
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
