
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::boxed::Box;
use std::ops::Deref;
use std::slice;
use {accessor, animation, buffer, camera, image, json, material, mesh, root, scene, skin, texture};

use self::accessor::Accessor;
use self::animation::Animation;
use self::buffer::{Buffer, View};
use self::camera::Camera;
use self::image::Image;
use self::material::Material;
use self::mesh::Mesh;
use self::scene::{Node, Scene};
use self::skin::Skin;
use self::texture::{Sampler, Texture};

/// Describes buffer data required to render a single glTF asset.
#[derive(Clone, Debug)]
pub enum BufferData {
    /// The buffer data is owned.
    Owned(Box<[u8]>),
}

/// Describes image data required to render a single glTF asset.
#[derive(Clone, Debug)]
pub enum ImageData {
    /// The image data is borrowed from the indexed buffer view.
    Borrowed(usize),

    /// The image data is owned.
    Owned(Box<[u8]>),
}

/// A loaded glTF complete with its data.
#[derive(Clone, Debug)]
pub struct Gltf {
    /// The glTF buffer data.
    buffer_data: Vec<BufferData>,

    /// The glTF image data.
    image_data: Vec<ImageData>,

    /// The root glTF struct (and also `Deref` target).
    root: root::Root,
}

/// An `Iterator` that visits every accessor in a glTF asset.
#[derive(Clone, Debug)]
pub struct Accessors<'a> {
    /// Internal accessor iterator.
    iter: slice::Iter<'a, json::accessor::Accessor>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every animation in a glTF asset.
#[derive(Clone, Debug)]
pub struct Animations<'a> {
    /// Internal animation iterator.
    iter: slice::Iter<'a, json::animation::Animation>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every pre-loaded buffer in a glTF asset.
#[derive(Clone, Debug)]
pub struct Buffers<'a> {
    /// Internal buffer iterator.
    iter: slice::Iter<'a, json::buffer::Buffer>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every pre-loaded buffer view in a glTF asset.
#[derive(Clone, Debug)]
pub struct Views<'a> {
    /// Internal buffer view iterator.
    iter: slice::Iter<'a, json::buffer::View>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every camera in a glTF asset.
#[derive(Clone, Debug)]
pub struct Cameras<'a> {
    /// Internal buffer view iterator.
    iter: slice::Iter<'a, json::camera::Camera>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every pre-loaded image in a glTF asset.
#[derive(Clone, Debug)]
pub struct Images<'a> {
    /// Internal image iterator.
    iter: slice::Iter<'a, json::image::Image>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every material in a glTF asset.
#[derive(Clone, Debug)]
pub struct Materials<'a> {
    /// Internal material iterator.
    iter: slice::Iter<'a, json::material::Material>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every mesh in a glTF asset.
#[derive(Clone, Debug)]
pub struct Meshes<'a> {
    /// Internal mesh iterator.
    iter: slice::Iter<'a, json::mesh::Mesh>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every node in a glTF asset.
#[derive(Clone, Debug)]
pub struct Nodes<'a> {
    /// Internal node iterator.
    iter: slice::Iter<'a, json::scene::Node>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every sampler in a glTF asset.
#[derive(Clone, Debug)]
pub struct Samplers<'a> {
    /// Internal sampler iterator.
    iter: slice::Iter<'a, json::texture::Sampler>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every scene in a glTF asset.
#[derive(Clone, Debug)]
pub struct Scenes<'a> {
    /// Internal scene iterator.
    iter: slice::Iter<'a, json::scene::Scene>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every skin in a glTF asset.
#[derive(Clone, Debug)]
pub struct Skins<'a> {
    /// Internal skin iterator.
    iter: slice::Iter<'a, json::skin::Skin>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every texture in a glTF asset.
#[derive(Clone, Debug)]
pub struct Textures<'a> {
    /// Internal texture iterator.
    iter: slice::Iter<'a, json::texture::Texture>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

impl Gltf {
    /// Constructor for a complete glTF asset.
    pub fn new(
        root: root::Root,
        buffer_data: Vec<BufferData>,
        image_data: Vec<ImageData>,
    ) -> Self {
        Self {
            buffer_data: buffer_data,
            image_data: image_data,
            root: root,
        }
    }

    /// Returns an `Iterator` that visits the accessors of the glTF asset.
    pub fn accessors<'a>(&'a self) -> Accessors<'a> {
        Accessors {
            iter: self.as_json().accessors.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the animations of the glTF asset.
    pub fn animations<'a>(&'a self) -> Animations<'a> {
        Animations {
            iter: self.as_json().animations.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffers of the glTF asset.
    pub fn buffers<'a>(&'a self) -> Buffers<'a> {
        Buffers {
            iter: self.as_json().buffers.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffer views of the glTF
    /// asset.
    pub fn views<'a>(&'a self) -> Views<'a> {
        Views {
            iter: self.as_json().buffer_views.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the cameras of the glTF asset.
    pub fn cameras<'a>(&'a self) -> Cameras<'a> {
        Cameras {
            iter: self.as_json().cameras.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded images of the glTF asset.
    pub fn images<'a>(&'a self) -> Images<'a> {
        Images {
            iter: self.as_json().images.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the materials of the glTF asset.
    pub fn materials<'a>(&'a self) -> Materials<'a> {
        Materials {
            iter: self.as_json().materials.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the meshes of the glTF asset.
    pub fn meshes<'a>(&'a self) -> Meshes<'a> {
        Meshes {
            iter: self.as_json().meshes.iter(),
            gltf: self,
        }
    }
    
    /// Returns an `Iterator` that visits the nodes of the glTF asset.
    pub fn nodes<'a>(&'a self) -> Nodes<'a> {
        Nodes {
            iter: self.as_json().nodes.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the scenes of the glTF asset.
    pub fn samplers<'a>(&'a self) -> Samplers<'a> {
        Samplers {
            iter: self.as_json().samplers.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the samplers of the glTF asset.
    pub fn scenes<'a>(&'a self) -> Scenes<'a> {
        Scenes {
            iter: self.as_json().scenes.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the skins of the glTF asset.
    pub fn skins<'a>(&'a self) -> Skins<'a> {
        Skins {
            iter: self.as_json().skins.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the textures of the glTF asset.
    pub fn textures<'a>(&'a self) -> Textures<'a> {
        Textures {
            iter: self.as_json().textures.iter(),
            gltf: self,
        }
    }
}

impl Deref for Gltf {
    type Target = root::Root;
    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl<'a> Iterator for Accessors<'a> {
    type Item = Accessor<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Accessor::new(self.gltf, json))
    }
}

impl<'a> Iterator for Animations<'a> {
    type Item = Animation<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Animation::new(self.gltf, json))
    }
}

impl<'a> Iterator for Buffers<'a> {
    type Item = Buffer<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Buffer::new(self.gltf, json))
    }
}

impl<'a> Iterator for Views<'a> {
    type Item = View<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| View::new(self.gltf, json))
    }
}

impl<'a> Iterator for Cameras<'a> {
    type Item = Camera<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Camera::new(self.gltf, json))
    }
}

impl<'a> Iterator for Images<'a> {
    type Item = Image<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Image::new(self.gltf, json))
    }
}

impl<'a> Iterator for Materials<'a> {
    type Item = Material<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Material::new(self.gltf, json))
    }
}

impl<'a> Iterator for Meshes<'a> {
    type Item = Mesh<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Mesh::new(self.gltf, json))
    }
}

impl<'a> Iterator for Nodes<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Node::new(self.gltf, json))
    }
}

impl<'a> Iterator for Samplers<'a> {
    type Item = Sampler<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Sampler::new(self.gltf, json))
    }
}

impl<'a> Iterator for Scenes<'a> {
    type Item = Scene<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Scene::new(self.gltf, json))
    }
}

impl<'a> Iterator for Skins<'a> {
    type Item = Skin<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Skin::new(self.gltf, json))
    }
}

impl<'a> Iterator for Textures<'a> {
    type Item = Texture<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Texture::new(self.gltf, json))
    }
}
