
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{image, Extras, Index, Root};

enum_number! {
    DataType {
        U8 = 5121,
        U16R5G6B5 = 33635,
        U16R4G4B4A4 = 32819,
        U16R5G5B5A1 = 32820,
    }
}

enum_number! {
    Format {
        Alpha = 6406,
        Rgb = 6407,
        Rgba = 6408,
        Luminance = 6409,
        LuminanceAlpha = 6410,
    }
}

enum_number! {
    MagFilter {
        Nearest = 9728,
        Linear = 9729,
    }
}

enum_number! {
    MinFilter {
        Nearest = 9728,
        Linear = 9729,
        NearestMipmapNearest = 9984,
        LinearMipmapNearest = 9985,
        NearestMipmapLinear = 9986,
        LinearMipmapLinear = 9987,
    }
}

enum_number! {
    Target {
        Texture2d = 3553,
    }
}

enum_number! {
    WrappingMode {
        ClampToEdge = 33071,
        MirroredRepeat = 33648,
        Repeat = 10497,
    }
}

/// Texture sampler properties for filtering and wrapping modes
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sampler<E: Extras> {
    /// Magnification filter
    #[serde(default, rename = "magFilter")]
    pub mag_filter: MagFilter,

    /// Minification filter
    #[serde(default, rename = "minFilter")]
    pub min_filter: MinFilter,

    /// Optional user-defined name for this object
    pub name: Option<String>,

    /// `s` wrapping mode
    #[serde(default, rename = "wrapS")]
    pub wrap_s: WrappingMode,

    /// `t` wrapping mode
    #[serde(default, rename = "wrapT")]
    pub wrap_t: WrappingMode,

    /// Extension specific data
    #[serde(default)]
    pub extensions: SamplerExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Sampler,
}

/// Extension specific data for `Sampler`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SamplerExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Texture<E: Extras> {
    /// Texel data type
    #[serde(default, rename = "type")]
    pub data_type: DataType,

    /// Optional user-defined name for this object
    pub name: Option<String>,

    /// The texture format
    #[serde(default)]
    pub format: Format,

    /// The texture internal format
    #[serde(default, rename = "internalFormat")]
    pub internal_format: Format,

    /// The index of the sampler used by this texture
    pub sampler: Index<Sampler<E>>,

    /// The index of the image used by this texture
    pub source: Index<image::Image<E>>,

    /// The target the texture should be bound to
    #[serde(default)]
    pub target: Target,

    /// Extension specific data
    #[serde(default)]
    pub extensions: TextureExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Texture,
}

/// Extension specific data for `Texture`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextureExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
/// Reference to a `Texture`
pub struct TextureInfo<E: Extras> {
    /// The index of the texture
    pub index: Index<Texture<E>>,

    /// The set index of the texture's `TEXCOORD` attribute
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,

    /// Extension specific data
    #[serde(default)]
    pub extensions: TextureInfoExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::TextureInfo,
}

/// Extension specific data for `TextureInfo`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextureInfoExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl<E: Extras> Sampler<E> {
    #[doc(hidden)]
    pub fn range_check(&self, _root: &Root<E>) -> Result<(), ()> {
        Ok(())
    }
}

impl<E: Extras> Texture<E> {
    #[doc(hidden)]
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
        let _ = root.try_get(&self.sampler)?;
        let _ = root.try_get(&self.source)?;
        Ok(())
    }
}

impl Default for DataType {
    fn default() -> Self {
        DataType::U8
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Rgba
    }
}

impl Default for MagFilter {
    fn default() -> Self {
        MagFilter::Linear
    }
}

impl Default for MinFilter {
    fn default() -> Self {
        MinFilter::NearestMipmapLinear
    }
}

impl Default for Target {
    fn default() -> Self {
        Target::Texture2d
    }
}

impl Default for WrappingMode {
    fn default() -> Self {
        WrappingMode::Repeat
    }
}
