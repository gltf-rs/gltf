
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;
use v2::{image, json};

pub enum MagFilter {}
pub enum MinFilter {}
pub enum WrappingMode {}

///  Texture sampler properties for filtering and wrapping modes.
pub struct Sampler<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::texture::Sampler,
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub fn new(gltf: &'a Gltf, json: &'a json::texture::Sampler) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::texture::Sampler {
        self.json
    }

    ///  Magnification filter.
    pub fn mag_filter(&self) -> Option<MagFilter> {
        unimplemented!()
    }

    ///  Minification filter.
    pub fn min_filter(&self) -> Option<MinFilter> {
        unimplemented!()
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> &Option<String> {
        unimplemented!()
    }

    ///  `s` wrapping mode.
    pub fn wrap_s(&self) -> WrappingMode {
        unimplemented!()
    }

    ///  `t` wrapping mode.
    pub fn wrap_t(&self) -> WrappingMode {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::texture::SamplerExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }
}
///  A texture and its sampler.
pub struct Texture<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::texture::Texture,
}

impl<'a> Texture<'a> {
    /// Constructs a `Texture`.
    pub fn new(gltf: &'a Gltf, json: &'a json::texture::Texture) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::texture::Texture {
        self.json
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> &Option<String> {
        unimplemented!()
    }

    ///  The index of the sampler used by this texture.
    pub fn sampler(&self) -> Option<Sampler<'a>> {
        unimplemented!()
    }

    ///  The index of the image used by this texture.
    pub fn source(&self) -> image::Image<'a> {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::texture::TextureExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }
}
///  Reference to a `Texture`.
pub struct Info<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::texture::Info,
}

impl<'a> Info<'a> {
    /// Constructs a `Info`.
    pub fn new(gltf: &'a Gltf, json: &'a json::texture::Info) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::texture::Info {
        self.json
    }

    ///  The index of the texture.
    pub fn index(&self) -> ! {
        unimplemented!()
    }

    ///  The set index of the texture's `TEXCOORD` attribute.
    pub fn tex_coord(&self) -> u32 {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::texture::InfoExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }
}
