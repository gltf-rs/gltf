
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {buffer, json};
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

/// Return type of `Loaded<Image>::data`.
pub enum Data<'a> {
    /// Image data is contained in a buffer view.
    FromBufferView {
        /// The buffer view containing the encoded image data.
        buffer_view: buffer::View<'a>,

        /// The image data MIME type.
        mime_type: &'a str,
    },

    /// Image data is contained in an external data source.
    External {
        /// The URI of the external data source.
        uri: &'a str,

        /// The image data MIME type, if provided.
        mime_type: Option<&'a str>,
    },
}

impl<'a> Image<'a> {
    /// Constructs an `Image` from owned data.
    pub(crate) fn new(
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

    /// Returns the raw image data.
    pub fn data(&self) -> Data<'a> {
        if let Some(index) = self.json.buffer_view.as_ref() {
            let buffer_view = self.gltf
                .views()
                .nth(index.value())
                .unwrap();
            let mime_type = self.json.mime_type
                .as_ref()
                .map(|x| x.0.as_str())
                .unwrap();
            Data::FromBufferView { buffer_view, mime_type }
        } else {
            let uri = self.json.uri.as_ref().unwrap();
            let mime_type = self.json.mime_type.as_ref().map(|x| x.0.as_str());
            Data::External { uri, mime_type }
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
