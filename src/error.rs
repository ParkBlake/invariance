use std::error::Error;
use std::fmt;

/// Error type representing issues encountered during configuration parsing or validation.
///
/// This error supports optional categorisation (`kind`), contextual information
/// (e.g., the field or file involved), and chaining of underlying source errors.
#[derive(Debug)]
pub struct ConfigError {
    /// A human-readable description of the error.
    message: String,

    /// Optional contextual detail (e.g., field name, file path, or section).
    context: Option<String>,

    /// Optional underlying error that triggered this one.
    source: Option<Box<dyn Error + Send + Sync + 'static>>,

    /// Optional classification of the error (e.g., `"ParseError"`, `"ValidationError"`).
    kind: Option<String>,
}

impl ConfigError {
    /// Constructs a new `ConfigError` with the given message.
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            context: None,
            source: None,
            kind: None,
        }
    }

    /// Attaches additional context to the error.
    ///
    /// Useful for indicating where or how the error occurred (e.g., `"field: port"`).
    pub fn with_context(mut self, ctx: impl Into<String>) -> Self {
        self.context = Some(ctx.into());
        self
    }

    /// Specifies a category or kind for the error.
    ///
    /// Examples include `"ParseError"`, `"ValidationError"`, or `"MissingField"`.
    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = Some(kind.into());
        self
    }

    /// Attaches a source error, allowing for error chaining.
    ///
    /// Useful for preserving the root cause of parsing or I/O failures.
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Configuration error")?;
        if let Some(kind) = &self.kind {
            write!(f, " [{}]", kind)?;
        }
        if let Some(ctx) = &self.context {
            write!(f, " in {}", ctx)?;
        }
        write!(f, ": {}", self.message)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|boxed| boxed.as_ref() as &(dyn Error + 'static))
    }
}
