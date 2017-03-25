
/// Names of glTF 2.0 extensions enabled by the user
pub const ENABLED_EXTENSION_NAMES: &'static [&'static str] = &[
    #[cfg(feature = "KHR_materials_common")]
    "KHR_materials_common",
];

/// Names of glTF 2.0 extensions supported by the library
pub const SUPPORTED_EXTENSION_NAMES: &'static [&'static str] = &[
    "KHR_materials_common",
];

/// Khronos extensions
pub mod khr {
    /// `KHR_materials_common`
    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct MaterialsCommon {
        pub ambient: [f32; 4],
        pub diffuse: [f32; 4],
        pub specular: [f32; 4],
        // TODO: Add the remaining fields
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Extensions {
    #[cfg(feature = "KHR_materials_common")]
    pub khr_materials_common: Option<khr::MaterialsCommon>,
}

