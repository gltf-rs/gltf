
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate gltf_derive;
extern crate inflections;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/// Contains (de)serializable data structures that represent the glTF JSON data.
pub mod json;

/// Contains functions for importing glTF 2.0 assets.
pub mod import;

/// Contains the `try_validate` macro.
#[macro_use]
pub mod macros;

/// Contains functions that validate glTF JSON data against the specification.
pub mod validation;

pub use self::import::import;
