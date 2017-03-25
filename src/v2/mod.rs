
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

use traits::{Extensions, Extras, Get};
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

/// [Contains metadata about the glTF asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#asset)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Asset<E: Extensions, X: Extras> {
    /// A copyright message suitable for display to credit the content creator
    pub copyright: Option<String>,
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: <E as Extensions>::Asset,
    /// Optional application specific data
    #[serde(default)]
    pub extras: <X as Extras>::Asset,
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
pub struct Root<E: Extensions, X: Extras> {
    #[serde(default)]
    accessors: Vec<accessor::Accessor<E, X>>,
    #[serde(default)]
    animations: Vec<animation::Animation<E, X>>,
    asset: Asset<E, X>,
    #[serde(default)]
    buffers: Vec<buffer::Buffer<E, X>>,
    #[serde(default, rename = "bufferViews")]
    buffer_views: Vec<buffer::BufferView<E, X>>,
    #[serde(default, rename = "extensionsUsed")]
    extensions_used: Vec<String>,
    #[serde(default, rename = "extensionsRequired")]
    extensions_required: Vec<String>,
    #[serde(default)]
    cameras: Vec<camera::Camera<E, X>>,
    #[serde(default)]
    images: Vec<texture::Image<E, X>>,
    #[serde(default)]
    materials: Vec<material::Material<E, X>>,
    #[serde(default)]
    meshes: Vec<mesh::Mesh<E, X>>,
    #[serde(default)]
    nodes: Vec<scene::Node<E, X>>,
    #[serde(default)]
    samplers: Vec<texture::Sampler<E, X>>,
    #[serde(default = "root_scene_default")]
    scene: Index<scene::Scene<E, X>>,
    #[serde(default)]
    scenes: Vec<scene::Scene<E, X>>,
    #[serde(default)]
    skins: Vec<skin::Skin<E, X>>,
    #[serde(default)]
    textures: Vec<texture::Texture<E, X>>,
}

fn root_scene_default<E, X>() -> Index<scene::Scene<E, X>>
    where E: Extensions, X: Extras
{
    Index(0, std::marker::PhantomData)
}

impl<E: Extensions, X: Extras> Root<E, X> {
    /// Loads a glTF version 2.0 asset from raw JSON
    pub fn import_from_str(json: &str) -> Result<Self, ImportError> {
        let root: Root<E, X> = serde_json::from_str(json)
            .map_err(|err| ImportError::Deserialize(err))?;
        if root.indices_are_valid() {
            Ok(root)
        } else {
            Err(ImportError::Invalid("index out of range".to_string()))
        }
    }

    /// Returns the accessor at the given index
    pub fn accessor(&self, index: Index<accessor::Accessor<E, X>>) -> &accessor::Accessor<E, X> {
        &self.accessors[index.0 as usize]
    }

    /// Returns all accessors as a slice
    pub fn accessors(&self) -> &[accessor::Accessor<E, X>] {
        &self.accessors
    }

    /// Returns the animation at the given index
    pub fn animation(&self, index: Index<animation::Animation<E, X>>) -> &animation::Animation<E, X> {
        &self.animations[index.0 as usize]
    }

    /// Returns all animations as a slice
    pub fn animations(&self) -> &[animation::Animation<E, X>] {
        &self.animations
    }

    /// Returns the metadata included with this asset
    pub fn asset(&self) -> &Asset<E, X> {
        &self.asset
    }

    /// Returns the buffer at the given index
    pub fn buffer(&self, index: Index<buffer::Buffer<E, X>>) -> &buffer::Buffer<E, X> {
        &self.buffers[index.0 as usize]
    }

    /// Returns all buffers as a slice
    pub fn buffers(&self) -> &[buffer::Buffer<E, X>] {
        &self.buffers
    }
    
    /// Returns the buffer view at the given index
    pub fn buffer_view(&self, index: Index<buffer::BufferView<E, X>>) -> &buffer::BufferView<E, X> {
        &self.buffer_views[index.0 as usize]
    }

    /// Returns all buffer views as a slice
    pub fn buffer_views(&self) -> &[buffer::BufferView<E, X>] {
        &self.buffer_views
    }

    /// Returns the camera at the given index
    pub fn camera(&self, index: Index<camera::Camera<E, X>>) -> &camera::Camera<E, X> {
        &self.cameras[index.0 as usize]
    }

    /// Returns all cameras as a slice
    pub fn cameras(&self) -> &[camera::Camera<E, X>] {
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

    /// Returns a single item from the root object
    pub fn get<T>(&self, index: Index<T>) -> &T
        where Self: Get<T, Id=Index<T>>
    {
        (self as &Get<T, Id=Index<T>>).get(index)
    }

    /// Returns the image at the given index
    pub fn image(&self, index: Index<texture::Image<E, X>>) -> &texture::Image<E, X> {
        &self.images[index.0 as usize]
    }

    /// Returns all images as a slice
    pub fn images(&self) -> &[texture::Image<E, X>] {
        &self.images
    }

    /// Returns the material at the given index
    pub fn material(&self, index: Index<material::Material<E, X>>) -> &material::Material<E, X> {
        &self.materials[index.0 as usize]
    }

    /// Returns all materials as a slice
    pub fn materials(&self) -> &[material::Material<E, X>] {
        &self.materials
    }

    /// Returns the mesh at the given index
    pub fn mesh(&self, index: Index<mesh::Mesh<E, X>>) -> &mesh::Mesh<E, X> {
        &self.meshes[index.0 as usize]
    }

    /// Returns all meshes as a slice
    pub fn meshes(&self) -> &[mesh::Mesh<E, X>] {
        &self.meshes
    }
    
    /// Returns the node at the given index
    pub fn node(&self, index: Index<scene::Node<E, X>>) -> &scene::Node<E, X> {
        &self.nodes[index.0 as usize]
    }

    /// Returns all nodes as a slice
    pub fn nodes(&self) -> &[scene::Node<E, X>] {
        &self.nodes
    }

    /// Returns the sampler at the given index
    pub fn sampler(&self, index: Index<texture::Sampler<E, X>>) -> &texture::Sampler<E, X> {
        &self.samplers[index.0 as usize]
    }

    /// Returns all samplers as a slice
    pub fn samplers(&self) -> &[texture::Sampler<E, X>] {
        &self.samplers
    }
    
    /// Returns the scene at the given index
    pub fn scene(&self, index: Index<scene::Scene<E, X>>) -> &scene::Scene<E, X> {
        &self.scenes[index.0 as usize]
    }

    /// Returns all scenes as a slice
    pub fn scenes(&self) -> &[scene::Scene<E, X>] {
        &self.scenes
    }

    /// Returns the skin at the given index
    pub fn skin(&self, index: Index<skin::Skin<E, X>>) -> &skin::Skin<E, X> {
        &self.skins[index.0 as usize]
    }

    /// Returns all skins as a slice
    pub fn skins(&self) -> &[skin::Skin<E, X>] {
        &self.skins
    }

    /// Returns the texture at the given index
    pub fn texture(&self, index: Index<texture::Texture<E, X>>) -> &texture::Texture<E, X> {
        &self.textures[index.0 as usize]
    }

    /// Returns all textures as a slice
    pub fn textures(&self) -> &[texture::Texture<E, X>] {
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

macro_rules! impl_get {
    ($ty:ty, $field:ident) => {
        impl<'a, E, X> Get<$ty> for Root<E, X>
            where E: Extensions, X: Extras
        {
            type Id = Index<$ty>;
            fn get(&self, id: Self::Id) -> &$ty {
                &self.$field[id.value() as usize]
            }
        }
    }
}

impl_get!(accessor::Accessor<E, X>, accessors);
impl_get!(animation::Animation<E, X>, animations);
impl_get!(buffer::Buffer<E, X>, buffers);
impl_get!(buffer::BufferView<E, X>, buffer_views);
impl_get!(camera::Camera<E, X>, cameras);
impl_get!(texture::Image<E, X>, images);
impl_get!(material::Material<E, X>, materials);
impl_get!(mesh::Mesh<E, X>, meshes);
impl_get!(scene::Node<E, X>, nodes);
impl_get!(scene::Scene<E, X>, scenes);
impl_get!(skin::Skin<E, X>, skins);
impl_get!(texture::Texture<E, X>, textures);

