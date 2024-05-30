/// Contains `Accessor` and other related data structures.
pub mod accessor;

/// Contains `Animation` and other related data structures.
pub mod animation;

/// Contains `Asset` metadata.
pub mod asset;

/// Contains `Buffer`, `View`, and other related data structures.
pub mod buffer;

/// Contains `Camera` and other related data structures.
pub mod camera;

/// Contains `Image` and other related data structures.
pub mod image;

/// Contains `Material` and other related data structures.
pub mod material;

/// Contains `Mesh` and other related data structures.
pub mod mesh;

/// Contains `Root`.
pub mod root;

/// Contains `Scene`, `Node`, and other related data structures.
pub mod scene;

/// Contains `Skin` and other related data structures.
pub mod skin;

/// Contains `Texture`, `Sampler`, and other related data structures.
pub mod texture;

pub use self::root::Root;

/// Names of glTF 2.0 extensions enabled by the user.
pub const ENABLED_EXTENSIONS: &[&str] = &[
    #[cfg(feature = "KHR_lights_punctual")]
    "KHR_lights_punctual",
    #[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
    "KHR_materials_pbrSpecularGlossiness",
    #[cfg(feature = "KHR_materials_unlit")]
    "KHR_materials_unlit",
    #[cfg(feature = "KHR_texture_transform")]
    "KHR_texture_transform",
    #[cfg(feature = "KHR_materials_transmission")]
    "KHR_materials_transmission",
    #[cfg(feature = "KHR_materials_ior")]
    "KHR_materials_ior",
    #[cfg(feature = "KHR_materials_emissive_strength")]
    "KHR_materials_emissive_strength",
    // Allowlisted texture extensions. Processing is delegated to the user.
    #[cfg(feature = "allow_empty_texture")]
    "KHR_texture_basisu",
    #[cfg(feature = "EXT_texture_webp")]
    "EXT_texture_webp",
    #[cfg(feature = "allow_empty_texture")]
    "MSFT_texture_dds",
];

/// Names of glTF 2.0 extensions supported by the library.
pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "KHR_lights_punctual",
    "KHR_materials_pbrSpecularGlossiness",
    "KHR_materials_unlit",
    "KHR_texture_transform",
    "KHR_materials_transmission",
    "KHR_materials_ior",
    "KHR_materials_emissive_strength",
    "EXT_texture_webp",
];
