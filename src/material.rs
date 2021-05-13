use crate::{texture, Document};

pub use json::material::AlphaMode;

lazy_static! {
    static ref DEFAULT_MATERIAL: json::material::Material = Default::default();
}

/// The material appearance of a primitive.
#[derive(Clone, Debug)]
pub struct Material<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index - `None` when the default material.
    index: Option<usize>,

    /// The corresponding JSON struct.
    json: &'a json::material::Material,
}

impl<'a> Material<'a> {
    /// Constructs a `Material`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::material::Material,
    ) -> Self {
        Self {
            document: document,
            index: Some(index),
            json: json,
        }
    }

    /// Constructs the default `Material`.
    pub(crate) fn default(document: &'a Document) -> Self {
        Self {
            document: document,
            index: None,
            json: &DEFAULT_MATERIAL,
        }
    }

    /// Returns the internal JSON index if this `Material` was explicity defined.
    ///
    /// This function returns `None` if the `Material` is the default material.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    ///  The optional alpha cutoff value of the material.
    pub fn alpha_cutoff(&self) -> Option<f32> {
        self.json.alpha_cutoff.map(|value| value.0)
    }

    /// The alpha rendering mode of the material.  The material's alpha rendering
    /// mode enumeration specifying the interpretation of the alpha value of the main
    /// factor and texture.
    ///
    /// * In `Opaque` mode (default) the alpha value is ignored
    ///   and the rendered output is fully opaque.
    /// * In `Mask` mode, the rendered
    ///   output is either fully opaque or fully transparent depending on the alpha
    ///   value and the specified alpha cutoff value.
    /// * In `Blend` mode, the alpha value is used to composite the source and
    ///   destination areas and the rendered output is combined with the background
    ///   using the normal painting operation (i.e. the Porter and Duff over
    ///   operator).
    pub fn alpha_mode(&self) -> AlphaMode {
        self.json.alpha_mode.unwrap()
    }

    /// Specifies whether the material is double-sided.
    ///
    /// * When this value is false, back-face culling is enabled.
    /// * When this value is true, back-face culling is disabled and double sided
    ///   lighting is enabled.  The back-face must have its normals reversed before
    ///   the lighting equation is evaluated.
    pub fn double_sided(&self) -> bool {
        self.json.double_sided
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(docsrs, doc(cfg(feature = "names")))]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Parameter values that define the metallic-roughness material model from
    /// Physically-Based Rendering (PBR) methodology.
    pub fn pbr_metallic_roughness(&self) -> PbrMetallicRoughness<'a> {
        PbrMetallicRoughness::new(self.document, &self.json.pbr_metallic_roughness)
    }

    /// Parameter values that define the specular-glossiness material model from
    /// Physically-Based Rendering (PBR) methodology.
    #[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
    #[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_pbrSpecularGlossiness")))]
    pub fn pbr_specular_glossiness(&self) -> Option<PbrSpecularGlossiness<'a>> {
        self.json.extensions
            .as_ref()?
            .pbr_specular_glossiness.as_ref()
            .map(|x| PbrSpecularGlossiness::new(self.document, x))
    }

    /// Parameter values that define the transmission of light through the material
    #[cfg(feature = "KHR_materials_transmission")]
    #[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_transmission")))]
    pub fn transmission(&self) -> Option<Transmission<'a>> {
        self.json.extensions
            .as_ref()?
            .transmission.as_ref()
            .map(|x| Transmission::new(self.document, x))
    }

    /// Parameter values that define the index of refraction of the material
    #[cfg(feature = "KHR_materials_ior")]
    #[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_ior")))]
    pub fn ior(&self) -> Option<f32> {
        self.json.extensions
            .as_ref()?
            .ior.as_ref()
            .map(|x| x.ior.0)
    }

    /// A tangent space normal map.
    ///
    /// The texture contains RGB components in linear space. Each texel represents
    /// the XYZ components of a normal vector in tangent space.
    ///
    /// * Red [0 to 255] maps to X [-1 to 1].
    /// * Green [0 to 255] maps to Y [-1 to 1].
    /// * Blue [128 to 255] maps to Z [1/255 to 1].
    ///
    /// The normal vectors use OpenGL conventions where +X is right, +Y is up, and
    /// +Z points toward the viewer.
    pub fn normal_texture(&self) -> Option<NormalTexture<'a>> {
        self.json.normal_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            NormalTexture::new(texture, json)
        })
    }

    /// The occlusion map texture.
    ///
    /// The occlusion values are sampled from the R channel. Higher values indicate
    /// areas that should receive full indirect lighting and lower values indicate
    /// no indirect lighting. These values are linear.
    ///
    /// If other channels are present (GBA), they are ignored for occlusion
    /// calculations.
    pub fn occlusion_texture(&self) -> Option<OcclusionTexture<'a>> {
        self.json.occlusion_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            OcclusionTexture::new(texture, json)
        })
    }

    /// The emissive map texture.
    ///
    /// The emissive map controls the color and intensity of the light being
    /// emitted by the material.
    ///
    /// This texture contains RGB components in sRGB color space. If a fourth
    /// component (A) is present, it is ignored.
    pub fn emissive_texture(&self) -> Option<texture::Info<'a>> {
        self.json.emissive_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    /// The emissive color of the material.
    ///
    /// The default value is `[0.0, 0.0, 0.0]`.
    pub fn emissive_factor(&self) -> [f32; 3] {
        self.json.emissive_factor.0
    }

    /// Specifies whether the material is unlit.
    /// 
    /// Returns `true` if the [`KHR_materials_unlit`] property was specified, in which 
    /// case the renderer should prefer to ignore all PBR values except `baseColor`.
    /// 
    /// [`KHR_materials_unlit`](https://github.com/KhronosGroup/glTF/tree/master/extensions/2.0/Khronos/KHR_materials_unlit#overview)
    #[cfg(feature = "KHR_materials_unlit")]
    #[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_unlit")))]
    pub fn unlit(&self) -> bool {
        self.json.extensions
            .as_ref()
            .map_or(false, |extensions| extensions.unlit.is_some())
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
pub struct PbrMetallicRoughness<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::material::PbrMetallicRoughness,
}

impl<'a> PbrMetallicRoughness<'a> {
    /// Constructs `PbrMetallicRoughness`.
    pub(crate) fn new(
        document: &'a Document,
        json: &'a json::material::PbrMetallicRoughness,
    ) -> Self {
        Self {
            document: document,
            json: json,
        }
    }

    /// Returns the material's base color factor.
    ///
    /// The default value is `[1.0, 1.0, 1.0, 1.0]`.
    pub fn base_color_factor(&self) -> [f32; 4] {
        self.json.base_color_factor.0
    }

    /// Returns the base color texture. The texture contains RGB(A) components
    /// in sRGB color space.
    pub fn base_color_texture(&self) -> Option<texture::Info<'a>> {
        self.json.base_color_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    /// Returns the metalness factor of the material.
    ///
    /// The default value is `1.0`.
    pub fn metallic_factor(&self) -> f32 {
        self.json.metallic_factor.0
    }

    /// Returns the roughness factor of the material.
    ///
    /// * A value of 1.0 means the material is completely rough.
    /// * A value of 0.0 means the material is completely smooth.
    ///
    /// The default value is `1.0`.
    pub fn roughness_factor(&self) -> f32 {
        self.json.roughness_factor.0
    }

    /// The metallic-roughness texture.
    ///
    /// The metalness values are sampled from the B channel.
    /// The roughness values are sampled from the G channel.
    /// These values are linear. If other channels are present (R or A),
    /// they are ignored for metallic-roughness calculations.
    pub fn metallic_roughness_texture(&self) -> Option<texture::Info<'a>> {
        self.json.metallic_roughness_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// A set of parameter values that are used to define the transmissions
/// factor of the material
#[cfg(feature = "KHR_materials_transmission")]
#[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_transmission")))]
pub struct Transmission<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::extensions::material::Transmission,
}

#[cfg(feature = "KHR_materials_transmission")]
#[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_transmission")))]
impl<'a> Transmission<'a> {
    /// Constructs `Ior`.
    pub(crate) fn new(
        document: &'a Document,
        json: &'a json::extensions::material::Transmission,
    ) -> Self {
        Self {
            document: document,
            json: json,
        }
    }

    /// Returns the material's transmission factor.
    ///
    /// The default value is `0.0`.
    pub fn transmission_factor(&self) -> f32 {
        self.json.transmission_factor.0
    }

    /// Returns the transmission texture.
    pub fn transmission_texture(&self) -> Option<texture::Info<'a>> {
        self.json.transmission_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// A set of parameter values that are used to define the specular-glossiness
/// material model from Physically-Based Rendering (PBR) methodology.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_pbrSpecularGlossiness")))]
pub struct PbrSpecularGlossiness<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::extensions::material::PbrSpecularGlossiness,
}

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_pbrSpecularGlossiness")))]
impl<'a> PbrSpecularGlossiness<'a> {
    /// Constructs `PbrSpecularGlossiness`.
    pub(crate) fn new(
        document: &'a Document,
        json: &'a json::extensions::material::PbrSpecularGlossiness,
    ) -> Self {
        Self {
            document: document,
            json: json,
        }
    }

    /// Returns the material's base color factor.
    ///
    /// The default value is `[1.0, 1.0, 1.0, 1.0]`.
    pub fn diffuse_factor(&self) -> [f32; 4] {
        self.json.diffuse_factor.0
    }

    /// Returns the base color texture.
    pub fn diffuse_texture(&self) -> Option<texture::Info<'a>> {
        self.json.diffuse_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    /// Returns the specular factor of the material.
    ///
    /// The default value is `[1.0, 1.0, 1.0]`.
    pub fn specular_factor(&self) -> [f32; 3] {
        self.json.specular_factor.0
    }

    /// Returns the glossiness factor of the material.
    ///
    /// A value of 1.0 means the material has full glossiness or is perfectly
    /// smooth. A value of 0.0 means the material has no glossiness or is
    /// completely rough. This value is linear.
    ///
    /// The default value is `1.0`.
    pub fn glossiness_factor(&self) -> f32 {
        self.json.glossiness_factor.0
    }

    /// The specular-glossiness texture.
    ///
    /// A RGBA texture, containing the specular color of the material (RGB
    /// components) and its glossiness (A component). The color values are in
    /// sRGB space.
    pub fn specular_glossiness_texture(&self) -> Option<texture::Info<'a>> {
        self.json.specular_glossiness_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// Defines the normal texture of a material.
pub struct NormalTexture<'a> {
    /// The parent `Texture` struct.
    texture: texture::Texture<'a>,

    /// The corresponding JSON struct.
    json: &'a json::material::NormalTexture,
}

impl<'a> NormalTexture<'a> {
    /// Constructs a `NormalTexture`.
    pub(crate) fn new(
        texture: texture::Texture<'a>,
        json: &'a json::material::NormalTexture,
    ) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the scalar multiplier applied to each normal vector of the texture.
    pub fn scale(&self) -> f32 {
        self.json.scale
    }

    /// The set index of the texture's `TEXCOORD` attribute.
    pub fn tex_coord(&self) -> u32 {
        self.json.tex_coord
    }

    /// Returns the referenced texture.
    pub fn texture(&self) -> texture::Texture<'a> {
        self.texture.clone()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// Defines the occlusion texture of a material.
pub struct OcclusionTexture<'a> {
    /// The parent `Texture` struct.
    texture: texture::Texture<'a>,

    /// The corresponding JSON struct.
    json: &'a json::material::OcclusionTexture,
}

impl<'a> OcclusionTexture<'a> {
    /// Constructs a `OcclusionTexture`.
    pub(crate) fn new(
        texture: texture::Texture<'a>,
        json: &'a json::material::OcclusionTexture,
    ) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the scalar multiplier controlling the amount of occlusion applied.
    pub fn strength(&self) -> f32 {
        self.json.strength.0
    }

    /// Returns the set index of the texture's `TEXCOORD` attribute.
    pub fn tex_coord(&self) -> u32 {
        self.json.tex_coord
    }

    /// Returns the referenced texture.
    pub fn texture(&self) -> texture::Texture<'a> {
        self.texture.clone()
    }
    
    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

impl<'a> AsRef<texture::Texture<'a>> for NormalTexture<'a> {
    fn as_ref(&self) -> &texture::Texture<'a> {
        &self.texture
    }
}

impl<'a> AsRef<texture::Texture<'a>> for OcclusionTexture<'a> {
    fn as_ref(&self) -> &texture::Texture<'a> {
        &self.texture
    }
}
