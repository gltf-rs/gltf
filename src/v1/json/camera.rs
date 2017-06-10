// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::json::Extras;

enum_string! {
    CameraType {
        Orthographic = "orthographic",
        Perspective = "perspective",
    }
}

/// A camera's projection.
///
/// A node can reference a camera ID to apply a transform to place the camera in
/// the scene.
#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    /// An orthographic camera containing properties to create an orthographic
    /// projection matrix.
    pub orthographic: Option<Orthographic>,

    /// A perspective camera containing properties to create a perspective
    /// projection matrix.
    pub perspective: Option<Perspective>,

    /// Specifies if the camera uses a perspective or orthographic projection.
    ///
    /// Based on this, either the camera's perspective or orthographic property
    /// will be defined.
    #[serde(rename = "type")]
    #[serde(default)]
    pub kind: CameraType,

    /// The user-defined name of this object.
    pub name: Option<String>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: CameraExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Camera`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CameraExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// An orthographic camera containing properties to create an orthographic
/// projection matrix.
#[derive(Debug, Deserialize, Serialize)]
pub struct Orthographic {
    /// The horizontal magnification of the view.
    #[serde(rename = "xmag")]
    pub x_mag: f32,

    /// The vertical magnification of the view.
    #[serde(rename = "ymag")]
    pub y_mag: f32,

    /// The distance to the far clipping plane.
    #[serde(rename = "zfar")]
    pub z_far: f32,

    /// The distance to the near clipping plane.
    #[serde(rename = "znear")]
    pub z_near: f32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: OrthographicExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Orthographic`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrthographicExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// A perspective camera containing properties to create a perspective projection
/// matrix.
#[derive(Debug, Deserialize, Serialize)]
pub struct Perspective {
    /// The aspect ratio of the field of view.
    ///
    /// When this is undefined, the aspect ratio of the canvas is used.
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: Option<f32>,

    /// The vertical field of view in radians.
    #[serde(rename = "yfov")]
    pub y_fov: f32,

    /// The distance to the far clipping plane.
    ///
    /// `z_far` must be greater than `z_near`.
    #[serde(rename = "zfar")]
    pub z_far: f32,

    /// The distance to the near clipping plane.
    ///
    /// `z_far` must be greater than `z_near`.
    #[serde(rename = "znear")]
    pub z_near: f32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: PerspectiveExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Perspective`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PerspectiveExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl Default for CameraType {
    fn default() -> CameraType {
        CameraType::Perspective
    }
}
