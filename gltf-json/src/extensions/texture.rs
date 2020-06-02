#[cfg(feature = "KHR_texture_transform")]
use crate::{extras::Extras, validation::Validate};
use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Sampler {}

/// A texture and its sampler.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Texture {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
/// Reference to a `Texture`.
pub struct Info {
    #[cfg(feature = "KHR_texture_transform")]
    #[serde(default, rename = "KHR_texture_transform", skip_serializing_if = "Option::is_none")]
    pub texture_transform: Option<TextureTransform>,
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
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct TextureTransform {
    // The offset of the UV coordinate origin as a factor of the texture dimensions.
    pub offset: TextureTransformOffset,

    /// Rotate the UVs by this many radians counter-clockwise around the origin.
    /// This is equivalent to a similar rotation of the image clockwise.
    pub rotation: TextureTransformRotation,

    /// The scale factor applied to the components of the UV coordinates.
    pub scale: TextureTransformScale,

    /// Overrides the textureInfo texCoord value if supplied, and if this extension is supported.
    pub tex_coord: Option<u32>,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// The offset of the UV coordinate origin as a factor of the texture dimensions.
#[cfg(feature = "KHR_texture_transform")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct TextureTransformOffset(pub [f32; 2]);

#[cfg(feature = "KHR_texture_transform")]
impl Default for TextureTransformOffset {
    fn default() -> Self {
        Self([0.0, 0.0])
    }
}

#[cfg(feature = "KHR_texture_transform")]
impl Validate for TextureTransformOffset {}

/// Rotate the UVs by this many radians counter-clockwise around the origin.
/// This is equivalent to a similar rotation of the image clockwise.
#[cfg(feature = "KHR_texture_transform")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct TextureTransformRotation(pub f32);

#[cfg(feature = "KHR_texture_transform")]
impl Default for TextureTransformRotation {
    fn default() -> Self {
        Self(0.0)
    }
}

#[cfg(feature = "KHR_texture_transform")]
impl Validate for TextureTransformRotation {}

/// The scale factor applied to the components of the UV coordinates.
#[cfg(feature = "KHR_texture_transform")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct TextureTransformScale(pub [f32; 2]);

#[cfg(feature = "KHR_texture_transform")]
impl Default for TextureTransformScale {
    fn default() -> Self {
        Self([1.0, 1.0])
    }
}

#[cfg(feature = "KHR_texture_transform")]
impl Validate for TextureTransformScale {}
