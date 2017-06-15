
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use json::Extras;

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator.
    pub copyright: Option<String>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: AssetExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// Tool that generated this glTF model.
    pub generator: Option<String>,

    /// The minimum glTF version that this asset targets.
    #[serde(rename = "minVersion")]
    pub min_version: Option<String>,
    
    /// The glTF version of this asset.
    pub version: String,
}

/// Extension specific data for `Asset`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct AssetExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            copyright: None,
            extensions: Default::default(),
            extras: Default::default(),
            generator: None,
            min_version: None,
            version: "2.0".to_string(),
        }
    }
}

