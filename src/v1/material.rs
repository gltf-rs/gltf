// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::Extensions;
use v1::Extras;

#[derive(Debug, Deserialize, Serialize)]
pub struct Material<E: Extras> {
    /// The ID of the technique.
    ///
    /// If this is not supplied, and no extension is present that defines
    /// material properties, then the primitive should be rendered using a
    /// default material with 50% gray emissive color
    pub technique: Option<String>,

    // TODO: implement values
    
    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a material and a buffer could have
    /// the same name, or two materials could even have the same name.
    pub name: Option<String>,

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: Extensions,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Material,
}
