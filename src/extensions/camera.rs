
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {json, Gltf};

/// A camera's projection.  A node can reference a camera to apply a transform to
/// place the camera in the scene.
#[derive(Clone, Debug)]
pub struct Camera<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::camera::Camera,
}
  
///  Values for an orthographic camera projection.
#[derive(Clone, Debug)]
pub struct Orthographic<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::camera::Orthographic,
}
  
/// Values for a perspective camera projection.
#[derive(Clone, Debug)]
pub struct Perspective<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::camera::Perspective,
}

impl<'a> Camera<'a> {
    /// Constructs a `Camera`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::camera::Camera) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::camera::Camera {
        self.json
    }
}

impl<'a> Orthographic<'a> {
    /// Constructs a `Orthographic` camera projection.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::camera::Orthographic) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::extensions::camera::Orthographic {
        self.json
    }
}

impl<'a> Perspective<'a> {
    /// Constructs a `Perspective` camera projection.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::camera::Perspective) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::extensions::camera::Perspective {
        self.json
    }
}
