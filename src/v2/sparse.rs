
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;

///  Indices of those attributes that deviate from their initialization value.
pub struct Indices<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a Indices,
}

impl<'a> Indices<'a> {
    /// Constructs a `Indices`.
    pub fn new(gltf: &'a Gltf, json: &'a Indices) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &Indices {
        self.json
    }

    ///  The parent buffer view containing the sparse indices.  The referenced buffer view must not have `ARRAY_BUFFER` nor `ELEMENT_ARRAY_BUFFER` as its target.
    pub fn buffer_view(&self) -> &Index<buffer::View> {
        unimplemented!()
    }

    ///  The offset relative to the start of the parent `BufferView` in bytes.
    pub fn byte_offset(&self) -> &u32 {
        unimplemented!()
    }

    ///  The data type of each index.
    pub fn component_type(&self) -> &IndexComponentType {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &IndicesExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &Extras {
        unimplemented!()
    }
}
///  Sparse storage of attributes that deviate from their initialization value.
pub struct Sparse<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a Sparse,
}

impl<'a> Sparse<'a> {
    /// Constructs a `Sparse`.
    pub fn new(gltf: &'a Gltf, json: &'a Sparse) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &Sparse {
        self.json
    }

    ///  The number of attributes encoded in this sparse accessor.
    pub fn count(&self) -> &u32 {
        unimplemented!()
    }

    ///  Index array of size `count` that points to those accessor attributes that deviate from their initialization value.  Indices must strictly increase.
    pub fn indices(&self) -> &Indices {
        unimplemented!()
    }

    ///  Array of size `count * number_of_components` storing the displaced accessor attributes pointed by `indices`.  Substituted values must have the same `component_type` and number of components as the base `Accessor`.
    pub fn values(&self) -> &Values {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &StorageExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &Extras {
        unimplemented!()
    }
}
///  Array of size `count * number_of_components` storing the displaced accessor attributes pointed by `accessor::sparse::Indices`.
pub struct Values<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a Values,
}

impl<'a> Values<'a> {
    /// Constructs a `Values`.
    pub fn new(gltf: &'a Gltf, json: &'a Values) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &Values {
        self.json
    }

    ///  The parent buffer view containing the sparse indices.  The referenced buffer view must not have `ARRAY_BUFFER` nor `ELEMENT_ARRAY_BUFFER` as its target.
    pub fn buffer_view(&self) -> &Index<buffer::View> {
        unimplemented!()
    }

    ///  The offset relative to the start of the parent buffer view in bytes.
    pub fn byte_offset(&self) -> &u32 {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &ValuesExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &Extras {
        unimplemented!()
    }
}
