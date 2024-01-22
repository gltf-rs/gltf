use crate::validation::Validate;
use crate::{texture, Extras, Index, UnrecognizedExtensions};

/// Support for the `KHR_materials_pbrSpecularGlossiness` extension.
pub mod khr_materials_pbr_specular_glossiness {
    /// A set of parameter values that are used to define the specular-glossiness
    /// material model from Physically-Based Rendering (PBR) methodology.
    ///
    /// This model supports more materials than metallic-roughness, at the cost of
    /// increased memory use. When both are available, specular-glossiness should be
    /// preferred.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct PbrSpecularGlossiness {
        /// The material's diffuse factor.
        ///
        /// The RGBA components of the reflected diffuse color of the
        /// material. Metals have a diffuse value of `[0.0, 0.0, 0.0]`. The fourth
        /// component (A) is the alpha coverage of the material. The `alphaMode`
        /// property specifies how alpha is interpreted. The values are linear.
        #[gltf(default = [1.0; 4])]
        pub diffuse_factor: [f32; 4],

        /// The diffuse texture.
        ///
        /// This texture contains RGB(A) components of the reflected diffuse color
        /// of the material in sRGB color space. If the fourth component (A) is
        /// present, it represents the alpha coverage of the material. Otherwise, an
        /// alpha of 1.0 is assumed. The `alphaMode` property specifies how alpha is
        /// interpreted. The stored texels must not be premultiplied.
        pub diffuse_texture: Option<crate::texture::Info>,

        /// The material's specular factor.
        #[gltf(default = [1.0; 3])]
        pub specular_factor: [f32; 3],

        /// The glossiness or smoothness of the material.
        ///
        /// A value of 1.0 means the material has full glossiness or is perfectly
        /// smooth. A value of 0.0 means the material has no glossiness or is
        /// completely rough. This value is linear.
        #[gltf(default = 1.0)]
        pub glossiness_factor: f32,

        /// The specular-glossiness texture.
        ///
        /// A RGBA texture, containing the specular color of the material (RGB
        /// components) and its glossiness (A component). The values are in sRGB
        /// space.
        pub specular_glossiness_texture: Option<crate::texture::Info>,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// Support for the `KHR_materials_unlit` extension.
pub mod khr_materials_unlit {
    /// Empty struct that should be present for primitives which should not be shaded with the PBR shading model.
    #[derive(
        Clone,
        Debug,
        Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct Unlit {
        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// Support for the `KHR_materials_transmission` extension.
pub mod khr_materials_transmission {
    /// Describes the optical transmission of a material.
    #[derive(
        Clone,
        Debug,
        Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct Transmission {
        /// The base percentage of light that is transmitted through the surface.
        ///
        /// The amount of light that is transmitted by the surface rather than diffusely re-emitted.
        /// This is a percentage of all the light that penetrates a surface (i.e. isn’t specularly reflected)
        /// rather than a percentage of the total light that hits a surface.
        /// A value of 1.0 means that 100% of the light that penetrates the surface is transmitted through.
        #[gltf(default)]
        pub transmission_factor: f32,

        /// The transmission texture.
        ///
        /// The R channel of this texture defines the amount of light that is transmitted by the surface
        /// rather than diffusely re-emitted. A value of 1.0 in the red channel means that 100% of the light
        /// that penetrates the surface (i.e. isn’t specularly reflected) is transmitted through.
        /// The value is linear and is multiplied by the transmissionFactor to determine the total transmission value.
        pub transmission_texture: Option<crate::texture::Info>,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// Support for the `KHR_materials_ior` extension.
pub mod khr_materials_ior {
    /// Defines the index of refraction for a material.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct Ior {
        /// The index of refraction.
        ///
        /// Typical values for the index of refraction range from 1 to 2.
        /// In rare cases, values greater than 2 are possible.
        /// For example, the ior of water is 1.33, and diamond is 2.42.
        #[gltf(default = 1.5)]
        pub ior: f32,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// Support for the `KHR_materials_emissive_strength` extension.
pub mod khr_materials_emissive_strength {
    /// Allows the strength of an emissive material to be adjusted.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct EmissiveStrength {
        /// The factor by which to scale the emissive factor or emissive texture.
        #[gltf(default = 1.0)]
        pub emissive_strength: f32,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// Support for the `KHR_materials_volume` extension.
pub mod khr_materials_volume {
    /// Volumetric material properties.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct Volume {
        /// The thickness of the volume beneath the surface. The value is
        /// given in the coordinate space of the mesh. If the value is 0
        /// the material is thin-walled. Otherwise the material is a
        /// volume boundary. The `doubleSided` property has no effect on
        /// volume boundaries. Range is [0, +inf).
        #[gltf(default)]
        pub thickness_factor: f32,

        /// A texture that defines the thickness, stored in the G channel.
        /// This will be multiplied by `thickness_factor`. Range is [0, 1].
        pub thickness_texture: Option<crate::texture::Info>,

        /// Density of the medium given as the average distance that light
        /// travels in the medium before interacting with a particle. The
        /// value is given in world space. Range is (0, +inf).
        #[gltf(default = f32::INFINITY)]
        pub attenuation_distance: f32,

        /// The color that white light turns into due to absorption when
        /// reaching the attenuation distance.
        #[gltf(default = [1.0; 3])]
        pub attenuation_color: [f32; 3],

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// Support for the `KHR_materials_volume` extension.
pub mod khr_materials_specular {
    /// Allows the strength of specular reflections to be adjusted.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct Specular {
        /// The strength of the specular reflection.
        #[gltf(default = 1.0)]
        pub specular_factor: f32,

        /// A texture that defines the strength of the specular reflection,
        /// stored in the alpha (`A`) channel. This will be multiplied by
        /// `specular_factor`.
        pub specular_texture: Option<crate::texture::Info>,

        /// The F0 (linear RGB) color of the specular reflection.
        #[gltf(default = [1.0; 3])]
        pub specular_color_factor: [f32; 3],

        /// A texture that defines the F0 color of the specular reflection,
        /// stored in the `RGB` channels and encoded in sRGB. This texture
        /// will be multiplied by `specular_color_factor`.
        pub specular_color_texture: Option<crate::texture::Info>,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// The alpha rendering mode of a material.
#[derive(
    Clone, Copy, Debug, Default, serde_derive::Deserialize, Eq, PartialEq, serde_derive::Serialize,
)]
pub enum AlphaMode {
    /// The alpha value is ignored and the rendered output is fully opaque.
    #[default]
    #[serde(rename = "OPAQUE")]
    Opaque = 1,

    /// The rendered output is either fully opaque or fully transparent depending on
    /// the alpha value and the specified alpha cutoff value.
    #[serde(rename = "MASK")]
    Mask,

    /// The alpha value is used, to determine the transparency of the rendered output.
    /// The alpha cutoff value is ignored.
    #[serde(rename = "BLEND")]
    Blend,
}

impl Validate for AlphaMode {}

/// The material appearance of a primitive.
#[derive(
    Clone,
    Debug,
    gltf_derive::Default,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Validate,
)]
pub struct Material {
    /// The alpha cutoff value of the material.
    #[gltf(default = 0.5)]
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
    #[gltf(default)]
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
    pub double_sided: bool,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// A set of parameter values that are used to define the metallic-roughness
    /// material model from Physically-Based Rendering (PBR) methodology. When not
    /// specified, all the default values of `pbrMetallicRoughness` apply.
    pub pbr_metallic_roughness: Option<PbrMetallicRoughness>,

    /// A tangent space normal map. The texture contains RGB components in linear
    /// space. Each texel represents the XYZ components of a normal vector in
    /// tangent space. Red [0 to 255] maps to X [-1 to 1]. Green [0 to 255] maps to
    /// Y [-1 to 1]. Blue [128 to 255] maps to Z [1/255 to 1]. The normal vectors
    /// use OpenGL conventions where +X is right and +Y is up. +Z points toward the
    /// viewer.
    pub normal_texture: Option<NormalTexture>,

    /// The occlusion map texture. The occlusion values are sampled from the R
    /// channel. Higher values indicate areas that should receive full indirect
    /// lighting and lower values indicate no indirect lighting. These values are
    /// linear. If other channels are present (GBA), they are ignored for occlusion
    /// calculations.
    pub occlusion_texture: Option<OcclusionTexture>,

    /// The emissive map controls the color and intensity of the light being emitted
    /// by the material. This texture contains RGB components in sRGB color space.
    /// If a fourth component (A) is present, it is ignored.
    pub emissive_texture: Option<texture::Info>,

    /// The emissive color of the material.
    #[gltf(default)]
    pub emissive_factor: [f32; 3],

    /// Support for the `KHR_materials_pbrSpecularGlossiness` extension.
    #[gltf(extension = "KHR_materials_pbrSpecularGlossiness")]
    pub pbr_specular_glossiness:
        Option<khr_materials_pbr_specular_glossiness::PbrSpecularGlossiness>,

    /// Support for the `KHR_materials_unlit` extension.
    #[gltf(extension = "KHR_materials_unlit")]
    pub unlit: Option<khr_materials_unlit::Unlit>,

    /// Support for the `KHR_materials_transmission` extension.
    #[gltf(extension = "KHR_materials_transmission")]
    pub transmission: Option<khr_materials_transmission::Transmission>,

    /// Support for the `KHR_materials_volume` extension.
    #[gltf(extension = "KHR_materials_volume")]
    pub volume: Option<khr_materials_volume::Volume>,

    /// Support for the `KHR_materials_specular` extension.
    #[gltf(extension = "KHR_materials_specular")]
    pub specular: Option<khr_materials_specular::Specular>,

    /// Support for the `KHR_materials_ior` extension.
    #[gltf(extension = "KHR_materials_ior")]
    pub ior: Option<khr_materials_ior::Ior>,

    /// Support for the `KHR_materials_emissive_strength` extension.
    #[gltf(extension = "KHR_materials_emissive_strength")]
    pub emissive_strength: Option<khr_materials_emissive_strength::EmissiveStrength>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
#[derive(
    Clone,
    Debug,
    gltf_derive::Default,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Validate,
)]
pub struct PbrMetallicRoughness {
    /// The material's base color factor.
    #[gltf(default = [1.0, 1.0, 1.0, 1.0])]
    pub base_color_factor: [f32; 4],

    /// The base color texture.
    pub base_color_texture: Option<texture::Info>,

    /// The metalness of the material.
    #[gltf(default = 1.0)]
    pub metallic_factor: f32,

    /// The roughness of the material.
    ///
    /// * A value of 1.0 means the material is completely rough.
    /// * A value of 0.0 means the material is completely smooth.
    #[gltf(default = 1.0)]
    pub roughness_factor: f32,

    /// The metallic-roughness texture.
    ///
    /// This texture has two components:
    ///
    /// The metalness values are sampled from the B channel.
    /// The roughness values are sampled from the G channel.
    /// These values are linear. If other channels are present (R or A),
    /// they are ignored for metallic-roughness calculations.
    pub metallic_roughness_texture: Option<texture::Info>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// Defines the normal texture of a material.
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate)]
pub struct NormalTexture {
    /// The index of the texture.
    pub index: Index<texture::Texture>,

    /// The scalar multiplier applied to each normal vector of the texture.
    ///
    /// This value is ignored if normalTexture is not specified.
    #[gltf(default = 1.0)]
    pub scale: f32,

    /// The set index of the texture's `TEXCOORD` attribute.
    #[gltf(default)]
    pub tex_coord: u32,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// Defines the occlusion texture of a material.
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate)]
pub struct OcclusionTexture {
    /// The index of the texture.
    pub index: Index<texture::Texture>,

    /// The scalar multiplier controlling the amount of occlusion applied.
    #[gltf(default = 1.0)]
    pub strength: f32,

    /// The set index of the texture's `TEXCOORD` attribute.
    #[gltf(default)]
    pub tex_coord: u32,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn material_default() {
        let m: super::Material = Default::default();
        assert_eq!(m.alpha_cutoff, 0.5);
        assert_eq!(m.alpha_mode, super::AlphaMode::Opaque);
        assert!(!m.double_sided);
        assert!(m.name.is_none());
        assert!(m.pbr_metallic_roughness.is_none());
        assert!(m.normal_texture.is_none());
        assert!(m.occlusion_texture.is_none());
        assert!(m.emissive_texture.is_none());
        assert_eq!(m.emissive_factor, [0.0, 0.0, 0.0]);
        assert!(m.extras.is_none());
    }
}
