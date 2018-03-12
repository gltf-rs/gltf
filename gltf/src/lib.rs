#![allow(unknown_lints)]
#![warn(missing_docs)]

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
//! # use std::{fs, io};
//! # let path = "examples/Box.gltf";
//! use gltf::Gltf;
//! let file = fs::File::open(path)?;
//! let gltf = Gltf::from_reader(io::BufReader::new(file))?.validate_minimally()?;
//! for scene in doc.scenes() {
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
extern crate base64;
extern crate byteorder;
extern crate cgmath;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate gltf_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/// Accessors for reading vertex attributes from buffer views.
pub mod accessor;

/// Animations, their channels, targets, and samplers.
pub mod animation;

/// Buffers and buffer views.
pub mod buffer;

/// Data structures for working with binary glTF.
pub mod binary;

/// Cameras and their projections.
pub mod camera;

/// The root of the glTF scene representation.
pub mod document;

/// Images that may be used by textures.
pub mod image;

/// Import library.
#[cfg(feature = "import")]
pub mod import;

/// Contains (de)serializable data structures that match the glTF JSON text.
mod json;

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

/// Utility library.
#[cfg(feature = "util")]
pub mod util;

#[doc(inline)]
pub use self::animation::Animation;

#[doc(inline)]
pub use self::accessor::Accessor;

#[doc(inline)]
pub use self::buffer::Buffer;

#[doc(inline)]
pub use self::camera::Camera;

#[doc(inline)]
pub use self::document::Document;

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

/// Untyped JSON value - depends on correct serde version.
pub type Value = serde_json::Value;

/// Represents a runtime error.
#[derive(Debug)]
pub enum Error {
    /// JSON deserialization error.
    Deserialize(json::Error),

    /// Binary glTF parsing error.
    Binary(binary::Error),

    /// glTF validation error.
    Validation(Vec<(json::Path, json::validation::Error)>),

    /// A glTF extension required by the asset has not been enabled by the user.
    ExtensionDisabled(String),

    /// A glTF extension required by the asset is not supported by the library.
    ExtensionUnsupported(String),

    /// The glTF version of the asset is incompatible with the crate, i.e. not glTF 2.0.
    IncompatibleVersion(String),

    /// A loaded glTF buffer is not of the required length.
    #[cfg(feature = "import")]
    BufferLength(json::Path),

    /// Base 64 decoding error.
    #[cfg(feature = "import")]
    Base64Decoding(base64::DecodeError),

    /// File not found.
    #[cfg(feature = "import")]
    FileNotFound(path::PathBuf),

    /// Standard I/O error.
    #[cfg(feature = "import")]
    Io(io::Error),
}

/// The main data structure of the crate.
pub enum Gltf {
    /// Standard glTF.
    Standard(Document),

    /// Binary glTF.
    Binary(binary::Header, Document, binary::Payload),
}

/// Represents `glTF` that hasn't been validated yet.
pub struct Unvalidated(Document);

impl Unvalidated {
    /// Returns the unvalidated JSON.
    pub(crate) fn as_json(&self) -> &json::Root {
        self.0.as_json()
    }

    /// Constructs an unvalidated document.
    pub(crate) fn new(document: Document) -> Self {
        Unvalidated(document)
    }

    /// Skip validation.  **Using this is highly recommended against** as
    /// malformed glTF assets might lead to program panics, huge values, NaNs,
    /// and general evil deeds.
    ///
    /// # Panics
    ///
    /// This function does not panic, but might cause an inherent panic later in
    /// your program during reading of a malformed asset.
    pub fn skip_validation(self) -> Document {
        self.0
    }

    /// Validates only the invariants required for the library to function safely.
    pub fn validate_minimally(self) -> Result<Document, Error> {
        use json::validation::Validate;
        let mut errs = vec![];
        {
            let json = self.as_json();
            json.validate_minimally(json, json::Path::new, &mut |path, err| {
                errs.push((path(), err))
            });
        }
        if errs.is_empty() {
            Ok(self.0)
        } else {
            Err(Error::Validation(errs))
        }
    }

    /// Validates the data against the `glTF` 2.0 specification.
    pub fn validate_completely(self) -> Result<Document, Error> {
        use json::validation::Validate;
        let mut errs = vec![];
        {
            let json = self.as_json();
            json.validate_minimally(json, json::Path::new, &mut |path, err| {
                errs.push((path(), err))
            });
            json.validate_completely(json, json::Path::new, &mut |path, err| {
                errs.push((path(), err))
            });
        }
        if errs.is_empty() {
            Ok(self.0)
        } else {
            Err(Error::Validation(errs))
        }
    }
}

impl Gltf {
    /// Constructs the `Gltf` wrapper from deserialized JSON.
    fn from_json(json: json::Root) -> Self {
        // Gltf { root: json }
        unimplemented!()
    }

    /// Constructs the `Gltf` wrapper from a reader.
    pub fn from_reader<R>(reader: R) -> Result<Unvalidated, Error>
    where
        R: std::io::Read,
    {
        // let json: json::Root = json::from_reader(reader)?;
        // Ok(Unvalidated(Gltf::from_json(json)))
        unimplemented!()
    }

    /// Constructs the `Gltf` wrapper from a slice of bytes.
    pub fn from_slice(slice: &[u8]) -> Result<Unvalidated, Error> {
        // let json: json::Root = json::from_slice(slice)?;
        // Ok(Unvalidated(Gltf::from_json(json)))
        unimplemented!()
    }

    /// Constructs the `Gltf` wrapper from a string slice.
    #[allow(should_implement_trait)]
    pub fn from_str(slice: &str) -> Result<Unvalidated, Error> {
        // let json: json::Root = json::from_str(slice)?;
        // Ok(Unvalidated(Gltf::from_json(json)))
        unimplemented!()
    }

    /// Constructs the `Gltf` wrapper from a `gltf_json::Value`.
    pub fn from_value(value: Value) -> Result<Unvalidated, Error> {
        // let json: json::Root = json::from_value(value)?;
        // Ok(Unvalidated(Gltf::from_json(json)))
        unimplemented!()
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
            Error::Deserialize(_) => "deserialization error",
            Error::Binary(ref e) => e.description(),
            Error::Validation(_) => "invalid glTF JSON",
            _ => unimplemented!(),
        }
    }
}

impl From<binary::Error> for Error {
    fn from(err: binary::Error) -> Self {
        Error::Binary(err)
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
