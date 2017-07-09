
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Describes the validation strategy of an `Importer`.
#[derive(Clone, Copy, Debug)]
pub enum ValidationStrategy {
    /// Validate the whole glTF against the 2.0 specification.
    ///
    /// This is achieved primarily by calling `Validate::validate_completely` on the
    /// root JSON data structure.
    Complete,

    /// Validate only the invariants required for the crate to function safely.
    ///
    /// This is achieved primarily by calling `Validate::validate_minimally` on the
    /// root JSON data structure.
    Minimal,

    /// Skip the validation stage (not recommended.)
    Skip,
}

/// A complete import configuration.
#[derive(Clone, Debug, Default)]
pub struct Config {
    /// Specifies how imported glTF should be validated.
    pub validation_strategy: ValidationStrategy,
}

impl Default for ValidationStrategy {
    fn default() -> ValidationStrategy {
        ValidationStrategy::Complete
    }
}
