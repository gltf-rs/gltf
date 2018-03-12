use json;
use std::{fmt, iter, slice};

use accessor::Accessor;
use animation::Animation;
use buffer::{Buffer, View};
use camera::Camera;
use image::Image;
use material::Material;
use mesh::Mesh;
use scene::{Node, Scene};
use skin::Skin;
use texture::{Sampler, Texture};

/// The glTF scene representation.
pub struct Document {
    /// The JSON root object.
    root: json::root::Root,
}

/// An `Iterator` that visits extension strings.
#[derive(Clone, Debug)]
pub struct Extensions<'a>(slice::Iter<'a, String>);

/// An `Iterator` that visits every accessor in a glTF asset.
#[derive(Clone, Debug)]
pub struct Accessors<'a> {
    /// Internal accessor iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::accessor::Accessor>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every animation in a glTF asset.
#[derive(Clone, Debug)]
pub struct Animations<'a> {
    /// Internal animation iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::animation::Animation>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every buffer in a glTF asset.
#[derive(Clone, Debug)]
pub struct Buffers<'a> {
    /// Internal buffer iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::buffer::Buffer>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every buffer view in a glTF asset.
#[derive(Clone, Debug)]
pub struct Views<'a> {
    /// Internal buffer view iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::buffer::View>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every camera in a glTF asset.
#[derive(Clone, Debug)]
pub struct Cameras<'a> {
    /// Internal buffer view iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::camera::Camera>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every pre-loaded image in a glTF asset.
#[derive(Clone, Debug)]
pub struct Images<'a> {
    /// Internal image iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::image::Image>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every material in a glTF asset.
#[derive(Clone, Debug)]
pub struct Materials<'a> {
    /// Internal material iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::material::Material>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every mesh in a glTF asset.
#[derive(Clone, Debug)]
pub struct Meshes<'a> {
    /// Internal mesh iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::mesh::Mesh>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every node in a glTF asset.
#[derive(Clone, Debug)]
pub struct Nodes<'a> {
    /// Internal node iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::scene::Node>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every sampler in a glTF asset.
#[derive(Clone, Debug)]
pub struct Samplers<'a> {
    /// Internal sampler iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::texture::Sampler>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every scene in a glTF asset.
#[derive(Clone, Debug)]
pub struct Scenes<'a> {
    /// Internal scene iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::scene::Scene>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every skin in a glTF asset.
#[derive(Clone, Debug)]
pub struct Skins<'a> {
    /// Internal skin iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::skin::Skin>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

/// An `Iterator` that visits every texture in a glTF asset.
#[derive(Clone, Debug)]
pub struct Textures<'a> {
    /// Internal texture iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::texture::Texture>>,

    /// The internal root glTF object.
    doc: &'a Document,
}

impl Document {
    /// Returns an `Iterator` that visits the accessors of the glTF asset.
    pub fn accessors(&self) -> Accessors {
        Accessors {
            iter: self.as_json().accessors.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns an `Iterator` that visits the animations of the glTF asset.
    pub fn animations(&self) -> Animations {
        Animations {
            iter: self.as_json().animations.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns the JSON.
    pub(crate) fn as_json(&self) -> &json::Root {
        &self.root
    }

    /// Returns an `Iterator` that visits the pre-loaded buffers of the glTF asset.
    pub fn buffers(&self) -> Buffers {
        Buffers {
            iter: self.as_json().buffers.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns an `Iterator` that visits the cameras of the glTF asset.
    pub fn cameras(&self) -> Cameras {
        Cameras {
            iter: self.as_json().cameras.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns the default scene, if provided.
    pub fn default_scene(&self) -> Option<Scene> {
        self.as_json()
            .scene
            .as_ref()
            .map(|index| self.scenes().nth(index.value()).unwrap())
    }

    /// Returns the extensions referenced in this .gltf file.
    pub fn extensions_used(&self) -> Extensions {
        Extensions(self.root.extensions_used.iter())
    }

    /// Returns the extensions required to load and render this asset.
    pub fn extensions_required(&self) -> Extensions {
        Extensions(self.root.extensions_required.iter())
    }

    /// Returns an `Iterator` that visits the pre-loaded images of the glTF asset.
    pub fn images(&self) -> Images {
        Images {
            iter: self.as_json().images.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns an `Iterator` that visits the materials of the glTF asset.
    pub fn materials(&self) -> Materials {
        Materials {
            iter: self.as_json().materials.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns an `Iterator` that visits the meshes of the glTF asset.
    pub fn meshes(&self) -> Meshes {
        Meshes {
            iter: self.as_json().meshes.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns an `Iterator` that visits the nodes of the glTF asset.
    pub fn nodes(&self) -> Nodes {
        Nodes {
            iter: self.as_json().nodes.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns an `Iterator` that visits the scenes of the glTF asset.
    pub fn samplers(&self) -> Samplers {
        Samplers {
            iter: self.as_json().samplers.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns an `Iterator` that visits the samplers of the glTF asset.
    pub fn scenes(&self) -> Scenes {
        Scenes {
            iter: self.as_json().scenes.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns an `Iterator` that visits the skins of the glTF asset.
    pub fn skins(&self) -> Skins {
        Skins {
            iter: self.as_json().skins.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns an `Iterator` that visits the textures of the glTF asset.
    pub fn textures(&self) -> Textures {
        Textures {
            iter: self.as_json().textures.iter().enumerate(),
            doc: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffer views of the glTF
    /// asset.
    pub fn views(&self) -> Views {
        Views {
            iter: self.as_json().buffer_views.iter().enumerate(),
            doc: self,
        }
    }
}

impl<'a> fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.root.fmt(f)
    }
}

impl<'a> ExactSizeIterator for Accessors<'a> {}
impl<'a> Iterator for Accessors<'a> {
    type Item = Accessor<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Accessor::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Animations<'a> {}
impl<'a> Iterator for Animations<'a> {
    type Item = Animation<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Animation::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Buffers<'a> {}
impl<'a> Iterator for Buffers<'a> {
    type Item = Buffer<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Buffer::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Extensions<'a> {}
impl<'a> Iterator for Extensions<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(String::as_str)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Views<'a> {}
impl<'a> Iterator for Views<'a> {
    type Item = View<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| View::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Cameras<'a> {}
impl<'a> Iterator for Cameras<'a> {
    type Item = Camera<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Camera::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Images<'a> {}
impl<'a> Iterator for Images<'a> {
    type Item = Image<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Image::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Materials<'a> {}
impl<'a> Iterator for Materials<'a> {
    type Item = Material<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Material::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Meshes<'a> {}
impl<'a> Iterator for Meshes<'a> {
    type Item = Mesh<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Mesh::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Nodes<'a> {}
impl<'a> Iterator for Nodes<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Node::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Samplers<'a> {}
impl<'a> Iterator for Samplers<'a> {
    type Item = Sampler<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Sampler::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Scenes<'a> {}
impl<'a> Iterator for Scenes<'a> {
    type Item = Scene<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Scene::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Skins<'a> {}
impl<'a> Iterator for Skins<'a> {
    type Item = Skin<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Skin::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Textures<'a> {}
impl<'a> Iterator for Textures<'a> {
    type Item = Texture<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Texture::new(self.doc, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
