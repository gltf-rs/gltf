#[allow(unused)]
use crate::{buffer, Document, Error, Result};

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

    /// Red only (16 bits).
    R16,

    /// Red, green (16 bits).
    R16G16,

    /// Red, green, blue (16 bits).
    R16G16B16,

    /// Red, green, blue, alpha (16 bits).
    R16G16B16A16,

    /// Red, green, blue (32 bits float)
    R32G32B32FLOAT,

    /// Red, green, blue, alpha (32 bits float)
    R32G32B32A32FLOAT,
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

    /// The image width in pixels.
    pub width: u32,

    /// The image height in pixels.
    pub height: u32,
}

impl<'a> Image<'a> {
    /// Constructs an `Image` from owned data.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a json::image::Image) -> Self {
        Self {
            document,
            index,
            json,
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
        self.json.name.as_deref()
    }

    /// Returns the image data source.
    pub fn source(&self) -> Source<'a> {
        if let Some(index) = self.json.buffer_view.as_ref() {
            let view = self.document.views().nth(index.value()).unwrap();
            let mime_type = self.json.mime_type.as_ref().map(|x| x.0.as_str()).unwrap();
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
    pub(crate) fn new(image: DynamicImage) -> Result<Self> {
        use image_crate::GenericImageView;
        let format = match image {
            DynamicImage::ImageLuma8(_) => Format::R8,
            DynamicImage::ImageLumaA8(_) => Format::R8G8,
            DynamicImage::ImageRgb8(_) => Format::R8G8B8,
            DynamicImage::ImageRgba8(_) => Format::R8G8B8A8,
            DynamicImage::ImageLuma16(_) => Format::R16,
            DynamicImage::ImageLumaA16(_) => Format::R16G16,
            DynamicImage::ImageRgb16(_) => Format::R16G16B16,
            DynamicImage::ImageRgba16(_) => Format::R16G16B16A16,
            DynamicImage::ImageRgb32F(_) => Format::R32G32B32FLOAT,
            DynamicImage::ImageRgba32F(_) => Format::R32G32B32A32FLOAT,
            image => return Err(Error::UnsupportedImageFormat(image)),
        };
        let (width, height) = image.dimensions();
        let pixels = image.into_bytes();
        Ok(Data {
            format,
            width,
            height,
            pixels,
        })
    }

    /// Attemps a conversion from to the requested pixel format.
    pub fn convert_format(self, target_format: Format) -> Result<Self> {
        use image_crate::{GrayAlphaImage, GrayImage, RgbImage, RgbaImage};

        if target_format == self.format {
            return Ok(self);
        }

        // This should never happen
        let limit_error = Error::Image(image_crate::ImageError::Limits(
            image_crate::error::LimitError::from_kind(
                image_crate::error::LimitErrorKind::DimensionError,
            ),
        ));
        // Temporary fix for unsupported formats
        let unsupported_error = Error::Image(image_crate::ImageError::Parameter(
            image_crate::error::ParameterError::from_kind(
                image_crate::error::ParameterErrorKind::Generic(
                    "This pixel format is not supported yet".to_owned(),
                ),
            ),
        ));
        let mut image = match self.format {
            Format::R8 => DynamicImage::ImageLuma8(
                GrayImage::from_vec(self.width, self.height, self.pixels).ok_or(limit_error)?,
            ),
            Format::R8G8 => DynamicImage::ImageLumaA8(
                GrayAlphaImage::from_vec(self.width, self.height, self.pixels)
                    .ok_or(limit_error)?,
            ),
            Format::R8G8B8 => DynamicImage::ImageRgb8(
                RgbImage::from_vec(self.width, self.height, self.pixels).ok_or(limit_error)?,
            ),
            Format::R8G8B8A8 => DynamicImage::ImageRgba8(
                RgbaImage::from_vec(self.width, self.height, self.pixels).ok_or(limit_error)?,
            ),
            Format::R16 => Err(unsupported_error)?,
            Format::R16G16 => Err(unsupported_error)?,
            Format::R16G16B16 => Err(unsupported_error)?,
            Format::R16G16B16A16 => Err(unsupported_error)?,
            Format::R32G32B32FLOAT => Err(unsupported_error)?,
            Format::R32G32B32A32FLOAT => Err(unsupported_error)?,
        };

        image = match target_format {
            Format::R8 => DynamicImage::ImageLuma8(image.into_luma8()),
            Format::R8G8 => DynamicImage::ImageLumaA8(image.into_luma_alpha8()),
            Format::R8G8B8 => DynamicImage::ImageRgb8(image.into_rgb8()),
            Format::R8G8B8A8 => DynamicImage::ImageRgba8(image.into_rgba8()),
            Format::R16 => DynamicImage::ImageLuma16(image.into_luma16()),
            Format::R16G16 => DynamicImage::ImageLumaA16(image.into_luma_alpha16()),
            Format::R16G16B16 => DynamicImage::ImageRgb16(image.into_rgb16()),
            Format::R16G16B16A16 => DynamicImage::ImageRgba16(image.into_rgba16()),
            Format::R32G32B32FLOAT => DynamicImage::ImageRgb32F(image.into_rgb32f()),
            Format::R32G32B32A32FLOAT => DynamicImage::ImageRgba32F(image.into_rgba32f()),
        };

        Self::new(image)
    }
}
