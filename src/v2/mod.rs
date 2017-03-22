
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde;
use serde_json;
use std;
use ImportError;

pub mod accessor;
pub mod animation;
pub mod buffer;
pub mod camera;
pub mod material;
pub mod mesh;
pub mod scene;
pub mod skin;
pub mod texture;

/// Index into an array owned by the root glTF object
#[derive(Clone, Copy, Debug)]
pub struct Index<T>(u32, std::marker::PhantomData<T>);

/// Generic untyped JSON object
pub type UntypedJsonObject = std::collections::HashMap<String, serde_json::Value>;

/// `extensions` field type
pub type Extensions = Option<UntypedJsonObject>;

/// `extras` field type
pub type Extras = Option<UntypedJsonObject>;

/// [Contains metadata about the glTF asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#asset)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator
    pub copyright: Option<String>,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Tool that generated this glTF model
    pub generator: Option<String>,
    /// glTF version
    #[serde(default = "asset_version_default")]
    pub version: String,
}

fn asset_version_default() -> String {
    "2.0".to_string()
}

/// [The root object for a glTF asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#gltf)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
    #[serde(default)]
    accessors: Vec<accessor::Accessor>,
    #[serde(default)]
    animations: Vec<animation::Animation>,
    asset: Asset,
    #[serde(default)]
    buffers: Vec<buffer::Buffer>,
    #[serde(default, rename = "bufferViews")]
    buffer_views: Vec<buffer::View>,
    #[serde(default, rename = "extensionsUsed")]
    extensions_used: Vec<String>,
    #[serde(default, rename = "extensionsRequired")]
    extensions_required: Vec<String>,
    #[serde(default)]
    cameras: Vec<camera::Camera>,
    #[serde(default)]
    images: Vec<texture::Image>,
    #[serde(default)]
    materials: Vec<material::Material>,
    #[serde(default)]
    meshes: Vec<mesh::Mesh>,
    #[serde(default)]
    nodes: Vec<scene::Node>,
    #[serde(default)]
    samplers: Vec<texture::Sampler>,
    #[serde(default = "root_scene_default")]
    scene: Index<scene::Scene>,
    #[serde(default)]
    scenes: Vec<scene::Scene>,
    #[serde(default)]
    skins: Vec<skin::Skin>,
    #[serde(default)]
    textures: Vec<texture::Texture>,
}

fn root_scene_default() -> Index<scene::Scene> {
    Index(0, std::marker::PhantomData)
}

impl Root {
    /// Loads a glTF version 2.0 asset from raw JSON
    pub fn import_from_str(json: &str) -> Result<Self, ImportError> {
        let root: Root = serde_json::from_str(json)
            .map_err(|err| ImportError::Deserialize(err))?;
        if root.indices_are_valid() {
            Ok(root)
        } else {
            Err(ImportError::Invalid("index out of range".to_string()))
        }
    }

    /// Returns the accessor at the given index
    pub fn accessor(&self, index: Index<accessor::Accessor>) -> &accessor::Accessor {
        &self.accessors[index.0 as usize]
    }

    /// Returns all accessors as a slice
    pub fn accessors(&self) -> &[accessor::Accessor] {
        &self.accessors
    }
    
    /// Returns the metadata included with this asset
    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    /// Returns the buffer at the given index
    pub fn buffer(&self, index: Index<buffer::Buffer>) -> &buffer::Buffer {
        &self.buffers[index.0 as usize]
    }

    /// Returns all buffers as a slice
    pub fn buffers(&self) -> &[buffer::Buffer] {
        &self.buffers
    }
    
    /// Returns the buffer view at the given index
    pub fn buffer_view(&self, index: Index<buffer::View>) -> &buffer::View {
        &self.buffer_views[index.0 as usize]
    }

    /// Returns all buffer views as a slice
    pub fn buffer_views(&self) -> &[buffer::View] {
        &self.buffer_views
    }

    /// Returns the camera at the given index
    pub fn camera(&self, index: Index<camera::Camera>) -> &camera::Camera {
        &self.cameras[index.0 as usize]
    }

    /// Returns all cameras as a slice
    pub fn cameras(&self) -> &[camera::Camera] {
        &self.cameras
    }

    /// Returns the extensions referenced in this .gltf file
    pub fn extensions_used(&self) -> &[String] {
        &self.extensions_used
    }

    /// Returns the extensions required to load and render this asset
    pub fn extensions_required(&self) -> &[String] {
        &self.extensions_required
    }

    /// Returns the image at the given index
    pub fn image(&self, index: Index<texture::Image>) -> &texture::Image {
        &self.images[index.0 as usize]
    }

    /// Returns all images as a slice
    pub fn images(&self) -> &[texture::Image] {
        &self.images
    }

    /// Returns the material at the given index
    pub fn material(&self, index: Index<material::Material>) -> &material::Material {
        &self.materials[index.0 as usize]
    }

    /// Returns all materials as a slice
    pub fn materials(&self) -> &[material::Material] {
        &self.materials
    }

    /// Returns the mesh at the given index
    pub fn mesh(&self, index: Index<mesh::Mesh>) -> &mesh::Mesh {
        &self.meshes[index.0 as usize]
    }

    /// Returns all meshes as a slice
    pub fn meshes(&self) -> &[mesh::Mesh] {
        &self.meshes
    }
    
    /// Returns the node at the given index
    pub fn node(&self, index: Index<scene::Node>) -> &scene::Node {
        &self.nodes[index.0 as usize]
    }

    /// Returns all nodes as a slice
    pub fn nodes(&self) -> &[scene::Node] {
        &self.nodes
    }

    /// Returns the sampler at the given index
    pub fn sampler(&self, index: Index<texture::Sampler>) -> &texture::Sampler {
        &self.samplers[index.0 as usize]
    }

    /// Returns all samplers as a slice
    pub fn samplers(&self) -> &[texture::Sampler] {
        &self.samplers
    }
    
    /// Returns the scene at the given index
    pub fn scene(&self, index: Index<scene::Scene>) -> &scene::Scene {
        &self.scenes[index.0 as usize]
    }

    /// Returns all scenes as a slice
    pub fn scenes(&self) -> &[scene::Scene] {
        &self.scenes
    }

    /// Returns the skin at the given index
    pub fn skin(&self, index: Index<skin::Skin>) -> &skin::Skin {
        &self.skins[index.0 as usize]
    }

    /// Returns all skins as a slice
    pub fn skins(&self) -> &[skin::Skin] {
        &self.skins
    }

    /// Returns the texture at the given index
    pub fn texture(&self, index: Index<texture::Texture>) -> &texture::Texture {
        &self.textures[index.0 as usize]
    }

    /// Returns all textures as a slice
    pub fn textures(&self) -> &[texture::Texture] {
        &self.textures
    }

    /// Performs a search for any indices that are out of range of the array
    /// they reference. Returns true if all indices are within range.
    fn indices_are_valid(&self) -> bool {
        // TODO: Implement me
        true
    }
}

impl<T> Index<T> {
    fn new(value: u32) -> Self {
        Index(value, std::marker::PhantomData)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl<T> serde::Serialize for Index<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer
    {
        serializer.serialize_u64(self.value() as u64)
    }
}

impl<T> serde::Deserialize for Index<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        struct Visitor<T>(std::marker::PhantomData<T>);
        impl<T> serde::de::Visitor for Visitor<T> {
            type Value = Index<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter)
                         -> std::fmt::Result
            {
                formatter.write_str("GLenum")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where E: serde::de::Error
            {
                Ok(Index::new(value as u32))
            }
        }
        deserializer.deserialize_u64(Visitor::<T>(std::marker::PhantomData))
    }
}
