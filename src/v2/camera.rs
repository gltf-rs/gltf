
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Gltf;
use v2::json;

#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Orthographic,
    Perspective,
}

///  A camera's projection.  A node can reference a camera to apply a transform to
/// place the camera in the scene.
pub struct Camera<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::camera::Camera,

    /// Specifies whether the camera projection is perspective or orthographic.
    kind: Kind,
}

impl<'a> Camera<'a> {
    /// Constructs a `Camera`.
    pub fn new(gltf: &'a Gltf, json: &'a json::camera::Camera) -> Self {
        Self {
            gltf: gltf,
            json: json,
            kind: match json.type_.0.as_str() {
                "orthographic" => Kind::Orthographic,
                "perspective" => Kind::Perspective,
                _ => unreachable!(),
            },
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::camera::Camera {
        self.json
    }

    ///  Optional user-defined name for this object.
    pub fn name(&self) -> &Option<String> {
        unimplemented!()
    }

    ///  Specifies if the camera uses a perspective or orthographic projection.
    pub fn kind(&self) -> Kind {
        self.kind
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::camera::CameraExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }
}

///  Values for an orthographic camera.
pub struct Orthographic<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::camera::Orthographic,
}

impl<'a> Orthographic<'a> {
    /// Constructs a `Orthographic`.
    pub fn new(gltf: &'a Gltf, json: &'a json::camera::Orthographic) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::camera::Orthographic {
        self.json
    }

    ///  The horizontal magnification of the view.
    pub fn xmag(&self) -> f32 {
        unimplemented!()
    }

    ///  The vertical magnification of the view.
    pub fn ymag(&self) -> f32 {
        unimplemented!()
    }

    ///  The distance to the far clipping plane.
    pub fn zfar(&self) -> f32 {
        unimplemented!()
    }

    ///  The distance to the near clipping plane.
    pub fn znear(&self) -> f32 {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::camera::OrthographicExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }
}
///  Values for a perspective camera.
pub struct Perspective<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::camera::Perspective,
}

impl<'a> Perspective<'a> {
    /// Constructs a `Perspective`.
    pub fn new(gltf: &'a Gltf, json: &'a json::camera::Perspective) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::camera::Perspective {
        self.json
    }

    ///  Aspect ratio of the field of view.
    pub fn aspect_ratio(&self) -> Option<f32> {
        unimplemented!()
    }

    ///  The vertical field of view in radians.
    pub fn yfov(&self) -> f32 {
        unimplemented!()
    }

    ///  The distance to the far clipping plane.
    pub fn zfar(&self) -> Option<f32> {
        unimplemented!()
    }

    ///  The distance to the near clipping plane.
    pub fn znear(&self) -> f32 {
        unimplemented!()
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::camera::PerspectiveExtensions {
        unimplemented!()
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        unimplemented!()
    }
}
