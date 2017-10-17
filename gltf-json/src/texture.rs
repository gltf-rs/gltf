use serde::de;
use std::fmt;
use validation::Checked;
use {extensions, image, Extras, Index};

/// Corresponds to `GL_NEAREST`.
pub const NEAREST: u32 = 9728;

/// Corresponds to `GL_LINEAR`.
pub const LINEAR: u32 = 9729;

/// Corresponds to `GL_NEAREST_MIPMAP_NEAREST`.
pub const NEAREST_MIPMAP_NEAREST: u32 = 9984;

/// Corresponds to `GL_LINEAR_MIPMAP_NEAREST`.
pub const LINEAR_MIPMAP_NEAREST: u32 = 9985;

/// Corresponds to `GL_NEAREST_MIPMAP_LINEAR`.
pub const NEAREST_MIPMAP_LINEAR: u32 = 9986;

/// Corresponds to `GL_LINEAR_MIPMAP_LINEAR`.
pub const LINEAR_MIPMAP_LINEAR: u32 = 9987;

/// Corresponds to `GL_CLAMP_TO_EDGE`.
pub const CLAMP_TO_EDGE: u32 = 33_071;

/// Corresponds to `GL_MIRRORED_REPEAT`.
pub const MIRRORED_REPEAT: u32 = 33_648;

/// Corresponds to `GL_REPEAT`.
pub const REPEAT: u32 = 10_497;

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

/// Magnification filter.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum MagFilter {
    /// Corresponds to `GL_NEAREST`.
    Nearest = 1,

    /// Corresponds to `GL_LINEAR`.
    Linear,
}

impl MagFilter {
    /// OpenGL enum
    pub fn as_gl_enum(&self) -> i32 {
        match *self {
            MagFilter::Nearest => NEAREST as i32,
            MagFilter::Linear => LINEAR as i32,
        }
    }
}

/// Minification filter.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum MinFilter {
    /// Corresponds to `GL_NEAREST`.
    Nearest = 1,

    /// Corresponds to `GL_LINEAR`.
    Linear,

    /// Corresponds to `GL_NEAREST_MIPMAP_NEAREST`.
    NearestMipmapNearest,

    /// Corresponds to `GL_LINEAR_MIPMAP_NEAREST`.
    LinearMipmapNearest,

    /// Corresponds to `GL_NEAREST_MIPMAP_LINEAR`.
    NearestMipmapLinear,

    /// Corresponds to `GL_LINEAR_MIPMAP_LINEAR`.
    LinearMipmapLinear,
}

impl MinFilter {
    /// Returns the corresponding OpenGL enum value.
    pub fn as_gl_enum(&self) -> i32 {
        match *self {
            MinFilter::Nearest => NEAREST as i32,
            MinFilter::Linear => LINEAR as i32,
            MinFilter::NearestMipmapNearest => NEAREST_MIPMAP_NEAREST as i32,
            MinFilter::LinearMipmapNearest => LINEAR_MIPMAP_NEAREST as i32,
            MinFilter::NearestMipmapLinear => NEAREST_MIPMAP_LINEAR as i32,
            MinFilter::LinearMipmapLinear => LINEAR_MIPMAP_LINEAR as i32,
        }
    }
}

/// Texture co-ordinate wrapping mode.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum WrappingMode {
    /// Corresponds to `GL_CLAMP_TO_EDGE`.
    ClampToEdge = 1,

    /// Corresponds to `GL_MIRRORED_REPEAT`.
    MirroredRepeat,

    /// Corresponds to `GL_REPEAT`.
    Repeat,
}

impl WrappingMode {
    /// Returns the corresponding OpenGL enum value.
    pub fn as_gl_enum(&self) -> i32 {
        match *self {
            WrappingMode::ClampToEdge => CLAMP_TO_EDGE as i32,
            WrappingMode::MirroredRepeat => MIRRORED_REPEAT as i32,
            WrappingMode::Repeat => REPEAT as i32,
        }
    }
}

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
#[serde(default)]
pub struct Sampler {
    /// Magnification filter.
    #[serde(rename = "magFilter")]
    pub mag_filter: Option<Checked<MagFilter>>,

    /// Minification filter.
    #[serde(rename = "minFilter")]
    pub min_filter: Option<Checked<MinFilter>>,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub name: Option<String>,

    /// `s` wrapping mode.
    #[serde(default, rename = "wrapS")]
    pub wrap_s: Checked<WrappingMode>,

    /// `t` wrapping mode.
    #[serde(default, rename = "wrapT")]
    pub wrap_t: Checked<WrappingMode>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::texture::Sampler,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// A texture and its sampler.
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct Texture {
    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub name: Option<String>,

    /// The index of the sampler used by this texture.
    pub sampler: Option<Index<Sampler>>,

    /// The index of the image used by this texture.
    pub source: Index<image::Image>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::texture::Texture,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

#[derive(Clone, Debug, Deserialize, Validate)]
/// Reference to a `Texture`.
pub struct Info {
    /// The index of the texture.
    pub index: Index<Texture>,

    /// The set index of the texture's `TEXCOORD` attribute.
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::texture::Info,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

impl<'de> de::Deserialize<'de> for Checked<MagFilter> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<MagFilter>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_MAG_FILTERS)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::MagFilter::*;
                use validation::Checked::*;
                Ok(match value as u32 {
                    NEAREST => Valid(Nearest),
                    LINEAR => Valid(Linear),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_u64(Visitor)
    }
}

impl<'de> de::Deserialize<'de> for Checked<MinFilter> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<MinFilter>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_MIN_FILTERS)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::MinFilter::*;
                use validation::Checked::*;
                Ok(match value as u32 {
                    NEAREST => Valid(Nearest),
                    LINEAR => Valid(Linear),
                    NEAREST_MIPMAP_NEAREST => Valid(NearestMipmapNearest),
                    LINEAR_MIPMAP_NEAREST => Valid(LinearMipmapNearest),
                    NEAREST_MIPMAP_LINEAR => Valid(NearestMipmapLinear),
                    LINEAR_MIPMAP_LINEAR => Valid(LinearMipmapLinear),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_u64(Visitor)
    }
}

impl<'de> de::Deserialize<'de> for Checked<WrappingMode> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<WrappingMode>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_WRAPPING_MODES)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::WrappingMode::*;
                use validation::Checked::*;
                Ok(match value as u32 {
                    CLAMP_TO_EDGE => Valid(ClampToEdge),
                    MIRRORED_REPEAT => Valid(MirroredRepeat),
                    REPEAT => Valid(Repeat),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_u64(Visitor)
    }
}

impl Default for WrappingMode {
    fn default() -> Self {
        WrappingMode::Repeat
    }
}
