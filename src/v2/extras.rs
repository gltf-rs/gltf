
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::{Deserialize, Serialize};
use serde::export::fmt::Debug;
use serde_json;
use std;

/// Defines a family of user-defined data structures to be (de)serialized
pub trait Extras: Clone + Debug + Default + Deserialize + Serialize {
    type Root: Clone + Debug + Default + Deserialize + Serialize;
    type Accessor: Clone + Debug + Default + Deserialize + Serialize;
    type AccessorSparseIndices: Clone + Debug + Default + Deserialize + Serialize;
    type AccessorSparseStorage: Clone + Debug + Default + Deserialize + Serialize;
    type AccessorSparseValues: Clone + Debug + Default + Deserialize + Serialize;
    type Asset: Clone + Debug + Default + Deserialize + Serialize;
    type Animation: Clone + Debug + Default + Deserialize + Serialize;
    type AnimationChannel: Clone + Debug + Default + Deserialize + Serialize;
    type AnimationSampler: Clone + Debug + Default + Deserialize + Serialize;
    type AnimationTarget: Clone + Debug + Default + Deserialize + Serialize;
    type Buffer: Clone + Debug + Default + Deserialize + Serialize;
    type BufferView: Clone + Debug + Default + Deserialize + Serialize;
    type Camera: Clone + Debug + Default + Deserialize + Serialize;
    type CameraOrthographic: Clone + Debug + Default + Deserialize + Serialize;
    type CameraPerspective: Clone + Debug + Default + Deserialize + Serialize;
    type Image: Clone + Debug + Default + Deserialize + Serialize;
    type Material: Clone + Debug + Default + Deserialize + Serialize;
    type MaterialPbrMetallicRoughness: Clone + Debug + Default + Deserialize + Serialize;
    type MaterialNormalTexture: Clone + Debug + Default + Deserialize + Serialize;
    type MaterialOcclusionTexture: Clone + Debug + Default + Deserialize + Serialize;
    type Mesh: Clone + Debug + Default + Deserialize + Serialize;
    type MeshPrimitive: Clone + Debug + Default + Deserialize + Serialize;
    type Node: Clone + Debug + Default + Deserialize + Serialize;
    type Program: Clone + Debug + Default + Deserialize + Serialize;
    type Sampler: Clone + Debug + Default + Deserialize + Serialize;
    type Scene: Clone + Debug + Default + Deserialize + Serialize;
    type Shader: Clone + Debug + Default + Deserialize + Serialize;
    type Skin: Clone + Debug + Default + Deserialize + Serialize;
    type Technique: Clone + Debug + Default + Deserialize + Serialize;
    type TechniqueState: Clone + Debug + Default + Deserialize + Serialize;
    type TechniqueFunction: Clone + Debug + Default + Deserialize + Serialize;
    type TechniqueParameter: Clone + Debug + Default + Deserialize + Serialize;
    type Texture: Clone + Debug + Default + Deserialize + Serialize;
    type TextureInfo: Clone + Debug + Default + Deserialize + Serialize;
}

/// Untyped JSON object
pub type UntypedJsonObject = std::collections::HashMap<String, serde_json::Value>;

/// Type representing any user-defined data whatsoever
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Any;

impl Extras for Any {
    type Root = UntypedJsonObject;
    type Accessor = UntypedJsonObject;
    type AccessorSparseIndices = UntypedJsonObject;
    type AccessorSparseStorage = UntypedJsonObject;
    type AccessorSparseValues = UntypedJsonObject;
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
    type MaterialPbrMetallicRoughness = UntypedJsonObject;
    type MaterialNormalTexture = UntypedJsonObject;
    type MaterialOcclusionTexture = UntypedJsonObject;
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
    type TextureInfo = UntypedJsonObject;
}

/// Type representing no user-defined data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct None {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl Extras for None {
    type Root = None;
    type Accessor = None;
    type AccessorSparseIndices = None;
    type AccessorSparseStorage = None;
    type AccessorSparseValues = None;
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
    type MaterialPbrMetallicRoughness = None;
    type MaterialNormalTexture = None;
    type MaterialOcclusionTexture = None;
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
    type TextureInfo = None;
}

