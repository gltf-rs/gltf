// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::Extras;

enum_number! {
    ShaderType {
        Fragment = 35632,
        Vertex = 35633,
    }
}

/// A vertex or fragment shader
#[derive(Debug, Deserialize, Serialize)]
pub struct Shader<E: Extras> {
    /// The uri of the GLSL source
    ///
    /// Relative paths are relative to the .gltf file
    pub uri: String,

    /// The shader stage
    #[serde(default)]
    #[serde(rename = "type")]
    pub kind: ShaderType,

    /// The user-defined name of this object
    pub name: Option<String>,

    /// Extension specific data
    #[serde(default)]
    pub extensions: ShaderExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Shader,
}

/// Extension specific data for `Shader`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShaderExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl Default for ShaderType {
    fn default() -> ShaderType {
        ShaderType::Fragment
    }
}

