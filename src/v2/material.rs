
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{texture, Extensions, Extras, Index};

/// [Describes the material appearance of a primitive]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#material)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Material {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Defines the metallic-roughness material model from Physically-Based Rendering (PBR) methodology
    #[serde(rename = "pbrMetallicRoughness")]
    pub pbr: PbrMetallicRoughness,
    #[serde(rename = "normalTexture")]
    pub normal_texture: NormalTexture,
    #[serde(rename = "occlusionTexture")]
    pub occlusion_texture: OcclusionTexture,
    #[serde(rename = "emissiveTexture")]
    pub emissive_texture: texture::Info,
    #[serde(rename = "emissiveFactor")]
    #[serde(default)]
    pub emissive_factor: [f32; 3],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PbrMetallicRoughness {
    /// The base color factor
    #[serde(default = "material_pbr_metallic_roughness_base_color_factor_default")]
    #[serde(rename = "baseColorFactor")]
    pub base_color_factor: [f32; 4],
    /// The base color texture
    #[serde(rename = "baseColorTexture")]
    pub base_color_texture: texture::Info,
    /// The metalness of the material
    #[serde(default = "material_pbr_metallic_roughness_metallic_factor_default")]
    #[serde(rename = "metallicFactor")]
    pub metallic_factor: f32,
    /// The roughness of the material
    #[serde(default = "material_pbr_metallic_roughness_roughness_factor_default")]
    #[serde(rename = "roughnessFactor")]
    pub roughness_factor: f32,
    /// The metallic-roughness texture
    #[serde(rename = "metallicRoughnessTexture")]
    pub metallic_roughness_texture: texture::Info,
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

/// Defines the normal texture of a material
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NormalTexture {
    /// The index of the texture
    pub index: Index<texture::Texture>,
    /// The scalar multiplier applied to each normal vector of the normal texture
    #[serde(default = "material_normal_texture_scale_default")]
    pub scale: f32,
    /// The set index of the texture's `TEXCOORD` attribute
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,
}

fn material_normal_texture_scale_default() -> f32 {
    1.0
}

/// Defines the occlusion texture of a material
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OcclusionTexture {
    /// The index of the texture
    pub index: Index<texture::Texture>,
    /// The scalar multiplier controlling the amount of occlusion applied
    #[serde(default = "material_occlusion_texture_strength_default")]
    pub strength: f32,
    /// The set index of the texture's `TEXCOORD` attribute
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,
}

fn material_occlusion_texture_strength_default() -> f32 {
    1.0
}
