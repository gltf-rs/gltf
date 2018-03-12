use {buffer, json};
use Document;

/// Image data used to create a texture.
pub struct Image<'a> {
    /// The parent `Gltf` struct.
    doc: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::image::Image,
}

/// The data referenced by an `Image`.
pub enum Data<'a> {
    /// Image data is contained in a buffer view.
    View {
        /// The buffer view containing the encoded image data.
        view: buffer::View<'a>,

        /// The image data MIME type.
        mime_type: &'a str,
    },

    /// Image data is contained in an external data source.
    Uri {
        /// The URI of the external data source.
        uri: &'a str,

        /// The image data MIME type, if provided.
        mime_type: Option<&'a str>,
    },
}

impl<'a> Image<'a> {
    /// Constructs an `Image` from owned data.
    pub(crate) fn new(doc: &'a Document, index: usize, json: &'a json::image::Image) -> Self {
        Self {
            doc,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns references to image data.
    pub fn data(&self) -> Data<'a> {
        if let Some(index) = self.json.buffer_view.as_ref() {
            let view = self.doc.views().nth(index.value()).unwrap();
            let mime_type = self.json.mime_type.as_ref().map(|x| x.0.as_str()).unwrap();
            Data::View { view, mime_type }
        } else {
            let uri = self.json.uri.as_ref().unwrap();
            let mime_type = self.json.mime_type.as_ref().map(|x| x.0.as_str());
            Data::Uri { uri, mime_type }
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
