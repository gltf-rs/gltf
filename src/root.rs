
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::slice;
use json;

use std::borrow::Cow;

/// Iterator over extension strings.
#[derive(Clone, Debug)]
pub struct Extension(slice::Iter<'a, Cow<'a, str>>);

/// The (immutable) root object for a glTF asset.
#[derive(Clone, Debug)]
pub struct Root(json::root::Root);

impl Root {
    /// Constructs a `Camera`.
    pub fn new(json: json::root::Root) -> Self {
        Root(json)
    }
    
    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::root::Root {
        &self.0
    }

    /// Returns the extensions referenced in this .gltf file.
    pub fn extensions_used(&'a self) -> Extension {
        Extension(self.0.extensions_used.iter())
    }

    /// Returns the extensions required to load and render this asset.
    pub fn extensions_required(&'a self) -> Extension {
        Extension(self.0.extensions_required.iter())
    }
}

impl Iterator for Extension {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Cow::as_ref)
    }
}
