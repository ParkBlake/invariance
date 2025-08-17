use crate::error::ConfigError;
use serde::{Serialize, de::DeserializeOwned};

/// Generate and print a JSON-formatted example configuration for type `T`.
///
/// This function serialises the default instance of `T` to a pretty-printed JSON string,
/// providing a template or reference for configuration authors.
///
/// Note that this is not a formal schema (e.g. JSON Schema), but rather
/// a concrete example based on the type’s `Default` implementation.
///
/// # Errors
///
/// Returns a `ConfigError` if serialisation fails.
pub fn print_schema<T>() -> Result<(), ConfigError>
where
    T: Serialize + Default + DeserializeOwned,
{
    let default = T::default();
    let json = serde_json::to_string_pretty(&default)
        .map_err(|e| ConfigError::new(format!("Failed to serialise default config: {e}")))?;

    println!("{json}");
    Ok(())
}
