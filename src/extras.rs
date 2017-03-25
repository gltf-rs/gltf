
use traits::Extras;

/// Type representing no user-defined data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct None;

impl Extras for None {
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
