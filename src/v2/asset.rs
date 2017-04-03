
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Extras;

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Asset<E: Extras> {
    /// A copyright message suitable for display to credit the content creator.
    pub copyright: Option<String>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: AssetExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Asset,
    
    /// Tool that generated this glTF model.
    pub generator: Option<String>,

    /// The glTF version of this asset.
    #[serde(default = "asset_version_default")]
    pub version: String,
}

fn asset_version_default() -> String {
    "2.0".to_string()
}

/// Extension specific data for `Asset`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AssetExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

