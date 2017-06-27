
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::marker::PhantomData;

use json::{image, Extras, Index, Root};
use validation::{Error, JsonPath, Validate};

/// Corresponds to `GL_LINEAR`.
pub const LINEAR: u32 = 9729;

/// Corresponds to `GL_NEAREST`.
pub const NEAREST: u32 = 9728;

/// Corresponds to `GL_LINEAR_MIPMAP_LINEAR`.
pub const LINEAR_MIPMAP_LINEAR: u32 = 9987;

/// Corresponds to `GL_LINEAR_MIPMAP_NEAREST`.
pub const LINEAR_MIPMAP_NEAREST: u32 = 9985;

/// Corresponds to `GL_NEAREST_MIPMAP_LINEAR`.
pub const NEAREST_MIPMAP_LINEAR: u32 = 9986;

/// Corresponds to `GL_NEAREST_MIPMAP_NEAREST`.
pub const NEAREST_MIPMAP_NEAREST: u32 = 9984;

/// Corresponds to `GL_CLAMP_TO_EDGE`.
pub const CLAMP_TO_EDGE: u32 = 33071;

/// Corresponds to `GL_MIRRORED_REPEAT`.
pub const MIRRORED_REPEAT: u32 = 33648;

/// Corresponds to `GL_REPEAT`.
pub const REPEAT: u32 = 10497;

/// All valid magnification filters.
pub const VALID_MAG_FILTERS: &'static [u32] = &[
    NEAREST,
    LINEAR,
];

/// All valid minification filters.
pub const VALID_MIN_FILTERS: &'static [u32] = &[
    NEAREST,
    LINEAR,
    NEAREST_MIPMAP_NEAREST,
    LINEAR_MIPMAP_NEAREST,
    NEAREST_MIPMAP_LINEAR,
    LINEAR_MIPMAP_LINEAR,
];

/// All valid wrapping modes.
pub const VALID_WRAPPING_MODES: &'static [u32] = &[
    CLAMP_TO_EDGE,
    MIRRORED_REPEAT,
    REPEAT,
];

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Sampler<'a> {
    /// Magnification filter.
    #[serde(rename = "magFilter")]
    pub mag_filter: Option<MagFilter>,

    /// Minification filter.
    #[serde(rename = "minFilter")]
    pub min_filter: Option<MinFilter>,

    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,

    /// `s` wrapping mode.
    #[serde(default, rename = "wrapS")]
    pub wrap_s: WrappingMode,

    /// `t` wrapping mode.
    #[serde(default, rename = "wrapT")]
    pub wrap_t: WrappingMode,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: SamplerExtensions<'a>,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
}

/// Extension specific data for `Sampler`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct SamplerExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// A texture and its sampler.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Texture<'a> {
    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,

    /// The index of the sampler used by this texture.
    pub sampler: Option<Index<Sampler<'a>>>,

    /// The index of the image used by this texture.
    pub source: Index<image::Image<'a>>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: TextureExtensions<'a>,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
}

/// Extension specific data for `Texture`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct TextureExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
/// Reference to a `Texture`.
pub struct Info<'a> {
    /// The index of the texture.
    pub index: Index<Texture<'a>>,

    /// The set index of the texture's `TEXCOORD` attribute.
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: InfoExtensions<'a>,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
}

/// Extension specific data for `Info`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct InfoExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// Magnification filter.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct MagFilter(pub u32);

/// Minification filter.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct MinFilter(pub u32);

/// Texture co-ordinate wrapping mode.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct WrappingMode(pub u32);

impl<'a> Validate<'a> for MagFilter {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if !VALID_MAG_FILTERS.contains(&self.0) {
            report(Error::invalid_enum(path(), self.0));
        }
    }
}

impl<'a> Validate<'a> for MinFilter {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if !VALID_MIN_FILTERS.contains(&self.0) {
            report(Error::invalid_enum(path(), self.0));
        }
    }
}

impl Default for WrappingMode {
    fn default() -> Self {
        WrappingMode(REPEAT)
    }
}

impl<'a> Validate<'a> for WrappingMode {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if !VALID_WRAPPING_MODES.contains(&self.0) {
            report(Error::invalid_enum(path(), self.0));
        }
    }
}
