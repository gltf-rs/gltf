
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use import;
use json;
use root;
use std::{fmt, iter, ops, slice};

use futures::future::{Shared, SharedError, SharedItem};
use futures::{BoxFuture, Future, Poll};

use std::boxed::Box;

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

#[derive(Clone, Copy, Debug)]
enum Region {
    Full,
    View {
        offset: usize,
        len: usize,
    },
}

impl Region {
    pub fn subview(self, offset: usize, len: usize) -> Region {
        match self {
            Region::Full => {
                Region::View {
                    offset: offset,
                    len: len,
                }
            },
            Region::View {
                offset: prev_offset,
                len: _,
            } => {
                Region::View {
                    offset: prev_offset + offset,
                    len: len,
                }
            },
        }
    }
}

/// A `Future` that drives the acquisition of glTF data.
#[derive(Clone)]
pub struct AsyncData {
    future: Shared<BoxFuture<Box<[u8]>, import::Error>>,
    region: Region,
}

/// Error that could occur during asynchronous imports.
pub type AsyncError = SharedError<import::Error>;

impl AsyncData {
    pub fn full(future: Shared<BoxFuture<Box<[u8]>, import::Error>>) -> Self {
        AsyncData {
            future: future,
            region: Region::Full,
        }
    }

    pub fn view(self, offset: usize, len: usize) -> Self {
        AsyncData {
            future: self.future,
            region: self.region.subview(offset, len),
        }
    }
}

impl Future for AsyncData {
    type Item = Data;
    type Error = AsyncError;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future
            .poll()
            .map(|async| {
                async.map(|item| {
                    match self.region {
                        Region::Full => Data::full(item),
                        Region::View { offset, len } => Data::view(item, offset, len),
                    }
                })
            })
    }
}

/// Concrete and thread-safe glTF data.
///
/// May represent `Buffer`, `View`, or `Image` data.
#[derive(Clone, Debug)]
pub struct Data {
    /// The resolved data from a `future::Shared`.
    item: SharedItem<Box<[u8]>>,

    /// The byte region the data reads from.
    region: Region,
}

impl Data {
    /// Constructs concrete and thread-safe glTF data.
    pub fn full(item: SharedItem<Box<[u8]>>) -> Self {
        Data {
            item: item,
            region: Region::Full,
        }
    }

    pub fn view(item: SharedItem<Box<[u8]>>, offset: usize, len: usize) -> Self {
        Data {
            item: item,
            region: Region::View { offset, len },
        }
    }
}

impl ops::Deref for Data {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self.region {
            Region::Full => &self.item[..],
            Region::View { offset, len } => &self.item[offset..(offset + len)],
        }
    }
}

/// A loaded glTF complete with its data.
#[derive(Clone)]
pub struct Gltf {
    /// The glTF buffer data.
    buffers: Vec<AsyncData>,

    /// The glTF image data.
    images: Vec<AsyncData>,

    /// The root glTF struct (and also `Deref` target).
    root: root::Root,
}

impl fmt::Debug for Gltf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.root)
    }
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

/// An `Iterator` that visits every buffer in a glTF asset.
#[derive(Clone, Debug)]
pub struct Buffers<'a> {
    /// Internal buffer iterator.
    iter: iter::Enumerate<slice::Iter<'a, json::buffer::Buffer>>,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every buffer view in a glTF asset.
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
    iter: iter::Enumerate<slice::Iter<'a, json::image::Image>>,

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
    /// Constructor for a complete lazy-loaded glTF asset.
    pub fn new(
        root: root::Root,
        buffers: Vec<AsyncData>,
        images: Vec<AsyncData>,
    ) -> Self {
        Self {
            buffers: buffers,
            images: images,
            root: root,
        }
    }

    /// Returns a shared `Future` that drives the lazy loading of buffer data.
    ///
    /// # Panics
    ///
    /// * If `index` is out of range.
    fn buffer_data<'a>(
        &'a self,
        index: usize,
    ) -> &'a AsyncData {
        &self.buffers[index]
    }

    /// Returns a shared `Future` that drives the lazy loading of image data.
    ///
    /// # Panics
    ///
    /// * If `index` is out of range.
    fn image_data<'a>(
        &'a self,
        index: usize,
    ) -> &'a AsyncData {
        &self.images[index]
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
            iter: self.as_json().buffers.iter().enumerate(),
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
            iter: self.as_json().images.iter().enumerate(),
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

impl ops::Deref for Gltf {
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
        self.iter.next().map(|(index, json)| {
            Buffer::new(self.gltf, json, self.gltf.buffer_data(index))
        })
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
        self.iter.next().map(|(index, json)| {
            Image::new(self.gltf, json, self.gltf.image_data(index))
        })
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
