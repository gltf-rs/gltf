#![deny(missing_docs)]
#![allow(unknown_lints)]

//! glTF 2.0 loader
//!
//! This crate is intended to load [glTF 2.0], a file format designed for the
//! efficient runtime transmission of 3D scenes. The crate aims to provide
//! rustic utilities that make working with glTF simple and intuitive.
//!
//! [glTF 2.0]: https://www.khronos.org/gltf
//!
//! ## Installation
//!
//! Add `gltf` version 0.11 to your `Cargo.toml`.
//!
//! ```toml
//! [dependencies.gltf]
//! version = "0.11"
//! ```
//!
//! ## Examples
//!
//! ### Walking the node hierarchy
//!
//! Below demonstates visiting the root [`Node`]s of every [`Scene`], printing the
//! number of children each node has.
//!
//! [`Node`]: scene/struct.Node.html
//! [`Scene`]: scene/struct.Scene.html
//! ```
//! # fn run() -> Result<(), Box<std::error::Error>> {
//! # let path = "examples/Box.gltf";
//! let file = std::fs::File::open(path)?;
//! let reader = std::io::BufReader::new(file);
//! let gltf = gltf::Gltf::from_reader(reader)?;
//! for scene in gltf.scenes() {
//!     for node in scene.nodes() {
//!         // Do something with this node.
//!         println!(
//!             "Node {} has {} children",
//!             node.index(),
//!             node.children().count(),
//!         );
//!     }
//! }
//! # Ok(())
//! # }
//! # fn main() {
//! #    let _ = run().expect("No runtime errors");
//! # }
//! ```

#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate byteorder;
extern crate cgmath;
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

/// Iterators for walking the glTF node hierarchy.
pub mod iter;

/// Material properties of primitives.
pub mod material;

/// Meshes and their primitives.
pub mod mesh;

/// The glTF node heirarchy.
pub mod scene;

/// Mesh skinning primitives.
pub mod skin;

/// Textures and their samplers.
pub mod texture;

#[doc(inline)]
pub use self::animation::Animation;
#[doc(inline)]
pub use self::accessor::Accessor;
#[doc(inline)]
pub use self::buffer::Buffer;
#[doc(inline)]
pub use self::camera::Camera;
#[doc(inline)]
pub use self::image::Image;
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

use std::{fmt, io};

pub(crate) trait Normalize<T> {
    fn normalize(self) -> T;
}

/// Represents a runtime error.
#[derive(Debug)]
pub enum Error {
    /// GLB parsing error.
    Binary(self::binary::Error),
    /// JSON deserialization error.
    Deserialize(json::Error),
    /// Standard I/O error.
    Io(std::io::Error),
    /// glTF validation error.
    Validation(Vec<(json::Path, json::validation::Error)>),
}

/// **The primary data structure of this crate.**
pub struct Gltf(json::Root);

/// Loads glTF from a reader without performing validation checks.
pub fn from_reader_without_validation<R>(mut reader: R) -> Result<(Gltf, Option<Vec<u8>>), Error>
where
    R: io::Read + io::Seek
{
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;
    reader.seek(io::SeekFrom::Start(0))?;
    let (json, bin): (json::Root, Option<Vec<u8>>);
    if magic.starts_with(b"glTF") {
        let mut glb = binary::Glb::from_reader(reader)?;
        // TODO: use `json::from_reader` instead of `json::from_slice`
        json = json::deserialize::from_slice(&glb.json)?;
        bin = glb.bin.take().map(|x| x.into_owned());
    } else {
        json = json::deserialize::from_reader(reader)?;
        bin = None;
    };
    Ok((Gltf::from_json_without_validation(json), bin ))
}

/// Loads glTF from a reader.
pub fn from_reader<R>(reader: R) -> Result<(Gltf, Option<Vec<u8>>), Error>
where
    R: io::Read + io::Seek,
{
    let (gltf, bin) = from_reader_without_validation(reader)?;
    let _ = gltf.validate()?;
    Ok((gltf, bin))
}

/// Loads glTF from a slice of bytes without performing validation
/// checks.
pub fn from_slice_without_validation(slice: &[u8]) -> Result<(Gltf, Option<Vec<u8>>), Error> {
    let (json, bin): (json::Root, Option<Vec<u8>>);
    if slice.starts_with(b"glTF") {
        let mut glb = binary::Glb::from_slice(slice)?;
        json = json::deserialize::from_slice(&glb.json)?;
        bin = glb.bin.take().map(|x| x.into_owned());
    } else {
        json = json::deserialize::from_slice(slice)?;
        bin = None;
    };
    Ok((Gltf::from_json_without_validation(json), bin))
}

/// Loads glTF from a slice of bytes.
pub fn from_slice(slice: &[u8]) -> Result<(Gltf, Option<Vec<u8>>), Error> {
    let (gltf, bin) = from_slice_without_validation(slice)?;
    let _ = gltf.validate()?;
    Ok((gltf, bin))
}

impl Gltf {
    /// Loads glTF from pre-deserialized JSON.
    pub fn from_json(json: json::Root) -> Result<Self, Error> {
        let gltf = Self::from_json_without_validation(json);
        let _ = gltf.validate()?;
        Ok(gltf)
    }

    /// Loads glTF from pre-deserialized JSON without performing
    /// validation checks.
    pub fn from_json_without_validation(json: json::Root) -> Self {
        Gltf(json)
    }

    /// Unwraps the glTF document.
    pub fn into_json(self) -> json::Root {
        self.0
    }

    /// Perform validation checks on loaded glTF.
    ///
    /// Note: this is called automatically during
    /// `Gltf::from_reader` and `Gltf::from_slice`.
    pub fn validate(&self) -> Result<(), Vec<(json::Path, json::validation::Error)>> {
        use json::validation::Validate;
        let mut errors = Vec::new();
        self.0.validate_minimally(
            &self.0,
            json::Path::new,
            &mut |path, error| errors.push((path(), error)),
        );
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Returns an `Iterator` that visits the accessors of the glTF asset.
    pub fn accessors(&self) -> iter::Accessors {
        iter::Accessors {
            iter: self.0.accessors.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the animations of the glTF asset.
    pub fn animations(&self) -> iter::Animations {
        iter::Animations {
            iter: self.0.animations.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffers of the glTF asset.
    pub fn buffers(&self) -> iter::Buffers {
        iter::Buffers {
            iter: self.0.buffers.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the cameras of the glTF asset.
    pub fn cameras(&self) -> iter::Cameras {
        iter::Cameras {
            iter: self.0.cameras.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns the default scene, if provided.
    pub fn default_scene(&self) -> Option<Scene> {
        self.0
            .scene
            .as_ref()
            .map(|index| self.scenes().nth(index.value()).unwrap())
    }

    /// Returns the extensions referenced in this .gltf file.
    pub fn extensions_used(&self) -> iter::Extensions {
        iter::Extensions(self.0.extensions_used.iter())
    }

    /// Returns the extensions required to load and render this asset.
    pub fn extensions_required(&self) -> iter::Extensions {
        iter::Extensions(self.0.extensions_required.iter())
    }

    /// Returns an `Iterator` that visits the pre-loaded images of the glTF asset.
    pub fn images(&self) -> iter::Images {
        iter::Images {
            iter: self.0.images.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the materials of the glTF asset.
    pub fn materials(&self) -> iter::Materials {
        iter::Materials {
            iter: self.0.materials.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the meshes of the glTF asset.
    pub fn meshes(&self) -> iter::Meshes {
        iter::Meshes {
            iter: self.0.meshes.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the nodes of the glTF asset.
    pub fn nodes(&self) -> iter::Nodes {
        iter::Nodes {
            iter: self.0.nodes.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the samplers of the glTF asset.
    pub fn samplers(&self) -> iter::Samplers {
        iter::Samplers {
            iter: self.0.samplers.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the scenes of the glTF asset.
    pub fn scenes(&self) -> iter::Scenes {
        iter::Scenes {
            iter: self.0.scenes.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the skins of the glTF asset.
    pub fn skins(&self) -> iter::Skins {
        iter::Skins {
            iter: self.0.skins.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the textures of the glTF asset.
    pub fn textures(&self) -> iter::Textures {
        iter::Textures {
            iter: self.0.textures.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffer views of the glTF
    /// asset.
    pub fn views(&self) -> iter::Views {
        iter::Views {
            iter: self.0.buffer_views.iter().enumerate(),
            gltf: self,
        }
    }
}

impl<'a> fmt::Debug for Gltf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
         match *self {
             Error::Binary(ref e) => e.description(),
             Error::Deserialize(_) => "deserialization error",
             Error::Io(ref e) => e.description(),
             Error::Validation(_) => "invalid glTF",
        }
    }
}

impl From<self::binary::Error> for Error {
    fn from(err: self::binary::Error) -> Self {
        Error::Binary(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
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
    fn normalize(self) -> i8 { self }
}

impl Normalize<u8> for i8 {
    fn normalize(self) -> u8 { self.max(0) as u8 * 2 }
}

impl Normalize<i16> for i8 {
    fn normalize(self) -> i16 { self as i16 * 0x100 }
}

impl Normalize<u16> for i8 {
    fn normalize(self) -> u16 { self.max(0) as u16 * 0x200 }
}

impl Normalize<f32> for i8 {
    fn normalize(self) -> f32 { (self as f32 * 127.0_f32.recip()).max(-1.0) }
}

impl Normalize<i8> for u8 {
    fn normalize(self) -> i8 { (self / 2) as i8 }
}

impl Normalize<u8> for u8 {
    fn normalize(self) -> u8 { self }
}

impl Normalize<i16> for u8 {
    fn normalize(self) -> i16 { self as i16 * 0x80 }
}

impl Normalize<u16> for u8 {
    fn normalize(self) -> u16 { self.max(0) as u16 * 2 }
}

impl Normalize<f32> for u8 {
    fn normalize(self) -> f32 { (self as f32 * 32767.0_f32.recip()).max(-1.0) }
}

impl Normalize<i8> for i16 {
    fn normalize(self) -> i8 { (self / 0x100) as i8 }
}

impl Normalize<u8> for i16 {
    fn normalize(self) -> u8 { (self.max(0) / 0x80) as u8 }
}

impl Normalize<i16> for i16 {
    fn normalize(self) -> i16 { self }
}

impl Normalize<u16> for i16 {
    fn normalize(self) -> u16 { self.max(0) as u16 * 2 }
}

impl Normalize<f32> for i16 {
    fn normalize(self) -> f32 { (self as f32 * 32767.0_f32.recip()).max(-1.0) }
}

impl Normalize<i8> for u16 {
    fn normalize(self) -> i8 { (self / 0x200) as i8 }
}

impl Normalize<u8> for u16 {
    fn normalize(self) -> u8 { (self / 0x100) as u8 }
}

impl Normalize<i16> for u16 {
    fn normalize(self) -> i16 { (self / 2) as i16 }
}

impl Normalize<u16> for u16 {
    fn normalize(self) -> u16 { self }
}

impl Normalize<f32> for u16 {
    fn normalize(self) -> f32 { self as f32 * 65535.0_f32.recip() }
}

impl Normalize<i8> for f32 {
    fn normalize(self) -> i8 { (self * 127.0) as i8 }
}

impl Normalize<u8> for f32 {
    fn normalize(self) -> u8 { (self.max(0.0) * 255.0) as u8 }
}

impl Normalize<i16> for f32 {
    fn normalize(self) -> i16 { (self * 32767.0) as i16 }
}

impl Normalize<u16> for f32 {
    fn normalize(self) -> u16 { (self.max(0.0) * 65535.0) as u16 }
}

impl Normalize<f32> for f32 {
    fn normalize(self) -> f32 { self }
}

impl<U, T> Normalize<[T; 2]> for [U; 2] where U: Normalize<T> + Copy {
    fn normalize(self) -> [T; 2] {
        [self[0].normalize(), self[1].normalize()]
    }
}

impl<U, T> Normalize<[T; 3]> for [U; 3] where U: Normalize<T> + Copy {
    fn normalize(self) -> [T; 3] {
        [self[0].normalize(), self[1].normalize(), self[2].normalize()]
    }
}

impl<U, T> Normalize<[T; 4]> for [U; 4] where U: Normalize<T> + Copy {
    fn normalize(self) -> [T; 4] {
        [self[0].normalize(), self[1].normalize(), self[2].normalize(), self[3].normalize()]
    }
}
