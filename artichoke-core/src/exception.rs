use std::fmt;

use crate::ArtichokeError;

/// Metadata about a Ruby exception.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Exception {
    /// The result of calling `exception.class.name`.
    pub class: String,
    /// The result of calling `exception.message`.
    pub message: String,
    /// The result of calling `exception.backtrace`.
    ///
    /// Some exceptions, like `SyntaxError` which is thrown directly by the
    /// VM, do not have backtraces, so this field is optional.
    pub backtrace: Option<Vec<String>>,
    /// The result of calling `exception.inspect`.
    pub inspect: String,
}

impl Exception {
    pub fn new(class: &str, message: &str, backtrace: Option<Vec<String>>, inspect: &str) -> Self {
        Self {
            class: class.to_owned(),
            message: message.to_owned(),
            backtrace,
            inspect: inspect.to_owned(),
        }
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inspect)?;
        if let Some(ref backtrace) = self.backtrace {
            for frame in backtrace {
                write!(f, "\n{}", frame)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LastError {
    Some(Exception),
    None,
    UnableToExtract(ArtichokeError),
}

/// Extract the last exception thrown on the interpreter.
#[allow(clippy::module_name_repetitions)]
pub trait ExceptionHandler {
    /// Extract the last thrown exception on the interpreter if there is one.
    ///
    /// If there is an error, return [`LastError::Some`], which contains the
    /// exception class name, message, and optional backtrace.
    fn last_error(&self) -> LastError;
}
