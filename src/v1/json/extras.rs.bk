
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use serde_json::Value;

/// Data type of the `extras` attribute of all glTF objects.
#[cfg(feature = "extras")]
pub type Extras = Option<Value>;
#[cfg(not(feature = "extras"))]
pub type Extras = Void;

/// Type representing no user-defined data.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Void {
    #[serde(default)]
    _allow_unknown_fields: (),
}

