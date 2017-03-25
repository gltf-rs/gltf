
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Extensions;
use traits::Extras;
use v2::{buffer, Index};

/// Image data used to create a texture
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image<E: Extras> {
    /// The index of the `BufferView` that contains the image
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<Index<buffer::BufferView<E>>>,
    
    /// The image's MIME type
    // N.B. The spec says this is required but the sample models don't provide it
    // TODO: Remove `Option` as necessary
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    
    /// Optional user-defined name for this object
    pub name: Option<String>,
    
    /// The uniform resource identifier of the image relative to the .gltf file
    pub uri: Option<String>,
    
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: Extensions,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Image,
}

/// [Defines texture sampler properties for filtering and wrapping modes]
/// (https://github.com/KhronosGroup/glTF/blob/d63b796e6b7f6b084c710b97b048d59d749cb04a/specification/2.0/schema/sampler.schema.json)
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
    
    /// s wrapping mode
    #[serde(default, rename = "wrapS")]
    pub wrap_s: WrappingMode,
    
    /// t wrapping mode
    #[serde(default, rename = "wrapT")]
    pub wrap_t: WrappingMode,
    
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: Extensions,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Sampler,
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
    WrappingMode {
        ClampToEdge = 33071,
        MirroredRepeat = 33648,
        Repeat = 10497,
    }
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
    pub source: Index<Image<E>>,
    
    /// The target the texture should be bound to
    #[serde(default)]
    pub target: Target,
    
    /// Optional data targeting official extensions
    #[serde(default)]
    pub extensions: Extensions,
    
    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Texture,
}

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
    Target {
        Texture2d = 3553,
    }
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
    
    // TODO: Add extensions and extras
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

impl Default for WrappingMode {
    fn default() -> Self {
        WrappingMode::Repeat
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

impl Default for Target {
    fn default() -> Self {
        Target::Texture2d
    }
}
