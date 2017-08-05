
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The material appearance of a primitive.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct Material {}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct PbrMetallicRoughness {}

/// Defines the normal texture of a material.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct NormalTexture {}

/// Defines the occlusion texture of a material.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct OcclusionTexture {}
