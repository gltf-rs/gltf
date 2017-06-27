
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//!
//! # Examples
//!
//! Loading glTF JSON text from a reader.
//!
//! ```rust
//! # use std::boxed::Box;
//! # use std::error::Error;
//! # use std::fs::File;
//! # fn run() -> Result<(), Box<Error>> {
//! let path = "glTF-Sample-Models/2.0/Box/glTF/Box.gltf";
//! let file = File::open(path)?;
//! let json: gltf::json::Root<'static> = gltf::json::from_reader(file)?;
//! println!("{:#?}", json);
//! # Ok(())
//! # }
//! #
//! # fn main() {
//! #     let _ = run().unwrap();
//! # }
//! ```
//!
//! Creating a wrapper interface around the raw glTF JSON text.
//!
//! ```rust
//! # use std::boxed::Box;
//! # use std::error::Error;
//! # use std::fs::File;
//! # fn run() -> Result<(), Box<Error>> {
//! # let path = "glTF-Sample-Models/2.0/Box/glTF/Box.gltf";
//! # let file = File::open(path)?;
//! # let json: gltf::json::Root<'static> = gltf::json::from_reader(file)?;
//! #
//! use gltf::{BufferData, Gltf, ImageData};
//! use gltf::root::Root;
//!
//! /// Loads the glTF buffers in the order they occur in the glTF JSON text.
//! fn load_buffers(_: &gltf::json::Root) -> Vec<BufferData> {
//!     // Your implementation here
//!     vec![]
//! }
//!
//! /// Loads the glTF images in the order they occur in the glTF JSON text.
//! fn load_images(_: &gltf::json::Root) -> Vec<ImageData> {
//!     // Your implementation here
//!     vec![]
//! }
//!
//! // The wrapper interface requires slices of the buffer and image data.
//! // The ordering of these buffer / image slices must match the ordering of the
//! // buffers / images in the glTF JSON text and contain the corresponding
//! // buffer / image data.
//! let buffers = load_buffers(&json);
//! let images = load_images(&json);
//! let gltf = Gltf::new(Root::new(json), buffers, images);
//! # let _ = gltf;
//! # Ok(())
//! # }
//! #
//! # fn main() {
//! #     let _ = run().unwrap();
//! # }
//! ```
//!
//! Validating the correctness of an entire glTF JSON text.
//!
//! ```rust
//! # use std::boxed::Box;
//! # use std::error::Error;
//! # use std::fs::File;
//! # fn run() -> Result<(), Box<Error>> {
//! # let path = "glTF-Sample-Models/2.0/Box/glTF/Box.gltf";
//! # let file = File::open(path)?;
//! # let json: gltf::json::Root<'static> = gltf::json::from_reader(file)?;
//! use gltf::validation::{JsonPath, Validate};
//!
//! // Validate the whole glTF structure, starting from the root object.
//! let mut errs = vec![];
//! let root = &json;
//! let root_path = || JsonPath::new();
//! json.validate(root, root_path, &mut |err| errs.push(err));
//! if !errs.is_empty() {
//!     println!("error: Invalid glTF ({:?})", errs[0]);
//! }
//! # Ok(())
//! # }
//! #
//! # fn main() {
//! #     let _ = run().unwrap();
//! # }
//! ```
//!
//! # Remarks
//!
//! * Instead of working with the raw JSON test, one can customize the behavior of
//!   the high-level importer by implementing `import::Source` and calling
//!   `import::Importer::from_source`. See the `import` module for further details.

use serde;

/// Contains `Accessor` and other related data structures.
pub mod accessor;

/// Contains `Animation` and other related data structures.
pub mod animation;

/// Contains `Asset` metadata.
pub mod asset;

/// Contains `Buffer`, `View`, and other related data structures.
pub mod buffer;

/// Contains `Camera` and other related data structures.
pub mod camera;

/// Contains the names of 2.0 extensions enabled and supported by the library.
pub mod extensions;

/// Contains `Extras`.
pub mod extras;

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

pub use self::extras::Extras;
pub use self::root::{Index, Root};

pub use serde_json::{
    from_reader,
    from_slice,
    from_str,
    from_value,
    to_string,
    to_string_pretty,
    to_value,
    to_vec,
    to_vec_pretty,
    to_writer,
    to_writer_pretty,
};

pub use serde_json::{
    Result as SerdeResult,
    Value,
};
