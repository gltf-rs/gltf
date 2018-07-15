use validation::{Error, Validate};
use material::StrengthFactor;
use {texture, Extras, Root, Path};

/// The material appearance of a primitive.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Material {
    #[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
    #[serde(default, rename = "KHR_materials_pbrSpecularGlossiness", skip_serializing_if = "Option::is_none")]
    pub pbr_specular_glossiness: Option<PbrSpecularGlossiness>,
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
impl Validate for PbrDiffuseFactor {
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
impl Validate for PbrSpecularFactor {
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
