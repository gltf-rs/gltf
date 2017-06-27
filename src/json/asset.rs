
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::marker::PhantomData;

use json::Extras;

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Asset<'a> {
    /// A copyright message suitable for display to credit the content creator.
    pub copyright: Option<Cow<'a, str>>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: AssetExtensions<'a>,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
    
    /// Tool that generated this glTF model.
    pub generator: Option<Cow<'a, str>>,

    /// The minimum glTF version that this asset targets.
    #[serde(rename = "minVersion")]
    pub min_version: Option<Cow<'a, str>>,
    
    /// The glTF version of this asset.
    pub version: Cow<'a, str>,
}

/// Extension specific data for `Asset`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct AssetExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

impl<'a> Default for Asset<'a> {
    fn default() -> Self {
        Self {
            copyright: None,
            extensions: Default::default(),
            extras: Default::default(),
            generator: None,
            min_version: None,
            version: Cow::from("2.0"),
        }
    }
}

