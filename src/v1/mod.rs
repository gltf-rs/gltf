// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_json;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

pub mod accessor;
pub mod animation;
pub mod asset;
pub mod buffer;
pub mod camera;
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

#[derive(Debug)]
pub enum Error {
    /// Standard input / output error
    Io(io::Error),
    /// Failure when parsing a .gltf metadata file
    Parse(serde_json::error::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::Parse(err)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Gltf {
    /// A dictionary object of accessor objects.
    ///
    /// The name of each accessor is an ID in the global glTF namespace that is
    /// used to reference the accessor. An accessor is a typed view into a
    /// bufferView.
    #[serde(default)]
    pub accessors: HashMap<String, accessor::Accessor>,

    /// A dictionary object of keyframe animation objects.
    ///
    /// The name of each animation is an ID in the global glTF namespace that is
    /// used to reference the animation.
    #[serde(default)]
    pub animation: HashMap<String, animation::Animation>,

    /// Metadata about the glTF asset.
    #[serde(default)]
    pub asset: asset::Asset,

    /// A dictionary object of buffer objects.
    ///
    /// The name of each buffer is an ID in the global glTF namespace that is
    /// used to reference the buffer. A buffer points to binary geometry,
    /// animation, or skins.
    #[serde(default)]
    pub buffers: HashMap<String, buffer::Buffer>,

    /// A dictionary object of bufferView objects.
    ///
    /// The name of each bufferView is an ID in the global glTF namespace that
    /// is used to reference the bufferView. A bufferView is a view into a
    /// buffer generally representing a subset of the buffer.
    #[serde(rename = "bufferViews")]
    #[serde(default)]
    pub buffer_views: HashMap<String, buffer::BufferView>,

    /// A dictionary object of camera objects.
    ///
    /// The name of each camera is an ID in the global glTF namespace that is
    /// used to reference the camera. A camera defines a projection matrix.
    #[serde(default)]
    pub cameras: HashMap<String, camera::Camera>,

    /// A dictionary object of image objects.
    ///
    /// The name of each image is an ID in the global glTF namespace that is
    /// used to reference the image. An image defines data used to create a
    /// texture.
    #[serde(default)]
    pub images: HashMap<String, image::Image>,

    /// A dictionary object of material objects.
    ///
    /// The name of each material is an ID in the global glTF namespace that is
    /// used to reference the material. A material defines the appearance of a
    /// primitive.
    #[serde(default)]
    pub materials: HashMap<String, material::Material>,

    /// A dictionary object of mesh objects.
    ///
    /// The name of each mesh is an ID in the global glTF namespace that is used
    /// to reference the mesh. A mesh is a set of primitives to be rendered.
    #[serde(default)]
    pub meshes: HashMap<String, mesh::Mesh>,

    /// A dictionary object of node objects in the node hierarchy.
    ///
    /// The name of each node is an ID in the global glTF namespace that is used
    /// to reference the node.
    #[serde(default)]
    pub nodes: HashMap<String, node::Node>,

    /// A dictionary object of shader program objects.
    ///
    /// The name of each program is an ID in the global glTF namespace that is
    /// used to reference the program.
    #[serde(default)]
    pub programs: HashMap<String, program::Program>,

    /// A dictionary object of sampler objects.
    ///
    /// The name of each sampler is an ID in the global glTF namespace that is
    /// used to reference the sampler. A sampler contains properties for texture
    /// filtering and wrapping modes.
    #[serde(default)]
    pub samplers: HashMap<String, sampler::Sampler>,

    /// The ID of the default scene.
    pub scene: Option<String>,

    /// A dictionary object of scene objects.
    ///
    /// The name of each scene is an ID in the global glTF namespace that is
    /// used to reference the scene.
    #[serde(default)]
    pub scenes: HashMap<String, scene::Scene>,

    /// A dictionary object of shader objects.
    ///
    /// The name of each shader is an ID in the global glTF namespace that is
    /// used to reference the shader.
    #[serde(default)]
    pub shaders: HashMap<String, shader::Shader>,

    /// A dictionary object of skin objects.
    ///
    /// The name of each skin is an ID in the global glTF namespace that is used
    /// to reference the skin. A skin is defined by joints and matrices.
    #[serde(default)]
    pub skins: HashMap<String, skin::Skin>,

    /// A dictionary object of technique objects.
    ///
    /// The name of each technique is an ID in the global glTF namespace that is
    /// used to reference the technique. A technique is a template for a
    /// material appearance.
    #[serde(default)]
    pub techniques: HashMap<String, technique::Technique>,

    /// A dictionary object of texture objects.
    ///
    /// The name of each texture is an ID in the global glTF namespace that is
    /// used to reference the texture.
    #[serde(default)]
    pub textures: HashMap<String, texture::Texture>, 

    // TODO: extension
    // TODO: extras
}

impl Gltf {
    fn open_impl(path: &Path) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;
        serde_json::from_str(&json).map_err(|err| Error::Parse(err))
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Gltf::open_impl(path.as_ref())
    }
}
