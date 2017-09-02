
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs)]

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

extern crate cgmath;
#[macro_use]
extern crate lazy_static;

/// Contains (de)serializable data structures that match the glTF JSON text.
pub extern crate gltf_json as json;

/// Contains `Accessor` and other related data structures.
pub mod accessor;

/// Contains `Animation` and other related data structures.
pub mod animation;

/// Contains `Buffer`, `View`, and other related data structures.
pub mod buffer;

/// Contains `Camera` and other related data structures.
pub mod camera;

/// Contains `Glb` and its parsing implementation.
pub mod glb;

/// Contains `Gltf`, and other related data structures.
pub mod gltf;

/// Contains `Image` and other related data structures.
pub mod image;

/// Contains `Material` and other related data structures.
pub mod material;

/// Contains `Mesh` and other related data structures.
pub mod mesh;

/// Contains `Root`.
pub mod root;

/// Contains `Scene`, `Node`, and other related data structures.
pub mod scene;

/// Contains `Skin` and other related data structures.
pub mod skin;

/// Contains `Texture`, `Sampler`, and other related data structures.
pub mod texture;

pub use self::animation::Animation;
pub use self::accessor::Accessor;
pub use self::buffer::Buffer;
pub use self::camera::Camera;
pub use self::glb::Glb;
pub use self::gltf::{Gltf, Unvalidated};
pub use self::image::Image;
pub use self::material::Material;
pub use self::mesh::{Attribute, Mesh, Primitive, Semantic};
pub use self::scene::{Node, Scene};
pub use self::skin::Skin;
pub use self::texture::Texture;

/// Represents a runtime error.
#[derive(Debug)]
pub enum Error {
    /// JSON deserialization error.
    Deserialize(json::Error),

    /// GLB parsing error.
    Glb(String),

    /// `glTF` validation error.
    Validation(Vec<(json::Path, json::validation::Error)>),
}

/// Returns `true` if the slice begins with the `b"glTF"` magic string, indicating
/// a binary `glTF` asset.
///
/// # Examples
///
/// ```rust
/// assert_eq!(false, gltf::is_binary(include_bytes!("examples/Box.gltf")));
/// assert_eq!(true, gltf::is_binary(include_bytes!("examples/Box.glb")));
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
            Error::Glb(_) => "invalid .glb format",
            Error::Validation(_) => "invalid glTF JSON",
        }
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
