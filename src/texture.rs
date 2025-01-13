use crate::{image, Document};

pub use json::texture::{MagFilter, MinFilter, WrappingMode};
#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

lazy_static! {
    static ref DEFAULT_SAMPLER: json::texture::Sampler = Default::default();
}

/// A reference to a `Texture`.
#[derive(Clone, Debug)]
pub struct Info<'a> {
    /// The parent `Texture` struct.
    texture: Texture<'a>,

    /// The corresponding JSON struct.
    json: &'a json::texture::Info,
}

///  Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug)]
pub struct Sampler<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index - `None` when the default sampler.
    index: Option<usize>,

    /// The corresponding JSON struct.
    json: &'a json::texture::Sampler,
}

/// A texture and its sampler.
#[derive(Clone, Debug)]
pub struct Texture<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::texture::Texture,
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::texture::Sampler,
    ) -> Self {
        Self {
            document,
            index: Some(index),
            json,
        }
    }

    /// Constructs the default `Sampler`.
    pub(crate) fn default(document: &'a Document) -> Self {
        Self {
            document,
            index: None,
            json: &DEFAULT_SAMPLER,
        }
    }

    /// Returns the internal JSON index if this `Sampler` was explicity defined.
    ///
    /// This function returns `None` if the `Sampler` is the default sampler.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    /// Magnification filter.
    pub fn mag_filter(&self) -> Option<MagFilter> {
        self.json.mag_filter.map(|filter| filter.unwrap())
    }

    /// Minification filter.
    pub fn min_filter(&self) -> Option<MinFilter> {
        self.json.min_filter.map(|filter| filter.unwrap())
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_deref()
    }

    /// `s` wrapping mode.
    pub fn wrap_s(&self) -> WrappingMode {
        self.json.wrap_s.unwrap()
    }

    /// `t` wrapping mode.
    pub fn wrap_t(&self) -> WrappingMode {
        self.json.wrap_t.unwrap()
    }

    /// Returns extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        let ext = self.json.extensions.as_ref()?;
        Some(&ext.others)
    }

    /// Queries extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extension_value(&self, ext_name: &str) -> Option<&Value> {
        let ext = self.json.extensions.as_ref()?;
        ext.others.get(ext_name)
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl<'a> Texture<'a> {
    /// Constructs a `Texture`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::texture::Texture,
    ) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_deref()
    }

    /// Returns the sampler used by this texture.
    pub fn sampler(&self) -> Sampler<'a> {
        self.json
            .sampler
            .as_ref()
            .map(|index| self.document.samplers().nth(index.value()).unwrap())
            .unwrap_or_else(|| Sampler::default(self.document))
    }

    /// Returns the image used by this texture.
    #[cfg(feature = "allow_empty_texture")]
    pub fn source(&self) -> Option<image::Image<'a>> {
        let index = self.json.primary_source().value();
        if index == u32::MAX as usize {
            None
        } else {
            Some(self.document.images().nth(index).unwrap())
        }
    }

    /// Returns the image used by this texture.
    #[cfg(not(feature = "allow_empty_texture"))]
    pub fn source(&self) -> image::Image<'a> {
        self.document
            .images()
            .nth(self.json.primary_source().value())
            .unwrap()
    }

    /// Returns extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        let ext = self.json.extensions.as_ref()?;
        Some(&ext.others)
    }

    /// Queries extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extension_value(&self, ext_name: &str) -> Option<&Value> {
        let ext = self.json.extensions.as_ref()?;
        ext.others.get(ext_name)
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl<'a> Info<'a> {
    /// Constructs a reference to a `Texture`.
    pub(crate) fn new(texture: Texture<'a>, json: &'a json::texture::Info) -> Self {
        Self { texture, json }
    }

    /// The set index of the texture's `TEXCOORD` attribute.
    pub fn tex_coord(&self) -> u32 {
        self.json.tex_coord
    }

    /// Returns the referenced `Texture`.
    pub fn texture(&self) -> Texture<'a> {
        self.texture.clone()
    }

    /// Returns texture transform information
    #[cfg(feature = "KHR_texture_transform")]
    #[cfg_attr(docsrs, doc(cfg(feature = "KHR_texture_transform")))]
    pub fn texture_transform(&self) -> Option<TextureTransform<'a>> {
        self.json
            .extensions
            .as_ref()?
            .texture_transform
            .as_ref()
            .map(TextureTransform::new)
    }

    /// Returns extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        let ext = self.json.extensions.as_ref()?;
        Some(&ext.others)
    }

    /// Queries extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extension_value(&self, ext_name: &str) -> Option<&Value> {
        let ext = self.json.extensions.as_ref()?;
        ext.others.get(ext_name)
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl<'a> AsRef<Texture<'a>> for Info<'a> {
    fn as_ref(&self) -> &Texture<'a> {
        &self.texture
    }
}

/// Many techniques can be used to optimize resource usage for a 3d scene.
/// Chief among them is the ability to minimize the number of textures the GPU must load.
/// To achieve this, many engines encourage packing many objects' low-resolution textures into a single large texture atlas.
/// The region of the resulting atlas that corresponds with each object is then defined by vertical and horizontal offsets,
/// and the width and height of the region.
///
/// To support this use case, this extension adds `offset`, `rotation`, and `scale` properties to textureInfo structures.
/// These properties would typically be implemented as an affine transform on the UV coordinates.
#[cfg(feature = "KHR_texture_transform")]
pub struct TextureTransform<'a> {
    /// The corresponding JSON struct.
    json: &'a json::extensions::texture::TextureTransform,
}

#[cfg(feature = "KHR_texture_transform")]
impl<'a> TextureTransform<'a> {
    /// Constructs `TextureTransform`
    pub(crate) fn new(json: &'a json::extensions::texture::TextureTransform) -> Self {
        Self { json }
    }

    /// The offset of the UV coordinate origin as a factor of the texture dimensions.
    pub fn offset(&self) -> [f32; 2] {
        self.json.offset.0
    }

    /// Rotate the UVs by this many radians counter-clockwise around the origin.
    /// This is equivalent to a similar rotation of the image clockwise.
    pub fn rotation(&self) -> f32 {
        self.json.rotation.0
    }

    /// The scale factor applied to the components of the UV coordinates.
    pub fn scale(&self) -> [f32; 2] {
        self.json.scale.0
    }

    /// Overrides the textureInfo texCoord value if supplied, and if this extension is supported.
    pub fn tex_coord(&self) -> Option<u32> {
        self.json.tex_coord
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
