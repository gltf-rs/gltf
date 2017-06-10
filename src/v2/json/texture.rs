
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::json::{image, Extras, Index, Root};

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sampler {
    /// Magnification filter.
    #[serde(default = "sampler_mag_filter_default", rename = "magFilter")]
    pub mag_filter: u32,

    /// Minification filter.
    #[serde(default = "sampler_min_filter_default", rename = "minFilter")]
    pub min_filter: u32,

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

fn sampler_mag_filter_default() -> u32 {
    9729
}

fn sampler_min_filter_default() -> u32 {
    9986
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
    _allow_extra_fields: (),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Texture {
    /// Texel data type.
    #[serde(default = "texture_type_default", rename = "type")]
    pub type_: u32,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The texture format.
    #[serde(default = "texture_format_default")]
    pub format: u32,

    /// The texture internal format.
    #[serde(default = "texture_internal_format_default", rename = "internalFormat")]
    pub internal_format: u32,

    /// The index of the sampler used by this texture.
    pub sampler: Option<Index<Sampler>>,

    /// The index of the image used by this texture.
    pub source: Index<image::Image>,

    /// The target the texture should be bound to.
    #[serde(default = "texture_target_default")]
    pub target: u32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: TextureExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

fn texture_format_default() -> u32 {
    6408
}

fn texture_internal_format_default() -> u32 {
    6408
}

fn texture_target_default() -> u32 {
    3553
}

fn texture_type_default() -> u32 {
    5121
}

/// Extension specific data for `Texture`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextureExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
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
    _allow_extra_fields: (),
}

impl Sampler {
    #[doc(hidden)]
    pub fn range_check(&self, _root: &Root) -> Result<(), ()> {
        Ok(())
    }
}

impl Texture {
    #[doc(hidden)]
    pub fn range_check(&self, root: &Root) -> Result<(), ()> {
        if let Some(ref sampler) = self.sampler {
            let _ = root.try_get(sampler)?;
        }
        let _ = root.try_get(&self.source)?;
        Ok(())
    }
}
