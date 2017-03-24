
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::{Deserialize, Serialize};
use serde::export::fmt::Debug;
use v2::Index;

/// Helper trait for retrieving top-level objects by an index
pub trait Get<T> {
    /// Retrieves a single value at the given index
    fn get(&self, index: Index<T>) -> &T;
}

/// Defines a family of extension data structures to be (de)serialized
pub trait Extensions: Clone + Debug + Default + Deserialize + Serialize {
    /// Extensions type for `Accessor`
    type Accessor: Clone + Debug + Default + Deserialize + Serialize;
}

/// Defines a family of user-defined data structures to be (de)serialized
pub trait Extras: Clone + Debug + Default + Deserialize + Serialize {
    /// Extras type for `Accessor`
    type Accessor: Clone + Debug + Default + Deserialize + Serialize;
}
