
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {json, Gltf};

/// Image data used to create a texture.
pub struct Image<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::image::Image,
}

impl<'a> Image<'a> {
    /// Constructs an `Image` from owned data.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::image::Image) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }
    
    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::extensions::image::Image {
        self.json
    }
}
