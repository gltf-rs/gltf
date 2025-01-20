#![deny(missing_docs)]
#![allow(unknown_lints)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! glTF 2.0 loader
//!
//! This crate is intended to load [glTF 2.0], a file format designed for the
//! efficient runtime transmission of 3D scenes. The crate aims to provide
//! rustic utilities that make working with glTF simple and intuitive.
//!
//! [glTF 2.0]: https://www.khronos.org/gltf

use std::{fs, io, ops, result};

/// Typed views into binary buffer data.
pub mod accessor;

/// Animation tracks and samplers.
pub mod animation;

/// Asset metadata.
pub mod asset;

/// Binary glTF.
pub mod binary;

/// Binary buffer data declarations.
pub mod buffer;

/// Scene viewpoints.
pub mod camera;

/// Binary image data declarations.
pub mod image;

/// Reference importer implementation.
#[cfg(feature = "import")]
pub mod import;

/// Material properties for rendering primitives.
pub mod material;

/// Renderable geometry.
pub mod mesh;

/// JSON paths.
pub mod path;

/// The root glTF data structure.
pub mod root;

/// Scene graph structure.
pub mod scene;

/// Skeletal animations.
pub mod skin;

/// Image sampling techniques.
pub mod texture;

/// Validation implementation details.
pub mod validation;

/// Wrapper implementation details.
pub mod wrapper;

#[doc(inline)]
pub use accessor::Accessor;
#[doc(inline)]
pub use animation::Animation;
#[doc(inline)]
pub use asset::Asset;
#[doc(inline)]
pub use buffer::Buffer;
#[doc(inline)]
pub use camera::Camera;
#[doc(inline)]
pub use image::Image;
#[doc(inline)]
#[cfg(feature = "import")]
pub use import::{import, import_slice};
#[doc(inline)]
pub use material::Material;
#[doc(inline)]
pub use mesh::Mesh;
#[doc(inline)]
pub use scene::Node;
#[doc(inline)]
pub use scene::Scene;
#[doc(inline)]
pub use skin::Skin;
#[doc(inline)]
pub use texture::Texture;

#[doc(inline)]
pub use self::path::Path;
#[doc(inline)]
pub use self::root::Index;
#[doc(inline)]
pub use self::root::Root;

#[doc(inline)]
pub use serde_json::Value;

pub(crate) use wrapper::Wrap;

/// Untyped extension object.
pub type UnrecognizedExtensions = serde_json::Map<String, serde_json::Value>;

/// Data type of the `extras` attribute on all glTF objects.
pub type Extras = std::boxed::Box<serde_json::value::RawValue>;

/// Provides a type with a well-defined but not necessarily valid default value.
///
/// If a type implements `Default` then its stub value will simply be its default value.
pub trait Stub {
    /// Stub value for this type.
    fn stub() -> Self;
}

impl<T: Default> Stub for T {
    fn stub() -> Self {
        T::default()
    }
}

impl<T> Stub for Index<T> {
    fn stub() -> Self {
        Index::new(u32::MAX)
    }
}

macro_rules! trivial_impl_wrap {
    ($($ty:ty),*) => {
        $(
            impl<'a> crate::Wrap<'a> for $ty {
                type Wrapped = $ty;
                fn wrap(&'a self, _root: &'a crate::Root) -> Self::Wrapped {
                    *self
                }
            }
        )*
    };
}

trivial_impl_wrap!(i8, i16, i32, i64, isize, f32, f64, u8, u16, u32, u64, usize);

/// Result type for convenience.
pub type Result<T> = result::Result<T, Error>;

/// Represents a runtime error.
#[derive(Debug)]
pub enum Error {
    /// Base 64 decoding error.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    Base64(base64::DecodeError),

    /// GLB parsing error.
    Binary(binary::Error),

    /// Buffer length does not match expected length.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    BufferLength {
        /// The index of the offending buffer.
        buffer: usize,

        /// The expected buffer length in bytes.
        expected: usize,

        /// The number of bytes actually available.
        actual: usize,
    },

    /// JSON deserialization error.
    Deserialize(serde_json::Error),

    /// Standard I/O error.
    Io(std::io::Error),

    /// Image decoding error.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    Image(image_crate::ImageError),

    /// The `BIN` chunk of binary glTF is referenced but does not exist.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    MissingBlob,

    /// An external file is referenced in a slice only import without path
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    ExternalReferenceInSliceImport,

    /// Unsupported image encoding.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    UnsupportedImageEncoding,

    /// Unsupported image format.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    UnsupportedImageFormat(image_crate::DynamicImage),

    /// Unsupported URI scheme.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    UnsupportedScheme,

    /// glTF validation error.
    Validation(Vec<(Path, validation::Error)>),
}

fn validate(root: &Root) -> Result<()> {
    let mut errors = Vec::new();
    validation::Validate::validate(root, root, Path::new, &mut |path, error| {
        errors.push((path(), error))
    });
    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::Validation(errors))
    }
}

/// glTF JSON plus binary payload.
#[derive(Clone, Debug)]
pub struct Gltf {
    /// The glTF JSON wrapper.
    pub root: Root,

    /// The glTF binary payload in the case of binary glTF.
    pub blob: Option<Vec<u8>>,
}

impl Gltf {
    /// Convenience function that loads glTF from the file system.
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let gltf = Self::from_reader(reader)?;
        Ok(gltf)
    }

    /// Loads glTF from a reader without performing validation checks.
    pub fn from_reader_without_validation<R>(mut reader: R) -> Result<Self>
    where
        R: io::Read + io::Seek,
    {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        reader.seek(io::SeekFrom::Current(-4))?;
        let (root, blob): (Root, Option<Vec<u8>>);
        if magic.starts_with(b"glTF") {
            let mut glb = binary::Glb::from_reader(reader)?;
            // TODO: use `json::from_reader` instead of `json::from_slice`
            root = serde_json::from_slice(&glb.json)?;
            blob = glb.bin.take().map(|x| x.into_owned());
        } else {
            root = serde_json::from_reader(reader)?;
            blob = None;
        };
        Ok(Gltf { root, blob })
    }

    /// Loads glTF from a reader.
    pub fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: io::Read + io::Seek,
    {
        let gltf = Self::from_reader_without_validation(reader)?;
        validate(&gltf.root)?;
        Ok(gltf)
    }

    /// Loads glTF from a slice of bytes without performing validation
    /// checks.
    pub fn from_slice_without_validation(slice: &[u8]) -> Result<Self> {
        let (root, blob): (Root, Option<Vec<u8>>);
        if slice.starts_with(b"glTF") {
            let mut glb = binary::Glb::from_slice(slice)?;
            root = serde_json::from_slice(&glb.json)?;
            blob = glb.bin.take().map(|x| x.into_owned());
        } else {
            root = serde_json::from_slice(slice)?;
            blob = None;
        };
        Ok(Gltf { root, blob })
    }

    /// Loads glTF from a slice of bytes.
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        let gltf = Self::from_slice_without_validation(slice)?;
        validate(&gltf.root)?;
        Ok(gltf)
    }
}

impl ops::Deref for Gltf {
    type Target = Root;
    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl ops::DerefMut for Gltf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.root
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            #[cfg(feature = "import")]
            Error::Base64(ref e) => e.fmt(f),
            Error::Binary(ref e) => e.fmt(f),
            #[cfg(feature = "import")]
            Error::BufferLength {
                buffer,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "buffer {}: expected {} bytes but received {} bytes",
                    buffer, expected, actual
                )
            }
            Error::Deserialize(ref e) => e.fmt(f),
            Error::Io(ref e) => e.fmt(f),
            #[cfg(feature = "import")]
            Error::Image(ref e) => e.fmt(f),
            #[cfg(feature = "import")]
            Error::MissingBlob => write!(f, "missing binary portion of binary glTF"),
            #[cfg(feature = "import")]
            Error::ExternalReferenceInSliceImport => {
                write!(f, "external reference in slice only import")
            }
            #[cfg(feature = "import")]
            Error::UnsupportedImageEncoding => write!(f, "unsupported image encoding"),
            #[cfg(feature = "import")]
            Error::UnsupportedImageFormat(image) => {
                write!(f, "unsupported image format: {:?}", image.color())
            }
            #[cfg(feature = "import")]
            Error::UnsupportedScheme => write!(f, "unsupported URI scheme"),
            Error::Validation(ref xs) => {
                write!(f, "invalid glTF:")?;
                for (ref path, ref error) in xs {
                    write!(f, " {}: {};", path, error)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<binary::Error> for Error {
    fn from(err: binary::Error) -> Self {
        Error::Binary(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

#[cfg(feature = "import")]
impl From<image_crate::ImageError> for Error {
    fn from(err: image_crate::ImageError) -> Self {
        Error::Image(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Deserialize(err)
    }
}

impl From<Vec<(Path, validation::Error)>> for Error {
    fn from(errs: Vec<(Path, validation::Error)>) -> Self {
        Error::Validation(errs)
    }
}

/// Names of glTF 2.0 extensions supported by the library.
pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "KHR_lights_punctual",
    "KHR_materials_pbrSpecularGlossiness",
    "KHR_materials_unlit",
    "KHR_texture_transform",
    "KHR_materials_transmission",
    "KHR_materials_ior",
    "KHR_materials_emissive_strength",
];
