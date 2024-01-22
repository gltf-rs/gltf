use crate::validation::Validate;
use crate::{buffer, Extras, Index, UnrecognizedExtensions};

/// All valid MIME types.
pub const VALID_MIME_TYPES: &[&str] = &["image/jpeg", "image/png"];

/// Image data used to create a texture.
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate)]
pub struct Image {
    /// The index of the buffer view that contains the image. Use this instead of
    /// the image's uri property.
    pub buffer_view: Option<Index<buffer::View>>,

    /// The image's MIME type.
    pub mime_type: Option<MimeType>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The uri of the image.  Relative paths are relative to the .gltf file.
    /// Instead of referencing an external file, the uri can also be a data-uri.
    /// The image format must be jpg or png.
    pub uri: Option<String>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// An image MIME type.
#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct MimeType(pub String);

impl Validate for MimeType {}
