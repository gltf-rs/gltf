
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::marker::PhantomData;

use json::{Extras, Root};
use validation::{Error, JsonPath, Validate};

/// All valid camera types.
pub const VALID_CAMERA_TYPES: &'static [&'static str] = &[
    "perspective",
    "orthographic",
];

/// A camera's projection.
///
/// A node can reference a camera to apply a transform to place the camera in the
/// scene.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Camera<'a> {
    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,

    /// An orthographic camera containing properties to create an orthographic
    /// projection matrix.
    pub orthographic: Option<Orthographic<'a>>,

    /// A perspective camera containing properties to create a perspective
    /// projection matrix.
    pub perspective: Option<Perspective<'a>>,

    /// Specifies if the camera uses a perspective or orthographic projection.
    #[serde(rename = "type")]
    pub type_: Type<'a>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: CameraExtensions<'a>,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
}

/// Extension specific data for `Camera`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct CameraExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// Values for an orthographic camera.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Orthographic<'a> {
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
    pub extensions: OrthographicExtensions<'a>,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
}

/// Extension specific data for `Orthographic`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct OrthographicExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// Values for a perspective camera.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Perspective<'a> {
    /// Aspect ratio of the field of view.
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: Option<f32>,

    /// The vertical field of view in radians.
    pub yfov: f32,

    /// The distance to the far clipping plane.
    pub zfar: Option<f32>,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: PerspectiveExtensions<'a>,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
}

/// Extension specific data for `Perspective`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct PerspectiveExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// Specifies the camera type.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Type<'a>(pub Cow<'a, str>);

impl<'a> Validate<'a> for Camera<'a> {
    fn validate<P, R>(&self, root: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if self.orthographic.is_none() && self.perspective.is_none() {
            let reason = "one of `orthographic` or `perspective` is required";
            report(Error::missing_data(path().clone(), reason.to_string()));
        }

        self.orthographic.validate(root, || path().field("orthographic"), report);
        self.perspective.validate(root, || path().field("perspective"), report);
        self.type_.validate(root, || path().field("type"), report);
        self.extensions.validate(root, || path().field("extensions"), report);
        self.extras.validate(root, || path().field("extras"), report);
    }
}

impl<'a> Validate<'a> for Orthographic<'a> {
    fn validate<P, R>(&self, root: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if self.znear < 0.0 {
            report(Error::invalid_value(path(), self.znear));
        }
 
        if self.zfar < 0.0  || self.zfar < self.znear {
            report(Error::invalid_value(path(), self.zfar));
        }

        self.extensions.validate(root, || path().field("extensions"), report);
        self.extras.validate(root, || path().field("extras"), report);
    }
}

impl<'a> Validate<'a> for Perspective<'a> {
    fn validate<P, R>(&self, root: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        self.aspect_ratio.map(|aspect_ratio| {
            if aspect_ratio < 0.0 {
                report(Error::invalid_value(path(), aspect_ratio));
            }
        });

        if self.yfov < 0.0 {
            report(Error::invalid_value(path(), self.yfov));
        }

        if self.znear < 0.0 {
            report(Error::invalid_value(path(), self.znear));
        }

        self.zfar.map(|zfar| {
            if zfar < 0.0 || zfar < self.znear {
                report(Error::invalid_value(path(), zfar));
            }
        });

        self.extensions.validate(root, || path().field("extensions"), report);
        self.extras.validate(root, || path().field("extras"), report);
    }
}

impl<'a> Validate<'a> for Type<'a> {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if !VALID_CAMERA_TYPES.contains(&self.0.as_ref()) {
            report(Error::invalid_enum(path(), self.0.clone()));
        }
    }
}
