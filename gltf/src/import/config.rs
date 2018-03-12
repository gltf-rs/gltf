/// Describes the validation strategy of an `Importer`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

    /// Skip the validation stage.  **Using this is highly recommended against**
    /// as malformed glTF assets might lead to program panics, huge values, NaNs
    /// and general evil deeds.
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
