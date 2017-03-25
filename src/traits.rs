
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::{Deserialize, Serialize};
use serde::export::fmt::Debug;

/// Helper trait for retrieving top-level objects by a universal identifier
pub trait Get<T> {
    /// Associated identifier type
    type Id;
    /// Retrieves a single value at the given index
    fn get(&self, id: Self::Id) -> &T;
}

/// Defines a family of user-defined data structures to be (de)serialized
pub trait Extras: Clone + Debug + Default + Deserialize + Serialize {
    type Root: Clone + Debug + Default + Deserialize + Serialize;
    type Accessor: Clone + Debug + Default + Deserialize + Serialize;
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
}
