
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{buffer, Extras, Index, Root};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ImageExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Image data used to create a texture
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image<E: Extras> {
    /// The index of the `BufferView` that contains the image
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<Index<buffer::BufferView<E>>>,
    
    /// The image's MIME type
    // N.B. The spec says this is required but the sample models don't provide it
    // TODO: Remove `Option` as necessary
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    
    /// Optional user-defined name for this object
    pub name: Option<String>,
    
    /// The uniform resource identifier of the image relative to the .gltf file
    pub uri: Option<String>,
    
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: ImageExtensions,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Image,
}

impl<E: Extras> Image<E> {
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
        if let Some(ref buffer_view) = self.buffer_view {
            let _ = root.try_get(buffer_view)?;
        }
        Ok(())
    }
}
