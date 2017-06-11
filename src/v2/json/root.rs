
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use std::fmt;
use v2::json::*;

/// Helper trait for retrieving top-level objects by a universal identifier.
pub trait Get<T> {
    /// Retrieves a single value at the given index.
    fn get(&self, id: &Index<T>) -> &T;
}

/// Helper trait for attempting to retrieve top-level objects by a universal
/// identifier.
pub trait TryGet<T> {
    /// Attempts to retrieve a single value at the given index.
    fn try_get(&self, id: &Index<T>) -> Result<&T, ()>;
}

/// Represents an offset into an array of type `T` owned by the root glTF object.
#[derive(Clone, Copy)]
pub struct Index<T>(u32, std::marker::PhantomData<T>);

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
    /// An array of accessors.
    #[serde(default)]
    pub accessors: Vec<accessor::Accessor>,
    
    /// An array of keyframe animations.
    #[serde(default)]
    pub animations: Vec<animation::Animation>,

    /// Metadata about the glTF asset.
    pub asset: asset::Asset,
    
    /// An array of buffers.
    #[serde(default)]
    pub buffers: Vec<buffer::Buffer>,
    
    /// An array of buffer views.
    #[serde(default, rename = "bufferViews")]
    pub buffer_views: Vec<buffer::View>,

    /// The default scene.
    #[serde(rename = "scene")]
    pub default_scene: Option<Index<scene::Scene>>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: RootExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// Names of glTF extensions used somewhere in this asset.
    #[serde(default, rename = "extensionsUsed")]
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    #[serde(default, rename = "extensionsRequired")]
    pub extensions_required: Vec<String>,
    
    /// An array of cameras.
    #[serde(default)]
    pub cameras: Vec<camera::Camera>,
    
    /// An array of images.
    #[serde(default)]
    pub images: Vec<image::Image>,
    
    /// An array of materials.
    #[serde(default)]
    pub materials: Vec<material::Material>,
    
    /// An array of meshes.
    #[serde(default)]
    pub meshes: Vec<mesh::Mesh>,
    
    /// An array of nodes.
    #[serde(default)]
    pub nodes: Vec<scene::Node>,
    
    /// An array of samplers.
    #[serde(default)]
    pub samplers: Vec<texture::Sampler>,
    
    /// An array of scenes.
    #[serde(default)]
    pub scenes: Vec<scene::Scene>,
    
    /// An array of skins.
    #[serde(default)]
    pub skins: Vec<skin::Skin>,
    
    /// An array of textures.
    #[serde(default)]
    pub textures: Vec<texture::Texture>,
}

/// Extension specific data for `Root`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RootExtensions {
    _allow_unknown_fields: (),
}

impl Root {
    /// Returns the accessor at the given index.
    pub fn accessor(&self, index: Index<accessor::Accessor>) -> &accessor::Accessor {
        &self.accessors[index.0 as usize]
    }

    /// Returns all accessors as a slice.
    pub fn accessors(&self) -> &[accessor::Accessor] {
        &self.accessors
    }

    /// Returns the animation at the given index.
    pub fn animation(&self, index: Index<animation::Animation>) -> &animation::Animation {
        &self.animations[index.0 as usize]
    }

    /// Returns all animations as a slice.
    pub fn animations(&self) -> &[animation::Animation] {
        &self.animations
    }

    /// Returns the metadata included with this asset.
    pub fn asset(&self) -> &asset::Asset {
        &self.asset
    }

    /// Returns the buffer at the given index.
    pub fn buffer(&self, index: Index<buffer::Buffer>) -> &buffer::Buffer {
        &self.buffers[index.0 as usize]
    }

    /// Returns all buffers as a slice.
    pub fn buffers(&self) -> &[buffer::Buffer] {
        &self.buffers
    }

    /// Returns the buffer view at the given index.
    pub fn buffer_view(&self, index: Index<buffer::View>) -> &buffer::View {
        &self.buffer_views[index.0 as usize]
    }

    /// Returns all buffer views as a slice.
    pub fn buffer_views(&self) -> &[buffer::View] {
        &self.buffer_views
    }

    /// Returns the camera at the given index.
    pub fn camera(&self, index: Index<camera::Camera>) -> &camera::Camera {
        &self.cameras[index.0 as usize]
    }

    /// Returns all cameras as a slice.
    pub fn cameras(&self) -> &[camera::Camera] {
        &self.cameras
    }

    /// Returns the default scene.
    pub fn default_scene(&self) -> Option<&scene::Scene> {
        self.default_scene.as_ref().map(|s| self.get(s))
    }

    /// Returns the extensions referenced in this .gltf file.
    pub fn extensions_used(&self) -> &[String] {
        &self.extensions_used
    }

    /// Returns the extensions required to load and render this asset.
    pub fn extensions_required(&self) -> &[String] {
        &self.extensions_required
    }

    /// Returns a single item from the root object.
    pub fn get<T>(&self, index: &Index<T>) -> &T
        where Self: Get<T>
    {
        (self as &Get<T>).get(index)
    }

    /// Returns a single item from the root object if the index is in range.
    // N.B. this is hidden from the docs because it's only necessary for
    // validation during `import()`.
    #[doc(hidden)]
    pub fn try_get<T>(&self, index: &Index<T>) -> Result<&T, ()>
        where Self: TryGet<T>
    {
        (self as &TryGet<T>).try_get(index)
    }

    /// Returns the image at the given index.
    pub fn image(&self, index: Index<image::Image>) -> &image::Image {
        &self.images[index.0 as usize]
    }

    /// Returns all images as a slice.
    pub fn images(&self) -> &[image::Image] {
        &self.images
    }

    /// Returns the material at the given index.
    pub fn material(&self, index: Index<material::Material>) -> &material::Material {
        &self.materials[index.0 as usize]
    }

    /// Returns all materials as a slice.
    pub fn materials(&self) -> &[material::Material] {
        &self.materials
    }

    /// Returns the mesh at the given index.
    pub fn mesh(&self, index: Index<mesh::Mesh>) -> &mesh::Mesh {
        &self.meshes[index.0 as usize]
    }

    /// Returns all meshes as a slice.
    pub fn meshes(&self) -> &[mesh::Mesh] {
        &self.meshes
    }

    /// Returns the node at the given index.
    pub fn node(&self, index: Index<scene::Node>) -> &scene::Node {
        &self.nodes[index.0 as usize]
    }

    /// Returns all nodes as a slice.
    pub fn nodes(&self) -> &[scene::Node] {
        &self.nodes
    }

    /// Returns the sampler at the given index.
    pub fn sampler(&self, index: Index<texture::Sampler>) -> &texture::Sampler {
        &self.samplers[index.0 as usize]
    }

    /// Returns all samplers as a slice.
    pub fn samplers(&self) -> &[texture::Sampler] {
        &self.samplers
    }

    /// Returns the scene at the given index.
    pub fn scene(&self, index: Index<scene::Scene>) -> &scene::Scene {
        &self.scenes[index.0 as usize]
    }

    /// Returns all scenes as a slice.
    pub fn scenes(&self) -> &[scene::Scene] {
        &self.scenes
    }

    /// Returns the skin at the given index.
    pub fn skin(&self, index: Index<skin::Skin>) -> &skin::Skin {
        &self.skins[index.0 as usize]
    }

    /// Returns all skins as a slice.
    pub fn skins(&self) -> &[skin::Skin] {
        &self.skins
    }

    /// Returns the texture at the given index.
    pub fn texture(&self, index: Index<texture::Texture>) -> &texture::Texture {
        &self.textures[index.0 as usize]
    }

    /// Returns all textures as a slice.
    pub fn textures(&self) -> &[texture::Texture] {
        &self.textures
    }
}

impl<T> Index<T> {
    /// Creates a new `Index` representing an offset into an array containing `T`.
    fn new(value: u32) -> Self {
        Index(value, std::marker::PhantomData)
    }

    /// Returns the internal offset value.
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

impl<'de, T> serde::Deserialize<'de> for Index<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        struct Visitor<T>(std::marker::PhantomData<T>);
        impl<'de, T> serde::de::Visitor<'de> for Visitor<T> {
            type Value = Index<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("index into child of root")
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

impl<T> fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> fmt::Display for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_get {
    ($ty:ty, $field:ident) => {
        impl<'a> Get<$ty> for Root {
            fn get(&self, index: &Index<$ty>) -> &$ty {
                &self.$field[index.value() as usize]
            }
        }
    }
}

macro_rules! impl_try_get {
    ($ty:ty, $field:ident) => {
        #[doc(hidden)]
        impl<'a> TryGet<$ty> for Root {
            fn try_get(&self, index: &Index<$ty>) -> Result<&$ty, ()> {
                self.$field.get(index.value() as usize).ok_or(())
            }
        }
    }
}

impl_get!(accessor::Accessor, accessors);
impl_get!(animation::Animation, animations);
impl_get!(buffer::Buffer, buffers);
impl_get!(buffer::View, buffer_views);
impl_get!(camera::Camera, cameras);
impl_get!(image::Image, images);
impl_get!(material::Material, materials);
impl_get!(mesh::Mesh, meshes);
impl_get!(scene::Node, nodes);
impl_get!(texture::Sampler, samplers);
impl_get!(scene::Scene, scenes);
impl_get!(skin::Skin, skins);
impl_get!(texture::Texture, textures);

impl_try_get!(accessor::Accessor, accessors);
impl_try_get!(animation::Animation, animations);
impl_try_get!(buffer::Buffer, buffers);
impl_try_get!(buffer::View, buffer_views);
impl_try_get!(camera::Camera, cameras);
impl_try_get!(image::Image, images);
impl_try_get!(material::Material, materials);
impl_try_get!(mesh::Mesh, meshes);
impl_try_get!(scene::Node, nodes);
impl_try_get!(texture::Sampler, samplers);
impl_try_get!(scene::Scene, scenes);
impl_try_get!(skin::Skin, skins);
impl_try_get!(texture::Texture, textures);
