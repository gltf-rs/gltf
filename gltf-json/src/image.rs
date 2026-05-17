use crate::validation::{Error, Validate};
use crate::{buffer, extensions, Extras, Index, Path, Root};
use serde_derive::{Deserialize, Serialize};

/// All valid MIME types.
pub const VALID_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    #[cfg(feature = "KHR_texture_basisu")]
    "image/ktx2",
    #[cfg(feature = "EXT_texture_webp")]
    "image/webp",
];

/// Image data used to create a texture.
#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl Validate for Image {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        // Generated part.
        self.buffer_view
            .validate(root, || path().field("bufferView"), report);
        self.mime_type
            .validate(root, || path().field("mimeType"), report);
        self.uri.validate(root, || path().field("uri"), report);
        self.extensions
            .validate(root, || path().field("extensions"), report);
        self.extras
            .validate(root, || path().field("extras"), report);

        // Custom part: spec requires exactly one of `bufferView` or `uri`,
        // and `mimeType` is required when `bufferView` is used.
        match (self.buffer_view.is_some(), self.uri.is_some()) {
            (true, true) | (false, false) => {
                report(&|| path().field("uri"), Error::Invalid);
            }
            (true, false) if self.mime_type.is_none() => {
                report(&|| path().field("mimeType"), Error::Missing);
            }
            _ => {}
        }
    }
}

/// An image MIME type.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MimeType(pub String);

impl Validate for MimeType {}

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_root(json: &str) -> Vec<(crate::Path, Error)> {
        let root: Root = serde_json::from_str(json).expect("parse root");
        let mut errors = Vec::new();
        root.validate(&root, crate::Path::new, &mut |path, error| {
            errors.push((path(), error));
        });
        errors
    }

    #[test]
    fn image_with_buffer_view_requires_mime_type() {
        // `bufferView` is set but `mimeType` is absent. `Image::source`
        // would otherwise unwrap the absent mime type.
        let json = r#"{
            "asset": { "version": "2.0" },
            "buffers": [{ "byteLength": 4 }],
            "bufferViews": [{ "buffer": 0, "byteLength": 4 }],
            "images": [{ "bufferView": 0 }]
        }"#;
        let errors = validate_root(json);
        assert!(
            errors.iter().any(|(_, e)| *e == Error::Missing),
            "expected Error::Missing, got {:?}",
            errors
        );
    }

    #[test]
    fn image_without_buffer_view_or_uri_is_rejected() {
        // Neither `bufferView` nor `uri` set. Spec requires exactly one.
        let json = r#"{
            "asset": { "version": "2.0" },
            "images": [ {} ]
        }"#;
        let errors = validate_root(json);
        assert!(
            errors.iter().any(|(_, e)| *e == Error::Invalid),
            "expected Error::Invalid, got {:?}",
            errors
        );
    }
}
