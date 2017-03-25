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
pub struct Scene<E: Extras> {
    /// The IDs of each root node.
    #[serde(default)]
    pub nodes: Vec<String>,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a scene and a buffer could have
    /// the same name, or two scenes could even have the same name.
    pub name: Option<String>,

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: Extensions,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Scene,
}
