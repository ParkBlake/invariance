use std::fmt;

/// Error type for configuration parsing and validation issues.
#[derive(Debug)]
pub struct ConfigError {
    /// Human-readable description of the error.
    message: String,
}

impl ConfigError {
    /// Create a new `ConfigError` with the given message.
    ///
    /// # Arguments
    ///
    /// * `msg` - A message describing the error.
    ///
    /// # Examples
    ///
    /// ```
    /// use invariance::ConfigError;
    ///
    /// let err = ConfigError::new("invalid config value");
    /// ```
    pub fn new(msg: impl Into<String>) -> Self {
        ConfigError {
            message: msg.into(),
        }
    }
}

impl fmt::Display for ConfigError {
    /// Formats the error for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Configuration error: {}", self.message)
    }
}

impl std::error::Error for ConfigError {}
