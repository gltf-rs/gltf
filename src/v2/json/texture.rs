
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::json::{image, Extras, Index};

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sampler {
    /// Magnification filter.
    #[serde(rename = "magFilter")]
    pub mag_filter: Option<u32>,

    /// Minification filter.
    #[serde(rename = "minFilter")]
    pub min_filter: Option<u32>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// `s` wrapping mode.
    #[serde(default = "sampler_wrap_s_default", rename = "wrapS")]
    pub wrap_s: u32,

    /// `t` wrapping mode.
    #[serde(default = "sampler_wrap_t_default", rename = "wrapT")]
    pub wrap_t: u32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: SamplerExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

fn sampler_wrap_s_default() -> u32 {
    10497
}

fn sampler_wrap_t_default() -> u32 {
    10497
}

/// Extension specific data for `Sampler`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SamplerExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Texture {
    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The index of the sampler used by this texture.
    pub sampler: Option<Index<Sampler>>,

    /// The index of the image used by this texture.
    pub source: Index<image::Image>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: TextureExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Texture`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextureExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
/// Reference to a `Texture`.
pub struct Info {
    /// The index of the texture.
    pub index: Index<Texture>,

    /// The set index of the texture's `TEXCOORD` attribute.
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: InfoExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Info`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InfoExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}
