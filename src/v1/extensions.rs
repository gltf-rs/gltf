
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Names of glTF 1.0 extensions enabled by the user
pub const ENABLED_EXTENSIONS: &'static [&'static str] = &[
    #[cfg(feature = "KHR_binary_glTF")]
    "KHR_binary_glTF",
    #[cfg(feature = "KHR_materials_common")]
    "KHR_materials_common"
];

/// Names of glTF 1.0 extensions supported by the library
pub const SUPPORTED_EXTENSIONS: &'static [&'static str] = &[
    "KHR_binary_glTF",
    "KHR_materials_common"
];
