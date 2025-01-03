#![deny(missing_docs)]
#![allow(unknown_lints)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! glTF 2.0 loader
//!
//! This crate is intended to load [glTF 2.0], a file format designed for the
//! efficient runtime transmission of 3D scenes. The crate aims to provide
//! rustic utilities that make working with glTF simple and intuitive.
//!
//! # Installation
//!
//! Add `gltf` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.gltf]
//! version = "1"
//! ```
//!
//! # Examples
//!
//! ## Basic usage
//!
//! Walking the node hierarchy.
//!
//! ```
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # use gltf::Gltf;
//! let gltf = Gltf::open("examples/Box.gltf")?;
//! for scene in gltf.scenes() {
//!     for node in scene.nodes() {
//!         println!(
//!             "Node #{} has {} children",
//!             node.index(),
//!             node.children().count(),
//!         );
//!     }
//! }
//! # Ok(())
//! # }
//! # fn main() {
//! #    let _ = run().expect("runtime error");
//! # }
//! ```
//!
//! ## Import function
//!
//! Reading a glTF document plus its buffers and images from the
//! file system.
//!
//! ```
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let (document, buffers, images) = gltf::import("examples/Box.gltf")?;
//! assert_eq!(buffers.len(), document.buffers().count());
//! assert_eq!(images.len(), document.images().count());
//! # Ok(())
//! # }
//! # fn main() {
//! #    let _ = run().expect("runtime error");
//! # }
//! ```
//!
//! ### Note
//!
//! This function is provided as a convenience for loading glTF and associated
//! resources from the file system. It is suitable for real world use but may
//! not be suitable for all real world use cases. More complex import scenarios
//! such downloading from web URLs are not handled by this function. These
//! scenarios are delegated to the user.
//!
//! You can read glTF without loading resources by constructing the [`Gltf`]
//! (standard glTF) or [`Glb`] (binary glTF) data structures explicitly. Buffer
//! and image data can then be imported separately using [`import_buffers`] and
//! [`import_images`] respectively.
//!
//! [glTF 2.0]: https://www.khronos.org/gltf
//! [`Gltf`]: struct.Gltf.html
//! [`Glb`]: struct.Glb.html
//! [`Node`]: struct.Node.html
//! [`Scene`]: struct.Scene.html

#[cfg(test)]
#[macro_use]
extern crate approx;
#[cfg(feature = "import")]
extern crate image as image_crate;
#[macro_use]
extern crate lazy_static;

/// Contains (de)serializable data structures that match the glTF JSON text.
pub extern crate gltf_json as json;

/// Accessors for reading vertex attributes from buffer views.
pub mod accessor;

/// Animations, their channels, targets, and samplers.
pub mod animation;

/// Primitives for working with binary glTF.
pub mod binary;

/// Buffers and buffer views.
pub mod buffer;

/// Cameras and their projections.
pub mod camera;

/// Images that may be used by textures.
pub mod image;

/// The reference importer.
#[cfg(feature = "import")]
#[cfg_attr(docsrs, doc(cfg(feature = "import")))]
mod import;

/// Iterators for walking the glTF node hierarchy.
pub mod iter;

/// Support for the `KHR_lights_punctual` extension.
#[cfg(feature = "KHR_lights_punctual")]
#[cfg_attr(docsrs, doc(cfg(feature = "KHR_lights_punctual")))]
pub mod khr_lights_punctual;

/// Support for the `KHR_materials_variants` extension.
#[cfg(feature = "KHR_materials_variants")]
#[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_variants")))]
pub mod khr_materials_variants;

/// Material properties of primitives.
pub mod material;

/// For internal use.
mod math;

/// Meshes and their primitives.
pub mod mesh;

/// The glTF node heirarchy.
pub mod scene;

/// Mesh skinning primitives.
pub mod skin;

/// Textures and their samplers.
pub mod texture;

#[cfg(feature = "extensions")]
use json::Value;
#[cfg(feature = "extensions")]
use serde_json::Map;

#[doc(inline)]
pub use self::accessor::Accessor;
#[doc(inline)]
pub use self::animation::Animation;
#[doc(inline)]
pub use self::binary::Glb;
#[doc(inline)]
pub use self::buffer::Buffer;
#[doc(inline)]
pub use self::camera::Camera;
#[doc(inline)]
pub use self::image::Image;
#[cfg(feature = "import")]
#[doc(inline)]
pub use self::import::import;
#[cfg(feature = "import")]
#[doc(inline)]
pub use self::import::import_buffers;
#[cfg(feature = "import")]
#[doc(inline)]
pub use self::import::import_images;
#[cfg(feature = "import")]
#[doc(inline)]
pub use self::import::import_slice;
#[doc(inline)]
pub use self::material::Material;
#[doc(inline)]
pub use self::mesh::{Attribute, Mesh, Primitive, Semantic};
#[doc(inline)]
pub use self::scene::{Node, Scene};
#[doc(inline)]
pub use self::skin::Skin;
#[doc(inline)]
pub use self::texture::Texture;

use std::path::Path;
use std::{fs, io, ops, result};

pub(crate) trait Normalize<T> {
    fn normalize(self) -> T;
}

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
    Deserialize(json::Error),

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
    Validation(Vec<(json::Path, json::validation::Error)>),
}

/// glTF JSON wrapper plus binary payload.
#[derive(Clone, Debug)]
pub struct Gltf {
    /// The glTF JSON wrapper.
    pub document: Document,

    /// The glTF binary payload in the case of binary glTF.
    pub blob: Option<Vec<u8>>,
}

/// glTF JSON wrapper.
#[derive(Clone, Debug)]
pub struct Document(json::Root);

impl Gltf {
    /// Convenience function that loads glTF from the file system.
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
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
        let (json, blob): (json::Root, Option<Vec<u8>>);
        if magic.starts_with(b"glTF") {
            let mut glb = binary::Glb::from_reader(reader)?;
            // TODO: use `json::from_reader` instead of `json::from_slice`
            json = json::deserialize::from_slice(&glb.json)?;
            blob = glb.bin.take().map(|x| x.into_owned());
        } else {
            json = json::deserialize::from_reader(reader)?;
            blob = None;
        };
        let document = Document::from_json_without_validation(json);
        Ok(Gltf { document, blob })
    }

    /// Loads glTF from a reader.
    pub fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: io::Read + io::Seek,
    {
        let gltf = Self::from_reader_without_validation(reader)?;
        gltf.document.validate()?;
        Ok(gltf)
    }

    /// Loads glTF from a slice of bytes without performing validation
    /// checks.
    pub fn from_slice_without_validation(slice: &[u8]) -> Result<Self> {
        let (json, blob): (json::Root, Option<Vec<u8>>);
        if slice.starts_with(b"glTF") {
            let mut glb = binary::Glb::from_slice(slice)?;
            json = json::deserialize::from_slice(&glb.json)?;
            blob = glb.bin.take().map(|x| x.into_owned());
        } else {
            json = json::deserialize::from_slice(slice)?;
            blob = None;
        };
        let document = Document::from_json_without_validation(json);
        Ok(Gltf { document, blob })
    }

    /// Loads glTF from a slice of bytes.
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        let gltf = Self::from_slice_without_validation(slice)?;
        gltf.document.validate()?;
        Ok(gltf)
    }
}

impl ops::Deref for Gltf {
    type Target = Document;
    fn deref(&self) -> &Self::Target {
        &self.document
    }
}

impl ops::DerefMut for Gltf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.document
    }
}

impl Document {
    /// Loads glTF from pre-deserialized JSON.
    pub fn from_json(json: json::Root) -> Result<Self> {
        let document = Self::from_json_without_validation(json);
        document.validate()?;
        Ok(document)
    }

    /// Loads glTF from pre-deserialized JSON without performing
    /// validation checks.
    pub fn from_json_without_validation(json: json::Root) -> Self {
        Document(json)
    }

    /// Unwraps the glTF document.
    pub fn into_json(self) -> json::Root {
        self.0
    }

    /// Unwraps the glTF document, without consuming it.
    pub fn as_json(&self) -> &json::Root {
        &self.0
    }

    /// Perform validation checks on loaded glTF.
    pub(crate) fn validate(&self) -> Result<()> {
        use json::validation::Validate;
        let mut errors = Vec::new();
        self.0
            .validate(&self.0, json::Path::new, &mut |path, error| {
                errors.push((path(), error))
            });
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Validation(errors))
        }
    }

    /// Returns an `Iterator` that visits the accessors of the glTF asset.
    pub fn accessors(&self) -> iter::Accessors {
        iter::Accessors {
            iter: self.0.accessors.iter().enumerate(),
            document: self,
        }
    }

    /// Returns an `Iterator` that visits the animations of the glTF asset.
    pub fn animations(&self) -> iter::Animations {
        iter::Animations {
            iter: self.0.animations.iter().enumerate(),
            document: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffers of the glTF asset.
    pub fn buffers(&self) -> iter::Buffers {
        iter::Buffers {
            iter: self.0.buffers.iter().enumerate(),
            document: self,
        }
    }

    /// Returns an `Iterator` that visits the cameras of the glTF asset.
    pub fn cameras(&self) -> iter::Cameras {
        iter::Cameras {
            iter: self.0.cameras.iter().enumerate(),
            document: self,
        }
    }

    /// Returns the default scene, if provided.
    pub fn default_scene(&self) -> Option<Scene> {
        self.0
            .scene
            .as_ref()
            .map(|index| self.scenes().nth(index.value()).unwrap())
    }

    /// Returns the extensions referenced in this .document file.
    pub fn extensions_used(&self) -> iter::ExtensionsUsed {
        iter::ExtensionsUsed(self.0.extensions_used.iter())
    }

    /// Returns the extensions required to load and render this asset.
    pub fn extensions_required(&self) -> iter::ExtensionsRequired {
        iter::ExtensionsRequired(self.0.extensions_required.iter())
    }

    /// Returns an `Iterator` that visits the pre-loaded images of the glTF asset.
    pub fn images(&self) -> iter::Images {
        iter::Images {
            iter: self.0.images.iter().enumerate(),
            document: self,
        }
    }

    /// Returns extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        let root = self.0.extensions.as_ref()?;
        Some(&root.others)
    }

    /// Queries extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extension_value(&self, ext_name: &str) -> Option<&Value> {
        let root = self.0.extensions.as_ref()?;
        root.others.get(ext_name)
    }

    /// Returns an `Iterator` that visits the lights of the glTF asset as defined by the
    /// `KHR_lights_punctual` extension.
    #[cfg(feature = "KHR_lights_punctual")]
    #[cfg_attr(docsrs, doc(cfg(feature = "KHR_lights_punctual")))]
    pub fn lights(&self) -> Option<iter::Lights> {
        let iter = self
            .0
            .extensions
            .as_ref()?
            .khr_lights_punctual
            .as_ref()?
            .lights
            .iter()
            .enumerate();

        Some(iter::Lights {
            iter,
            document: self,
        })
    }

    /// Returns an `Iterator` that visits the variants of the glTF asset as defined by the
    /// `KHR_materials_variants` extension.
    #[cfg(feature = "KHR_materials_variants")]
    #[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_variants")))]
    pub fn variants(&self) -> Option<iter::Variants> {
        let iter = self
            .0
            .extensions
            .as_ref()?
            .khr_materials_variants
            .as_ref()?
            .variants
            .iter()
            .enumerate();

        Some(iter::Variants {
            iter,
            document: self,
        })
    }

    /// Returns an `Iterator` that visits the materials of the glTF asset.
    pub fn materials(&self) -> iter::Materials {
        iter::Materials {
            iter: self.0.materials.iter().enumerate(),
            document: self,
        }
    }

    /// Returns an `Iterator` that visits the meshes of the glTF asset.
    pub fn meshes(&self) -> iter::Meshes {
        iter::Meshes {
            iter: self.0.meshes.iter().enumerate(),
            document: self,
        }
    }

    /// Returns an `Iterator` that visits the nodes of the glTF asset.
    pub fn nodes(&self) -> iter::Nodes {
        iter::Nodes {
            iter: self.0.nodes.iter().enumerate(),
            document: self,
        }
    }

    /// Returns an `Iterator` that visits the samplers of the glTF asset.
    pub fn samplers(&self) -> iter::Samplers {
        iter::Samplers {
            iter: self.0.samplers.iter().enumerate(),
            document: self,
        }
    }

    /// Returns an `Iterator` that visits the scenes of the glTF asset.
    pub fn scenes(&self) -> iter::Scenes {
        iter::Scenes {
            iter: self.0.scenes.iter().enumerate(),
            document: self,
        }
    }

    /// Returns an `Iterator` that visits the skins of the glTF asset.
    pub fn skins(&self) -> iter::Skins {
        iter::Skins {
            iter: self.0.skins.iter().enumerate(),
            document: self,
        }
    }

    /// Returns an `Iterator` that visits the textures of the glTF asset.
    pub fn textures(&self) -> iter::Textures {
        iter::Textures {
            iter: self.0.textures.iter().enumerate(),
            document: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffer views of the glTF
    /// asset.
    pub fn views(&self) -> iter::Views {
        iter::Views {
            iter: self.0.buffer_views.iter().enumerate(),
            document: self,
        }
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

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Self {
        Error::Deserialize(err)
    }
}

impl From<Vec<(json::Path, json::validation::Error)>> for Error {
    fn from(errs: Vec<(json::Path, json::validation::Error)>) -> Self {
        Error::Validation(errs)
    }
}

impl Normalize<i8> for i8 {
    fn normalize(self) -> i8 {
        self
    }
}

impl Normalize<u8> for i8 {
    fn normalize(self) -> u8 {
        self.max(0) as u8 * 2
    }
}

impl Normalize<i16> for i8 {
    fn normalize(self) -> i16 {
        self as i16 * 0x100
    }
}

impl Normalize<u16> for i8 {
    fn normalize(self) -> u16 {
        self.max(0) as u16 * 0x200
    }
}

impl Normalize<f32> for i8 {
    fn normalize(self) -> f32 {
        (self as f32 * 127.0_f32.recip()).max(-1.0)
    }
}

impl Normalize<i8> for u8 {
    fn normalize(self) -> i8 {
        (self / 2) as i8
    }
}

impl Normalize<u8> for u8 {
    fn normalize(self) -> u8 {
        self
    }
}

impl Normalize<i16> for u8 {
    fn normalize(self) -> i16 {
        self as i16 * 0x80
    }
}

impl Normalize<u16> for u8 {
    fn normalize(self) -> u16 {
        self as u16 * 0x100
    }
}

impl Normalize<f32> for u8 {
    fn normalize(self) -> f32 {
        self as f32 * 255.0_f32.recip()
    }
}

impl Normalize<i8> for i16 {
    fn normalize(self) -> i8 {
        (self / 0x100) as i8
    }
}

impl Normalize<u8> for i16 {
    fn normalize(self) -> u8 {
        (self.max(0) / 0x80) as u8
    }
}

impl Normalize<i16> for i16 {
    fn normalize(self) -> i16 {
        self
    }
}

impl Normalize<u16> for i16 {
    fn normalize(self) -> u16 {
        self.max(0) as u16 * 2
    }
}

impl Normalize<f32> for i16 {
    fn normalize(self) -> f32 {
        (self as f32 * 32767.0_f32.recip()).max(-1.0)
    }
}

impl Normalize<i8> for u16 {
    fn normalize(self) -> i8 {
        (self / 0x200) as i8
    }
}

impl Normalize<u8> for u16 {
    fn normalize(self) -> u8 {
        (self / 0x100) as u8
    }
}

impl Normalize<i16> for u16 {
    fn normalize(self) -> i16 {
        (self / 2) as i16
    }
}

impl Normalize<u16> for u16 {
    fn normalize(self) -> u16 {
        self
    }
}

impl Normalize<f32> for u16 {
    fn normalize(self) -> f32 {
        self as f32 * 65535.0_f32.recip()
    }
}

impl Normalize<i8> for f32 {
    fn normalize(self) -> i8 {
        (self * 127.0) as i8
    }
}

impl Normalize<u8> for f32 {
    fn normalize(self) -> u8 {
        (self.max(0.0) * 255.0) as u8
    }
}

impl Normalize<i16> for f32 {
    fn normalize(self) -> i16 {
        (self * 32767.0) as i16
    }
}

impl Normalize<u16> for f32 {
    fn normalize(self) -> u16 {
        (self.max(0.0) * 65535.0) as u16
    }
}

impl Normalize<f32> for f32 {
    fn normalize(self) -> f32 {
        self
    }
}

impl<U, T> Normalize<[T; 2]> for [U; 2]
where
    U: Normalize<T> + Copy,
{
    fn normalize(self) -> [T; 2] {
        [self[0].normalize(), self[1].normalize()]
    }
}

impl<U, T> Normalize<[T; 3]> for [U; 3]
where
    U: Normalize<T> + Copy,
{
    fn normalize(self) -> [T; 3] {
        [
            self[0].normalize(),
            self[1].normalize(),
            self[2].normalize(),
        ]
    }
}

impl<U, T> Normalize<[T; 4]> for [U; 4]
where
    U: Normalize<T> + Copy,
{
    fn normalize(self) -> [T; 4] {
        [
            self[0].normalize(),
            self[1].normalize(),
            self[2].normalize(),
            self[3].normalize(),
        ]
    }
}
