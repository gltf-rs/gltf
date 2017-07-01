
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use {json, Gltf};

/// A camera's projection.
#[derive(Clone, Debug)]
pub enum Projection {
    /// Describes an orthographic projection.
    Orthographic(Orthographic),

    /// Describes a perspective projection.
    Perspective(Perspective),
}

/// A camera's projection.  A node can reference a camera to apply a transform to
/// place the camera in the scene.
#[derive(Clone, Debug)]
pub struct Camera {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::camera::Camera,
}
  
///  Values for an orthographic camera projection.
#[derive(Clone, Debug)]
pub struct Orthographic {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::camera::Orthographic,
}
  
/// Values for a perspective camera projection.
#[derive(Clone, Debug)]
pub struct Perspective {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::camera::Perspective,
}

impl Camera {
    /// Constructs a `Camera`.
    pub fn new(gltf: &'a Gltf, json: &'a json::camera::Camera) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::camera::Camera {
        self.json
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(Cow::as_ref)
    }

    /// Returns the camera's projection.
    pub fn projection(&self) -> Projection {
        match self.json.type_.0.as_ref() {
            "orthographic" => {
                let json = self.json.orthographic.as_ref().unwrap();
                Projection::Orthographic(Orthographic::new(self.gltf, json))
            },
            "perspective" => {
                let json = self.json.perspective.as_ref().unwrap();
                Projection::Perspective(Perspective::new(self.gltf, json))
            },
            _ => unreachable!(),
        }
    } 
    
    /// Extension specific data.
    pub fn extensions(&self) -> &json::camera::CameraExtensions {
        &self.json.extensions
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl Orthographic {
    /// Constructs a `Orthographic` camera projection.
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
        self.json.xmag
    }

    ///  The vertical magnification of the view.
    pub fn ymag(&self) -> f32 {
        self.json.ymag
    }

    ///  The distance to the far clipping plane.
    pub fn zfar(&self) -> f32 {
        self.json.zfar
    }

    ///  The distance to the near clipping plane.
    pub fn znear(&self) -> f32 {
        self.json.znear
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::camera::OrthographicExtensions {
        &self.json.extensions
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl Perspective {
    /// Constructs a `Perspective` camera projection.
    pub fn new(gltf: &'a Gltf, json: &'a json::camera::Perspective) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::camera::Perspective {
        self.json
    }

    ///  Aspect ratio of the field of view.
    pub fn aspect_ratio(&self) -> Option<f32> {
        self.json.aspect_ratio
    }

    ///  The vertical field of view in radians.
    pub fn yfov(&self) -> f32 {
        self.json.yfov
    }

    ///  The distance to the far clipping plane.
    pub fn zfar(&self) -> Option<f32> {
        self.json.zfar
    }

    ///  The distance to the near clipping plane.
    pub fn znear(&self) -> f32 {
        self.json.znear
    }

    ///  Extension specific data.
    pub fn extensions(&self) -> &json::camera::PerspectiveExtensions {
        &self.json.extensions
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}
