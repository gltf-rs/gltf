
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_json;
use std;
use std::collections::HashMap;

/// Contains `Accessor` and other related data structures
pub mod accessor;

/// Contains `Animation` and other related data structures
pub mod animation;

/// Contains `Asset` and `AssetProfile` metadata
pub mod asset;

/// Contains `Buffer`, `BufferView`, and other related data structures
pub mod buffer;

/// Contains `Camera` and other related data structures
pub mod camera;

/// Contains the names of 1.0 extensions enabled and supported by the library
pub mod extensions;

/// Contains convenience implementations of the `Extra` trait
pub mod extras;

/// Contains `Image` and other related data structures
pub mod image;

/// Contains `Material` and other related data structures
pub mod material;

/// Contains `Mesh` and other related data structures
pub mod mesh;

/// Contains `Program` and other related data structures
pub mod program;

/// Contains `Scene`, `Node`, and other related data structures
pub mod scene;

/// Contains `Shader`, and other related data structures
pub mod shader;

/// Contains `Skin` and other related data structures
pub mod skin;

/// Contains `Technique` and other related data structures
pub mod technique;

/// Contains `Texture`, `Sampler`, and other related data structures
pub mod texture;

/// Trait for (de)serializing user data in glTF 1.0 assets
pub use self::extras::Extras;

/// Error encountered when loading a glTF asset
#[derive(Debug)]
pub enum ImportError {
    /// Failure when deserializing a .gltf metadata file
    Deserialize(serde_json::error::Error),

    /// A glTF extension required by the asset has not been enabled by the user
    ExtensionDisabled(String),

    /// A glTF extension required by the asset is not supported by the library
    ExtensionUnsupported(String),

    /// The .gltf data is invalid
    Invalid(String),

    /// Standard input / output error
    Io(std::io::Error),

    /// The glTF version of the asset is incompatible with this function
    IncompatibleVersion(String),
}

/// The root object for a glTF 1.0 asset
#[derive(Debug, Deserialize, Serialize)]
pub struct Root<E: Extras> {
    /// A dictionary object of accessor objects.
    ///
    /// The name of each accessor is an ID in the global glTF namespace that is
    /// used to reference the accessor. An accessor is a typed view into a
    /// bufferView.
    #[serde(default)]
    pub accessors: HashMap<String, accessor::Accessor<E>>,

    /// A dictionary object of keyframe animation objects.
    ///
    /// The name of each animation is an ID in the global glTF namespace that is
    /// used to reference the animation.
    #[serde(default)]
    pub animation: HashMap<String, animation::Animation<E>>,

    /// Metadata about the glTF asset.
    pub asset: asset::Asset<E>,

    /// A dictionary object of buffer objects.
    ///
    /// The name of each buffer is an ID in the global glTF namespace that is
    /// used to reference the buffer. A buffer points to binary geometry,
    /// animation, or skins.
    #[serde(default)]
    pub buffers: HashMap<String, buffer::Buffer<E>>,

    /// A dictionary object of bufferView objects.
    ///
    /// The name of each bufferView is an ID in the global glTF namespace that
    /// is used to reference the bufferView. A bufferView is a view into a
    /// buffer generally representing a subset of the buffer.
    #[serde(rename = "bufferViews")]
    #[serde(default)]
    pub buffer_views: HashMap<String, buffer::BufferView<E>>,

    /// A dictionary object of camera objects.
    ///
    /// The name of each camera is an ID in the global glTF namespace that is
    /// used to reference the camera. A camera defines a projection matrix.
    #[serde(default)]
    pub cameras: HashMap<String, camera::Camera<E>>,

    /// Names of glTF extensions used somewhere in this asset.
    #[serde(rename = "extensionsUsed")]
    #[serde(default)]
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    #[serde(rename = "extensionsRequired")]
    #[serde(default)]
    pub extensions_required: Vec<String>,

    /// A dictionary object of image objects.
    ///
    /// The name of each image is an ID in the global glTF namespace that is
    /// used to reference the image. An image defines data used to create a
    /// texture.
    #[serde(default)]
    pub images: HashMap<String, image::Image<E>>,

    /// A dictionary object of material objects.
    ///
    /// The name of each material is an ID in the global glTF namespace that is
    /// used to reference the material. A material defines the appearance of a
    /// primitive.
    #[serde(default)]
    pub materials: HashMap<String, material::Material<E>>,

    /// A dictionary object of mesh objects.
    ///
    /// The name of each mesh is an ID in the global glTF namespace that is used
    /// to reference the mesh. A mesh is a set of primitives to be rendered.
    #[serde(default)]
    pub meshes: HashMap<String, mesh::Mesh<E>>,

    /// A dictionary object of node objects in the node hierarchy.
    ///
    /// The name of each node is an ID in the global glTF namespace that is used
    /// to reference the node.
    #[serde(default)]
    pub nodes: HashMap<String, scene::Node<E>>,

    /// A dictionary object of shader program objects.
    ///
    /// The name of each program is an ID in the global glTF namespace that is
    /// used to reference the program.
    #[serde(default)]
    pub programs: HashMap<String, program::Program<E>>,

    /// A dictionary object of sampler objects.
    ///
    /// The name of each sampler is an ID in the global glTF namespace that is
    /// used to reference the sampler. A sampler contains properties for texture
    /// filtering and wrapping modes.
    #[serde(default)]
    pub samplers: HashMap<String, texture::Sampler<E>>,

    /// The ID of the default scene.
    pub scene: Option<String>,

    /// A dictionary object of scene objects.
    ///
    /// The name of each scene is an ID in the global glTF namespace that is
    /// used to reference the scene.
    #[serde(default)]
    pub scenes: HashMap<String, scene::Scene<E>>,

    /// A dictionary object of shader objects.
    ///
    /// The name of each shader is an ID in the global glTF namespace that is
    /// used to reference the shader.
    #[serde(default)]
    pub shaders: HashMap<String, shader::Shader<E>>,

    /// A dictionary object of skin objects.
    ///
    /// The name of each skin is an ID in the global glTF namespace that is used
    /// to reference the skin. A skin is defined by joints and matrices.
    #[serde(default)]
    pub skins: HashMap<String, skin::Skin<E>>,

    /// A dictionary object of technique objects.
    ///
    /// The name of each technique is an ID in the global glTF namespace that is
    /// used to reference the technique. A technique is a template for a
    /// material appearance.
    #[serde(default)]
    pub techniques: HashMap<String, technique::Technique<E>>,

    /// A dictionary object of texture objects.
    ///
    /// The name of each texture is an ID in the global glTF namespace that is
    /// used to reference the texture.
    #[serde(default)]
    pub textures: HashMap<String, texture::Texture<E>>,

    /// Extension specific data
    #[serde(default)]
    pub extensions: RootExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Root,
}

/// Extension specific data for `Root`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RootExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Imports a glTF 1.0 asset
pub fn import<P, E>(path: P) -> Result<Root<E>, ImportError>
    where P: AsRef<std::path::Path>,
          E: Extras
{
    use self::ImportError::*;
    use std::io::Read;
    let mut file = std::fs::File::open(path).map_err(Io)?;
    let mut json = String::new();
    file.read_to_string(&mut json).map_err(Io)?;
    serde_json::from_str(&json).map_err(Deserialize)
}

