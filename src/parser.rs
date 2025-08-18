//! Utilities for loading, parsing, and validating configuration files.
//!
//! Supports automatic format detection (TOML or JSON) based on file extension,
//! or explicit format hints.

use crate::{Config, ConfigError};
use serde::de::DeserializeOwned;
use std::{fs, path::Path};

/// Supported configuration file formats.
///
/// Used to guide or infer the deserialisation strategy.
#[derive(Debug, Clone, Copy)]
pub enum ConfigFormat {
    /// TOML (`.toml`)
    Toml,
    /// JSON (`.json`)
    Json,
}

impl ConfigFormat {
    /// Attempts to infer the format from a file extension.
    ///
    /// Recognises `.toml` and `.json`, case-insensitively.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_ascii_lowercase().as_str() {
            "toml" => Some(Self::Toml),
            "json" => Some(Self::Json),
            _ => None,
        }
    }
}

/// Loads, parses, and validates a configuration file.
///
/// This function reads the file from disk, deserialises it into the provided type `T`,
/// and then invokes `T::validate_and_build()` to ensure correctness.
///
/// If no format is explicitly provided, the file extension is used to determine whether
/// the contents should be interpreted as TOML or JSON.
///
/// # Parameters
///
/// - `path`: Path to the configuration file.
/// - `format_hint`: Optional explicit format override. If `None`, the file extension is used.
///
/// # Returns
///
/// On success, returns a validated and constructed configuration object of type `T`.
///
/// # Errors
///
/// Returns a [`ConfigError`] if:
/// - the file cannot be read,
/// - the contents fail to parse,
/// - or validation fails.
pub fn parse_config_file<T>(
    path: impl AsRef<Path>,
    format_hint: Option<ConfigFormat>,
) -> Result<T, ConfigError>
where
    T: Config + DeserializeOwned,
{
    let path = path.as_ref();

    let contents = fs::read_to_string(path).map_err(|e| {
        ConfigError::new("Failed to read config file")
            .with_kind("IOError")
            .with_context(path.display().to_string())
            .with_source(e)
    })?;

    let format = format_hint.or_else(|| {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(ConfigFormat::from_extension)
    });

    match format {
        Some(ConfigFormat::Toml) => T::from_toml_str(&contents)?.validate_and_build(),
        Some(ConfigFormat::Json) => T::from_json_str(&contents)?.validate_and_build(),
        None => Err(ConfigError::new("Unknown configuration format")
            .with_kind("FormatError")
            .with_context(path.display().to_string())),
    }
}
