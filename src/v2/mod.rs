
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

pub mod accessor;
pub mod animation;
pub mod buffer;
pub mod extensions;
pub mod extras;
pub mod camera;
pub mod image;
pub mod material;
pub mod mesh;
pub mod scene;
pub mod skin;
pub mod texture;

pub use self::extras::Extras;

/// Helper trait for retrieving top-level objects by a universal identifier
pub trait Get<T> {
    /// Retrieves a single value at the given index
    fn get(&self, id: &Index<T>) -> &T;
}

/// Helper trait for retrieving top-level objects by a universal identifier
pub trait TryGet<T> {
    /// Attempts to retrieve a single value at the given index
    fn try_get(&self, id: &Index<T>) -> Result<&T, ()>;
}

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
    /// The asset glTF version is not supported by the library
    VersionUnsupported(String),
}

/// Index into an array owned by the root glTF object
#[derive(Clone, Copy, Debug)]
pub struct Index<T>(u32, std::marker::PhantomData<T>);

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AssetExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// [Contains metadata about the glTF asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#asset)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Asset<E: Extras> {
    /// A copyright message suitable for display to credit the content creator
    pub copyright: Option<String>,
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: AssetExtensions,
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Asset,
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
pub struct Root<E: Extras> {
    #[serde(default)]
    accessors: Vec<accessor::Accessor<E>>,
    #[serde(default)]
    animations: Vec<animation::Animation<E>>,
    asset: Asset<E>,
    #[serde(default)]
    buffers: Vec<buffer::Buffer<E>>,
    #[serde(default, rename = "bufferViews")]
    buffer_views: Vec<buffer::BufferView<E>>,
    #[serde(default, rename = "extensionsUsed")]
    extensions_used: Vec<String>,
    #[serde(default, rename = "extensionsRequired")]
    extensions_required: Vec<String>,
    #[serde(default)]
    cameras: Vec<camera::Camera<E>>,
    #[serde(default)]
    images: Vec<image::Image<E>>,
    #[serde(default)]
    materials: Vec<material::Material<E>>,
    #[serde(default)]
    meshes: Vec<mesh::Mesh<E>>,
    #[serde(default)]
    nodes: Vec<scene::Node<E>>,
    #[serde(default)]
    samplers: Vec<texture::Sampler<E>>,
    #[serde(default = "root_scene_default")]
    scene: Index<scene::Scene<E>>,
    #[serde(default)]
    scenes: Vec<scene::Scene<E>>,
    #[serde(default)]
    skins: Vec<skin::Skin<E>>,
    #[serde(default)]
    textures: Vec<texture::Texture<E>>,
}

fn root_scene_default<E>() -> Index<scene::Scene<E>>
    where E: Extras
{
    Index(0, std::marker::PhantomData)
}

pub fn import<P, E>(path: P) -> Result<Root<E>, ImportError>
    where P: AsRef<std::path::Path>, E: Extras
{
    use self::ImportError::*;
    use std::io::Read;
    let mut file = std::fs::File::open(path).map_err(Io)?;
    let mut json = String::new();
    file.read_to_string(&mut json).map_err(Io)?;
    let root: Root<E> = serde_json::from_str(&json).map_err(Deserialize)?;
    if root.range_check().is_ok() {
        Ok(root)
    } else {
        Err(Invalid("index out of range".to_string()))
    }
}

impl<E: Extras> Root<E> {
    /// Returns the accessor at the given index
    pub fn accessor(&self, index: Index<accessor::Accessor<E>>) -> &accessor::Accessor<E> {
        &self.accessors[index.0 as usize]
    }

    /// Returns all accessors as a slice
    pub fn accessors(&self) -> &[accessor::Accessor<E>] {
        &self.accessors
    }

    /// Returns the animation at the given index
    pub fn animation(&self, index: Index<animation::Animation<E>>) -> &animation::Animation<E> {
        &self.animations[index.0 as usize]
    }

    /// Returns all animations as a slice
    pub fn animations(&self) -> &[animation::Animation<E>] {
        &self.animations
    }

    /// Returns the metadata included with this asset
    pub fn asset(&self) -> &Asset<E> {
        &self.asset
    }

    /// Returns the buffer at the given index
    pub fn buffer(&self, index: Index<buffer::Buffer<E>>) -> &buffer::Buffer<E> {
        &self.buffers[index.0 as usize]
    }

    /// Returns all buffers as a slice
    pub fn buffers(&self) -> &[buffer::Buffer<E>] {
        &self.buffers
    }
    
    /// Returns the buffer view at the given index
    pub fn buffer_view(&self, index: Index<buffer::BufferView<E>>) -> &buffer::BufferView<E> {
        &self.buffer_views[index.0 as usize]
    }

    /// Returns all buffer views as a slice
    pub fn buffer_views(&self) -> &[buffer::BufferView<E>] {
        &self.buffer_views
    }

    /// Returns the camera at the given index
    pub fn camera(&self, index: Index<camera::Camera<E>>) -> &camera::Camera<E> {
        &self.cameras[index.0 as usize]
    }

    /// Returns all cameras as a slice
    pub fn cameras(&self) -> &[camera::Camera<E>] {
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
    pub fn get<T>(&self, index: &Index<T>) -> &T
        where Self: Get<T>
    {
        (self as &Get<T>).get(index)
    }

    /// Returns a single item from the root object if the index is in range
    pub fn try_get<T>(&self, index: &Index<T>) -> Result<&T, ()>
        where Self: TryGet<T>
    {
        (self as &TryGet<T>).try_get(index)
    }

    /// Returns the image at the given index
    pub fn image(&self, index: Index<image::Image<E>>) -> &image::Image<E> {
        &self.images[index.0 as usize]
    }

    /// Returns all images as a slice
    pub fn images(&self) -> &[image::Image<E>] {
        &self.images
    }

    /// Returns the material at the given index
    pub fn material(&self, index: Index<material::Material<E>>) -> &material::Material<E> {
        &self.materials[index.0 as usize]
    }

    /// Returns all materials as a slice
    pub fn materials(&self) -> &[material::Material<E>] {
        &self.materials
    }

    /// Returns the mesh at the given index
    pub fn mesh(&self, index: Index<mesh::Mesh<E>>) -> &mesh::Mesh<E> {
        &self.meshes[index.0 as usize]
    }

    /// Returns all meshes as a slice
    pub fn meshes(&self) -> &[mesh::Mesh<E>] {
        &self.meshes
    }
    
    /// Returns the node at the given index
    pub fn node(&self, index: Index<scene::Node<E>>) -> &scene::Node<E> {
        &self.nodes[index.0 as usize]
    }

    /// Returns all nodes as a slice
    pub fn nodes(&self) -> &[scene::Node<E>] {
        &self.nodes
    }

    /// Returns the sampler at the given index
    pub fn sampler(&self, index: Index<texture::Sampler<E>>) -> &texture::Sampler<E> {
        &self.samplers[index.0 as usize]
    }

    /// Returns all samplers as a slice
    pub fn samplers(&self) -> &[texture::Sampler<E>] {
        &self.samplers
    }
    
    /// Returns the scene at the given index
    pub fn scene(&self, index: Index<scene::Scene<E>>) -> &scene::Scene<E> {
        &self.scenes[index.0 as usize]
    }

    /// Returns all scenes as a slice
    pub fn scenes(&self) -> &[scene::Scene<E>] {
        &self.scenes
    }

    /// Returns the skin at the given index
    pub fn skin(&self, index: Index<skin::Skin<E>>) -> &skin::Skin<E> {
        &self.skins[index.0 as usize]
    }

    /// Returns all skins as a slice
    pub fn skins(&self) -> &[skin::Skin<E>] {
        &self.skins
    }

    /// Returns the texture at the given index
    pub fn texture(&self, index: Index<texture::Texture<E>>) -> &texture::Texture<E> {
        &self.textures[index.0 as usize]
    }

    /// Returns all textures as a slice
    pub fn textures(&self) -> &[texture::Texture<E>] {
        &self.textures
    }
    
    /// Performs a search for any indices that are out of range of the array
    /// they reference. Returns true if all indices are within range.
    fn range_check(&self) -> Result<(), ()> {
        macro_rules! range_check {
            ($field:ident) => {
                for item in self.$field.iter() {
                    let _ = item.range_check(self)?; 
                }
            }
        }
        range_check!(accessors);
        range_check!(animations);
        range_check!(buffers);
        range_check!(buffer_views);
        range_check!(cameras);
        range_check!(images);
        range_check!(materials);
        range_check!(meshes);
        range_check!(nodes);
        range_check!(samplers);
        range_check!(scenes);
        range_check!(skins);
        range_check!(textures);
        let _ = self.try_get(&self.scene)?;
        Ok(())
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
        impl<'a, E: Extras> Get<$ty> for Root<E> {
            fn get(&self, index: &Index<$ty>) -> &$ty {
                &self.$field[index.value() as usize]
            }
        }
    }
}

macro_rules! impl_try_get {
    ($ty:ty, $field:ident) => {
        impl<'a, E: Extras> TryGet<$ty> for Root<E> {
            fn try_get(&self, index: &Index<$ty>) -> Result<&$ty, ()> {
                self.$field.get(index.value() as usize).ok_or(())
            }
        }
    }
}

impl_get!(accessor::Accessor<E>, accessors);
impl_get!(animation::Animation<E>, animations);
impl_get!(buffer::Buffer<E>, buffers);
impl_get!(buffer::BufferView<E>, buffer_views);
impl_get!(camera::Camera<E>, cameras);
impl_get!(image::Image<E>, images);
impl_get!(material::Material<E>, materials);
impl_get!(mesh::Mesh<E>, meshes);
impl_get!(scene::Node<E>, nodes);
impl_get!(texture::Sampler<E>, samplers);
impl_get!(scene::Scene<E>, scenes);
impl_get!(skin::Skin<E>, skins);
impl_get!(texture::Texture<E>, textures);

impl_try_get!(accessor::Accessor<E>, accessors);
impl_try_get!(animation::Animation<E>, animations);
impl_try_get!(buffer::Buffer<E>, buffers);
impl_try_get!(buffer::BufferView<E>, buffer_views);
impl_try_get!(camera::Camera<E>, cameras);
impl_try_get!(image::Image<E>, images);
impl_try_get!(material::Material<E>, materials);
impl_try_get!(mesh::Mesh<E>, meshes);
impl_try_get!(scene::Node<E>, nodes);
impl_try_get!(texture::Sampler<E>, samplers);
impl_try_get!(scene::Scene<E>, scenes);
impl_try_get!(skin::Skin<E>, skins);
impl_try_get!(texture::Texture<E>, textures);

