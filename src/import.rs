use base64;
use buffer;
use image;
use image_crate;
use std::{fs, io};

use image_crate::ImageFormat::{JPEG as Jpeg, PNG as Png};
use std::path::Path;
use {Document, Error, Gltf, Result};

/// Loader abstraction, to faciliate loading from any number of sources (filesystem, network etc).
pub trait Loader {
    /// Load the asset with the given name from the source
    fn load(&self, name: &str) -> Result<Vec<u8>>;
}

impl<'a> Loader for &'a Path {
    fn load(&self, name: &str) -> Result<Vec<u8>> {
        use std::io::Read;
        use std::path::PathBuf;
        let path = if name.starts_with("/") {
            PathBuf::from(name)
        } else {
            self.join(name)
        };
        let file = fs::File::open(path).map_err(Error::Io)?;
        let length = file.metadata().map(|x| x.len());
        let mut reader = io::BufReader::new(file);
        let mut v = length
            .map(|l| Vec::with_capacity(l as usize))
            .unwrap_or_else(|_| Vec::new());
        reader.read_to_end(&mut v)?;
        Ok(v)
    }
}

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

    fn read<L>(loader: &L, uri: &str) -> Result<Vec<u8>>
    where
        L: Loader,
    {
        match Scheme::parse(uri) {
            Scheme::Data(_, base64) => base64::decode(&base64).map_err(Error::Base64),
            Scheme::File(path) => read_to_end(loader, path),
            Scheme::Relative => read_to_end(loader, uri),
            Scheme::Unsupported => Err(Error::UnsupportedScheme),
        }
    }
}

fn read_to_end<L>(loader: &L, name: &str) -> Result<Vec<u8>>
where
    L: Loader,
{
    loader.load(name)
}

/// Import the buffer data referenced by a glTF document.
pub fn import_buffer_data<L>(
    document: &Document,
    loader: &L,
    mut blob: Option<Vec<u8>>,
) -> Result<Vec<buffer::Data>>
where
    L: Loader,
{
    let mut buffers = Vec::new();
    for buffer in document.buffers() {
        let mut data = match buffer.source() {
            buffer::Source::Uri(uri) => Scheme::read(loader, uri),
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
pub fn import_image_data<L>(
    document: &Document,
    loader: &L,
    buffer_data: &[buffer::Data],
) -> Result<Vec<image::Data>>
where
    L: Loader,
{
    let mut images = Vec::new();
    for image in document.images() {
        match image.source() {
            image::Source::Uri { uri, mime_type } => {
                match Scheme::parse(uri) {
                    Scheme::Data(Some(annoying_case), base64) => {
                        let format = match annoying_case.as_ref() {
                            "image/png" => Png,
                            "image/jpeg" => Jpeg,
                            _ => return Err(Error::UnsupportedImageEncoding),
                        };
                        let encoded_image = base64::decode(&base64).map_err(Error::Base64)?;
                        let decoded_image =
                            image_crate::load_from_memory_with_format(&encoded_image, format)?;
                        images.push(image::Data::new(decoded_image));
                        continue;
                    }
                    Scheme::Unsupported => return Err(Error::UnsupportedScheme),
                    _ => {}
                }
                let encoded_image = Scheme::read(loader, uri)?;
                let encoded_format = match mime_type {
                    Some("image/png") => Png,
                    Some("image/jpeg") => Jpeg,
                    Some(_) => return Err(Error::UnsupportedImageEncoding),
                    None => match uri.rsplit(".").next() {
                        Some("png") => Png,
                        Some("jpg") | Some("jpeg") => Jpeg,
                        _ => return Err(Error::UnsupportedImageEncoding),
                    },
                };
                let decoded_image =
                    image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?;
                images.push(image::Data::new(decoded_image));
            }
            image::Source::View { view, mime_type } => {
                let parent_buffer_data = &buffer_data[view.buffer().index()].0;
                let begin = view.offset();
                let end = begin + view.length();
                let encoded_image = &parent_buffer_data[begin..end];
                let encoded_format = match mime_type {
                    "image/png" => Png,
                    "image/jpeg" => Jpeg,
                    _ => return Err(Error::UnsupportedImageEncoding),
                };
                let decoded_image =
                    image_crate::load_from_memory_with_format(encoded_image, encoded_format)?;
                images.push(image::Data::new(decoded_image));
            }
        }
    }

    Ok(images)
}

/// Import some glTF 2.0 from the file system.
///
/// ```
/// # extern crate gltf;
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
pub fn import<P>(path: P) -> Result<Import>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let loader = path.parent().unwrap_or(Path::new("./"));
    let file = path.file_name().and_then(|f| f.to_str()).unwrap(); // FIXME: error
    load_from(&loader, file)
}

/// Import some glTF 2.0 from a `Loader`.
///
/// ```
/// # extern crate gltf;
/// # fn run() -> Result<(), gltf::Error> {
/// # let loader = "examples/".as_ref();
/// # let name = "Box.gltf";
/// # #[allow(unused)]
/// let (document, buffers, images) = gltf::load(loader, name)?;
/// # Ok(())
/// # }
/// # fn main() {
/// #     run().expect("test failure");
/// # }
/// ```
pub fn load_from<L>(loader: &L, name: &str) -> Result<Import>
where
    L: Loader,
{
    let data = loader.load(name)?;
    let Gltf { document, blob } = Gltf::from_slice(&data)?;
    let buffer_data = import_buffer_data(&document, loader, blob)?;
    let image_data = import_image_data(&document, loader, &buffer_data)?;
    let import = (document, buffer_data, image_data);
    Ok(import)
}
