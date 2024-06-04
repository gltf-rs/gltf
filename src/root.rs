use crate::buffer;
use crate::texture;
use crate::validation;
use std::{fmt, marker};

use crate::path::Path;
use crate::{
    Accessor, Animation, Asset, Buffer, Camera, Extras, Image, Material, Mesh, Node, Scene, Skin,
    Texture, UnrecognizedExtensions, Wrap,
};
use validation::Validate;

/// Support for the `KHR_lights_punctual` extension.
pub mod khr_lights_punctual {
    /// Defines a set of lights that can be placed into a scene.
    #[derive(
        Clone,
        Debug,
        Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct Lights {
        /// An array of punctual light definitions.
        pub lights: Vec<crate::scene::khr_lights_punctual::Light>,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// Support for the `KHR_materials_variants` extension.
pub mod khr_materials_variants {
    /// Defines an alternative material that may be applied to a mesh primitive.
    #[derive(
        Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate,
    )]
    pub struct Variant {
        /// The name of the material variant.
        pub name: String,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }

    /// Defines a set of alternative materials that may be applied to mesh primitives.
    #[derive(
        Clone,
        Debug,
        Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct Variants {
        /// The available material variants.
        pub variants: Vec<Variant>,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// Represents an offset into a vector of type `T` owned by the root glTF object.
///
/// This type may be used with the following functions:
///
/// * [`Root::get()`] to retrieve objects from [`Root`].
/// * [`Root::push()`] to add new objects to [`Root`].
pub struct Index<T>(u32, marker::PhantomData<fn() -> T>);

/// The root object of a glTF 2.0 asset.
#[derive(
    Clone, Debug, Default, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate,
)]
#[gltf(validate = "validate_root")]
pub struct Root {
    /// An array of accessors.
    pub accessors: Vec<Accessor>,

    /// An array of keyframe animations.
    pub animations: Vec<Animation>,

    /// Metadata about the glTF asset.
    pub asset: Asset,

    /// An array of buffers.
    pub buffers: Vec<Buffer>,

    /// An array of buffer views.
    pub buffer_views: Vec<buffer::View>,

    /// The default scene.
    pub scene: Option<Index<Scene>>,

    /// Names of glTF extensions used somewhere in this asset.
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    pub extensions_required: Vec<String>,

    /// An array of cameras.
    pub cameras: Vec<Camera>,

    /// An array of images.
    pub images: Vec<Image>,

    /// An array of materials.
    pub materials: Vec<Material>,

    /// An array of meshes.
    pub meshes: Vec<Mesh>,

    /// An array of nodes.
    pub nodes: Vec<Node>,

    /// An array of samplers.
    pub samplers: Vec<texture::Sampler>,

    /// An array of scenes.
    pub scenes: Vec<Scene>,

    /// An array of skins.
    pub skins: Vec<Skin>,

    /// An array of textures.
    pub textures: Vec<Texture>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,

    /// Support for the `KHR_lights_punctual` extension.
    #[gltf(extension = "KHR_lights_punctual")]
    pub lights: Option<khr_lights_punctual::Lights>,

    /// Support for the `KHR_materials_variants` extension.
    #[gltf(extension = "KHR_materials_variants")]
    pub variants: Option<khr_materials_variants::Variants>,
}

fn validate_root<P, R>(root: &Root, _also_root: &Root, path: P, report: &mut R)
where
    P: Fn() -> Path,
    R: FnMut(&dyn Fn() -> Path, crate::validation::Error),
{
    for (i, ext) in root.extensions_required.iter().enumerate() {
        if !crate::SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
            report(
                &|| {
                    path()
                        .field("extensionsRequired")
                        .index(i)
                        .value_str(ext.as_str())
                },
                crate::validation::Error::Unsupported,
            );
        }
    }
}

impl<T> std::ops::Index<Index<T>> for Root
where
    Root: AsRef<[T]>,
{
    type Output = T;

    fn index(&self, index: Index<T>) -> &Self::Output {
        let slice: &[T] = self.as_ref();
        &slice[index.value()]
    }
}

impl<T> std::ops::IndexMut<Index<T>> for Root
where
    Root: AsRef<[T]> + AsMut<Vec<T>>,
{
    fn index_mut(&mut self, index: Index<T>) -> &mut Self::Output {
        let slice: &mut Vec<T> = self.as_mut();
        &mut slice[index.value()]
    }
}

impl Root {
    /// Returns a single item from the root object.
    pub fn get<T>(&self, index: Index<T>) -> Option<&T>
    where
        Self: AsRef<[T]>,
    {
        self.as_ref().get(index.value())
    }

    /// Insert the given value into this (as via [`Vec::push()`]), then return the [`Index`] to it.
    ///
    /// This allows you to easily obtain [`Index`] values with the correct index and type when
    /// creating a glTF asset.
    ///
    /// If you have a mutable borrow conflict when using this method, consider using the more
    /// explicit [`Index::push()`] method, passing it only the necessary vector.
    ///
    /// # Panics
    ///
    /// Panics if there are already [`u32::MAX`] or more elements of this type,
    /// in which case an `Index` cannot be created.
    #[track_caller]
    pub fn push<T>(&mut self, value: T) -> Index<T>
    where
        Self: AsMut<Vec<T>>,
    {
        Index::push(self.as_mut(), value)
    }
}

impl<T> Index<T> {
    /// Creates a new `Index` representing an offset into an array containing `T`.
    pub fn new(value: u32) -> Self {
        Index(value, std::marker::PhantomData)
    }

    /// Given a vector of glTF objects, call [`Vec::push()`] to insert it into the vector,
    /// then return an [`Index`] for it.
    ///
    /// This allows you to easily obtain [`Index`] values with the correct index and type when
    /// creating a glTF asset. Note that for [`Root`], you can call [`Root::push()`] without
    /// needing to retrieve the correct vector first.
    ///
    /// # Panics
    ///
    /// Panics if the vector has [`u32::MAX`] or more elements, in which case an `Index` cannot be
    /// created.
    pub fn push(vec: &mut Vec<T>, value: T) -> Index<T> {
        let len = vec.len();
        let Ok(index): Result<u32, _> = len.try_into() else {
            panic!(
                "glTF vector of {ty} has {len} elements, which exceeds the Index limit",
                ty = std::any::type_name::<T>(),
            );
        };
        vec.push(value);
        Index::new(index)
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
    Root: AsRef<[T]>,
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

impl<'a, T> Wrap<'a> for Index<T>
where
    T: 'a + Wrap<'a>,
    Root: AsRef<[T]>,
{
    type Wrapped = <T as Wrap<'a>>::Wrapped;

    fn wrap(&'a self, root: &'a Root) -> Self::Wrapped {
        let items = root.as_ref();
        items[self.value()].wrap_indexed(root, self.value())
    }
}

impl<'a, T> Wrap<'a> for std::boxed::Box<T>
where
    T: 'a + Wrap<'a>,
{
    type Wrapped = <T as Wrap<'a>>::Wrapped;

    fn wrap(&'a self, root: &'a Root) -> Self::Wrapped {
        use std::ops::Deref;
        self.deref().wrap(root)
    }
}

impl<'a, K: 'a, V: 'a> Wrap<'a> for serde_json::Map<K, V> {
    type Wrapped = &'a Self;

    fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped {
        self
    }
}

impl<'a> Wrap<'a> for serde_json::Value {
    type Wrapped = &'a Self;

    fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped {
        self
    }
}

macro_rules! impl_as_ref {
    ($field:ident, $ty:ty) => {
        impl AsRef<[$ty]> for Root {
            fn as_ref(&self) -> &[$ty] {
                &self.$field
            }
        }

        impl AsMut<Vec<$ty>> for Root {
            fn as_mut(&mut self) -> &mut Vec<$ty> {
                &mut self.$field
            }
        }
    };

    ($extension:ident, $field:ident, $ty:ty) => {
        impl AsRef<[$ty]> for Root {
            fn as_ref(&self) -> &[$ty] {
                self.$extension
                    .as_ref()
                    .map(|extension| extension.$field.as_slice())
                    .unwrap_or(&[])
            }
        }

        impl AsMut<Vec<$ty>> for Root {
            fn as_mut(&mut self) -> &mut Vec<$ty> {
                &mut self.$extension.get_or_insert_with(Default::default).$field
            }
        }
    };
}

impl_as_ref!(accessors, Accessor);
impl_as_ref!(animations, Animation);
impl_as_ref!(buffers, Buffer);
impl_as_ref!(buffer_views, buffer::View);
impl_as_ref!(cameras, Camera);
impl_as_ref!(images, Image);
impl_as_ref!(materials, Material);
impl_as_ref!(meshes, Mesh);
impl_as_ref!(nodes, Node);
impl_as_ref!(samplers, texture::Sampler);
impl_as_ref!(scenes, Scene);
impl_as_ref!(skins, Skin);
impl_as_ref!(textures, Texture);
impl_as_ref!(lights, lights, crate::scene::khr_lights_punctual::Light);
impl_as_ref!(variants, variants, khr_materials_variants::Variant);

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

    #[test]
    fn index_push() {
        let some_object = "hello";

        let mut vec = Vec::new();
        assert_eq!(Index::push(&mut vec, some_object), Index::new(0));
        assert_eq!(Index::push(&mut vec, some_object), Index::new(1));
    }

    #[test]
    fn root_push() {
        let some_object = Buffer {
            length: validation::USize64(1),
            name: None,
            uri: None,
            extras: Default::default(),
            unrecognized_extensions: Default::default(),
        };

        let mut root = Root::default();
        assert_eq!(root.push(some_object.clone()), Index::new(0));
        assert_eq!(root.push(some_object), Index::new(1));
    }

    #[test]
    fn root_extensions() {
        use crate::validation::Error;
        use crate::Path;

        let mut root = super::Root {
            extensions_required: vec!["KHR_lights_punctual".to_owned()],
            ..Default::default()
        };

        let mut errors = Vec::new();
        root.validate(&root, Path::new, &mut |path, error| {
            errors.push((path(), error));
        });
        assert!(errors.is_empty());

        root.extensions_required = vec!["KHR_mesh_quantization".to_owned()];
        errors.clear();
        root.validate(&root, Path::new, &mut |path, error| {
            errors.push((path(), error));
        });
        assert_eq!(1, errors.len());
        let (path, error) = errors.get(0).unwrap();
        assert_eq!(
            path.as_str(),
            "extensionsRequired[0] = \"KHR_mesh_quantization\""
        );
        assert_eq!(*error, Error::Unsupported);
    }
}
