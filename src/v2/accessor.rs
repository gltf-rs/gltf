
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
    pub fn buffer_view(&self) -> buffer::View<'a> {
        unimplemented!()
    }

    ///  The offset relative to the start of the parent `BufferView` in bytes.
    pub fn byte_offset(&self) -> &u32 {
        unimplemented!()
    }

    ///  The number of components within the buffer view - not to be confused with the number of bytes in the buffer view.
    pub fn count(&self) -> &u32 {
        unimplemented!()
    }

    ///  The data type of components in the attribute.
    pub fn component_type(&self) -> &json::accessor::GenericComponentType {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::accessor::AccessorExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }

    ///  Specifies if the attribute is a scalar, vector, or matrix.
    pub fn type_(&self) -> &json::accessor::Type {
        unimplemented!()
    }

    ///  Minimum value of each component in this attribute.
    pub fn min(&self) -> &Vec<f32> {
        unimplemented!()
    }

    ///  Maximum value of each component in this attribute.
    pub fn max(&self) -> &Vec<f32> {
        unimplemented!()
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> &Option<String> {
        unimplemented!()
    }

    ///  Specifies whether integer data values should be normalized.
    pub fn normalized(&self) -> &bool {
        unimplemented!()
    }

    ///  Sparse storage of attributes that deviate from their initialization value.
    pub fn sparse(&self) -> &Option<json::accessor::sparse::Sparse> {
        unimplemented!()
    }
}
