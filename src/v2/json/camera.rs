
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::json::{Extras};

/// A camera's projection.
///
/// A node can reference a camera to apply a transform to place the camera in the
/// scene.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Camera {
    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// An orthographic camera containing properties to create an orthographic
    /// projection matrix.
    pub orthographic: Option<Orthographic>,

    /// A perspective camera containing properties to create a perspective
    /// projection matrix.
    pub perspective: Option<Perspective>,

    /// Specifies if the camera uses a perspective or orthographic projection.
    #[serde(rename = "type")]
    pub type_: String,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: CameraExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Camera`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct CameraExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Values for an orthographic camera.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Orthographic {
    /// The horizontal magnification of the view.
    pub xmag: f32,

    /// The vertical magnification of the view.
    pub ymag: f32,

    /// The distance to the far clipping plane.
    pub zfar: f32,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: OrthographicExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Orthographic`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct OrthographicExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Values for a perspective camera.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Perspective {
    /// Aspect ratio of the field of view.
    #[serde(default, rename = "aspectRatio")]
    pub aspect_ratio: f32,

    /// The vertical field of view in radians.
    pub yfov: f32,

    /// The distance to the far clipping plane.
    #[serde(default)]
    pub zfar: f32,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: PerspectiveExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Perspective`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct PerspectiveExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}
