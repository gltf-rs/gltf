use crate::{buffer, Document};

#[cfg(feature = "import")]
#[cfg_attr(docsrs, doc(cfg(feature = "import")))]
use image_crate::DynamicImage;

/// Format of image pixel data.
#[cfg(feature = "import")]
#[cfg_attr(docsrs, doc(cfg(feature = "import")))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Format {
    /// Red only.
    R8,

    /// Red, green.
    R8G8,

    /// Red, green, blue.
    R8G8B8,

    /// Red, green, blue, alpha.
    R8G8B8A8,

    /// Blue, green, red.
    B8G8R8,

    /// Blue, green, red, alpha.
    B8G8R8A8,

    /// Red only (16 bits).
    R16,

    /// Red, green (16 bits).
    R16G16,

    /// Red, green, blue (16 bits).
    R16G16B16,

    /// Red, green, blue, alpha (16 bits).
    R16G16B16A16,
}

/// Describes an image data source.
#[derive(Clone, Debug)]
pub enum Source<'a> {
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

/// Image data used to create a texture.
#[derive(Clone, Debug)]
pub struct Image<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::image::Image,
}

/// Image data belonging to an imported glTF asset.
#[cfg(feature = "import")]
#[cfg_attr(docsrs, doc(cfg(feature = "import")))]
#[derive(Clone, Debug)]
pub struct Data {
    /// The image pixel data (8 bits per channel).
    pub pixels: Vec<u8>,

    /// The image pixel data format.
    pub format: Format,

    /// The image height in pixels.
    pub width: u32,

    /// The image width in pixels.
    pub height: u32,
}

impl<'a> Image<'a> {
    /// Constructs an `Image` from owned data.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::image::Image,
    ) -> Self {
        Self {
            document: document,
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
    #[cfg_attr(docsrs, doc(cfg(feature = "names")))]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns the image data source.
    pub fn source(&self) -> Source<'a> {
        if let Some(index) = self.json.buffer_view.as_ref() {
            let view = self.document
                .views()
                .nth(index.value())
                .unwrap();
            let mime_type = self.json.mime_type
                .as_ref()
                .map(|x| x.0.as_str())
                .unwrap();
            Source::View { view, mime_type }
        } else {
            let uri = self.json.uri.as_ref().unwrap();
            let mime_type = self.json.mime_type.as_ref().map(|x| x.0.as_str());
            Source::Uri { uri, mime_type }
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

#[cfg(feature = "import")]
impl Data {
    /// Note: We don't implement `From<DynamicImage>` since we don't want
    /// to expose such functionality to the user.
    pub(crate) fn new(image: DynamicImage) -> Self {
        use image_crate::GenericImageView;
        let format = match image {
            DynamicImage::ImageLuma8(_) => Format::R8,
            DynamicImage::ImageLumaA8(_) => Format::R8G8,
            DynamicImage::ImageRgb8(_) => Format::R8G8B8,
            DynamicImage::ImageRgba8(_) => Format::R8G8B8A8,
            DynamicImage::ImageBgr8(_) => Format::B8G8R8,
            DynamicImage::ImageBgra8(_) => Format::B8G8R8A8,
            DynamicImage::ImageLuma16(_) => Format::R16,
            DynamicImage::ImageLumaA16(_) => Format::R16G16,
            DynamicImage::ImageRgb16(_) => Format::R16G16B16,
            DynamicImage::ImageRgba16(_) => Format::R16G16B16A16,
        };
        let (width, height) = image.dimensions();
        let pixels = image.to_bytes();
        Data { format, width, height, pixels }
    }
}
