// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use serde_json::Value;
use v1::Extras;

/// An untyped JSON object.
pub type UntypedJsonObject = HashMap<String, Value>;

/// The material appearance of a primitive.
#[derive(Debug, Deserialize, Serialize)]
pub struct Material<E: Extras> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: MaterialExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Material,
    
    /// The user-defined name of this object.
    pub name: Option<String>,

    /// The ID of the technique.
    ///
    /// If this is not supplied, and no extension is present that defines
    /// material properties, then the primitive should be rendered using a
    /// default material with 50% gray emissive color.
    pub technique: Option<String>,

    /// An untyped dictionary object of parameter values.
    ///
    /// Parameters with the same name as the technique's parameter override the
    /// technique's parameter value.
    pub values: UntypedJsonObject,
}

/// Extension specific data for `Material`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MaterialExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}
