
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use json;

/// The (immutable) root object for a glTF asset.
#[derive(Clone, Debug)]
pub struct Root(json::extensions::root::Root);

impl Root {
    /// Constructs a `Camera`.
    pub fn new(json: json::extensions::root::Root) -> Self {
        Root(json)
    }
    
    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::extensions::root::Root {
        &self.0
    }
}
