
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::Extras;

/// Image data used to create a texture.
#[derive(Debug, Deserialize, Serialize)]
pub struct Image<E: Extras> {
    /// The uri of the image.
    ///
    /// Relative paths are relative to the .gltf file.
    ///
    /// The image format must be jpg, png, bmp, or gif.
    pub uri: String,

    /// The user-defined name of this object.
    pub name: Option<String>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: ImageExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Image,
}

/// Extension specific data for `Image`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ImageExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

