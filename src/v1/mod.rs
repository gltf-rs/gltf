
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Contains (de)serializable data structures that represent the glTF JSON data.
pub mod json;

/// Contains functions for importing glTF 1.0 assets.
pub mod import;

pub use self::import::{import, ImportError};
