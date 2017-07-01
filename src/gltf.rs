
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::boxed::Box;
use std::iter::Enumerate;
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
pub struct Accessors<'b> {
    /// Internal accessor iterator.
    iter: slice::Iter<'b, json::accessor::Accessor<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every animation in a glTF asset.
#[derive(Clone, Debug)]
pub struct Animations<'b> {
    /// Internal animation iterator.
    iter: slice::Iter<'b, json::animation::Animation<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every pre-loaded buffer in a glTF asset.
#[derive(Clone, Debug)]
pub struct Buffers<'b> {
    /// Internal buffer iterator.
    iter: Enumerate<slice::Iter<'b, json::buffer::Buffer<'b>>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every pre-loaded buffer view in a glTF asset.
#[derive(Clone, Debug)]
pub struct Views<'b> {
    /// Internal buffer view iterator.
    iter: slice::Iter<'b, json::buffer::View<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every camera in a glTF asset.
#[derive(Clone, Debug)]
pub struct Cameras<'b> {
    /// Internal buffer view iterator.
    iter: slice::Iter<'b, json::camera::Camera<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every pre-loaded image in a glTF asset.
#[derive(Clone, Debug)]
pub struct Images<'b> {
    /// Internal image iterator.
    iter: Enumerate<slice::Iter<'b, json::image::Image<'b>>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every material in a glTF asset.
#[derive(Clone, Debug)]
pub struct Materials<'b> {
    /// Internal material iterator.
    iter: slice::Iter<'b, json::material::Material<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every mesh in a glTF asset.
#[derive(Clone, Debug)]
pub struct Meshes<'b> {
    /// Internal mesh iterator.
    iter: slice::Iter<'b, json::mesh::Mesh<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every node in a glTF asset.
#[derive(Clone, Debug)]
pub struct Nodes<'b> {
    /// Internal node iterator.
    iter: slice::Iter<'b, json::scene::Node<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every sampler in a glTF asset.
#[derive(Clone, Debug)]
pub struct Samplers<'b> {
    /// Internal sampler iterator.
    iter: slice::Iter<'b, json::texture::Sampler<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every scene in a glTF asset.
#[derive(Clone, Debug)]
pub struct Scenes<'b> {
    /// Internal scene iterator.
    iter: slice::Iter<'b, json::scene::Scene<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every skin in a glTF asset.
#[derive(Clone, Debug)]
pub struct Skins<'b> {
    /// Internal skin iterator.
    iter: slice::Iter<'b, json::skin::Skin<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
}

/// An `Iterator` that visits every texture in a glTF asset.
#[derive(Clone, Debug)]
pub struct Textures<'b> {
    /// Internal texture iterator.
    iter: slice::Iter<'b, json::texture::Texture<'b>>,

    /// The internal root glTF object.
    gltf: &'b Gltf<'b>,
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

    /// Returns the loaded buffer data for the corresponding indexed glTF buffer.
    fn buffer_data(&self, index: usize) -> &[u8] {
        match self.buffer_data[index] {
            BufferData::Owned(ref slice) => slice,
        }
    }

    /// Returns the loaded buffer view data for the corresponding index glTF buffer
    /// view.
    fn view_data(&self, index: usize) -> &[u8] {
        let ref view = self.as_json().buffer_views[index];
        let begin = view.byte_offset as usize;
        let end = begin + view.byte_length as usize;
        let data = self.buffer_data(view.buffer.value());
        &data[begin..end]
    }
    
    /// Returns the loaded image data for the corresponding indexed glTF image.
    fn image_data(&self, index: usize) -> &[u8] {
        match self.image_data[index] {
            ImageData::Borrowed(index) => self.view_data(index),
            ImageData::Owned(ref data) => data,
        }
    }

    /// Returns an `Iterator` that visits the accessors of the glTF asset.
    pub fn accessors<'b>(&'b self) -> Accessors<'b> {
        Accessors {
            iter: self.as_json().accessors.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the animations of the glTF asset.
    pub fn animations<'b>(&'b self) -> Animations<'b> {
        Animations {
            iter: self.as_json().animations.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffers of the glTF asset.
    pub fn buffers<'b>(&'b self) -> Buffers<'b> {
        Buffers {
            iter: self.as_json().buffers.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffer views of the glTF
    /// asset.
    pub fn views<'b>(&'b self) -> Views<'b> {
        Views {
            iter: self.as_json().buffer_views.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the cameras of the glTF asset.
    pub fn cameras<'b>(&'b self) -> Cameras<'b> {
        Cameras {
            iter: self.as_json().cameras.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded images of the glTF asset.
    pub fn images<'b>(&'b self) -> Images<'b> {
        Images {
            iter: self.as_json().images.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the materials of the glTF asset.
    pub fn materials<'b>(&'b self) -> Materials<'b> {
        Materials {
            iter: self.as_json().materials.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the meshes of the glTF asset.
    pub fn meshes<'b>(&'b self) -> Meshes<'b> {
        Meshes {
            iter: self.as_json().meshes.iter(),
            gltf: self,
        }
    }
    
    /// Returns an `Iterator` that visits the nodes of the glTF asset.
    pub fn nodes<'b>(&'b self) -> Nodes<'b> {
        Nodes {
            iter: self.as_json().nodes.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the scenes of the glTF asset.
    pub fn samplers<'b>(&'b self) -> Samplers<'b> {
        Samplers {
            iter: self.as_json().samplers.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the samplers of the glTF asset.
    pub fn scenes<'b>(&'b self) -> Scenes<'b> {
        Scenes {
            iter: self.as_json().scenes.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the skins of the glTF asset.
    pub fn skins<'b>(&'b self) -> Skins<'b> {
        Skins {
            iter: self.as_json().skins.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the textures of the glTF asset.
    pub fn textures<'b>(&'b self) -> Textures<'b> {
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

impl Iterator for Accessors {
    type Item = Accessor;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Accessor::new(self.gltf, json))
    }
}

impl Iterator for Animations {
    type Item = Animation;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Animation::new(self.gltf, json))
    }
}

impl Iterator for Buffers {
    type Item = Buffer;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, json)| {
            Buffer::new(self.gltf, json, &self.gltf.buffer_data(index))
        })
    }
}

impl Iterator for Views {
    type Item = View;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|json| {
                let buffer_data = self.gltf.buffer_data(json.buffer.value());
                let begin = json.byte_offset as usize;
                let end = begin + json.byte_length as usize;
                View::new(self.gltf, json, &buffer_data[begin..end])
            })
    }
}

impl<'b> Iterator for Cameras<'b> {
    type Item = Camera<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Camera::new(self.gltf, json))
    }
}

impl<'b> Iterator for Images<'b> {
    type Item = Image<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, json)| {
            Image::new(self.gltf, json, &self.gltf.image_data(index))
        })
    }
}

impl<'b> Iterator for Materials<'b> {
    type Item = Material<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Material::new(self.gltf, json))
    }
}

impl<'b> Iterator for Meshes<'b> {
    type Item = Mesh<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Mesh::new(self.gltf, json))
    }
}

impl<'b> Iterator for Nodes<'b> {
    type Item = Node<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Node::new(self.gltf, json))
    }
}

impl<'b> Iterator for Samplers<'b> {
    type Item = Sampler<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Sampler::new(self.gltf, json))
    }
}

impl<'b> Iterator for Scenes<'b> {
    type Item = Scene<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Scene::new(self.gltf, json))
    }
}

impl<'b> Iterator for Skins<'b> {
    type Item = Skin<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Skin::new(self.gltf, json))
    }
}

impl<'b> Iterator for Textures<'b> {
    type Item = Texture<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Texture::new(self.gltf, json))
    }
}
