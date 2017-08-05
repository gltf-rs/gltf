
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// A camera's projection.
///
/// A node can reference a camera to apply a transform to place the camera in the
/// scene.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct Camera {}

/// Values for an orthographic camera.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct Orthographic {}

/// Values for a perspective camera.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct Perspective {}
