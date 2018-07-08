use serde::{de, ser};
use std::fmt;
use validation::{Checked, Error, Validate};
use {extensions, texture, Extras, Index, Root, Path};

/// All valid alpha modes.
pub const VALID_ALPHA_MODES: &'static [&'static str] = &[
    "OPAQUE",
    "MASK",
    "BLEND",
];

/// The alpha rendering mode of a material.
#[derive(Clone, Copy, Debug)]
pub enum AlphaMode {
    /// The alpha value is ignored and the rendered output is fully opaque.
    Opaque = 1,

    /// The rendered output is either fully opaque or fully transparent depending on
    /// the alpha value and the specified alpha cutoff value.
    Mask,

    /// The rendered output is either fully opaque or fully transparent depending on
    /// the alpha value and the specified alpha cutoff value.
    Blend,
}

impl ser::Serialize for AlphaMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        match *self {
            AlphaMode::Opaque => serializer.serialize_str("OPAQUE"),
            AlphaMode::Mask => serializer.serialize_str("MASK"),
            AlphaMode::Blend => serializer.serialize_str("BLEND"),
        }
    }
}

/// The material appearance of a primitive.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default)]
pub struct Material {
    /// The alpha cutoff value of the material.
    #[serde(rename = "alphaCutoff")]
    pub alpha_cutoff: AlphaCutoff,
    
    /// The alpha rendering mode of the material.
    ///
    /// The material's alpha rendering mode enumeration specifying the
    /// interpretation of the alpha value of the main factor and texture.
    ///
    /// * In `Opaque` mode (default) the alpha value is ignored and the rendered
    ///   output is fully opaque.
    ///
    /// * In `Mask` mode, the rendered output is either fully opaque or fully
    ///   transparent depending on the alpha value and the specified alpha cutoff
    ///   value.
    ///
    /// * In `Blend` mode, the alpha value is used to composite the source and
    ///   destination areas and the rendered output is combined with the
    ///   background using the normal painting operation (i.e. the Porter and
    ///   Duff over operator).
    #[serde(rename = "alphaMode")]
    pub alpha_mode: Checked<AlphaMode>,

    /// Specifies whether the material is double-sided.
    ///
    /// * When this value is false, back-face culling is enabled.
    ///
    /// * When this value is true, back-face culling is disabled and double sided
    ///   lighting is enabled.
    ///
    /// The back-face must have its normals reversed before the lighting
    /// equation is evaluated.
    #[serde(rename = "doubleSided")]
    pub double_sided: bool,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// A set of parameter values that are used to define the metallic-roughness
    /// material model from Physically-Based Rendering (PBR) methodology. When not
    /// specified, all the default values of `pbrMetallicRoughness` apply.
    #[serde(default, rename = "pbrMetallicRoughness")]
    pub pbr_metallic_roughness: PbrMetallicRoughness,

    /// A tangent space normal map. The texture contains RGB components in linear
    /// space. Each texel represents the XYZ components of a normal vector in
    /// tangent space. Red [0 to 255] maps to X [-1 to 1]. Green [0 to 255] maps to
    /// Y [-1 to 1]. Blue [128 to 255] maps to Z [1/255 to 1]. The normal vectors
    /// use OpenGL conventions where +X is right and +Y is up. +Z points toward the
    /// viewer.
    #[serde(rename = "normalTexture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<NormalTexture>,

    /// The occlusion map texture. The occlusion values are sampled from the R
    /// channel. Higher values indicate areas that should receive full indirect
    /// lighting and lower values indicate no indirect lighting. These values are
    /// linear. If other channels are present (GBA), they are ignored for occlusion
    /// calculations.
    #[serde(rename = "occlusionTexture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<OcclusionTexture>,

    /// The emissive map controls the color and intensity of the light being emitted
    /// by the material. This texture contains RGB components in sRGB color space.
    /// If a fourth component (A) is present, it is ignored.
    #[serde(rename = "emissiveTexture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<texture::Info>,

    /// The emissive color of the material.
    #[serde(rename = "emissiveFactor")]
    pub emissive_factor: EmissiveFactor,

    /// Extension specific data.
    pub extensions: extensions::material::Material,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default)]
pub struct PbrMetallicRoughness {
    /// The material's base color factor.
    #[serde(rename = "baseColorFactor")]
    pub base_color_factor: PbrBaseColorFactor,

    /// The base color texture.
    #[serde(rename = "baseColorTexture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<texture::Info>,

    /// The metalness of the material.
    #[serde(rename = "metallicFactor")]
    pub metallic_factor: StrengthFactor,

    /// The roughness of the material.
    ///
    /// * A value of 1.0 means the material is completely rough.
    /// * A value of 0.0 means the material is completely smooth.
    #[serde(rename = "roughnessFactor")]
    pub roughness_factor: StrengthFactor,

    /// The metallic-roughness texture.
    ///
    /// This texture has two components:
    ///
    /// The metalness values are sampled from the B channel.
    /// The roughness values are sampled from the G channel.
    /// These values are linear. If other channels are present (R or A),
    /// they are ignored for metallic-roughness calculations.
    #[serde(rename = "metallicRoughnessTexture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<texture::Info>,

    /// Extension specific data.
    pub extensions: extensions::material::PbrMetallicRoughness,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// Defines the normal texture of a material.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct NormalTexture {
    /// The index of the texture.
    pub index: Index<texture::Texture>,

    /// The scalar multiplier applied to each normal vector of the texture.
    ///
    /// This value is ignored if normalTexture is not specified.
    #[serde(default = "material_normal_texture_scale_default")]
    pub scale: f32,

    /// The set index of the texture's `TEXCOORD` attribute.
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::material::NormalTexture,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

fn material_normal_texture_scale_default() -> f32 {
    1.0
}

/// Defines the occlusion texture of a material.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct OcclusionTexture {
    /// The index of the texture.
    pub index: Index<texture::Texture>,

    /// The scalar multiplier controlling the amount of occlusion applied.
    #[serde(default)]
    pub strength: StrengthFactor,

    /// The set index of the texture's `TEXCOORD` attribute.
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::material::OcclusionTexture,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// The alpha cutoff value of a material.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct AlphaCutoff(pub f32);

/// The emissive color of a material.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct EmissiveFactor(pub [f32; 3]);

/// The base color factor of a material.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PbrBaseColorFactor(pub [f32; 4]);

/// A number in the inclusive range [0.0, 1.0] with a default value of 1.0.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct StrengthFactor(pub f32);

impl Default for AlphaCutoff {
    fn default() -> Self {
        AlphaCutoff(0.5)
    }
}

impl Default for AlphaMode {
    fn default() -> Self {
        AlphaMode::Opaque
    }
}

impl Validate for AlphaCutoff {
    fn validate_completely<P, R>(&self, _: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        if self.0 < 0.0 {
            report(&path, Error::Invalid);
        }
    }
}

impl<'de> de::Deserialize<'de> for Checked<AlphaMode> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<AlphaMode>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_ALPHA_MODES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::AlphaMode::*;
                use validation::Checked::*;
                Ok(match value {
                    "OPAQUE" => Valid(Opaque),
                    "MASK" => Valid(Mask),
                    "BLEND" => Valid(Blend),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl Validate for EmissiveFactor {
    fn validate_completely<P, R>(&self, _: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        for x in &self.0 {
            if *x < 0.0 || *x > 1.0 {
                report(&path, Error::Invalid);
                // Only report once
                break;
            }
        }
    }
}

impl Default for PbrBaseColorFactor {
    fn default() -> Self {
        PbrBaseColorFactor([1.0, 1.0, 1.0, 1.0])
    }
}

impl Validate for PbrBaseColorFactor {
    fn validate_completely<P, R>(&self, _: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        for x in &self.0 {
            if *x < 0.0 || *x > 1.0 {
                report(&path, Error::Invalid);
                // Only report once
                break;
            }
        }
    }
}

impl Default for StrengthFactor {
    fn default() -> Self {
        StrengthFactor(1.0)
    }
}

impl Validate for StrengthFactor {
    fn validate_completely<P, R>(&self, _: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        if self.0 < 0.0 || self.0 > 1.0 {
            report(&path, Error::Invalid);
        }
    }
}

