use crate::error::ConfigError;
use crate::validate::Validate;
use serde::de::DeserializeOwned;

/// Trait for configuration types that can be parsed from a string and validated.
///
/// Provides a default implementation for deserializing from formats like TOML or YAML,
/// and a convenience method to validate and build the final config.
pub trait Config: Validate + Sized {
    /// Parses the configuration from a string slice.
    ///
    /// This method requires that the implementing type also implements
    /// `DeserializeOwned` to support deserialization (e.g. via Serde).
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if parsing fails.
    fn from_str(s: &str) -> Result<Self, ConfigError>
    where
        Self: DeserializeOwned,
    {
        toml::from_str(s).map_err(|e| ConfigError::new(e.to_string()))
    }

    /// Validates the config and returns the owned, validated instance.
    ///
    /// Returns an error if validation fails.
    fn validate_and_build(self) -> Result<Self, ConfigError> {
        self.validate()?;
        Ok(self)
    }
}

/// Blanket implementation of `Config` for any type that
/// implements `Validate` and `DeserializeOwned`.
impl<T> Config for T where T: Validate + DeserializeOwned {}
