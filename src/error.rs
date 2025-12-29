use std::fmt::{Display, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
    context: Vec<String>,
}

impl Error {
    #[must_use]
    pub const fn new(message: String) -> Self {
        Self {
            message,
            context: Vec::new(),
        }
    }

    /// Add context in-place.
    pub fn add_context(&mut self, context: impl Into<String>) {
        self.context.push(context.into());
    }

    /// Add context and return itself.
    #[must_use = "returns a new error with additional context"]
    pub fn push_context(mut self, context: impl Into<String>) -> Self {
        self.add_context(context);
        self
    }

    #[must_use]
    pub fn chain(&self) -> String {
        let mut output = self.message.clone() + "\n";
        for context in &self.context {
            let _ = writeln!(output, "> while {context}");
        }
        output
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        Self::new(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Context<T> {
    fn context(self, context: impl Into<String>) -> Result<T>;
    fn with_context(self, f: impl FnOnce() -> String) -> Result<T>;
}

impl<T, S: ToString> Context<T> for std::result::Result<T, S> {
    fn context(self, context: impl Into<String>) -> Result<T> {
        self.map_err(|string| Error::new(string.to_string()).push_context(context))
    }

    fn with_context(self, f: impl FnOnce() -> String) -> Result<T> {
        self.map_err(|string| Error::new(string.to_string()).push_context(f()))
    }
}

impl<T> Context<T> for Option<T> {
    fn context(self, context: impl Into<String>) -> Result<T> {
        self.ok_or_else(|| Error::new(context.into()))
    }

    fn with_context(self, f: impl FnOnce() -> String) -> Result<T> {
        self.ok_or_else(f).map_err(Error::new)
    }
}

/// Perform an early return with the specified formatted message.
/// This is a simple alias for `return Err(Error::new(format!(...))`.
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::error::Error::new(format!($($arg)*)))
    };
}

pub(crate) use bail;
