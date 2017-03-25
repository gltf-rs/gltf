
use traits::Extensions;

/// Type representing no extensions
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct None;

impl Extensions for None {
    type Root = ();
    type Accessor = ();
    type Asset = ();
    type Animation = ();
    type AnimationChannel = ();
    type AnimationSampler = ();
    type AnimationTarget = ();
    type Buffer = ();
    type BufferView = ();
    type Camera = ();
    type CameraOrthographic = ();
    type CameraPerspective = ();
    type Image = ();
    type Material = ();
    type Mesh = ();
    type MeshPrimitive = ();
    type Node = ();
    type Program = ();
    type Sampler = ();
    type Scene = ();
    type Shader = ();
    type Skin = ();
    type Technique = ();
    type TechniqueState = ();
    type TechniqueFunction = ();
    type TechniqueParameter = ();
    type Texture = ();
}

/// Khronos extensions
pub mod khr {
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MaterialsCommon {
        pub ambient: [f32; 4],
        pub diffuse: [f32; 4],
        pub specular: [f32; 4],
    }
}


