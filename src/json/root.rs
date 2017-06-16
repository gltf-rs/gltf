
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use std::fmt;
use std::borrow::Cow;
use std::marker::PhantomData;

use json::*;
use validation::{Error, JsonPath, Validate};

/// Helper trait for retrieving top-level objects by index.
pub trait Get<T> {
    /// Retrieves a single value at the given index.
    fn get(&self, id: &Index<T>) -> &T;
}

/// Helper trait for attempting to retrieve top-level objects by index.
pub trait TryGet<T> {
    /// Attempts to retrieve a single value at the given index.
    fn try_get(&self, id: &Index<T>) -> Result<&T, ()>;
}

/// Represents an offset into an array of type `T` owned by the root glTF object.
#[derive(Clone, Copy)]
pub struct Index<T>(u32, std::marker::PhantomData<T>);

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Root<'a> {
    /// An array of accessors.
    #[serde(default)]
    pub accessors: Vec<accessor::Accessor<'a>>,
    
    /// An array of keyframe animations.
    #[serde(default)]
    pub animations: Vec<animation::Animation<'a>>,

    /// Metadata about the glTF asset.
    pub asset: asset::Asset<'a>,
    
    /// An array of buffers.
    #[serde(default)]
    pub buffers: Vec<buffer::Buffer<'a>>,
    
    /// An array of buffer views.
    #[serde(default, rename = "bufferViews")]
    pub buffer_views: Vec<buffer::View<'a>>,

    /// The default scene.
    pub scene: Option<Index<scene::Scene<'a>>>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: RootExtensions<'a>,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
    
    /// Names of glTF extensions used somewhere in this asset.
    #[serde(default, rename = "extensionsUsed")]
    pub extensions_used: Vec<Cow<'a, str>>,

    /// Names of glTF extensions required to properly load this asset.
    #[serde(default, rename = "extensionsRequired")]
    pub extensions_required: Vec<Cow<'a, str>>,
    
    /// An array of cameras.
    #[serde(default)]
    pub cameras: Vec<camera::Camera<'a>>,
    
    /// An array of images.
    #[serde(default)]
    pub images: Vec<image::Image<'a>>,
    
    /// An array of materials.
    #[serde(default)]
    pub materials: Vec<material::Material<'a>>,
    
    /// An array of meshes.
    #[serde(default)]
    pub meshes: Vec<mesh::Mesh<'a>>,
    
    /// An array of nodes.
    #[serde(default)]
    pub nodes: Vec<scene::Node<'a>>,
    
    /// An array of samplers.
    #[serde(default)]
    pub samplers: Vec<texture::Sampler<'a>>,
    
    /// An array of scenes.
    #[serde(default)]
    pub scenes: Vec<scene::Scene<'a>>,
    
    /// An array of skins.
    #[serde(default)]
    pub skins: Vec<skin::Skin<'a>>,
    
    /// An array of textures.
    #[serde(default)]
    pub textures: Vec<texture::Texture<'a>>,
}

/// Extension specific data for `Root`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct RootExtensions<'a> {
    _allow_unknown_fields: PhantomData<&'a ()>,
}

impl<'a> Root<'a> {
    /// Returns the default scene.
    pub fn default_scene(&self) -> Option<&scene::Scene<'a>> {
        self.scene.as_ref().map(|s| self.get(s))
    }

    /// Returns a single item from the root object.
    pub fn get<T>(&self, index: &Index<T>) -> &T
        where Self: Get<T>
    {
        (self as &Get<T>).get(index)
    }

    /// Returns a single item from the root object if the index is in range.
    pub fn try_get<T>(&self, index: &Index<T>) -> Result<&T, ()>
        where Self: TryGet<T>
    {
        (self as &TryGet<T>).try_get(index)
    }
}

impl<T> Index<T> {
    /// Creates a new `Index` representing an offset into an array containing `T`.
    fn new(value: u32) -> Self {
        Index(value, std::marker::PhantomData)
    }

    /// Returns the internal offset value.
    pub fn value(&self) -> usize {
        self.0 as usize
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

impl<'a, T: Validate<'a>> Validate<'a> for Index<T>
    where Root<'a>: TryGet<T>
{
    fn validate<P, R>(&self, root: &Root<'a>, path: P, mut report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if root.try_get(self).is_err() {
            report(Error::index_out_of_bounds(path()));
        }
    }
}

macro_rules! impl_get {
    ($ty:ty, $field:ident) => {
        impl<'a> Get<$ty> for Root<'a> {
            fn get(&self, index: &Index<$ty>) -> &$ty {
                &self.$field[index.value() as usize]
            }
        }
    }
}

macro_rules! impl_try_get {
    ($ty:ty, $field:ident) => {
        #[doc(hidden)]
        impl<'a> TryGet<$ty> for Root<'a> {
            fn try_get(&self, index: &Index<$ty>) -> Result<&$ty, ()> {
                self.$field.get(index.value() as usize).ok_or(())
            }
        }
    }
}

impl_get!(accessor::Accessor<'a>, accessors);
impl_get!(animation::Animation<'a>, animations);
impl_get!(buffer::Buffer<'a>, buffers);
impl_get!(buffer::View<'a>, buffer_views);
impl_get!(camera::Camera<'a>, cameras);
impl_get!(image::Image<'a>, images);
impl_get!(material::Material<'a>, materials);
impl_get!(mesh::Mesh<'a>, meshes);
impl_get!(scene::Node<'a>, nodes);
impl_get!(texture::Sampler<'a>, samplers);
impl_get!(scene::Scene<'a>, scenes);
impl_get!(skin::Skin<'a>, skins);
impl_get!(texture::Texture<'a>, textures);

impl_try_get!(accessor::Accessor<'a>, accessors);
impl_try_get!(animation::Animation<'a>, animations);
impl_try_get!(buffer::Buffer<'a>, buffers);
impl_try_get!(buffer::View<'a>, buffer_views);
impl_try_get!(camera::Camera<'a>, cameras);
impl_try_get!(image::Image<'a>, images);
impl_try_get!(material::Material<'a>, materials);
impl_try_get!(mesh::Mesh<'a>, meshes);
impl_try_get!(scene::Node<'a>, nodes);
impl_try_get!(texture::Sampler<'a>, samplers);
impl_try_get!(scene::Scene<'a>, scenes);
impl_try_get!(skin::Skin<'a>, skins);
impl_try_get!(texture::Texture<'a>, textures);
