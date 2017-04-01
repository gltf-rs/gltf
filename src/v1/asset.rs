// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::Extras;

/// Metadata about the glTF asset
#[derive(Debug, Deserialize, Serialize)]
pub struct Asset<E: Extras> {
    /// A copyright message suitable for display to credit the content creator
    pub copyright: Option<String>,

    /// Tool that generated this glTF model
    pub generator: Option<String>,

    /// Specifies if the shaders were generated with pre-multiplied alpha
    #[serde(default, rename = "premultipliedAlpha")]
    pub pre_multiplied_alpha: bool,

    /// Specifies the target rendering API and version
    pub profile: AssetProfile<E>,

    /// The glTF version
    pub version: String,

    /// Extension specific data
    #[serde(default)]
    pub extensions: AssetExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::Asset,
}

/// Extension specific data for `Asset`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AssetExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Specifies the target rendering API and version
#[derive(Debug, Deserialize, Serialize)]
pub struct AssetProfile<E: Extras> {
    /// Specifies the target rendering API
    #[serde(default = "asset_profile_api_default")]
    pub api: String,

    /// The API version
    #[serde(default = "asset_profile_version_default")]
    pub version: String, 

    /// Extension specific data
    #[serde(default)]
    pub extensions: AssetProfileExtensions,

    /// Optional application specific data
    #[serde(default)]
    pub extras: <E as Extras>::AssetProfile,
}

fn asset_profile_api_default() -> String {
    "WebGL".to_string()
}

fn asset_profile_version_default() -> String {
    "1.0.3".to_string()
}

/// Extension specific data for `AssetProfile`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AssetProfileExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

