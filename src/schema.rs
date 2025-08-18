use crate::error::ConfigError;
use serde::{Serialize, de::DeserializeOwned};

/// Prints the default configuration as an example in both JSON and TOML formats.
///
/// This is not a formal schema definition, but a concrete instantiation of the
/// `Default` implementation for a type `T`, serialised into both supported formats.
/// It provides users with a clear reference for how their configuration files should be structured.
///
/// This is particularly useful for documentation, onboarding, or generating starter config templates.
///
/// # Type Parameters
///
/// - `T`: The configuration type. Must implement [`Default`], [`Serialize`], and [`DeserializeOwned`].
///
/// # Output
///
/// Prints two formatted blocks to stdout:
/// - A JSON representation.
/// - A TOML representation.
///
/// # Errors
///
/// Returns a [`ConfigError`] if either JSON or TOML serialisation fails.
pub fn print_schema<T>() -> Result<(), ConfigError>
where
    T: Serialize + Default + DeserializeOwned,
{
    let default = T::default();

    let json = serde_json::to_string_pretty(&default).map_err(|e| {
        ConfigError::new("Failed to serialise default config to JSON")
            .with_kind("SerialisationError")
            .with_source(e)
    })?;

    let toml = toml::to_string_pretty(&default).map_err(|e| {
        ConfigError::new("Failed to serialise default config to TOML")
            .with_kind("SerialisationError")
            .with_source(e)
    })?;

    println!("--- JSON Example ---\n{json}\n");
    println!("--- TOML Example ---\n{toml}");

    Ok(())
}
