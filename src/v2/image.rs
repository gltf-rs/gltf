
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;
use v2::{buffer, json};

///  Image data used to create a texture.
pub struct Image<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf<'a>,

    /// The corresponding JSON struct.
    json: &'a json::image::Image,
}

impl<'a> Image<'a> {
    /// Constructs a `Image`.
    pub fn new(gltf: &'a Gltf<'a>, json: &'a json::image::Image) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::image::Image {
        self.json
    }

    /// The index of the buffer view that contains the image. Use this instead of
    /// the image's uri property.
    pub fn buffer_view(&self) -> Option<buffer::View<'a>> {
        self.json.buffer_view.as_ref().map(|index| {
            buffer::View::new(self.gltf, self.gltf.as_json().get(index))
        })
    }

    /// The image's MIME type.
    pub fn mime_type(&self) -> Option<&str> {
        // TODO: Implement an enum for this?
        self.json.mime_type.as_ref().map(|x| x.0.as_ref())
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// The uri of the image. Relative paths are relative to the .gltf file. Instead
    /// of referencing an external file, the uri can also be a data-uri. The image
    /// format must be jpg or png.
    pub fn uri(&self) -> Option<&str> {
        self.json.uri.as_ref().map(String::as_ref)
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::image::ImageExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
