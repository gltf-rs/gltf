
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;
use v2::{buffer, json};

pub enum MimeType {}

///  Image data used to create a texture.
pub struct Image<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::image::Image,
}

impl<'a> Image<'a> {
    /// Constructs a `Image`.
    pub fn new(gltf: &'a Gltf, json: &'a json::image::Image) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::image::Image {
        self.json
    }

    ///  The index of the buffer view that contains the image. Use this instead of the image's uri property.
    pub fn buffer_view(&self) -> buffer::View<'a> {
        unimplemented!()
    }

    ///  The image's MIME type.
    pub fn mime_type(&self) -> &Option<MimeType> {
        unimplemented!()
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> &Option<String> {
        unimplemented!()
    }

    ///  The uri of the image. Relative paths are relative to the .gltf file. Instead of referencing an external file, the uri can also be a data-uri. The image format must be jpg or png.
    pub fn uri(&self) -> &Option<String> {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::image::ImageExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }
}
