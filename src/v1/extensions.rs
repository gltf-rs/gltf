
/// Names of glTF 1.0 extensions enabled by the user
pub const ENABLED_EXTENSION_NAMES: &'static [&'static str] = &[
    #[cfg(feature = "KHR_binary_glTF")]
    "KHR_binary_glTF",
    #[cfg(feature = "KHR_materials_common")]
    "KHR_materials_common",
];

/// Names of glTF 1.0 extensions supported by the library
pub const SUPPORTED_EXTENSION_NAMES: &'static [&'static str] = &[
    "KHR_binary_glTF",
    "KHR_materials_common",
];

