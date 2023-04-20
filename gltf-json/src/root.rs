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
pub struct Index<T>(u32, marker::PhantomData<fn() -> T>);

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {
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
    pub extensions: Option<extensions::root::Root>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
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

impl Root {
    /// Returns a single item from the root object.
    pub fn get<T>(&self, index: Index<T>) -> Option<&T>
    where
        Self: Get<T>,
    {
        (self as &dyn Get<T>).get(index)
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

impl<T> Ord for Index<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<T> PartialOrd for Index<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for Index<T> {}
impl<T> PartialEq for Index<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> std::hash::Hash for Index<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
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

impl<T: Validate> Validate for Index<T>
where
    Root: Get<T>,
{
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, validation::Error),
    {
        if root.get(*self).is_none() {
            report(&path, validation::Error::IndexOutOfBounds);
        }
    }
}

macro_rules! impl_get {
    ($ty:ty, $field:ident) => {
        impl<'a> Get<$ty> for Root {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn index_is_partialeq() {
        assert_eq!(Index::<Node>::new(1), Index::new(1));
        assert_ne!(Index::<Node>::new(1), Index::new(2));
    }

    #[test]
    fn index_is_hash() {
        let set = HashSet::from([Index::<Node>::new(1), Index::new(1234)]);
        assert!(set.contains(&Index::new(1234)));
        assert!(!set.contains(&Index::new(999)));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn index_is_ord() {
        assert!(Index::<Node>::new(1) < Index::new(1234));
    }

    fn _index_is_send_sync()
    where
        Index<Material>: Send + Sync,
    {
    }
}
