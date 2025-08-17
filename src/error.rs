use std::fmt;

/// Error type for configuration parsing and validation issues.
#[derive(Debug)]
pub struct ConfigError {
    /// Human-readable description of the error.
    message: String,
    /// Optional source of the error, e.g., field name or file path.
    source: Option<String>,
    /// Optional kind/category of the error, e.g., "ParseError", "ValidationError".
    kind: Option<String>,
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
            source: None,
            kind: None,
        }
    }

    /// Adds source information to the error.
    ///
    /// # Arguments
    ///
    /// * `source` - The source or context of the error.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Adds kind/category information to the error.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind/category of the error.
    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = Some(kind.into());
        self
    }
}

impl fmt::Display for ConfigError {
    /// Formats the error for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Configuration error")?;
        if let Some(kind) = &self.kind {
            write!(f, " [{}]", kind)?;
        }
        if let Some(source) = &self.source {
            write!(f, " in {}", source)?;
        }
        write!(f, ": {}", self.message)
    }
}

impl std::error::Error for ConfigError {}
