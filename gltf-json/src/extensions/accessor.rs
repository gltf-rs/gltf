
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Contains data structures for sparse storage.
pub mod sparse {
    /// Indices of those attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Default, Deserialize, Validate)]
    pub struct Indices {}

    /// Sparse storage of attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Default, Deserialize, Validate)]
    pub struct Sparse {}

    /// Array of size `count * number_of_components` storing the displaced
    /// accessor attributes pointed by `accessor::sparse::Indices`.
    #[derive(Clone, Debug, Default, Deserialize, Validate)]
    pub struct Values {}
}

/// A typed view into a buffer view.
#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct Accessor {}
