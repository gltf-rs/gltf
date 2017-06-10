
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::json::{buffer, Extras, Index};

/// Image data used to create a texture.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
    /// The index of the buffer view that contains the image. Use this instead of the image's uri property.
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<Index<buffer::View>>,

    /// The image's MIME type.
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The uri of the image.  Relative paths are relative to the .gltf file.  Instead of referencing an external file, the uri can also be a data-uri.  The image format must be jpg or png.
    pub uri: Option<String>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: ImageExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Image`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ImageExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}
