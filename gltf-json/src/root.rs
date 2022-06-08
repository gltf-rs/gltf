use crate::buffer;
use crate::extensions;
use crate::texture;
use crate::validation;
use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
use std::{self, fmt, io, marker};

use crate::path::Path;
use crate::{
    Accessor, Animation, Asset, Buffer, Camera, Error, Extras, Image, Material, Mesh, Node, Scene,
    Skin, Texture, Value,
};
use validation::Validate;

/// Helper trait for retrieving top-level objects by a universal identifier.
pub trait Get<T> {
    /// Retrieves a single value at the given index.
    fn get(&self, id: Index<T>) -> Option<&T>;
}

/// Represents an offset into an array of type `T` owned by the root glTF object.
pub struct Index<T>(u32, marker::PhantomData<*const T>);

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(bound = "E: 'static")]
pub struct Root<E: crate::ThirdPartyExtensions> {
    /// An array of accessors.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accessors: Vec<Accessor>,

    /// An array of keyframe animations.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub animations: Vec<Animation>,

    /// Metadata about the glTF asset.
    pub asset: Asset,

    /// An array of buffers.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub buffers: Vec<Buffer>,

    /// An array of buffer views.
    #[serde(default, rename = "bufferViews")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub buffer_views: Vec<buffer::View>,

    /// The default scene.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<Index<Scene>>,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::root::Root<E>>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,

    /// Names of glTF extensions used somewhere in this asset.
    #[serde(default, rename = "extensionsUsed")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    #[serde(default, rename = "extensionsRequired")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extensions_required: Vec<String>,

    /// An array of cameras.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cameras: Vec<Camera>,

    /// An array of images.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Image>,

    /// An array of materials.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub materials: Vec<Material>,

    /// An array of meshes.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meshes: Vec<Mesh>,

    /// An array of nodes.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<Node>,

    /// An array of samplers.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub samplers: Vec<texture::Sampler>,

    /// An array of scenes.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scenes: Vec<Scene>,

    /// An array of skins.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skins: Vec<Skin>,

    /// An array of textures.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub textures: Vec<Texture>,
}

impl<E: crate::ThirdPartyExtensions> Root<E> {
    /// Returns a single item from the root object.
    pub fn get<T>(&self, index: Index<T>) -> Option<&T>
    where
        Self: Get<T>,
    {
        (self as &dyn Get<T>).get(index)
    }

    fn index_is_valid(&self, index: IndexEnum) -> bool {
        match index {
            IndexEnum::Accessor(index) => self.get(index).is_some(),
            IndexEnum::Animation(index) => self.get(index).is_some(),
            IndexEnum::Buffer(index) => self.get(index).is_some(),
            IndexEnum::BufferView(index) => self.get(index).is_some(),
            IndexEnum::Camera(index) => self.get(index).is_some(),
            IndexEnum::Image(index) => self.get(index).is_some(),
            IndexEnum::Material(index) => self.get(index).is_some(),
            IndexEnum::Mesh(index) => self.get(index).is_some(),
            IndexEnum::Node(index) => self.get(index).is_some(),
            IndexEnum::Sampler(index) => self.get(index).is_some(),
            IndexEnum::Scene(index) => self.get(index).is_some(),
            IndexEnum::Skin(index) => self.get(index).is_some(),
            IndexEnum::Texture(index) => self.get(index).is_some(),
        }
    }

    /// Deserialize from a JSON string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(str_: &str) -> Result<Self, Error> {
        serde_json::from_str(str_)
    }

    /// Deserialize from a JSON byte slice.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        serde_json::from_slice(slice)
    }

    /// Deserialize from a stream of JSON.
    pub fn from_reader<R>(reader: R) -> Result<Self, Error>
    where
        R: io::Read,
    {
        serde_json::from_reader(reader)
    }

    /// Serialize as a `String` of JSON.
    pub fn to_string(&self) -> Result<String, Error> {
        serde_json::to_string(self)
    }

    /// Serialize as a pretty-printed `String` of JSON.
    pub fn to_string_pretty(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serialize as a generic JSON value.
    pub fn to_value(&self) -> Result<Value, Error> {
        serde_json::to_value(self)
    }

    /// Serialize as a JSON byte vector.
    pub fn to_vec(&self) -> Result<Vec<u8>, Error> {
        serde_json::to_vec(self)
    }

    /// Serialize as a pretty-printed JSON byte vector.
    pub fn to_vec_pretty(&self) -> Result<Vec<u8>, Error> {
        serde_json::to_vec_pretty(self)
    }

    /// Serialize as a JSON byte writertor.
    pub fn to_writer<W>(&self, writer: W) -> Result<(), Error>
    where
        W: io::Write,
    {
        serde_json::to_writer(writer, self)
    }

    /// Serialize as a pretty-printed JSON byte writertor.
    pub fn to_writer_pretty<W>(&self, writer: W) -> Result<(), Error>
    where
        W: io::Write,
    {
        serde_json::to_writer_pretty(writer, self)
    }
}

impl<T> Index<T> {
    /// Creates a new `Index` representing an offset into an array containing `T`.
    pub fn new(value: u32) -> Self {
        Index(value, std::marker::PhantomData)
    }

    /// Returns the internal offset value.
    pub fn value(&self) -> usize {
        self.0 as usize
    }
}

impl<T> serde::Serialize for Index<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_u64(self.value() as u64)
    }
}

impl<'de, T> serde::Deserialize<'de> for Index<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<T>(marker::PhantomData<T>);
        impl<'de, T> serde::de::Visitor<'de> for Visitor<T> {
            type Value = Index<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("index into child of root")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Index::new(value as u32))
            }
        }
        deserializer.deserialize_u64(Visitor::<T>(marker::PhantomData))
    }
}

impl<T> Clone for Index<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Index<T> {}

unsafe impl<T> Send for Index<T> {}
unsafe impl<T> Sync for Index<T> {}

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

impl<T: Copy> Validate for T where IndexEnum: From<T> {
    fn validate<P, R, E: crate::ThirdPartyExtensions>(&self, root: &Root<E>, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, validation::Error),
    {
        if !root.index_is_valid(IndexEnum::from(*self)) {
            report(&path, validation::Error::IndexOutOfBounds);
        }
    }
}

macro_rules! impl_get {
    ($ty:ty, $field:ident) => {
        impl<'a, E: crate::ThirdPartyExtensions> Get<$ty> for Root<E> {
            fn get(&self, index: Index<$ty>) -> Option<&$ty> {
                self.$field.get(index.value())
            }
        }
    };
}

impl_get!(Accessor, accessors);
impl_get!(Animation, animations);
impl_get!(Buffer, buffers);
impl_get!(buffer::View, buffer_views);
impl_get!(Camera, cameras);
impl_get!(Image, images);
impl_get!(Material, materials);
impl_get!(Mesh, meshes);
impl_get!(Node, nodes);
impl_get!(texture::Sampler, samplers);
impl_get!(Scene, scenes);
impl_get!(Skin, skins);
impl_get!(Texture, textures);

enum IndexEnum {
    Accessor(Index<Accessor>),
    Animation(Index<Animation>),
    Buffer(Index<Buffer>),
    BufferView(Index<buffer::View>),
    Camera(Index<Camera>),
    Image(Index<Image>),
    Material(Index<Material>),
    Mesh(Index<Mesh>),
    Node(Index<Node>),
    Sampler(Index<texture::Sampler>),
    Scene(Index<Scene>),
    Skin(Index<Skin>),
    Texture(Index<Texture>),
}

impl From<Index<Accessor>> for IndexEnum {
    fn from(index: Index<Accessor>) -> Self {
        Self::Accessor(index)
    }
}

impl From<Index<Animation>> for IndexEnum {
    fn from(index: Index<Animation>) -> Self {
        Self::Animation(index)
    }
}

impl From<Index<Buffer>> for IndexEnum {
    fn from(index: Index<Buffer>) -> Self {
        Self::Buffer(index)
    }
}

impl From<Index<buffer::View>> for IndexEnum {
    fn from(index: Index<buffer::View>) -> Self {
        Self::BufferView(index)
    }
}

impl From<Index<Camera>> for IndexEnum {
    fn from(index: Index<Camera>) -> Self {
        Self::Camera(index)
    }
}

impl From<Index<Image>> for IndexEnum {
    fn from(index: Index<Image>) -> Self {
        Self::Image(index)
    }
}

impl From<Index<Material>> for IndexEnum {
    fn from(index: Index<Material>) -> Self {
        Self::Material(index)
    }
}

impl From<Index<Mesh>> for IndexEnum {
    fn from(index: Index<Mesh>) -> Self {
        Self::Mesh(index)
    }
}

impl From<Index<Node>> for IndexEnum {
    fn from(index: Index<Node>) -> Self {
        Self::Node(index)
    }
}

impl From<Index<texture::Sampler>> for IndexEnum {
    fn from(index: Index<texture::Sampler>) -> Self {
        Self::Sampler(index)
    }
}

impl From<Index<Scene>> for IndexEnum {
    fn from(index: Index<Scene>) -> Self {
        Self::Scene(index)
    }
}

impl From<Index<Skin>> for IndexEnum {
    fn from(index: Index<Skin>) -> Self {
        Self::Skin(index)
    }
}

impl From<Index<Texture>> for IndexEnum {
    fn from(index: Index<Texture>) -> Self {
        Self::Texture(index)
    }
}