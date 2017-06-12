
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::json::{texture, Extras, Index, Root};
use v2::validation::{Error, JsonPath, Validate};

/// All valid alpha modes.
pub const VALID_ALPHA_MODES: &'static [&'static str] = &[
    "OPAQUE",
    "MASK",
    "BLEND",
];

/// The material appearance of a primitive.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Material {
    /// The alpha cutoff value of the material.
    #[serde(default, rename = "alphaCutoff")]
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
    #[serde(default, rename = "alphaMode")]
    pub alpha_mode: AlphaMode,

    /// Specifies whether the material is double-sided.
    ///
    /// * When this value is false, back-face culling is enabled.
    ///
    /// * When this value is true, back-face culling is disabled and double sided
    ///   lighting is enabled.
    ///
    /// The back-face must have its normals reversed before the lighting
    /// equation is evaluated.
    #[serde(default, rename = "doubleSided")]
    pub double_sided: bool,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology. When not specified, all the default values of `pbrMetallicRoughness` apply.
    #[serde(rename = "pbrMetallicRoughness")]
    pub pbr_metallic_roughness: Option<PbrMetallicRoughness>,

    /// A tangent space normal map. The texture contains RGB components in linear space. Each texel represents the XYZ components of a normal vector in tangent space. Red [0 to 255] maps to X [-1 to 1]. Green [0 to 255] maps to Y [-1 to 1]. Blue [128 to 255] maps to Z [1/255 to 1]. The normal vectors use OpenGL conventions where +X is right and +Y is up. +Z points toward the viewer.
    #[serde(rename = "normalTexture")]
    pub normal_texture: Option<NormalTexture>,

    /// The occlusion map texture. The occlusion values are sampled from the R channel. Higher values indicate areas that should receive full indirect lighting and lower values indicate no indirect lighting. These values are linear. If other channels are present (GBA), they are ignored for occlusion calculations.
    #[serde(rename = "occlusionTexture")]
    pub occlusion_texture: Option<OcclusionTexture>,

    /// The emissive map controls the color and intensity of the light being emitted by the material. This texture contains RGB components in sRGB color space. If a fourth component (A) is present, it is ignored.
    #[serde(rename = "emissiveTexture")]
    pub emissive_texture: Option<texture::Info>,

    /// The emissive color of the material.
    #[serde(default, rename = "emissiveFactor")]
    pub emissive_factor: EmissiveFactor,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: MaterialExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Material`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct MaterialExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct PbrMetallicRoughness {
    /// The material's base color factor.
    #[serde(default, rename = "baseColorFactor")]
    pub base_color_factor: PbrBaseColorFactor,

    /// The base color texture.
    #[serde(rename = "baseColorTexture")]
    pub base_color_texture: Option<texture::Info>,

    /// The metalness of the material.
    #[serde(default, rename = "metallicFactor")]
    pub metallic_factor: StrengthFactor,

    /// The roughness of the material.
    ///
    /// * A value of 1.0 means the material is completely rough.
    /// * A value of 0.0 means the material is completely smooth.
    #[serde(default, rename = "roughnessFactor")]
    pub roughness_factor: StrengthFactor,

    /// The metallic-roughness texture.
    ///
    /// This texture has two components:
    ///
    /// * The first component (R) contains the metallic-ness of the material.
    /// * The second component (G) contains the roughness of the material.
    /// * If the third component (B) and/or the fourth component (A) are present
    ///   then they are ignored.
    #[serde(rename = "metallicRoughnessTexture")]
    pub metallic_roughness_texture: Option<texture::Info>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: PbrMetallicRoughnessExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `PbrMetallicRoughness`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct PbrMetallicRoughnessExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Defines the normal texture of a material.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
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
    pub extensions: NormalTextureExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `NormalTexture`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct NormalTextureExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

fn material_normal_texture_scale_default() -> f32 {
    1.0
}

/// Defines the occlusion texture of a material.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
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
    pub extensions: OcclusionTextureExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `OcclusionTexture`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct OcclusionTextureExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// The alpha cutoff value of a material.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct AlphaCutoff(pub f32);

/// The alpha rendering mode of a material.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlphaMode(pub String);

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
        AlphaMode("OPAQUE".to_string())
    }
}

impl Validate for AlphaCutoff {
    fn validate<F>(&self, _: &Root, path: JsonPath, report: &mut F)
        where F: FnMut(Error)
    {
        if self.0 < 0.0 {
            report(Error::invalid_value(path, self.0));
        }
    }
}

impl Validate for AlphaMode {
    fn validate<F>(&self, _: &Root, path: JsonPath, report: &mut F)
        where F: FnMut(Error)
    {
        if !VALID_ALPHA_MODES.contains(&self.0.as_str()) {
            report(Error::invalid_enum(path, self.0.clone()));
        }
    }
}

impl Validate for EmissiveFactor {
    fn validate<F>(&self, _: &Root, path: JsonPath, report: &mut F)
        where F: FnMut(Error)
    {
        for x in &self.0 {
            if *x < 0.0 || *x > 1.0 {
                report(Error::invalid_value(path, self.0.to_vec()));
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
    fn validate<F>(&self, _: &Root, path: JsonPath, report: &mut F)
        where F: FnMut(Error)
    {
        for x in &self.0 {
            if *x < 0.0 || *x > 1.0 {
                report(Error::invalid_value(path, self.0.to_vec()));
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
    fn validate<F>(&self, _: &Root, path: JsonPath, report: &mut F)
        where F: FnMut(Error)
    {
        if self.0 < 0.0 || self.0 > 1.0 {
            report(Error::invalid_value(path, self.0));
        }
    }
}

