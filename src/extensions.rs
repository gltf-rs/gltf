
/// Khronos extensions
pub mod khr {
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MaterialsCommon {
        pub ambient: [f32; 4],
        pub diffuse: [f32; 4],
        pub specular: [f32; 4],
    }
}

