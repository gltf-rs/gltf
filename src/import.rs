use crate::buffer;
use crate::image;
use std::borrow::Cow;
use std::{fs, io};

use crate::{Document, Error, Gltf, Result};
#[cfg(feature = "EXT_texture_webp")]
use image_crate::ImageFormat::WebP;
use image_crate::ImageFormat::{Jpeg, Png};
use std::path::Path;

/// Return type of `import`.
type Import = (Document, Vec<buffer::Data>, Vec<image::Data>);

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
            Scheme::File(path) if base.is_some() => read_to_end(path),
            Scheme::Relative(path) if base.is_some() => read_to_end(base.unwrap().join(&*path)),
            Scheme::Unsupported => Err(Error::UnsupportedScheme),
            _ => Err(Error::ExternalReferenceInSliceImport),
        }
    }
}

fn read_to_end<P>(path: P) -> Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    use io::Read;
    let file = fs::File::open(path.as_ref()).map_err(Error::Io)?;
    // Allocate one extra byte so the buffer doesn't need to grow before the
    // final `read` call at the end of the file.  Don't worry about `usize`
    // overflow because reading will fail regardless in that case.
    let length = file.metadata().map(|x| x.len() + 1).unwrap_or(0);
    let mut reader = io::BufReader::new(file);
    let mut data = Vec::with_capacity(length as usize);
    reader.read_to_end(&mut data).map_err(Error::Io)?;
    Ok(data)
}

impl buffer::Data {
    /// Construct a buffer data object by reading the given source.
    /// If `base` is provided, then external filesystem references will
    /// be resolved from this directory.
    pub fn from_source(source: buffer::Source<'_>, base: Option<&Path>) -> Result<Self> {
        Self::from_source_and_blob(source, base, &mut None)
    }

    /// Construct a buffer data object by reading the given source.
    /// If `base` is provided, then external filesystem references will
    /// be resolved from this directory.
    /// `blob` represents the `BIN` section of a binary glTF file,
    /// and it will be taken to fill the buffer if the `source` refers to it.
    pub fn from_source_and_blob(
        source: buffer::Source<'_>,
        base: Option<&Path>,
        blob: &mut Option<Vec<u8>>,
    ) -> Result<Self> {
        let mut data = match source {
            buffer::Source::Uri(uri) => Scheme::read(base, uri),
            buffer::Source::Bin => blob.take().ok_or(Error::MissingBlob),
        }?;
        while data.len() % 4 != 0 {
            data.push(0);
        }
        Ok(buffer::Data(data))
    }
}

/// Import buffer data referenced by a glTF document.
///
/// ### Note
///
/// This function is intended for advanced users who wish to forego loading image data.
/// A typical user should call [`import`] instead.
pub fn import_buffers(
    document: &Document,
    base: Option<&Path>,
    mut blob: Option<Vec<u8>>,
) -> Result<Vec<buffer::Data>> {
    let mut buffers = Vec::new();
    for buffer in document.buffers() {
        let data = buffer::Data::from_source_and_blob(buffer.source(), base, &mut blob)?;
        if data.len() < buffer.length() {
            return Err(Error::BufferLength {
                buffer: buffer.index(),
                expected: buffer.length(),
                actual: data.len(),
            });
        }
        buffers.push(data);
    }
    Ok(buffers)
}

impl image::Data {
    /// Construct an image data object by reading the given source.
    /// If `base` is provided, then external filesystem references will
    /// be resolved from this directory.
    pub fn from_source(
        source: image::Source<'_>,
        base: Option<&Path>,
        buffer_data: &[buffer::Data],
    ) -> Result<Self> {
        #[cfg(feature = "guess_mime_type")]
        let guess_format = |encoded_image: &[u8]| match image_crate::guess_format(encoded_image) {
            Ok(image_crate::ImageFormat::Png) => Some(Png),
            Ok(image_crate::ImageFormat::Jpeg) => Some(Jpeg),
            #[cfg(feature = "EXT_texture_webp")]
            Ok(image_crate::ImageFormat::WebP) => Some(WebP),
            _ => None,
        };
        #[cfg(not(feature = "guess_mime_type"))]
        let guess_format = |_encoded_image: &[u8]| None;
        let decoded_image = match source {
            image::Source::Uri { uri, mime_type } if base.is_some() => match Scheme::parse(uri) {
                Scheme::Data(Some(annoying_case), base64) => {
                    let encoded_image = base64::decode(base64).map_err(Error::Base64)?;
                    let encoded_format = match annoying_case {
                        "image/png" => Png,
                        "image/jpeg" => Jpeg,
                        #[cfg(feature = "EXT_texture_webp")]
                        "image/webp" => WebP,
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
                        #[cfg(feature = "EXT_texture_webp")]
                        Some("image/webp") => WebP,
                        Some(_) => match guess_format(&encoded_image) {
                            Some(format) => format,
                            None => return Err(Error::UnsupportedImageEncoding),
                        },
                        None => match uri.rsplit('.').next() {
                            Some("png") => Png,
                            Some("jpg") | Some("jpeg") => Jpeg,
                            #[cfg(feature = "EXT_texture_webp")]
                            Some("webp") => WebP,
                            _ => match guess_format(&encoded_image) {
                                Some(format) => format,
                                None => return Err(Error::UnsupportedImageEncoding),
                            },
                        },
                    };
                    image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?
                }
            },
            image::Source::View { view, mime_type } => {
                let parent_buffer_data = &buffer_data[view.buffer().index()].0;
                let begin = view.offset();
                let end = begin + view.length();
                let encoded_image = &parent_buffer_data[begin..end];
                let encoded_format = match mime_type {
                    "image/png" => Png,
                    "image/jpeg" => Jpeg,
                    #[cfg(feature = "EXT_texture_webp")]
                    "image/webp" => WebP,
                    _ => match guess_format(encoded_image) {
                        Some(format) => format,
                        None => return Err(Error::UnsupportedImageEncoding),
                    },
                };
                image_crate::load_from_memory_with_format(encoded_image, encoded_format)?
            }
            _ => return Err(Error::ExternalReferenceInSliceImport),
        };

        image::Data::new(decoded_image)
    }
}

/// Import image data referenced by a glTF document.
///
/// ### Note
///
/// This function is intended for advanced users who wish to forego loading buffer data.
/// A typical user should call [`import`] instead.
pub fn import_images(
    document: &Document,
    base: Option<&Path>,
    buffer_data: &[buffer::Data],
) -> Result<Vec<image::Data>> {
    let mut images = Vec::new();
    for image in document.images() {
        images.push(image::Data::from_source(image.source(), base, buffer_data)?);
    }
    Ok(images)
}

fn import_impl(Gltf { document, blob }: Gltf, base: Option<&Path>) -> Result<Import> {
    let buffer_data = import_buffers(&document, base, blob)?;
    let image_data = import_images(&document, base, &buffer_data)?;
    let import = (document, buffer_data, image_data);
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
/// # let path = "examples/Box.gltf";
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
/// # let path = "examples/Box.glb";
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
