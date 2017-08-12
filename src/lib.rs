
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
//! Add `gltf` version 0.8 to your `Cargo.toml`.
//!
//! ```toml
//! [dependencies.gltf]
//! version = "0.8"
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
//! let file = std::fs::File::open("examples/Box.gltf")?;
//! let reader = std::io::BufReader::new(file);
//! let json = gltf::json::from_reader(reader)?;
//! let gltf = gltf::Gltf::from_json(json);
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
//!
//! ### Providing `Gltf` with external buffer data
//!
//! The [`Source`] trait provides `glTF` objects with their buffer data. This allows
//! the crate to provide more abstractions such as iterating over the positions of
//! a `Primitive`. See the documentation of [`Loaded`] for all the methods available
//! for loaded `glTF`.
//!
//! The `gltf-importer` crate contains the reference implementation of the
//! `Source` trait and may be used to read buffer data from the file system.
//!
//! [`Source`]: trait.Source.html
//! [`Loaded`]: struct.Loaded.html
//! ```
//! # use gltf::json;
//! # use gltf::Gltf;
//! # fn run() -> Result<(), Box<std::error::Error>> {
//! # let path = "./glTF-Sample-Models/2.0/Box/glTF/Box.gltf";
//! # let file = std::fs::File::open(path)?;
//! # let reader = std::io::BufReader::new(file);
//! # let json = json::from_reader(reader)?;
//! # let gltf = Gltf::from_json(json);
//! #[derive(Debug)]
//! struct BoxExampleData(&'static [u8]);
//!
//! impl gltf::Source for BoxExampleData {
//!     fn source_buffer(&self, _: &gltf::Buffer) -> &[u8] {
//!         // In a real implementation, the `Source` must provide all the data
//!         // necessary to load the object, and must not fail.
//!         //
//!         // This example meets the above criteria, since it provides all the data
//!         // for the 'Box' sample model, which has exactly one external buffer.
//!         self.0
//!     }
//! }
//!
//! let data = BoxExampleData(include_bytes!("examples/Box0.bin"));
//! let loaded_gltf = gltf.loaded(&data);
//! for mesh in loaded_gltf.meshes() {
//!     for primitive in mesh.primitives() {
//!         if let Some(iter) = primitive.indices_u32() {
//!             // Do something with the primitive data.
//!             let indices: Vec<u32> = iter.collect();
//!             println!("{:?}", indices);
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! # fn main() {
//! #    let _ = run().expect("No runtime errors");
//! # }
//! ```

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
pub use self::gltf::Gltf;
pub use self::image::Image;
pub use self::material::Material;
pub use self::mesh::{Mesh, Primitive};
pub use self::scene::{Node, Scene};
pub use self::skin::Skin;
pub use self::texture::Texture;
