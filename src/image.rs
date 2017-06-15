
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {json, Gltf};

///  Image data used to create a texture.
pub struct Image<'a> {
    /// The corresponding image data.
    data: &'a [u8],

    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::image::Image,
}

impl<'a> Image<'a> {
    /// Constructs an `Image` from owned data.
    pub fn new(gltf: &'a Gltf, json: &'a json::image::Image, data: &'a [u8]) -> Self {
        Self {
            data: data,
            gltf: gltf,
            json: json,
        }
    }
    
    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::image::Image {
        self.json
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns the image data.
    pub fn data(&self) -> &[u8] {
        self.data
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
