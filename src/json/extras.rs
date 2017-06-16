
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::marker::PhantomData;

pub use serde_json::Value;

/// Data type of the `extras` attribute on all glTF objects.
#[cfg(feature = "extras")]
pub type Extras<'a> = Option<Value>;

/// Data type of the `extras` attribute on all glTF objects.
#[cfg(not(feature = "extras"))]
pub type Extras<'a> = Void<'a>;

/// Type representing no user-defined data.
#[derive(Clone, Default, Deserialize, Serialize, Validate)]
pub struct Void<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

impl<'a> fmt::Debug for Void<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}

impl<'a> fmt::Display for Void<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}
