
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::Index;

/// Helper trait for retrieving top-level objects by an index
pub trait Get<T> {
    /// Retrieves a single value at the given index
    fn get(&self, index: Index<T>) -> &T;
}

