use std::{iter, slice};

use crate::accessor::Accessor;
use crate::animation::Animation;
use crate::buffer::{Buffer, View};
use crate::camera::Camera;
use crate::image::Image;
use crate::material::Material;
use crate::mesh::Mesh;
use crate::scene::{Node, Scene};
use crate::skin::Skin;
use crate::texture::{Sampler, Texture};
use crate::Document;

/// An `Iterator` that visits extension strings used by a glTF asset.
#[derive(Clone, Debug)]
pub struct ExtensionsUsed<'a>(pub(crate) slice::Iter<'a, String>);

/// An `Iterator` that visits extension strings required by a glTF asset.
#[derive(Clone, Debug)]
pub struct ExtensionsRequired<'a>(pub(crate) slice::Iter<'a, String>);

/// An `Iterator` that visits every accessor in a glTF asset.
#[derive(Clone, Debug)]
pub struct Accessors<'a, E: json::CustomExtensions> {
    /// Internal accessor iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::accessor::Accessor>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every animation in a glTF asset.
#[derive(Clone, Debug)]
pub struct Animations<'a, E: json::CustomExtensions = ()> {
    /// Internal animation iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::animation::Animation>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every buffer in a glTF asset.
#[derive(Clone, Debug)]
pub struct Buffers<'a, E: json::CustomExtensions> {
    /// Internal buffer iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::buffer::Buffer>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every buffer view in a glTF asset.
#[derive(Clone, Debug)]
pub struct Views<'a, E: json::CustomExtensions> {
    /// Internal buffer view iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::buffer::View>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every camera in a glTF asset.
#[derive(Clone, Debug)]
pub struct Cameras<'a, E: json::CustomExtensions> {
    /// Internal buffer view iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::camera::Camera>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every pre-loaded image in a glTF asset.
#[derive(Clone, Debug)]
pub struct Images<'a, E: json::CustomExtensions> {
    /// Internal image iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::image::Image>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every light in a glTF asset.
#[cfg(feature = "KHR_lights_punctual")]
#[derive(Clone, Debug)]
pub struct Lights<'a, E: json::CustomExtensions> {
    /// Internal image iterator.
    pub(crate) iter:
        iter::Enumerate<slice::Iter<'a, json::extensions::scene::khr_lights_punctual::Light>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every variant in a glTF asset.
#[cfg(feature = "KHR_materials_variants")]
#[derive(Clone, Debug)]
pub struct Variants<'a, E: json::CustomExtensions> {
    /// Internal variant iterator.
    pub(crate) iter:
        iter::Enumerate<slice::Iter<'a, json::extensions::scene::khr_materials_variants::Variant>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every material in a glTF asset.
#[derive(Clone, Debug)]
pub struct Materials<'a, E: json::CustomExtensions> {
    /// Internal material iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::material::Material>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every mesh in a glTF asset.
#[derive(Clone, Debug)]
pub struct Meshes<'a, E: json::CustomExtensions> {
    /// Internal mesh iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::mesh::Mesh>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every node in a glTF asset.
#[derive(Debug)]
pub struct Nodes<'a, E: json::CustomExtensions = ()> {
    /// Internal node iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::scene::Node>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

impl<'a, E: json::CustomExtensions> Clone for Nodes<'a, E> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            document: self.document
        }
    }
}

/// An `Iterator` that visits every sampler in a glTF asset.
#[derive(Clone, Debug)]
pub struct Samplers<'a, E: json::CustomExtensions> {
    /// Internal sampler iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::texture::Sampler>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every scene in a glTF asset.
#[derive(Clone, Debug)]
pub struct Scenes<'a, E: json::CustomExtensions> {
    /// Internal scene iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::scene::Scene>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every skin in a glTF asset.
#[derive(Clone, Debug)]
pub struct Skins<'a, E: json::CustomExtensions> {
    /// Internal skin iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::skin::Skin>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

/// An `Iterator` that visits every texture in a glTF asset.
#[derive(Clone, Debug)]
pub struct Textures<'a, E: json::CustomExtensions> {
    /// Internal texture iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::texture::Texture>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document<E>,
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Accessors<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Accessors<'a, E> {
    type Item = Accessor<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Accessor::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Accessor::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Accessor::new(self.document, index, json))
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Animations<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Animations<'a, E> {
    type Item = Animation<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Animation::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Animation::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Animation::new(self.document, index, json))
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Buffers<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Buffers<'a, E> {
    type Item = Buffer<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Buffer::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Buffer::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Buffer::new(self.document, index, json))
    }
}

impl<'a> ExactSizeIterator for ExtensionsUsed<'a> {}
impl<'a> Iterator for ExtensionsUsed<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(String::as_str)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    fn count(self) -> usize {
        self.0.count()
    }
    fn last(self) -> Option<Self::Item> {
        self.0.last().map(String::as_str)
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(String::as_str)
    }
}

impl<'a> ExactSizeIterator for ExtensionsRequired<'a> {}
impl<'a> Iterator for ExtensionsRequired<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(String::as_str)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    fn count(self) -> usize {
        self.0.count()
    }
    fn last(self) -> Option<Self::Item> {
        self.0.last().map(String::as_str)
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(String::as_str)
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Views<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Views<'a, E> {
    type Item = View<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| View::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| View::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| View::new(self.document, index, json))
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Cameras<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Cameras<'a, E> {
    type Item = Camera<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Camera::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Camera::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Camera::new(self.document, index, json))
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Images<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Images<'a, E> {
    type Item = Image<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Image::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Image::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Image::new(self.document, index, json))
    }
}

#[cfg(feature = "KHR_lights_punctual")]
impl<'a, E: json::CustomExtensions> ExactSizeIterator for Lights<'a, E> {}

#[cfg(feature = "KHR_lights_punctual")]
impl<'a, E: json::CustomExtensions> Iterator for Lights<'a, E> {
    type Item = crate::khr_lights_punctual::Light<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| crate::khr_lights_punctual::Light::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| crate::khr_lights_punctual::Light::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| crate::khr_lights_punctual::Light::new(self.document, index, json))
    }
}

#[cfg(feature = "KHR_materials_variants")]
impl<'a, E: json::CustomExtensions> ExactSizeIterator for Variants<'a, E> {}

#[cfg(feature = "KHR_materials_variants")]
impl<'a, E: json::CustomExtensions> Iterator for Variants<'a, E> {
    type Item = crate::khr_materials_variants::Variant<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, json)| {
            crate::khr_materials_variants::Variant::new(self.document, index, json)
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| crate::khr_materials_variants::Variant::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|(index, json)| {
            crate::khr_materials_variants::Variant::new(self.document, index, json)
        })
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Materials<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Materials<'a, E> {
    type Item = Material<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Material::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Material::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Material::new(self.document, index, json))
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Meshes<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Meshes<'a, E> {
    type Item = Mesh<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Mesh::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Mesh::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Mesh::new(self.document, index, json))
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Nodes<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Nodes<'a, E> {
    type Item = Node<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Node::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Node::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Node::new(self.document, index, json))
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Samplers<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Samplers<'a, E> {
    type Item = Sampler<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Sampler::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Sampler::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Sampler::new(self.document, index, json))
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Scenes<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Scenes<'a, E> {
    type Item = Scene<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Scene::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Scene::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Scene::new(self.document, index, json))
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Skins<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Skins<'a, E> {
    type Item = Skin<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Skin::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Skin::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Skin::new(self.document, index, json))
    }
}

impl<'a, E: json::CustomExtensions> ExactSizeIterator for Textures<'a, E> {}
impl<'a, E: json::CustomExtensions> Iterator for Textures<'a, E> {
    type Item = Texture<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Texture::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Texture::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Texture::new(self.document, index, json))
    }
}
