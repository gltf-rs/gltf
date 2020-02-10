use base64;
use crate::buffer;
use crate::image;
use std::{fs, io};

use image_crate::ImageFormat::{JPEG as Jpeg, PNG as Png};
use std::path::Path;
use crate::{Document, Error, Gltf, Result};

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
    Relative,

    /// Placeholder for an unsupported URI scheme identifier.
    Unsupported,
}

impl<'a> Scheme<'a> {
    fn parse<'s>(uri: &'s str) -> Scheme<'s> {
        if uri.contains(":") {
            if uri.starts_with("data:") {
                let match0 = &uri["data:".len()..].split(";base64,").nth(0);
                let match1 = &uri["data:".len()..].split(";base64,").nth(1);
                if match1.is_some() {
                    Scheme::Data(Some(match0.unwrap()), match1.unwrap())
                } else if match0.is_some() {
                    Scheme::Data(None, match0.unwrap())
                } else {
                    Scheme::Unsupported
                }
            } else if uri.starts_with("file://") {
                Scheme::File(&uri["file://".len()..])
            } else if uri.starts_with("file:") {
                Scheme::File(&uri["file:".len()..])
            } else {
                Scheme::Unsupported
            }
        } else {
            Scheme::Relative
        }
    }

    fn read(base: &Path, uri: &str) -> Result<Vec<u8>> {
        match Scheme::parse(uri) {
            Scheme::Data(_, base64) => base64::decode(&base64).map_err(Error::Base64),
            Scheme::File(path) => read_to_end(path),
            Scheme::Relative => read_to_end(base.join(uri)),
            Scheme::Unsupported => Err(Error::UnsupportedScheme),
        }
    }
}

fn read_to_end<P>(path: P) -> Result<Vec<u8>>
where P: AsRef<Path>
{
    use io::Read;
    let file = fs::File::open(path.as_ref()).map_err(Error::Io)?;
    let length = file.metadata().map(|x| x.len()).unwrap_or(0);
    let mut reader = io::BufReader::new(file);
    let mut data = Vec::with_capacity(length as usize);
    reader.read_to_end(&mut data).map_err(Error::Io)?;
    Ok(data)
}

/// Import the buffer data referenced by a glTF document.
pub fn import_buffer_data(
    document: &Document,
    base: Option<&Path>,
    mut blob: Option<Vec<u8>>,
) -> Result<Vec<buffer::Data>> {
    let mut buffers = Vec::new();
    for buffer in document.buffers() {
        let mut data = match buffer.source() {
            buffer::Source::Uri(uri) if base.is_some() => Scheme::read(base.unwrap(), uri),
            buffer::Source::Bin => blob.take().ok_or(Error::MissingBlob),
            _ => Err(Error::ExternalReferenceInSliceImport)
        }?;
        if data.len() < buffer.length() {
            return Err(
                Error::BufferLength {
                    buffer: buffer.index(),
                    expected: buffer.length(),
                    actual: data.len(),
                }
            );
        }
        while data.len() % 4 != 0 {
            data.push(0);
        }
        buffers.push(buffer::Data(data));
    }
    Ok(buffers)
}

/// Import the image data referenced by a glTF document.
pub fn import_image_data(
    document: &Document,
    base: Option<&Path>,
    buffer_data: &[buffer::Data],
) -> Result<Vec<image::Data>> {
    let mut images = Vec::new();
    #[cfg(feature = "guess_mime_type")]
    let guess_format = |encoded_image: &[u8]| {
        match image_crate::guess_format(encoded_image) {
            Ok(image_crate::ImageFormat::PNG) => Some(Png),
            Ok(image_crate::ImageFormat::JPEG) => Some(Jpeg),
            _ => None,
        }
    };
    #[cfg(not(feature = "guess_mime_type"))]
    let guess_format = |_encoded_image: &[u8]| {
        None
    };
    for image in document.images() {
        match image.source() {
            image::Source::Uri { uri, mime_type } if base.is_some() => {
                match Scheme::parse(uri) {
                    Scheme::Data(Some(annoying_case), base64) => {
                        let encoded_image = base64::decode(&base64).map_err(Error::Base64)?;
                        let encoded_format = match annoying_case.as_ref() {
                            "image/png" => Png,
                            "image/jpeg" => Jpeg,
                            _ => match guess_format(&encoded_image) {
                                Some(format) => format,
                                None => return Err(Error::UnsupportedImageEncoding),
                            },
                        };
                        let decoded_image = image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?;
                        images.push(image::Data::new(decoded_image));
                        continue;
                    },
                    Scheme::Unsupported => return Err(Error::UnsupportedScheme),
                    _ => {},
                }
                let encoded_image = Scheme::read(base.unwrap(), uri)?;
                let encoded_format =  match mime_type {
                    Some("image/png") => Png,
                    Some("image/jpeg") => Jpeg,
                    Some(_) => match guess_format(&encoded_image) {
                        Some(format) => format,
                        None => return Err(Error::UnsupportedImageEncoding),
                    },
                    None => match uri.rsplit(".").next() {
                        Some("png") => Png,
                        Some("jpg") | Some("jpeg") => Jpeg,
                        _ => match guess_format(&encoded_image) {
                            Some(format) => format,
                            None => return Err(Error::UnsupportedImageEncoding),
                        },
                    },
                };
                let decoded_image = image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?;
                images.push(image::Data::new(decoded_image));
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
                let decoded_image = image_crate::load_from_memory_with_format(encoded_image, encoded_format)?;
                images.push(image::Data::new(decoded_image));
            },
            _ => return Err(Error::ExternalReferenceInSliceImport)
        }
    }

    Ok(images)
}

fn import_impl(Gltf { document, blob }: Gltf, base: Option<&Path>) -> Result<Import> {
    let buffer_data = import_buffer_data(&document, base, blob)?;
    let image_data = import_image_data(&document, base, &buffer_data)?;
    let import = (document, buffer_data, image_data);
    Ok(import)
}

fn import_path(path: &Path) -> Result<Import> {
    let base = path.parent().unwrap_or(Path::new("./"));
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
    where P: AsRef<Path>
{
    import_path(path.as_ref())
}

pub fn import_slice_impl(slice: &[u8]) -> Result<Import> {
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
/// let (document, buffers, images) = gltf::import_slice(bytes.as_slice())?;
/// # Ok(())
/// # }
/// # fn main() {
/// #     run().expect("test failure");
/// # }
/// ```
pub fn import_slice<S>(slice: S) -> Result<Import>
    where S: AsRef<[u8]>
{
    import_slice_impl(slice.as_ref())
}
