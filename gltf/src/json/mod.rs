//! JSON module.

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

pub use self::accessor::Accessor;
pub use self::animation::Animation;
pub use self::asset::Asset;
pub use self::buffer::Buffer;
pub use self::camera::Camera;
pub use self::extras::Extras;
pub use self::image::Image;
pub use self::material::Material;
pub use self::mesh::Mesh;
pub use self::path::Path;
pub use self::root::{Index, Root};
pub use self::scene::{Node, Scene};
pub use self::skin::Skin;
pub use self::texture::Texture;

pub use serde_json::{from_reader, from_slice, from_str, from_value};
pub use serde_json::{Error, Value};
