
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {extensions, import, json};

use Gltf;

/// Image data used to create a texture.
pub struct Image<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::image::Image,
}

impl<'a> Image<'a> {
    /// Constructs an `Image` from owned data.
    pub fn new(
        gltf: &'a Gltf,
        index: usize,
        json: &'a json::image::Image,
    ) -> Self {
        Self {
            gltf: gltf,
            index: index,
            json: json,
        }
    }
    
    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::image::Image {
        self.json
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns the image data.
    pub fn data(&self) -> import::Data {
        unimplemented!()
    }

    /// Extension specific data.
    pub fn extensions(&self) -> extensions::image::Image<'a> {
        extensions::image::Image::new(
            self.gltf,
            &self.json.extensions,
        )
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
