// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde_json;

enum_number!(
    ShaderType {
        Fragment = 35632,
        Vertex   = 35633,
    }
);

impl Default for ShaderType {
    fn default() -> ShaderType {
        ShaderType::Fragment
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Shader {
    /// The uri of the GLSL source.
    ///
    /// Relative paths are relative to the .gltf file. Instead of referencing an
    /// external file, the uri can also be a data-uri.
    pub uri: String,

    /// The shader stage.
    ///
    /// Allowed values are 35632 (FRAGMENT_SHADER) and 35633 (VERTEX_SHADER).
    #[serde(default)]
    #[serde(rename = "type")]
    pub kind: ShaderType,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a shader and a buffer could have
    /// the same name, or two shaders could even have the same name.
    pub name: Option<String>,
}
