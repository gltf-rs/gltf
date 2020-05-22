use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

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

/// Foo
#[cfg(feature = "KHR_texture_transform")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct TextureTransform {
    /// Foo
    pub offset: TextureTransformOffset,

    /// Foo
    pub rotation: TextureTransformRotation,

    /// Foo
    pub scale: TextureTransformScale,

    /// Foo
    pub tex_coord: Option<u32>,
}

/// Foo
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

/// Foo
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

/// Foo
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
