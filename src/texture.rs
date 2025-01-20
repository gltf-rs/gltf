use crate::validation::Validate;
use crate::{image, Extras, Index, UnrecognizedExtensions};

/// Support for the `KHR_texture_transform` extension.
pub mod khr_texture_transform {
    /// Many techniques can be used to optimize resource usage for a 3d scene.
    /// Chief among them is the ability to minimize the number of textures the GPU must load.
    /// To achieve this, many engines encourage packing many objects' low-resolution textures into a single large texture atlas.
    /// The region of the resulting atlas that corresponds with each object is then defined by vertical and horizontal offsets,
    /// and the width and height of the region.
    ///
    /// To support this use case, this extension adds `offset`, `rotation`, and `scale` properties to textureInfo structures.
    /// These properties would typically be implemented as an affine transform on the UV coordinates.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct Transform {
        /// The offset of the UV coordinate origin as a factor of the texture dimensions.
        #[gltf(default)]
        pub offset: [f32; 2],

        /// Rotate the UVs by this many radians counter-clockwise around the origin.
        /// This is equivalent to a similar rotation of the image clockwise.
        #[gltf(default)]
        pub rotation: f32,

        /// The scale factor applied to the components of the UV coordinates.
        #[gltf(default = [1.0; 2])]
        pub scale: [f32; 2],

        /// Overrides the textureInfo texCoord value if supplied, and if this extension is supported.
        pub tex_coord: Option<u32>,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// Magnification filter.
#[derive(
    Clone, Copy, Debug, serde_repr::Deserialize_repr, Eq, PartialEq, serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum MagFilter {
    /// Corresponds to `GL_NEAREST`.
    Nearest = 9728,

    /// Corresponds to `GL_LINEAR`.
    Linear = 9729,
}

impl Validate for MagFilter {}

impl MagFilter {
    /// OpenGL enum
    pub fn as_gl_enum(self) -> u32 {
        self as u32
    }
}

/// Minification filter.
#[derive(
    Clone, Copy, Debug, serde_repr::Deserialize_repr, Eq, PartialEq, serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum MinFilter {
    /// Corresponds to `GL_NEAREST`.
    Nearest = MagFilter::Nearest as u32,

    /// Corresponds to `GL_LINEAR`.
    Linear = MagFilter::Linear as u32,

    /// Corresponds to `GL_NEAREST_MIPMAP_NEAREST`.
    NearestMipmapNearest = 9984,

    /// Corresponds to `GL_LINEAR_MIPMAP_NEAREST`.
    LinearMipmapNearest = 9985,

    /// Corresponds to `GL_NEAREST_MIPMAP_LINEAR`.
    NearestMipmapLinear = 9986,

    /// Corresponds to `GL_LINEAR_MIPMAP_LINEAR`.
    LinearMipmapLinear = 9987,
}

impl Validate for MinFilter {}

impl MinFilter {
    /// Returns the corresponding OpenGL enum value.
    pub fn as_gl_enum(self) -> u32 {
        self as u32
    }
}

/// Texture co-ordinate wrapping mode.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    serde_repr::Deserialize_repr,
    Eq,
    PartialEq,
    serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum WrappingMode {
    /// Corresponds to `GL_CLAMP_TO_EDGE`.
    ClampToEdge = 33_071,

    /// Corresponds to `GL_MIRRORED_REPEAT`.
    MirroredRepeat = 33_648,

    /// Corresponds to `GL_REPEAT`.
    #[default]
    Repeat = 10_497,
}

impl Validate for WrappingMode {}

impl WrappingMode {
    /// Returns the corresponding OpenGL enum value.
    pub fn as_gl_enum(self) -> u32 {
        self as u32
    }
}

/// Texture sampler properties for filtering and wrapping modes.
#[derive(
    Clone, Debug, Default, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate,
)]
pub struct Sampler {
    /// Magnification filter.
    pub mag_filter: Option<MagFilter>,

    /// Minification filter.
    pub min_filter: Option<MinFilter>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// `s` wrapping mode.
    #[gltf(default)]
    pub wrap_s: WrappingMode,

    /// `t` wrapping mode.
    #[gltf(default)]
    pub wrap_t: WrappingMode,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// A texture and its sampler.
#[derive(
    Clone,
    Debug,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Stub,
    gltf_derive::Validate,
)]
pub struct Texture {
    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The index of the sampler used by this texture.
    pub sampler: Option<Index<Sampler>>,

    /// The index of the image used by this texture.
    pub source: Index<image::Image>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

#[derive(
    Clone,
    Debug,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Stub,
    gltf_derive::Validate,
)]
/// Reference to a `Texture`.
pub struct Info {
    /// The index of the texture.
    pub index: Index<Texture>,

    /// The set index of the texture's `TEXCOORD` attribute.
    #[gltf(default)]
    pub tex_coord: u32,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,

    /// Support for the `KHR_texture_transform` extension.
    #[gltf(extension = "KHR_texture_transform")]
    pub transform: Option<khr_texture_transform::Transform>,
}
