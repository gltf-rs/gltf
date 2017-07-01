
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::ops::Deref;
use {image, json, Gltf};

/// Magnification filter.
pub enum MagFilter {
    /// Corresponds to `GL_NEAREST`.
    Linear = 9729,

    /// Corresponds to `GL_LINEAR`.
    Nearest = 9728,
}

/// Minification filter.
pub enum MinFilter {
    /// Corresponds to `GL_NEAREST`.
    Linear = 9729,

    /// Corresponds to `GL_LINEAR`.
    Nearest = 9728,

    /// Corresponds to `GL_LINEAR_MIPMAP_LINEAR`.
    LinearMipmapLinear = 9987,

    /// Corresponds to `GL_LINEAR_MIPMAP_NEAREST`.
    LinearMipmapNearest = 9985,

    /// Corresponds to `GL_NEAREST_MIPMAP_LINEAR`.
    NearestMipmapLinear = 9986,

    /// Corresponds to `GL_NEAREST_MIPMAP_NEAREST`.
    NearestMipmapNearest = 9984,
}

/// Texture co-ordinate wrapping mode.
pub enum WrappingMode {
    /// Corresponds to `GL_CLAMP_TO_EDGE`.
    ClampToEdge = 33071,

    /// Corresponds to `GL_MIRRORED_REPEAT`.
    MirroredRepeat = 33648,

    /// Corresponds to `GL_REPEAT`.
    Repeat = 10497,
}

///  Texture sampler properties for filtering and wrapping modes.
pub struct Sampler {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::texture::Sampler,
}

impl Sampler {
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

    /// Magnification filter.
    pub fn mag_filter(&self) -> Option<MagFilter> {
        use self::MagFilter::*;
        self.json.mag_filter.map(|x| match x.0 {
            json::texture::LINEAR => Linear,
            json::texture::NEAREST => Nearest,
            _ => unreachable!(),
        })
    }

    /// Minification filter.
    pub fn min_filter(&self) -> Option<MinFilter> {
        use self::MinFilter::*;
        self.json.min_filter.map(|x| match x.0 {
            json::texture::LINEAR => Linear,
            json::texture::NEAREST => Nearest,    
            json::texture::LINEAR_MIPMAP_LINEAR => LinearMipmapLinear,
            json::texture::LINEAR_MIPMAP_NEAREST => LinearMipmapNearest,
            json::texture::NEAREST_MIPMAP_LINEAR => NearestMipmapLinear,
            json::texture::NEAREST_MIPMAP_NEAREST => NearestMipmapNearest,
            _ => unreachable!(),
        })
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
    }

    /// `s` wrapping mode.
    pub fn wrap_s(&self) -> WrappingMode {
        use self::WrappingMode::*;
        match self.json.wrap_s.0 {
            json::texture::CLAMP_TO_EDGE => ClampToEdge,
            json::texture::MIRRORED_REPEAT => MirroredRepeat,
            json::texture::REPEAT => Repeat,
            _ => unreachable!(),
        }
    }

    /// `t` wrapping mode.
    pub fn wrap_t(&self) -> WrappingMode {
        use self::WrappingMode::*;
        match self.json.wrap_t.0 {
            json::texture::CLAMP_TO_EDGE => ClampToEdge,
            json::texture::MIRRORED_REPEAT => MirroredRepeat,
            json::texture::REPEAT => Repeat,
            _ => unreachable!(),
        }
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::texture::SamplerExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
/// A texture and its sampler.
pub struct Texture {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::texture::Texture,
}

impl Texture {
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

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
    }

    /// The index of the sampler used by this texture.
    pub fn sampler(&self) -> Option<Sampler> {
        self.json.sampler.as_ref().map(|index| {
            self.gltf.samplers().nth(index.value() as usize).unwrap()
        })
    }

    /// The index of the image used by this texture.
    pub fn source(&self) -> image::Image {
        self.gltf.images().nth(self.json.source.value() as usize).unwrap()
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::texture::TextureExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
/// A reference to a `Texture`.
pub struct Info {
    /// The parent `Texture` struct.
    texture: Texture,

    /// The corresponding JSON struct.
    json: &'a json::texture::Info,
}

impl Info {
    /// Constructs a reference to a `Texture`.
    pub fn new(texture: Texture, json: &'a json::texture::Info) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::texture::Info {
        self.json
    }

    /// The set index of the texture's `TEXCOORD` attribute.
    pub fn tex_coord(&self) -> u32 {
        self.json.tex_coord
    }

    /// Extension specific data.
    pub fn extensions(&self) -> &json::texture::InfoExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl Deref for Info {
    type Target = Texture;
    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}
