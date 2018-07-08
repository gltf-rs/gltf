use validation::{Error, Validate};
use {buffer, extensions, Extras, Index, Root, Path};

/// All valid MIME types.
pub const VALID_MIME_TYPES: &'static [&'static str] = &[
    "image/jpeg",
    "image/png",
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
    #[serde(default)]
    pub extensions: extensions::image::Image,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// An image MIME type.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MimeType(pub String);

impl Validate for MimeType {
    fn validate_completely<P, R>(&self, _: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        if !VALID_MIME_TYPES.contains(&self.0.as_str()) {
            report(&path, Error::Invalid);
        }
    }
}
