// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

use traits::Extras;
use ImportError;

pub mod accessor;
pub mod animation;
pub mod asset;
pub mod buffer;
pub mod camera;
pub mod extensions;
pub mod image;
pub mod material;
pub mod mesh;
pub mod node;
pub mod program;
pub mod sampler;
pub mod scene;
pub mod shader;
pub mod skin;
pub mod technique;
pub mod texture;

pub use self::extensions::Extensions;

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
    pub nodes: HashMap<String, node::Node<E>>,

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
    pub samplers: HashMap<String, sampler::Sampler<E>>,

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

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: Extensions,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Root,
}

impl<E: Extras> Root<E> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ImportError> {
        let mut file = File::open(path).map_err(ImportError::Io)?;
        let mut json = String::new();
        file.read_to_string(&mut json).map_err(ImportError::Io)?;

        serde_json::from_str(&json).map_err(|cause| ImportError::Deserialize(cause))
    }

    pub fn import_from_str(json: &str) -> Result<Self, ImportError> {
        serde_json::from_str(&json).map_err(|cause| ImportError::Deserialize(cause))
    }
}
