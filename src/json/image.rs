
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::marker::PhantomData;

use json::{buffer, Extras, Index, Root};
use validation::{Error, JsonPath, Validate};

/// All valid MIME types.
pub const VALID_MIME_TYPES: &'static [&'static str] = &[
    "image/jpeg",
    "image/png",
];

/// Image data used to create a texture.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Image<'a> {
    /// The index of the buffer view that contains the image. Use this instead of
    /// the image's uri property.
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<Index<buffer::View<'a>>>,

    /// The image's MIME type.
    #[serde(rename = "mimeType")]
    pub mime_type: Option<MimeType<'a>>,

    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,

    /// The uri of the image.  Relative paths are relative to the .gltf file.
    /// Instead of referencing an external file, the uri can also be a data-uri.
    /// The image format must be jpg or png.
    pub uri: Option<Cow<'a, str>>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: ImageExtensions<'a>,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
}

/// Extension specific data for `Image`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct ImageExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// An image MIME type.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MimeType<'a>(pub Cow<'a, str>);

impl<'a> Validate<'a> for MimeType<'a> {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if !VALID_MIME_TYPES.contains(&self.0.as_ref()) {
            report(Error::invalid_enum(path(), self.0.clone()));
        }
    }
}
