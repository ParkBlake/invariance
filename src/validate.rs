use crate::error::ConfigError;

/// Trait for validating configuration types.
///
/// By default, validation passes with no error.
/// Override `validate` to implement custom invariants.
///
/// If not overridden, the config is considered valid.
pub trait Validate {
    /// Perform validation checks on the configuration.
    ///
    /// Returns `Ok(())` if the config is valid,
    /// or a `ConfigError` describing the failure.
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}
