// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum_number! {
    Filter {
        Nearest = 9728,
        Linear = 9729,
        NearestMipmapNearest = 9984,
        LinearMipmapNearest = 9985,
        NearestMipmapLinear = 9986,
        LinearMipmapLinear = 9987,
    }
}

impl Default for Filter {
    fn default() -> Filter {
        Filter::Nearest
    }
}

enum_number! {
    Wrap {
        Repeat = 10497,
        ClampToEdge = 33071,
        MirroredRepeat = 33648,
    }
}

impl Default for Wrap {
    fn default() -> Wrap {
        Wrap::Repeat
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

impl Default for Format {
    fn default() -> Format {
        Format::Rgba
    }
}

enum_number! {
    Target {
        Texture2d = 3553,
    }
}

impl Default for Target {
    fn default() -> Target {
        Target::Texture2d
    }
}

enum_number! {
    TexelType {
        U8 = 5121,
        U16R5G6B5 = 33635,
        U16R4G4B4A4 = 32819,
        U16R5G5B5A1 = 32820,
    }
}

impl Default for TexelType {
    fn default() -> TexelType {
        TexelType::U8
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Texture {
    /// The texture's format.
    #[serde(default)]
    pub format: Format,

    /// The texture's internal format.
    #[serde(rename = "internalFormat")]
    #[serde(default)]
    pub internal_format: Format,

    /// The ID of the sampler used by this texture.
    pub sampler: String,

    /// The ID of the image used by this texture.
    pub source: String,

    /// The target that the WebGL texture should be bound to.
    #[serde(default)]
    pub target: Target,

    /// Texel datatype.
    #[serde(rename = "type")]
    #[serde(default)]
    pub kind: TexelType,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a texture and a buffer could have
    /// the same name, or two textures could even have the same name.
    pub name: Option<String>,
}
