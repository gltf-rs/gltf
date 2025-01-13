use crate::validation::Validate;
use crate::{buffer, extensions, Extras, Index};
use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// All valid MIME types.
pub const VALID_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    #[cfg(feature = "EXT_texture_webp")]
    "image/webp",
];

/// Image data used to create a texture.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Image {
    /// The index of the buffer view that contains the image. Use this instead of
    /// the image's uri property.
    #[serde(rename = "bufferView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<Index<buffer::View>>,

    /// The image's MIME type.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<MimeType>,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// The uri of the image.  Relative paths are relative to the .gltf file.
    /// Instead of referencing an external file, the uri can also be a data-uri.
    /// The image format must be jpg or png.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::image::Image>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// An image MIME type.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MimeType(pub String);

impl Validate for MimeType {}
