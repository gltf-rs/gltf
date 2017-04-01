
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
    /// `Extras` type for `Root`
    type Root: Clone + Debug + Default + Deserialize + Serialize;
    
    /// `Extras` type for `accessor::Accessor`
    type Accessor: Clone + Debug + Default + Deserialize + Serialize;
    
    /// `Extras` type for `accessor::sparse::Indices`
    type AccessorSparseIndices: Clone + Debug + Default + Deserialize + Serialize;
    
    /// `Extras` type for `accessor::sparse::Storage`
    type AccessorSparseStorage: Clone + Debug + Default + Deserialize + Serialize;
    
    /// `Extras` type for `accessor::sparse::Values`
    type AccessorSparseValues: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `Asset`
    type Asset: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `animation::Animation`
    type Animation: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `animation::Channel`
    type AnimationChannel: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `animation::Sampler`
    type AnimationSampler: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `animation::Target`
    type AnimationTarget: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `buffer::Buffer`
    type Buffer: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `buffer::BufferView`
    type BufferView: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `camera::Camera`
    type Camera: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `camera::Orthographic`
    type CameraOrthographic: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `camera::Perspective`
    type CameraPerspective: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `image::Image`
    type Image: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `material::Material`
    type Material: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `material::PbrMetallicRoughness`
    type MaterialPbrMetallicRoughness: Clone + Debug + Default + Deserialize + Serialize;
    
    /// `Extras` type for `material::NormalTexture`
    type MaterialNormalTexture: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `material::OcclusionTexture`
    type MaterialOcclusionTexture: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `mesh::Mesh`
    type Mesh: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `mesh::Primitive`
    type MeshPrimitive: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `scene::Node`
    type Node: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `texture::Sampler`
    type Sampler: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `scene::Scene`
    type Scene: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `skin::Skin`
    type Skin: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `texture::Texture`
    type Texture: Clone + Debug + Default + Deserialize + Serialize;

    /// `Extras` type for `texture::TextureInfo`
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
    type Sampler = UntypedJsonObject;
    type Scene = UntypedJsonObject;
    type Skin = UntypedJsonObject;
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
    type Sampler = None;
    type Scene = None;
    type Skin = None;
    type Texture = None;
    type TextureInfo = None;
}
