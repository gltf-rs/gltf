
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct Buffer {}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct View {}
