use alloc::string::String;
use core::{iter, slice};

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
pub struct Accessors<'a> {
    /// Internal accessor iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::accessor::Accessor>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every animation in a glTF asset.
#[derive(Clone, Debug)]
pub struct Animations<'a> {
    /// Internal animation iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::animation::Animation>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every buffer in a glTF asset.
#[derive(Clone, Debug)]
pub struct Buffers<'a> {
    /// Internal buffer iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::buffer::Buffer>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every buffer view in a glTF asset.
#[derive(Clone, Debug)]
pub struct Views<'a> {
    /// Internal buffer view iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::buffer::View>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every camera in a glTF asset.
#[derive(Clone, Debug)]
pub struct Cameras<'a> {
    /// Internal buffer view iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::camera::Camera>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every pre-loaded image in a glTF asset.
#[derive(Clone, Debug)]
pub struct Images<'a> {
    /// Internal image iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::image::Image>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every light in a glTF asset.
#[cfg(feature = "KHR_lights_punctual")]
#[derive(Clone, Debug)]
pub struct Lights<'a> {
    /// Internal image iterator.
    pub(crate) iter:
        iter::Enumerate<slice::Iter<'a, json::extensions::scene::khr_lights_punctual::Light>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every variant in a glTF asset.
#[cfg(feature = "KHR_materials_variants")]
#[derive(Clone, Debug)]
pub struct Variants<'a> {
    /// Internal variant iterator.
    pub(crate) iter:
        iter::Enumerate<slice::Iter<'a, json::extensions::scene::khr_materials_variants::Variant>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every material in a glTF asset.
#[derive(Clone, Debug)]
pub struct Materials<'a> {
    /// Internal material iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::material::Material>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every mesh in a glTF asset.
#[derive(Clone, Debug)]
pub struct Meshes<'a> {
    /// Internal mesh iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::mesh::Mesh>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every node in a glTF asset.
#[derive(Clone, Debug)]
pub struct Nodes<'a> {
    /// Internal node iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::scene::Node>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every sampler in a glTF asset.
#[derive(Clone, Debug)]
pub struct Samplers<'a> {
    /// Internal sampler iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::texture::Sampler>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every scene in a glTF asset.
#[derive(Clone, Debug)]
pub struct Scenes<'a> {
    /// Internal scene iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::scene::Scene>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every skin in a glTF asset.
#[derive(Clone, Debug)]
pub struct Skins<'a> {
    /// Internal skin iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::skin::Skin>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

/// An `Iterator` that visits every texture in a glTF asset.
#[derive(Clone, Debug)]
pub struct Textures<'a> {
    /// Internal texture iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::texture::Texture>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> ExactSizeIterator for Accessors<'a> {}
impl<'a> Iterator for Accessors<'a> {
    type Item = Accessor<'a>;
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

impl<'a> ExactSizeIterator for Animations<'a> {}
impl<'a> Iterator for Animations<'a> {
    type Item = Animation<'a>;
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

impl<'a> ExactSizeIterator for Buffers<'a> {}
impl<'a> Iterator for Buffers<'a> {
    type Item = Buffer<'a>;
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

impl<'a> ExactSizeIterator for Views<'a> {}
impl<'a> Iterator for Views<'a> {
    type Item = View<'a>;
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

impl<'a> ExactSizeIterator for Cameras<'a> {}
impl<'a> Iterator for Cameras<'a> {
    type Item = Camera<'a>;
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

impl<'a> ExactSizeIterator for Images<'a> {}
impl<'a> Iterator for Images<'a> {
    type Item = Image<'a>;
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
impl<'a> ExactSizeIterator for Lights<'a> {}

#[cfg(feature = "KHR_lights_punctual")]
impl<'a> Iterator for Lights<'a> {
    type Item = crate::khr_lights_punctual::Light<'a>;
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
impl<'a> ExactSizeIterator for Variants<'a> {}

#[cfg(feature = "KHR_materials_variants")]
impl<'a> Iterator for Variants<'a> {
    type Item = crate::khr_materials_variants::Variant<'a>;
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

impl<'a> ExactSizeIterator for Materials<'a> {}
impl<'a> Iterator for Materials<'a> {
    type Item = Material<'a>;
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

impl<'a> ExactSizeIterator for Meshes<'a> {}
impl<'a> Iterator for Meshes<'a> {
    type Item = Mesh<'a>;
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

impl<'a> ExactSizeIterator for Nodes<'a> {}
impl<'a> Iterator for Nodes<'a> {
    type Item = Node<'a>;
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

impl<'a> ExactSizeIterator for Samplers<'a> {}
impl<'a> Iterator for Samplers<'a> {
    type Item = Sampler<'a>;
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

impl<'a> ExactSizeIterator for Scenes<'a> {}
impl<'a> Iterator for Scenes<'a> {
    type Item = Scene<'a>;
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

impl<'a> ExactSizeIterator for Skins<'a> {}
impl<'a> Iterator for Skins<'a> {
    type Item = Skin<'a>;
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

impl<'a> ExactSizeIterator for Textures<'a> {}
impl<'a> Iterator for Textures<'a> {
    type Item = Texture<'a>;
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
