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
//! Add `gltf` version 0.9 to your `Cargo.toml`.
//!
//! ```toml
//! [dependencies.gltf]
//! version = "0.9"
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
//! # use std::{fs, io};
//! # let path = "examples/Box.gltf";
//! use gltf::Gltf;
//! let file = fs::File::open(path)?;
//! let gltf = Gltf::from_reader(io::BufReader::new(file))?.validate_minimally()?;
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
#[deprecated(since = "0.9.1", note = "Will be removed in 1.0.0")]
#[doc(hidden)]
pub extern crate gltf_json as json;

/// Accessors for reading vertex attributes from buffer views.
pub mod accessor;

/// Animations, their channels, targets, and samplers.
pub mod animation;

/// Buffers and buffer views.
pub mod buffer;

/// Cameras and their projections.
pub mod camera;

/// Primitives for working with binary glTF.
pub mod glb;

/// **The main module - start here.**
pub mod gltf;

/// Images that may be used by textures.
pub mod image;

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
pub use self::glb::Glb;
#[doc(inline)]
pub use self::gltf::{Gltf, Unvalidated};
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

/// Represents a runtime error.
#[derive(Debug)]
pub enum Error {
    /// JSON deserialization error.
    Deserialize(json::Error),
    /// GLB parsing error.
    Glb(self::glb::Error),
    /// `glTF` validation error.
    Validation(Vec<(json::Path, json::validation::Error)>),
}

/// Represents a Glb loader error.
#[derive(Debug)]
pub enum GlbError {
    /// Slice ended before we could even read the header.
    MissingHeader,
    /// Unsupported version in GLB header.
    Version,
    /// Magic says that file is not glTF.
    Magic([u8; 4]),
    /// Length is header exceeeds that of slice.
    Length,
    /// JSON chunkLength exceeeds slice length.
    JsonChunkLength,
    /// JSON chunkType is not JSON.
    JsonChunkType,
    /// BIN chunkLength exceeds length of data.
    BinChunkLength,
    /// BIN chunkType is not BIN\0
    BinChunkType,
}

/// Returns `true` if the slice begins with the `b"glTF"` magic string, indicating
/// a binary `glTF` asset.
///
/// # Examples
///
/// ```rust
/// assert_eq!(true, gltf::is_binary(b"glTF..."));
/// assert_eq!(false, gltf::is_binary(b"{...}"));
/// ```
pub fn is_binary(slice: &[u8]) -> bool {
    slice.starts_with(b"glTF")
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
             Error::Deserialize(_) => "deserialization error",
             Error::Glb(ref e) => e.description(),
             Error::Validation(_) => "invalid glTF JSON",
        }
    }
}

impl From<self::glb::Error> for Error {
    fn from(err: self::glb::Error) -> Self {
        Error::Glb(err)
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

impl std::fmt::Display for GlbError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for GlbError {
    fn description(&self) -> &str {
         match *self {
             GlbError::MissingHeader => "missing header",
             GlbError::Version => "unsupported version",
             GlbError::Magic(_) => "not glTF magic",
             GlbError::Length => "length in header exceeds that of slice",
             GlbError::JsonChunkLength => "JSON chunkLength exceeeds slice length",
             GlbError::JsonChunkType => "JSON chunkType is not JSON",
             GlbError::BinChunkLength => "BIN chunkLength exceeds slice length",
             GlbError::BinChunkType => "BIN chunkType is not BIN\\0",
        }
    }
}
