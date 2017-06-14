
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::iter::Enumerate;
use std::ops::Deref;
use std::slice::Iter as SliceIter;
use v2::{
    accessor,
    animation,
    buffer,
    camera,
    image,
    json,
    material,
    root,
    scene,
    skin,
    texture,
};

use self::accessor::Accessor;
use self::animation::Animation;
use self::buffer::{Buffer, View};
use self::camera::Camera;
use self::image::Image;
use self::material::Material;
use self::scene::{Node, Scene};
use self::skin::Skin;
use self::texture::{Sampler, Texture};

/// A complete glTF asset.
#[derive(Debug)]
pub struct Gltf {
    /// The loaded buffer data.
    buffer_data: Vec<BufferData>,

    /// The loaded image data.
    image_data: Vec<ImageData>,
    
    /// The root glTF JSON struct and `Deref` target.
    root: root::Root,
}

/// Contains all buffer data required to render a single glTF asset.
#[derive(Clone, Debug)]
pub struct BufferData(pub Vec<u8>);

/// Contains all image data required to render a single glTF asset.
#[derive(Clone, Debug)]
pub enum ImageData {
    /// The image data is borrowed from a buffer.
    Borrowed(usize),

    /// The image data is owned.
    Owned(Vec<u8>),
}

/// An `Iterator` that visits every accessor in a glTF asset.
pub struct IterAccessors<'a> {
    /// Internal accessor iterator.
    iter: SliceIter<'a, json::accessor::Accessor>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every animation in a glTF asset.
pub struct IterAnimations<'a> {
    /// Internal animation iterator.
    iter: SliceIter<'a, json::animation::Animation>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every pre-loaded buffer in a glTF asset.
pub struct IterBuffers<'a> {
    /// Internal buffer iterator.
    iter: Enumerate<SliceIter<'a, json::buffer::Buffer>>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every pre-loaded buffer view in a glTF asset.
pub struct IterViews<'a> {
    /// Internal buffer view iterator.
    iter: Enumerate<SliceIter<'a, json::buffer::View>>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every camera in a glTF asset.
pub struct IterCameras<'a> {
    /// Internal buffer view iterator.
    iter: SliceIter<'a, json::camera::Camera>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every pre-loaded image in a glTF asset.
pub struct IterImages<'a> {
    /// Internal image iterator.
    iter: Enumerate<SliceIter<'a, json::image::Image>>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every material in a glTF asset.
pub struct IterMaterials<'a> {
    /// Internal material iterator.
    iter: SliceIter<'a, json::material::Material>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every node in a glTF asset.
pub struct IterNodes<'a> {
    /// Internal node iterator.
    iter: SliceIter<'a, json::scene::Node>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every sampler in a glTF asset.
pub struct IterSamplers<'a> {
    /// Internal sampler iterator.
    iter: SliceIter<'a, json::texture::Sampler>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every scene in a glTF asset.
pub struct IterScenes<'a> {
    /// Internal scene iterator.
    iter: SliceIter<'a, json::scene::Scene>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every skin in a glTF asset.
pub struct IterSkins<'a> {
    /// Internal skin iterator.
    iter: SliceIter<'a, json::skin::Skin>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every texture in a glTF asset.
pub struct IterTextures<'a> {
    /// Internal texture iterator.
    iter: SliceIter<'a, json::texture::Texture>,

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
        debug_assert!(root.as_json().buffers.len() == buffer_data.len());
        debug_assert!(root.as_json().images.len() == image_data.len());
        Gltf {
            buffer_data: buffer_data,
            image_data: image_data,
            root: root,
        }
    }

    /// Transfers ownership of the loaded glTF data to the caller.
    pub fn take(self) -> (root::Root, Vec<BufferData>, Vec<ImageData>) {
        (self.root, self.buffer_data, self.image_data)
    }
    
    /// Returns the loaded buffer data.
    pub fn buffer_data(&self) -> &[BufferData] {
        &self.buffer_data
    }

    /// Returns the loaded image data.
    pub fn image_data(&self) -> &[ImageData] {
        &self.image_data
    }

    /// Returns an `Iterator` that visits the accessors of the glTF asset.
    pub fn iter_accessors<'a>(&'a self) -> IterAccessors<'a> {
        IterAccessors {
            iter: self.as_json().accessors.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the animations of the glTF asset.
    pub fn iter_animations<'a>(&'a self) -> IterAnimations<'a> {
        IterAnimations {
            iter: self.as_json().animations.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffers of the glTF asset.
    pub fn iter_buffers<'a>(&'a self) -> IterBuffers<'a> {
        IterBuffers {
            iter: self.as_json().buffers.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffer views of the glTF
    /// asset.
    pub fn iter_views<'a>(&'a self) -> IterViews<'a> {
        IterViews {
            iter: self.as_json().buffer_views.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the cameras of the glTF asset.
    pub fn iter_cameras<'a>(&'a self) -> IterCameras<'a> {
        IterCameras {
            iter: self.as_json().cameras.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded images of the glTF asset.
    pub fn iter_images<'a>(&'a self) -> IterImages<'a> {
        IterImages {
            iter: self.as_json().images.iter().enumerate(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the materials of the glTF asset.
    pub fn iter_materials<'a>(&'a self) -> IterMaterials<'a> {
        IterMaterials {            
            iter: self.as_json().materials.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the nodes of the glTF asset.
    pub fn iter_nodes<'a>(&'a self) -> IterNodes<'a> {
        IterNodes {            
            iter: self.as_json().nodes.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the scenes of the glTF asset.
    pub fn iter_samplers<'a>(&'a self) -> IterSamplers<'a> {
        IterSamplers {            
            iter: self.as_json().samplers.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the samplers of the glTF asset.
    pub fn iter_scenes<'a>(&'a self) -> IterScenes<'a> {
        IterScenes {            
            iter: self.as_json().scenes.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the skins of the glTF asset.
    pub fn iter_skins<'a>(&'a self) -> IterSkins<'a> {
        IterSkins {            
            iter: self.as_json().skins.iter(),
            gltf: self,
        }
    }

    /// Returns an `Iterator` that visits the textures of the glTF asset.
    pub fn iter_textures<'a>(&'a self) -> IterTextures<'a> {
        IterTextures {            
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

impl<'a> Iterator for IterAccessors<'a> {
    type Item = Accessor<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Accessor::new(self.gltf, json))
    }
}

impl<'a> Iterator for IterAnimations<'a> {
    type Item = Animation<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Animation::new(self.gltf, json))
    }
}

impl<'a> Iterator for IterBuffers<'a> {
    type Item = Buffer<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, json)| {
            Buffer::new(self.gltf, json, self.gltf.buffer_data[index].0.as_slice())
        })
    }
}

impl<'a> Iterator for IterViews<'a> {
    type Item = View<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, json)| {
            let buffer_data = self.gltf.buffer_data[index].0.as_slice();
            let begin = json.byte_offset as usize;
            let end = begin + json.byte_length as usize;
            View::new(self.gltf, json, &buffer_data[begin..end])
        })
    }
}

impl<'a> Iterator for IterCameras<'a> {
    type Item = Camera<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Camera::new(self.gltf, json))
    }
}

impl<'a> Iterator for IterImages<'a> {
    type Item = Image<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, json)| {
            use self::ImageData::*;
            let slice = match self.gltf.image_data()[index] {
                Owned(ref data) => data.as_slice(),
                Borrowed(index) => self.gltf.buffer_data()[index].0.as_slice(),
            };
            Image::new(self.gltf, json, slice)
        })
    }
}

impl<'a> Iterator for IterMaterials<'a> {
    type Item = Material<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Material::new(self.gltf, json))
    }
}

impl<'a> Iterator for IterNodes<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Node::new(self.gltf, json))
    }
}

impl<'a> Iterator for IterSamplers<'a> {
    type Item = Sampler<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Sampler::new(self.gltf, json))
    }
}

impl<'a> Iterator for IterScenes<'a> {
    type Item = Scene<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Scene::new(self.gltf, json))
    }
}

impl<'a> Iterator for IterSkins<'a> {
    type Item = Skin<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Skin::new(self.gltf, json))
    }
}

impl<'a> Iterator for IterTextures<'a> {
    type Item = Texture<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Texture::new(self.gltf, json))
    }
}
