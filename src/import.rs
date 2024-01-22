use crate::buffer;
use std::borrow::Cow;
use std::{fs, io};

use crate::{Error, Gltf, Result, Root};
use image_crate::DynamicImage;
use image_crate::ImageFormat::{Jpeg, Png};
use std::path::Path;

/// Buffer data belonging to an imported glTF asset.
#[derive(Clone)]
pub struct BufferData(pub Vec<u8>);

/// Describes a buffer data source.
#[derive(Clone, Debug)]
pub enum BufferSource<'a> {
    /// Buffer data is contained in the `BIN` section of binary glTF.
    Bin,

    /// Buffer data is contained in an external data source.
    Uri(&'a str),
}

/// Image pixel format.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PixelFormat {
    /// Red only.
    R = 1,

    /// Red, green.
    Rg = 2,

    /// Red, green, blue, alpha.
    Rgba = 4,
}

/// Image data buffer.
#[derive(Clone)]
pub enum PixelBuffer {
    /// `u8` component type.
    U8(Vec<u8>),
    /// `u16` component type.
    U16(Vec<u16>),
    /// `f32` component type.
    F32(Vec<f32>),
}

/// Image data belonging to an imported glTF asset.
#[derive(Clone)]
pub struct ImageData {
    /// Pixel data.
    pub pixels: PixelBuffer,

    /// The image pixel data format.
    pub format: PixelFormat,

    /// The image width in pixels.
    pub width: usize,

    /// The image height in pixels.
    pub height: usize,
}

impl ImageData {
    pub(crate) fn new(image: DynamicImage) -> Result<Self> {
        use image_crate::GenericImageView;
        let (width, height) = image.dimensions();
        let (pixels, format) = match image {
            DynamicImage::ImageLuma8(image) => (PixelBuffer::U8(image.into_vec()), PixelFormat::R),
            DynamicImage::ImageLumaA8(image) => {
                (PixelBuffer::U8(image.into_vec()), PixelFormat::Rg)
            }
            image @ DynamicImage::ImageRgb8(_) => (
                PixelBuffer::U8(image.to_rgba8().into_vec()),
                PixelFormat::Rgba,
            ),
            DynamicImage::ImageRgba8(image) => {
                (PixelBuffer::U8(image.into_vec()), PixelFormat::Rgba)
            }
            DynamicImage::ImageLuma16(image) => {
                (PixelBuffer::U16(image.into_vec()), PixelFormat::R)
            }
            DynamicImage::ImageLumaA16(image) => {
                (PixelBuffer::U16(image.into_vec()), PixelFormat::Rg)
            }
            image @ DynamicImage::ImageRgb16(_) => (
                PixelBuffer::U16(image.to_rgba16().into_vec()),
                PixelFormat::Rgba,
            ),
            DynamicImage::ImageRgba16(image) => {
                (PixelBuffer::U16(image.into_vec()), PixelFormat::Rgba)
            }
            image @ DynamicImage::ImageRgb32F(_) => (
                PixelBuffer::F32(image.to_rgba32f().into_vec()),
                PixelFormat::Rgba,
            ),
            DynamicImage::ImageRgba32F(image) => {
                (PixelBuffer::F32(image.into_vec()), PixelFormat::Rgba)
            }
            image => return Err(Error::UnsupportedImageFormat(image)),
        };
        Ok(Self {
            format,
            width: width as usize,
            height: height as usize,
            pixels,
        })
    }
}

/// Return type of `import`.
pub type Import = (Root, Vec<BufferData>, Vec<ImageData>);

/// Represents the set of URI schemes the importer supports.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Scheme<'a> {
    /// `data:[<media type>];base64,<data>`.
    Data(Option<&'a str>, &'a str),

    /// `file:[//]<absolute file path>`.
    ///
    /// Note: The file scheme does not implement authority.
    File(&'a str),

    /// `../foo`, etc.
    Relative(Cow<'a, str>),

    /// Placeholder for an unsupported URI scheme identifier.
    Unsupported,
}

/// Describes an image data source.
#[derive(Clone, Debug)]
pub enum ImageSource<'a> {
    /// Image data is contained in a buffer view.
    View {
        /// The buffer view containing the encoded image data.
        view: &'a buffer::View,

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

impl<'a> Scheme<'a> {
    fn parse(uri: &str) -> Scheme<'_> {
        if uri.contains(':') {
            if let Some(rest) = uri.strip_prefix("data:") {
                let mut it = rest.split(";base64,");

                match (it.next(), it.next()) {
                    (match0_opt, Some(match1)) => Scheme::Data(match0_opt, match1),
                    (Some(match0), _) => Scheme::Data(None, match0),
                    _ => Scheme::Unsupported,
                }
            } else if let Some(rest) = uri.strip_prefix("file://") {
                Scheme::File(rest)
            } else if let Some(rest) = uri.strip_prefix("file:") {
                Scheme::File(rest)
            } else {
                Scheme::Unsupported
            }
        } else {
            Scheme::Relative(urlencoding::decode(uri).unwrap())
        }
    }

    fn read(base: Option<&Path>, uri: &str) -> Result<Vec<u8>> {
        match Scheme::parse(uri) {
            // The path may be unused in the Scheme::Data case
            // Example: "uri" : "data:application/octet-stream;base64,wsVHPgA...."
            Scheme::Data(_, base64) => base64::decode(base64).map_err(Error::Base64),
            Scheme::File(path) if base.is_some() => std::fs::read(path).map_err(Error::Io),
            Scheme::Relative(path) if base.is_some() => {
                std::fs::read(base.unwrap().join(&*path)).map_err(Error::Io)
            }
            Scheme::Unsupported => Err(Error::UnsupportedScheme),
            _ => Err(Error::ExternalReferenceInSliceImport),
        }
    }
}

impl BufferData {
    /// Construct a buffer data object by reading the given source.
    /// If `base` is provided, then external filesystem references will
    /// be resolved from this directory.
    pub fn from_source(source: BufferSource<'_>, base: Option<&Path>) -> Result<Self> {
        Self::from_source_and_blob(source, base, &mut None)
    }

    /// Construct a buffer data object by reading the given source.
    /// If `base` is provided, then external filesystem references will
    /// be resolved from this directory.
    /// `blob` represents the `BIN` section of a binary glTF file,
    /// and it will be taken to fill the buffer if the `source` refers to it.
    pub fn from_source_and_blob(
        source: BufferSource<'_>,
        base: Option<&Path>,
        blob: &mut Option<Vec<u8>>,
    ) -> Result<Self> {
        let mut data = match source {
            BufferSource::Uri(uri) => Scheme::read(base, uri),
            BufferSource::Bin => blob.take().ok_or(Error::MissingBlob),
        }?;
        while data.len() % 4 != 0 {
            data.push(0);
        }
        Ok(Self(data))
    }
}

/// Import buffer data referenced by a glTF document.
///
/// ### Note
///
/// This function is intended for advanced users who wish to forego loading image data.
/// A typical user should call [`import`] instead.
pub fn import_buffers(
    root: &Root,
    base: Option<&Path>,
    mut blob: Option<Vec<u8>>,
) -> Result<Vec<BufferData>> {
    let mut buffers = Vec::new();
    for (index, buffer) in root.buffers.iter().enumerate() {
        let buffer_source = if let Some(uri) = buffer.uri.as_deref() {
            BufferSource::Uri(uri)
        } else {
            BufferSource::Bin
        };
        let data = BufferData::from_source_and_blob(buffer_source, base, &mut blob)?;
        if data.0.len() < buffer.length.value() {
            return Err(Error::BufferLength {
                buffer: index,
                expected: buffer.length.value(),
                actual: data.0.len(),
            });
        }
        buffers.push(data);
    }
    Ok(buffers)
}

impl ImageData {
    /// Construct an image data object by reading the given source.
    /// If `base` is provided, then external filesystem references will
    /// be resolved from this directory.
    pub fn from_source(
        source: ImageSource<'_>,
        base: Option<&Path>,
        buffer_data: &[BufferData],
    ) -> Result<Self> {
        #[cfg(feature = "guess_mime_type")]
        let guess_format = |encoded_image: &[u8]| match image_crate::guess_format(encoded_image) {
            Ok(image_crate::ImageFormat::Png) => Some(Png),
            Ok(image_crate::ImageFormat::Jpeg) => Some(Jpeg),
            _ => None,
        };
        #[cfg(not(feature = "guess_mime_type"))]
        let guess_format = |_encoded_image: &[u8]| None;
        let decoded_image = match source {
            ImageSource::Uri { uri, mime_type } if base.is_some() => match Scheme::parse(uri) {
                Scheme::Data(Some(annoying_case), base64) => {
                    let encoded_image = base64::decode(base64).map_err(Error::Base64)?;
                    let encoded_format = match annoying_case {
                        "image/png" => Png,
                        "image/jpeg" => Jpeg,
                        _ => match guess_format(&encoded_image) {
                            Some(format) => format,
                            None => return Err(Error::UnsupportedImageEncoding),
                        },
                    };

                    image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?
                }
                Scheme::Unsupported => return Err(Error::UnsupportedScheme),
                _ => {
                    let encoded_image = Scheme::read(base, uri)?;
                    let encoded_format = match mime_type {
                        Some("image/png") => Png,
                        Some("image/jpeg") => Jpeg,
                        Some(_) => match guess_format(&encoded_image) {
                            Some(format) => format,
                            None => return Err(Error::UnsupportedImageEncoding),
                        },
                        None => match uri.rsplit('.').next() {
                            Some("png") => Png,
                            Some("jpg") | Some("jpeg") => Jpeg,
                            _ => match guess_format(&encoded_image) {
                                Some(format) => format,
                                None => return Err(Error::UnsupportedImageEncoding),
                            },
                        },
                    };
                    image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?
                }
            },
            ImageSource::View { view, mime_type } => {
                let parent_buffer_data = &buffer_data[view.buffer.value()].0;
                let begin = view.offset.value();
                let end = begin + view.length.value();
                let encoded_image = &parent_buffer_data[begin..end];
                let encoded_format = match mime_type {
                    "image/png" => Png,
                    "image/jpeg" => Jpeg,
                    _ => match guess_format(encoded_image) {
                        Some(format) => format,
                        None => return Err(Error::UnsupportedImageEncoding),
                    },
                };
                image_crate::load_from_memory_with_format(encoded_image, encoded_format)?
            }
            _ => return Err(Error::ExternalReferenceInSliceImport),
        };

        ImageData::new(decoded_image)
    }
}

/// Import image data referenced by a glTF document.
///
/// ### Note
///
/// This function is intended for advanced users who wish to forego loading buffer data.
/// A typical user should call [`import`] instead.
pub fn import_images(
    root: &Root,
    base: Option<&Path>,
    buffer_data: &[BufferData],
) -> Result<Vec<ImageData>> {
    let mut images = Vec::new();
    for image in &root.images {
        let image_source = if let Some(index) = image.buffer_view.as_ref() {
            let view = &root.buffer_views[index.value()];
            let mime_type = image.mime_type.as_ref().map(|x| x.0.as_str()).unwrap();
            ImageSource::View { view, mime_type }
        } else {
            let uri = image.uri.as_ref().unwrap();
            let mime_type = image.mime_type.as_ref().map(|x| x.0.as_str());
            ImageSource::Uri { uri, mime_type }
        };
        images.push(ImageData::from_source(image_source, base, buffer_data)?);
    }
    Ok(images)
}

fn import_impl(Gltf { root, blob }: Gltf, base: Option<&Path>) -> Result<Import> {
    let buffer_data = import_buffers(&root, base, blob)?;
    let image_data = import_images(&root, base, &buffer_data)?;
    let import = (root, buffer_data, image_data);
    Ok(import)
}

fn import_path(path: &Path) -> Result<Import> {
    let base = path.parent().unwrap_or_else(|| Path::new("./"));
    let file = fs::File::open(path).map_err(Error::Io)?;
    let reader = io::BufReader::new(file);
    import_impl(Gltf::from_reader(reader)?, Some(base))
}

/// Import glTF 2.0 from the file system.
///
/// ```
/// # fn run() -> Result<(), gltf::Error> {
/// # let path = "glTF-Sample-Assets/Models/Box/glTF/Box.gltf";
/// # #[allow(unused)]
/// let (document, buffers, images) = gltf::import(path)?;
/// # Ok(())
/// # }
/// # fn main() {
/// #     run().expect("test failure");
/// # }
/// ```
///
/// ### Note
///
/// This function is provided as a convenience for loading glTF and associated
/// resources from the file system. It is suitable for real world use but may
/// not be suitable for all real world use cases. More complex import scenarios
/// such downloading from web URLs are not handled by this function. These
/// scenarios are delegated to the user.
///
/// You can read glTF without loading resources by constructing the [`Gltf`]
/// (standard glTF) or [`Glb`] (binary glTF) data structures explicitly.
///
/// [`Gltf`]: struct.Gltf.html
/// [`Glb`]: struct.Glb.html
pub fn import<P>(path: P) -> Result<Import>
where
    P: AsRef<Path>,
{
    import_path(path.as_ref())
}

fn import_slice_impl(slice: &[u8]) -> Result<Import> {
    import_impl(Gltf::from_slice(slice)?, None)
}

/// Import glTF 2.0 from a slice.
///
/// File paths in the document are assumed to be relative to the current working
/// directory.
///
/// ### Note
///
/// This function is intended for advanced users.
/// A typical user should call [`import`] instead.
///
/// ```
/// # extern crate gltf;
/// # use std::fs;
/// # use std::io::Read;
/// # fn run() -> Result<(), gltf::Error> {
/// # let path = "glTF-Sample-Assets/Models/Box/glTF-Binary/Box.glb";
/// # let mut file = fs::File::open(path).map_err(gltf::Error::Io)?;
/// # let mut bytes = Vec::new();
/// # file.read_to_end(&mut bytes).map_err(gltf::Error::Io)?;
/// # #[allow(unused)]
/// let (document, buffers, images) = gltf::import_slice(bytes.as_slice())?;
/// # Ok(())
/// # }
/// # fn main() {
/// #     run().expect("test failure");
/// # }
/// ```
pub fn import_slice<S>(slice: S) -> Result<Import>
where
    S: AsRef<[u8]>,
{
    import_slice_impl(slice.as_ref())
}
