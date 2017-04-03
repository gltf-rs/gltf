
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde;

/// Contains `Accessor` and other related data structures
pub mod accessor;

/// Contains `Animation` and other related data structures
pub mod animation;

/// Contains `Asset` metadata
pub mod asset;

/// Contains `Buffer`, `BufferView`, and other related data structures
pub mod buffer;

/// Contains `Camera` and other related data structures
pub mod camera;

/// Contains the names of 2.0 extensions enabled and supported by the library
pub mod extensions;

/// Contains convenience implementations of the `Extra` trait
pub mod extras;

/// Contains `Image` and other related data structures
pub mod image;

/// Contains functions for importing glTF 2.0 assets
pub mod import;

/// Contains `Material` and other related data structures
pub mod material;

/// Contains `Mesh` and other related data structures
pub mod mesh;

/// Contains `Root`
pub mod root;

/// Contains `Scene`, `Node`, and other related data structures
pub mod scene;

/// Contains `Skin` and other related data structures
pub mod skin;

/// Contains `Texture`, `Sampler`, and other related data structures
pub mod texture;

/// Contains convenience iterators for traversing the glTF tree
pub mod tree;

pub use self::extras::Extras;
pub use self::import::{import, ImportError};
pub use self::root::{Index, Root};

