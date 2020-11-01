use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};
#[cfg(any(feature = "KHR_materials_pbrSpecularGlossiness", feature = "KHR_materials_transmission", feature = "KHR_materials_ior"))]
use crate::{Extras, validation::Validate};
#[cfg(any(feature = "KHR_materials_pbrSpecularGlossiness", feature = "KHR_materials_transmission"))]
use crate::texture;
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
use crate::material::StrengthFactor;

/// The material appearance of a primitive.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Material {
    #[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
    #[serde(default, rename = "KHR_materials_pbrSpecularGlossiness", skip_serializing_if = "Option::is_none")]
    pub pbr_specular_glossiness: Option<PbrSpecularGlossiness>,

    #[cfg(feature = "KHR_materials_unlit")]
    #[serde(default, rename = "KHR_materials_unlit", skip_serializing_if = "Option::is_none")]
    pub unlit: Option<Unlit>,

    #[cfg(feature = "KHR_materials_transmission")]
    #[serde(default, rename = "KHR_materials_transmission", skip_serializing_if = "Option::is_none")]
    pub transmission: Option<Transmission>,

    #[cfg(feature = "KHR_materials_ior")]
    #[serde(default, rename = "KHR_materials_ior", skip_serializing_if = "Option::is_none")]
    pub ior: Option<Ior>,
}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct PbrMetallicRoughness {}

/// A set of parameter values that are used to define the specular-glossiness
/// material model from Physically-Based Rendering (PBR) methodology.
///
/// This model supports more materials than metallic-roughness, at the cost of
/// increased memory use. When both are available, specular-glossiness should be
/// preferred.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct PbrSpecularGlossiness {
    /// The material's diffuse factor.
    ///
    /// The RGBA components of the reflected diffuse color of the
    /// material. Metals have a diffuse value of `[0.0, 0.0, 0.0]`. The fourth
    /// component (A) is the alpha coverage of the material. The `alphaMode`
    /// property specifies how alpha is interpreted. The values are linear.
    pub diffuse_factor: PbrDiffuseFactor,

    /// The diffuse texture.
    ///
    /// This texture contains RGB(A) components of the reflected diffuse color
    /// of the material in sRGB color space. If the fourth component (A) is
    /// present, it represents the alpha coverage of the material. Otherwise, an
    /// alpha of 1.0 is assumed. The `alphaMode` property specifies how alpha is
    /// interpreted. The stored texels must not be premultiplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffuse_texture: Option<texture::Info>,

    /// The material's specular factor.
    pub specular_factor: PbrSpecularFactor,

    /// The glossiness or smoothness of the material.
    ///
    /// A value of 1.0 means the material has full glossiness or is perfectly
    /// smooth. A value of 0.0 means the material has no glossiness or is
    /// completely rough. This value is linear.
    pub glossiness_factor: StrengthFactor,

    /// The specular-glossiness texture.
    ///
    /// A RGBA texture, containing the specular color of the material (RGB
    /// components) and its glossiness (A component). The values are in sRGB
    /// space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_glossiness_texture: Option<texture::Info>,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// Defines the normal texture of a material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct NormalTexture {}

/// Defines the occlusion texture of a material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct OcclusionTexture {}

/// The diffuse factor of a material.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PbrDiffuseFactor(pub [f32; 4]);

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Default for PbrDiffuseFactor {
    fn default() -> Self {
        PbrDiffuseFactor([1.0, 1.0, 1.0, 1.0])
    }
}

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Validate for PbrDiffuseFactor {}

/// The specular factor of a material.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PbrSpecularFactor(pub [f32; 3]);

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Default for PbrSpecularFactor {
    fn default() -> Self {
        PbrSpecularFactor([1.0, 1.0, 1.0])
    }
}

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Validate for PbrSpecularFactor {}

/// Empty struct that should be present for primitives which should not be shaded with the PBR shading model.
#[cfg(feature = "KHR_materials_unlit")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Unlit {}

/// A number in the inclusive range [0.0, 1.0] with a default value of 0.0.
#[cfg(feature = "KHR_materials_transmission")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct TransmissionFactor(pub f32);

#[cfg(feature = "KHR_materials_transmission")]
impl Default for TransmissionFactor {
    fn default() -> Self {
        TransmissionFactor(0.0)
    }
}

#[cfg(feature = "KHR_materials_transmission")]
impl Validate for TransmissionFactor {}

#[cfg(feature = "KHR_materials_transmission")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct Transmission {
    /// The base percentage of light that is transmitted through the surface.
    ///
    /// The amount of light that is transmitted by the surface rather than diffusely re-emitted. 
    /// This is a percentage of all the light that penetrates a surface (i.e. isn’t specularly reflected) 
    /// rather than a percentage of the total light that hits a surface. 
    /// A value of 1.0 means that 100% of the light that penetrates the surface is transmitted through.
    pub transmission_factor: TransmissionFactor,

    /// The transmission texture.
    ///
    /// The R channel of this texture defines the amount of light that is transmitted by the surface 
    /// rather than diffusely re-emitted. A value of 1.0 in the red channel means that 100% of the light
    /// that penetrates the surface (i.e. isn’t specularly reflected) is transmitted through. 
    /// The value is linear and is multiplied by the transmissionFactor to determine the total transmission value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission_texture: Option<texture::Info>,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// A positive number with default value of 1.5
#[cfg(feature = "KHR_materials_ior")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct IndexOfRefraction(pub f32);

#[cfg(feature = "KHR_materials_ior")]
impl Default for IndexOfRefraction {
    fn default() -> Self {
        IndexOfRefraction(1.5)
    }
}

#[cfg(feature = "KHR_materials_ior")]
impl Validate for IndexOfRefraction {}

#[cfg(feature = "KHR_materials_ior")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct Ior {
    /// The index of refraction.
    ///
    /// Typical values for the index of refraction range from 1 to 2. 
    /// In rare cases values greater than 2 are possible.
    /// For example, the ior of water is 1.33, and diamond is 2.42
    pub ior: IndexOfRefraction,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}