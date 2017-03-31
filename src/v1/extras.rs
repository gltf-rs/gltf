
use serde_json;
use std;

/// Untyped JSON object
pub type UntypedJsonObject = std::collections::HashMap<String, serde_json::Value>;

/// Type representing any user-defined data whatsoever
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Any;

impl Extras for Any {
    type Root = UntypedJsonObject;
    type Accessor = UntypedJsonObject;
    type Asset = UntypedJsonObject;
    type Animation = UntypedJsonObject;
    type AnimationChannel = UntypedJsonObject;
    type AnimationSampler = UntypedJsonObject;
    type AnimationTarget = UntypedJsonObject;
    type Buffer = UntypedJsonObject;
    type BufferView = UntypedJsonObject;
    type Camera = UntypedJsonObject;
    type CameraOrthographic = UntypedJsonObject;
    type CameraPerspective = UntypedJsonObject;
    type Image = UntypedJsonObject;
    type Material = UntypedJsonObject;
    type Mesh = UntypedJsonObject;
    type MeshPrimitive = UntypedJsonObject;
    type Node = UntypedJsonObject;
    type Program = UntypedJsonObject;
    type Sampler = UntypedJsonObject;
    type Scene = UntypedJsonObject;
    type Shader = UntypedJsonObject;
    type Skin = UntypedJsonObject;
    type Technique = UntypedJsonObject;
    type TechniqueState = UntypedJsonObject;
    type TechniqueFunction = UntypedJsonObject;
    type TechniqueParameter = UntypedJsonObject;
    type Texture = UntypedJsonObject;
}

/// Type representing no user-defined data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct None {
    #[serde(default)]
    _ignore_extra_fields: (),
}

impl Extras for None {
    type Root = None;
    type Accessor = None;
    type Asset = None;
    type Animation = None;
    type AnimationChannel = None;
    type AnimationSampler = None;
    type AnimationTarget = None;
    type Buffer = None;
    type BufferView = None;
    type Camera = None;
    type CameraOrthographic = None;
    type CameraPerspective = None;
    type Image = None;
    type Material = None;
    type Mesh = None;
    type MeshPrimitive = None;
    type Node = None;
    type Program = None;
    type Sampler = None;
    type Scene = None;
    type Shader = None;
    type Skin = None;
    type Technique = None;
    type TechniqueState = None;
    type TechniqueFunction = None;
    type TechniqueParameter = None;
    type Texture = None;
}

