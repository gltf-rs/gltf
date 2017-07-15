
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {json, texture, Gltf};

///  The material appearance of a primitive.
pub struct Material<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::material::Material,
}

impl<'a> Material<'a> {
    /// Constructs a `Material`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::material::Material) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::material::Material {
        self.json
    }
}
///  A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
pub struct PbrMetallicRoughness<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::material::PbrMetallicRoughness,
}

impl<'a> PbrMetallicRoughness<'a> {
    /// Constructs a `PbrMetallicRoughness`.
    pub fn new(
        gltf: &'a Gltf,
        json: &'a json::extensions::material::PbrMetallicRoughness,
    ) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::extensions::material::PbrMetallicRoughness {
        self.json
    }
}

///  Defines the normal texture of a material.
pub struct NormalTexture<'a> {
    /// The parent `Texture` struct.
    #[allow(dead_code)]
    texture: texture::Texture<'a>,

    /// The corresponding JSON struct.
    json: &'a json::extensions::material::NormalTexture,
}

impl<'a> NormalTexture<'a> {
    /// Constructs a `NormalTexture`.
    pub fn new(
        texture: texture::Texture<'a>,
        json: &'a json::extensions::material::NormalTexture,
    ) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::material::NormalTexture {
        self.json
    }
}
///  Defines the occlusion texture of a material.
pub struct OcclusionTexture<'a> {
    /// The parent `Texture` struct.
    #[allow(dead_code)]
    texture: texture::Texture<'a>,

    /// The corresponding JSON struct.
    json: &'a json::extensions::material::OcclusionTexture,
}

impl<'a> OcclusionTexture<'a> {
    /// Constructs a `OcclusionTexture`.
    pub fn new(
        texture: texture::Texture<'a>,
        json: &'a json::extensions::material::OcclusionTexture,
    ) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::material::OcclusionTexture {
        self.json
    }
}
