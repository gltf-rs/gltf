
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{self, fs, io, path};
use std::slice::Iter as SliceIter;
use v2::{
    accessor,
    animation,
    buffer,
    camera,
    image,
    json,
    material,
    texture,
    scene,
    skin,
    Gltf,
};

use self::accessor::Accessor;
use self::animation::Animation;
use self::buffer::{Buffer, View};
use self::camera::Camera;
use self::image::Image;
use self::material::Material;
use self::scene::{Node, Scene};
use self::skin::Skin;
use self::texture::{Info, Sampler, Texture};

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
    /// Index of next buffer in the iteration.
    index: usize,

    /// The internal root glTF object.
    gltf: &'a Gltf,
}

/// An `Iterator` that visits every pre-loaded buffer view in a glTF asset.
pub struct IterBufferViews<'a> {
    /// Internal buffer view iterator.
    iter: SliceIter<'a, json::buffer::View>,

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
    /// Index of next image in the iteration.
    index: usize,

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

/// The root object for a glTF asset.
pub struct Root<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::root::Root,
}

impl<'a> Root<'a> {
    /// Constructs a `Camera`.
    pub fn new(gltf: &'a Gltf, json: &'a json::root::Root) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }
    
    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::root::Root {
        self.json
    }

    /// Returns the extensions referenced in this .gltf file.
    pub fn extensions_used(&self) -> &[String] {
        &self.json.extensions_used
    }

    /// Returns the extensions required to load and render this asset.
    pub fn extensions_required(&self) -> &[String] {
        &self.json.extensions_required
    }

    /// Returns an `Iterator` that visits the accessors of the glTF asset.
    pub fn iter_accessors(&self) -> IterAccessors<'a> {
        IterAccessors {
            iter: self.json.accessors.iter(),
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the animations of the glTF asset.
    pub fn iter_animations(&self) -> IterAnimations<'a> {
        IterAnimations {
            iter: self.json.animations.iter(),
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffers of the glTF
    /// asset.
    pub fn iter_buffers(&self) -> IterBuffers<'a> {
        IterBuffers {
            index: 0,
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffer views of the glTF
    /// asset.
    pub fn iter_buffer_views(&self) -> IterBufferViews<'a> {
        IterBufferViews {
            iter: self.json.buffer_views.iter(),
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the cameras of the glTF asset.
    pub fn iter_cameras(&self) -> IterCameras<'a> {
        IterCameras {
            iter: self.json.cameras.iter(),
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded images of the glTF asset.
    pub fn iter_images(&self) -> IterImages<'a> {
        IterImages {
            index: 0,
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the materials of the glTF asset.
    pub fn iter_materials(&self) -> IterMaterials<'a> {
        IterMaterials {            
            iter: self.json.materials.iter(),
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the nodes of the glTF asset.
    pub fn iter_nodes(&self) -> IterNodes<'a> {
        IterNodes {            
            iter: self.json.nodes.iter(),
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the scenes of the glTF asset.
    pub fn iter_samplers(&self) -> IterSamplers<'a> {
        IterSamplers {            
            iter: self.json.samplers.iter(),
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the samplers of the glTF asset.
    pub fn iter_scenes(&self) -> IterScenes<'a> {
        IterScenes {            
            iter: self.json.scenes.iter(),
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the skins of the glTF asset.
    pub fn iter_skins(&self) -> IterSkins<'a> {
        IterSkins {            
            iter: self.json.skins.iter(),
            gltf: self.gltf,
        }
    }

    /// Returns an `Iterator` that visits the textures of the glTF asset.
    pub fn iter_textures(&self) -> IterTextures<'a> {
        IterTextures {            
            iter: self.json.textures.iter(),
            gltf: self.gltf,
        }
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
        unimplemented!()
    }
}

impl<'a> Iterator for IterBufferViews<'a> {
    type Item = View<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| View::new(self.gltf, json))
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
        unimplemented!()
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
