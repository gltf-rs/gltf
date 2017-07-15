
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {json, Gltf};

/// A typed view into a buffer view.
#[derive(Clone, Debug)]
pub struct Accessor<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON struct.
    json: &'a json::extensions::accessor::Accessor,
}

impl<'a> Accessor<'a> {
    /// Constructs an `Accessor`.
    pub fn new(gltf: &'a Gltf, json: &'a json::extensions::accessor::Accessor) -> Self {
        Self {
            gltf: gltf,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) -> &json::extensions::accessor::Accessor {
        self.json
    }
}

/// Contains data structures for sparse storage.
pub mod sparse {
    use {json, Gltf};
    
    ///  Indices of those attributes that deviate from their initialization value.
    pub struct Indices<'a> {
        /// The parent `Gltf` struct.
        #[allow(dead_code)]
        gltf: &'a Gltf,

        /// The corresponding JSON struct.
        json: &'a json::extensions::accessor::sparse::Indices,
    }

    impl<'a> Indices<'a> {
        /// Constructs a `Indices`.
        pub fn new(
            gltf: &'a Gltf,
            json: &'a json::extensions::accessor::sparse::Indices,
        ) -> Self {
            Self {
                gltf: gltf,
                json: json,
            }
        }

        /// Returns the internal JSON item.
        pub fn as_json(&self) ->  &json::extensions::accessor::sparse::Indices {
            self.json
        }
    }
    
    /// Sparse storage of attributes that deviate from their initialization value.
    pub struct Sparse<'a> {
        /// The parent `Gltf` struct.
        #[allow(dead_code)]
        gltf: &'a Gltf,

        /// The corresponding JSON struct.
        json: &'a json::extensions::accessor::sparse::Sparse,
    }

    impl<'a> Sparse<'a> {
        /// Constructs a `Sparse`.
        pub fn new(
            gltf: &'a Gltf,
            json: &'a json::extensions::accessor::sparse::Sparse,
        ) -> Self {
            Self {
                gltf: gltf,
                json: json,
            }
        }

        /// Returns the internal JSON item.
        pub fn as_json(&self) -> &json::extensions::accessor::sparse::Sparse {
            self.json
        }
    }

    /// Array of size `count * number_of_components` storing the displaced accessor
    /// attributes pointed by `accessor::sparse::Indices`.
    pub struct Values<'a> {
        /// The parent `Gltf` struct.
        #[allow(dead_code)]
        gltf: &'a Gltf,

        /// The corresponding JSON struct.
        json: &'a json::extensions::accessor::sparse::Values,
    }

    impl<'a> Values<'a> {
        /// Constructs a `Values`.
        pub fn new(
            gltf: &'a Gltf,
            json: &'a json::extensions::accessor::sparse::Values,
        ) -> Self {
            Self {
                gltf: gltf,
                json: json,
            }
        }

        /// Returns the internal JSON item.
        pub fn as_json(&self) ->  &json::extensions::accessor::sparse::Values {
            self.json
        }
    }
}
