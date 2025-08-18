use crate::error::ConfigError;
use crate::validate::Validate;
use serde::de::DeserializeOwned;
use serde_json;
use toml;

/// Trait for configuration types that can be parsed from strings and validated.
///
/// This trait provides default implementations for deserialising from formats such as
/// TOML and JSON, along with a convenience method to perform validation after parsing.
pub trait Config: Validate + Sized {
    /// Parses the configuration from a TOML string slice.
    ///
    /// Requires the implementing type to also implement `DeserializeOwned` to support
    /// deserialisation via Serde.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if TOML deserialisation fails.
    fn from_toml_str(s: &str) -> Result<Self, ConfigError>
    where
        Self: DeserializeOwned,
    {
        toml::from_str(s).map_err(|e| {
            ConfigError::new("Failed to parse TOML")
                .with_kind("TOML parse error")
                .with_source(e)
        })
    }

    /// Parses the configuration from a JSON string slice.
    ///
    /// Requires the implementing type to also implement `DeserializeOwned` to support
    /// deserialisation via Serde.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if JSON deserialisation fails.
    fn from_json_str(s: &str) -> Result<Self, ConfigError>
    where
        Self: DeserializeOwned,
    {
        serde_json::from_str(s).map_err(|e| {
            ConfigError::new("Failed to parse JSON")
                .with_kind("JSON parse error")
                .with_source(e)
        })
    }

    /// Validates the configuration and returns the validated instance.
    ///
    /// This method is intended to be called after parsing to ensure
    /// the configuration adheres to any domain-specific rules.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if validation fails.
    fn validate_and_build(self) -> Result<Self, ConfigError> {
        self.validate()?;
        Ok(self)
    }
}

/// Blanket implementation of `Config` for any type that
/// implements both `Validate` and `DeserializeOwned`.
impl<T> Config for T where T: Validate + DeserializeOwned {}
