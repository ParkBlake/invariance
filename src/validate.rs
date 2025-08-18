use crate::error::ConfigError;

/// Trait for validating configuration structures.
///
/// Provides a mechanism to enforce invariants or sanity checks
/// on deserialised configuration data before it is used at runtime.
///
/// This is especially useful for catching invalid combinations or
/// missing fields that cannot be expressed via types alone.
///
/// # Default Behaviour
///
/// If not overridden, the `validate` method returns `Ok(())`, treating
/// all instances as valid. Implementors should override `validate`
/// when domain-specific validation is required.
pub trait Validate {
    /// Run validation logic on the configuration instance.
    ///
    /// Returns:
    /// - `Ok(())` if the configuration is valid.
    /// - `Err(ConfigError)` if any invariant is violated.
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}
