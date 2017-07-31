
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
//! Add `gltf` version 0.6 to your `Cargo.toml`.
//!
//! ```toml
//! [dependencies.gltf]
//! version = "0.6"
//! ```
//!
//! ## Examples
//!
//! ### Loading glTF from the file system
//!
//! The crate provides a `from_path` method whereby one can import glTF from the
//! system.
//!
//! ```
//! extern crate gltf;
//!
//! fn main() {
//!     # #[allow(unused_variables)]
//!     let path = "path/to/asset.gltf";
//!     # let path = "./glTF-Sample-Models/2.0/Box/glTF/Box.gltf";
//!     // This creates a `Future` that drives the loading
//!     // of glTF and all of its data.
//!     let import = gltf::Import::from_path(path);
//!     // The simpliest way of working with futures is to
//!     // block the thread until the glTF is ready.
//!     match import.sync() {
//!         Ok(gltf) => println!("{:#?}", gltf),
//!         Err(err) => println!("error: {:?}", err),
//!     }
//! }
//! ```
//!
//! An [`Import`] resolves to [`Gltf`], a data structure that provides helpful utilities
//! such as iterators for working with glTF.
//!
//! [`Import`]: import/struct.Import.html
//! [`Gltf`]: gltf/struct.Gltf.html
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
//! # let path = "./glTF-Sample-Models/2.0/Box/glTF/Box.gltf";
//! let gltf = gltf::Import::from_path(path).sync()?;
//! for scene in gltf.scenes() {
//!     for node in scene.nodes() {
//!         // Do something with this node
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

extern crate base64;
extern crate futures;
#[macro_use]
extern crate gltf_derive;
extern crate image as image_crate;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/// Contains `Accessor` and other related data structures.
pub mod accessor;

/// Contains `Animation` and other related data structures.
pub mod animation;

/// Contains `Buffer`, `View`, and other related data structures.
pub mod buffer;

/// Contains `Camera` and other related data structures.
pub mod camera;

/// Contains extension specific data structures.
pub mod extensions;

/// Contains `Gltf`, and other related data structures.
pub mod gltf;

/// Contains `Image` and other related data structures.
pub mod image;

/// Contains functions for importing glTF 2.0 assets.
pub mod import;

/// Contains (de)serializable data structures that match the glTF JSON text.
pub mod json;

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

/// Contains functions that validate glTF JSON data against the specification.
pub mod validation;

/// Contains re-exports of commonly used glTF data structures.
pub mod prelude {
    pub use ::animation::Animation;
    pub use ::accessor::Accessor;
    pub use ::buffer::Buffer;
    pub use ::camera::Camera;
    pub use ::gltf::Gltf;
    pub use ::image::Image;
    pub use ::import::{Data, DynamicImage, Import};
    pub use ::material::Material;
    pub use ::mesh::{Mesh, Primitive};
    pub use ::scene::{Node, Scene};
    pub use ::skin::Skin;
    pub use ::texture::Texture;
}

pub use self::prelude::*;
