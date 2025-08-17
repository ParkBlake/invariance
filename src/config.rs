use crate::error::ConfigError;
use crate::validate::Validate;
use serde::de::DeserializeOwned;

/// Trait for configuration types that can be parsed from strings and validated.
///
/// Provides default implementations for deserializing from formats like TOML and JSON,
/// and a convenience method to validate and build the final config.
pub trait Config: Validate + Sized {
    /// Parses the configuration from a TOML string slice.
    ///
    /// This method requires that the implementing type also implements
    /// `DeserializeOwned` to support deserialization (e.g., via Serde).
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if TOML parsing fails.
    fn from_toml_str(s: &str) -> Result<Self, ConfigError>
    where
        Self: DeserializeOwned,
    {
        toml::from_str(s).map_err(|e| ConfigError::new(e.to_string()).with_kind("TOML parse error"))
    }

    /// Parses the configuration from a JSON string slice.
    ///
    /// Requires `DeserializeOwned` to support deserialization.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if JSON parsing fails.
    fn from_json_str(s: &str) -> Result<Self, ConfigError>
    where
        Self: DeserializeOwned,
    {
        serde_json::from_str(s)
            .map_err(|e| ConfigError::new(e.to_string()).with_kind("JSON parse error"))
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
