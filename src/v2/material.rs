
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{texture, Extras, Index, Root};

enum_string! {
    AlphaMode {
        Blend = "BLEND",
        Mask = "MASK",
        Opaque = "OPAQUE",
    }
}

/// The material appearance of a primitive.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Material {
    /// The alpha cutoff of the material.
    ///
    /// * In `Mask` mode this value specifies the cutoff threshold and is
    /// otherwise ignored.
    ///
    /// * If the alpha value is greater than or equal to this value then it is
    ///   rendered as fully opaque, otherwise, it is rendered as fully transparent.
    #[serde(default = "material_alpha_cutoff_default", rename = "alphaCutoff")]
    pub alpha_cutoff: f32,
    
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

    /// Defines the metallic-roughness material model from Physically-Based
    /// Rendering (PBR) methodology.
    #[serde(rename = "pbrMetallicRoughness")]
    pub pbr_metallic_roughness: PbrMetallicRoughness,

    /// A tangent space normal map.
    ///
    /// Each texel represents the XYZ components of a normal vector in tangent
    /// space.
    #[serde(rename = "normalTexture")]
    pub normal_texture: Option<NormalTexture>,

    /// The occlusion map texture.
    ///
    /// The occlusion map is a greyscale texture, with white indicating areas that
    /// should receive full indirect lighting and black indicating no indirect
    /// lighting.
    #[serde(rename = "occlusionTexture")]
    pub occlusion_texture: Option<OcclusionTexture>,

    /// The emissive map texture.
    ///
    /// The emissive map controls the color and intensity of the light being
    /// emitted by the material.
    ///
    /// This texture contains RGB components in sRGB color space.
    ///
    /// If a fourth component (A) is present, it is ignored.
    #[serde(rename = "emissiveTexture")]
    pub emissive_texture: Option<texture::TextureInfo>,

    /// The emissive color of the material.
    ///
    /// The RGB components of the emissive color of the material.
    ///
    /// If `emissive_texture` is provided then this value is multiplied with the
    /// texel values of the emissive texture.
    #[serde(rename = "emissiveFactor")]
    #[serde(default)]
    pub emissive_factor: [f32; 3],

    /// Extension specific data.
    #[serde(default)]
    pub extensions: MaterialExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

fn material_alpha_cutoff_default() -> f32 {
    0.5
}

/// Extension specific data for `Material`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MaterialExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PbrMetallicRoughness {
    /// The base color factor.
    ///
    /// The RGBA components of the base color of the material.
    ///
    /// The fourth component is the alpha coverage of the material and the
    /// `alpha_mode` field specifies how these values are interpreted.
    #[serde(default = "material_pbr_metallic_roughness_base_color_factor_default")]
    #[serde(rename = "baseColorFactor")]
    pub base_color_factor: [f32; 4],

    /// The base color texture.
    ///
    /// This texture contains RGB(A) components in sRGB color space.
    ///
    /// * The first three components (RGB) specify the base color of the material
    /// * If the fourth component (A) is present, it represents the alpha
    ///   coverage of the material and otherwise, an alpha of 1.0 is assumed
    /// * The `alpha_mode` field specifies how alpha is interpreted.
    ///
    /// The stored texels must not be premultiplied.
    #[serde(rename = "baseColorTexture")]
    pub base_color_texture: texture::TextureInfo,

    /// The metalness of the material.
    ///
    /// * A value of 1.0 means the material is a metal.
    /// * A value of 0.0 means the material is a dielectric.
    /// * Values between 0.0 and 1.0 are for blending between metals and
    ///   dielectrics such as dirty metallic surfaces.
    #[serde(default = "material_pbr_metallic_roughness_metallic_factor_default")]
    #[serde(rename = "metallicFactor")]
    pub metallic_factor: f32,

    /// The roughness of the material.
    ///
    /// * A value of 1.0 means the material is completely rough.
    /// * A value of 0.0 means the material is completely smooth.
    #[serde(default = "material_pbr_metallic_roughness_roughness_factor_default")]
    #[serde(rename = "roughnessFactor")]
    pub roughness_factor: f32,

    /// The metallic-roughness texture.
    ///
    /// This texture has two components:
    ///
    /// * The first component (R) contains the metallic-ness of the material.
    /// * The second component (G) contains the roughness of the material.
    /// * If the third component (B) and/or the fourth component (A) are present
    ///   then they are ignored.
    #[serde(rename = "metallicRoughnessTexture")]
    pub metallic_roughness_texture: texture::TextureInfo,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: PbrMetallicRoughnessExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `PbrMetallicRoughness`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PbrMetallicRoughnessExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

fn material_pbr_metallic_roughness_base_color_factor_default() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

fn material_pbr_metallic_roughness_metallic_factor_default() -> f32 {
    1.0
}

fn material_pbr_metallic_roughness_roughness_factor_default() -> f32 {
    1.0
}

/// Defines the normal texture of a material.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NormalTextureExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

fn material_normal_texture_scale_default() -> f32 {
    1.0
}

/// Defines the occlusion texture of a material.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OcclusionTexture {
    /// The index of the texture.
    pub index: Index<texture::Texture>,

    /// The scalar multiplier controlling the amount of occlusion applied.
    ///
    /// * A value of 0.0 means no occlusion.
    /// * A value of 1.0 means full occlusion.
    ///
    /// This value is ignored if the corresponding texture is not specified.
    #[serde(default = "material_occlusion_texture_strength_default")]
    pub strength: f32,

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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OcclusionTextureExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

fn material_occlusion_texture_strength_default() -> f32 {
    1.0
}

impl Material {
    #[doc(hidden)]
    pub fn range_check(&self, root: &Root) -> Result<(), ()> {
        if let Some(ref texture) = self.normal_texture {
            let _ = root.try_get(&texture.index)?;
        }
        if let Some(ref texture) = self.occlusion_texture {
            let _ = root.try_get(&texture.index)?;
        }
        if let Some(ref texture) = self.emissive_texture {
            let _ = root.try_get(&texture.index)?;
        }
        let _ = root.try_get(&self.pbr_metallic_roughness.base_color_texture.index)?;
        let _ = root.try_get(&self.pbr_metallic_roughness.metallic_roughness_texture.index)?;
        Ok(())
    }
}

impl Default for AlphaMode {
    fn default() -> Self {
        AlphaMode::Opaque
    }
}
