// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::Extensions;
use traits::Extras;

#[derive(Debug, Deserialize, Serialize)]
pub struct Program<E: Extras> {
    /// Names of GLSL vertex shader attributes.
    #[serde(default)]
    pub attributes: Vec<String>,

    /// The ID of the fragment shader.
    #[serde(rename = "fragmentShader")]
    pub fragment_shader: String,

    /// The ID of the vertex shader.
    #[serde(rename = "vertexShader")]
    pub vertex_shader: String,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a program and a buffer could have
    /// the same name, or two programs could even have the same name.
    pub name: Option<String>,

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: Extensions,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Program,
}
