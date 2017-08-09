
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate gltf_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

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

/// Contains extension specific data structures and the names of all
/// 2.0 extensions supported by the library.
pub mod extensions;

/// Contains `Extras`.
pub mod extras;

/// Contains `Image` and other related data structures.
pub mod image;

/// Contains `Material` and other related data structures.
pub mod material;

/// Contains `Mesh` and other related data structures.
pub mod mesh;

/// Contains `Path`.
pub mod path;

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

pub use accessor::Accessor;
pub use animation::Animation;
pub use asset::Asset;
pub use buffer::Buffer;
pub use camera::Camera;
pub use image::Image;
pub use material::Material;
pub use mesh::Mesh;
pub use scene::{Node, Scene};
pub use skin::Skin;
pub use texture::Texture;

pub use self::extras::Extras;
pub use self::path::Path;
pub use self::root::{Index, Root};
pub use serde_json::{from_reader, from_slice, from_str, from_value};
pub use serde_json::Error;
