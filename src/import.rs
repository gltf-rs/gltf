use crate::buffer;
use crate::image;
use std::borrow::Cow;
use std::{fs, io};

use crate::{Document, Error, Gltf, Result};
use image_crate::ImageFormat::{Jpeg, Png};
use std::path::Path;

/// Return type of `import`.
type Import<E> = (Document<E>, Vec<buffer::Data>, Vec<image::Data>);

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
            Scheme::Data(_, base64) => base64::decode(&base64).map_err(Error::Base64),
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

/// Import the buffer data referenced by a glTF document.
pub fn import_buffer_data<E: json::ThirdPartyExtensions>(
    document: &Document<E>,
    base: Option<&Path>,
    mut blob: Option<Vec<u8>>,
) -> Result<Vec<buffer::Data>> {
    let mut buffers = Vec::new();
    for buffer in document.buffers() {
        let mut data = match buffer.source() {
            buffer::Source::Uri(uri) => Scheme::read(base, uri),
            buffer::Source::Bin => blob.take().ok_or(Error::MissingBlob),
        }?;
        if data.len() < buffer.length() {
            return Err(Error::BufferLength {
                buffer: buffer.index(),
                expected: buffer.length(),
                actual: data.len(),
            });
        }
        while data.len() % 4 != 0 {
            data.push(0);
        }
        buffers.push(buffer::Data(data));
    }
    Ok(buffers)
}

/// Import the image data referenced by a glTF document.
pub fn import_image_data<E: json::ThirdPartyExtensions>(
    document: &Document<E>,
    base: Option<&Path>,
    buffer_data: &[buffer::Data],
) -> Result<Vec<image::Data>> {
    let mut images = Vec::new();
    #[cfg(feature = "guess_mime_type")]
    let guess_format = |encoded_image: &[u8]| match image_crate::guess_format(encoded_image) {
        Ok(image_crate::ImageFormat::Png) => Some(Png),
        Ok(image_crate::ImageFormat::Jpeg) => Some(Jpeg),
        _ => None,
    };
    #[cfg(not(feature = "guess_mime_type"))]
    let guess_format = |_encoded_image: &[u8]| None;
    for image in document.images() {
        let decoded_image = match image.source() {
            image::Source::Uri { uri, mime_type } if base.is_some() => match Scheme::parse(uri) {
                Scheme::Data(Some(annoying_case), base64) => {
                    let encoded_image = base64::decode(&base64).map_err(Error::Base64)?;
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
            image::Source::View { view, mime_type } => {
                let parent_buffer_data = &buffer_data[view.buffer().index()].0;
                let begin = view.offset();
                let end = begin + view.length();
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

        images.push(image::Data::new(decoded_image)?);
    }

    Ok(images)
}

fn import_impl<E: json::ThirdPartyExtensions>(Gltf { document, blob }: Gltf<E>, base: Option<&Path>) -> Result<Import<E>> {
    let buffer_data = import_buffer_data(&document, base, blob)?;
    let image_data = import_image_data(&document, base, &buffer_data)?;
    let import = (document, buffer_data, image_data);
    Ok(import)
}

fn import_path<E: json::ThirdPartyExtensions>(path: &Path) -> Result<Import<E>> {
    let base = path.parent().unwrap_or_else(|| Path::new("./"));
    let file = fs::File::open(path).map_err(Error::Io)?;
    let reader = io::BufReader::new(file);
    import_impl(Gltf::from_reader(reader)?, Some(base))
}

/// Import some glTF 2.0 from the file system.
///
/// ```
/// # fn run() -> Result<(), gltf::Error> {
/// # let path = "examples/Box.gltf";
/// # #[allow(unused)]
/// let (document, buffers, images): (gltf::Document, _, _) = gltf::import(path)?;
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
pub fn import<P, E: json::ThirdPartyExtensions>(path: P) -> Result<Import<E>>
where
    P: AsRef<Path>,
{
    import_path(path.as_ref())
}

pub fn import_slice_impl<E: json::ThirdPartyExtensions>(slice: &[u8]) -> Result<Import<E>> {
    import_impl(Gltf::from_slice(slice)?, None)
}

/// Import some glTF 2.0 from a slice
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
/// let (document, buffers, images): (gltf::Document, _, _) = gltf::import_slice(bytes.as_slice())?;
/// # Ok(())
/// # }
/// # fn main() {
/// #     run().expect("test failure");
/// # }
/// ```
pub fn import_slice<S, E: json::ThirdPartyExtensions>(slice: S) -> Result<Import<E>>
where
    S: AsRef<[u8]>,
{
    import_slice_impl(slice.as_ref())
}
