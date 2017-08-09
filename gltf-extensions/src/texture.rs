
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {json, texture, Gltf};

///  Texture sampler properties for filtering and wrapping modes.
pub struct Sampler<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::texture::Sampler,
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::texture::Sampler) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::texture::Sampler {
        self.json
    }
}
/// A texture and its sampler.
pub struct Texture<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::texture::Texture,
}

impl<'a> Texture<'a> {
    /// Constructs a `Texture`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::texture::Texture) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::texture::Texture {
        self.json
    }
}

/// A reference to a `Texture`.
pub struct Info<'a> {
    /// The parent `Texture` struct.
    #[allow(dead_code)]
    texture: texture::Texture<'a>,

    /// The corresponding JSON struct.
    json: &'a json::extensions::texture::Info,
}

impl<'a> Info<'a> {
    /// Constructs a reference to a `Texture`.
    pub fn new(texture: texture::Texture<'a>, json: &'a json::extensions::texture::Info) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::texture::Info {
        self.json
    }
}
