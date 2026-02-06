//! Error types for the Woflang runtime.
//!
//! This module provides a comprehensive error hierarchy using `thiserror`
//! for the library crates, designed to be wrapped in `eyre` at the binary
//! entry point for rich error reporting.
//!
//! Errors can optionally carry source location information via [`Span`].

use crate::span::Span;
use crate::WofType;
use std::fmt;
use thiserror::Error;

/// Result type alias for Woflang operations.
pub type Result<T, E = WofError> = std::result::Result<T, E>;

/// Primary error type for Woflang operations.
#[derive(Error, Debug, Clone)]
pub enum WofError {
    /// Stack underflow when popping.
    #[error("stack underflow: expected at least {expected} value(s), found {found}")]
    StackUnderflow {
        /// Number of values expected.
        expected: usize,
        /// Number of values actually present.
        found: usize,
    },

    /// Type mismatch during operation.
    #[error("type mismatch: expected {expected}, found {found}")]
    TypeMismatch {
        /// Expected type description.
        expected: String,
        /// Actual type found.
        found: WofType,
    },

    /// Division by zero.
    #[error("division by zero")]
    DivisionByZero,

    /// Invalid operation or unknown command.
    #[error("unknown operation: {0}")]
    UnknownOperation(String),

    /// Parse error during tokenization.
    #[error("parse error at {span}: {message}")]
    Parse {
        /// Error message.
        message: String,
        /// Source location.
        span: Span,
    },

    /// IO error (file operations).
    #[error("I/O error: {0}")]
    Io(String),

    /// Plugin loading error.
    #[error("plugin error: {0}")]
    Plugin(String),

    /// Generic runtime error.
    #[error("runtime error: {0}")]
    Runtime(String),

    /// Runtime error with source location.
    #[error("error at {span}: {message}")]
    RuntimeAt {
        /// Error message.
        message: String,
        /// Source location.
        span: Span,
    },

    /// Numeric overflow.
    #[error("numeric overflow: {0}")]
    Overflow(String),

    /// Index out of bounds.
    #[error("index out of bounds: {index} (size: {size})")]
    IndexOutOfBounds {
        /// The index that was accessed.
        index: usize,
        /// The size of the collection.
        size: usize,
    },

    /// Invalid argument to operation.
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    /// Undefined variable.
    #[error("undefined variable: {name}")]
    UndefinedVariable {
        /// Variable name.
        name: String,
    },

    /// Undefined function.
    #[error("undefined function: {name}")]
    UndefinedFunction {
        /// Function name.
        name: String,
    },

    /// Undefined label.
    #[error("undefined label: {name}")]
    UndefinedLabel {
        /// Label name.
        name: String,
    },

    /// Unclosed block.
    #[error("unclosed block starting at {span}")]
    UnclosedBlock {
        /// Where the block started.
        span: Span,
    },

    /// Unexpected block close.
    #[error("unexpected block close at {span}")]
    UnexpectedBlockClose {
        /// Where the unexpected close occurred.
        span: Span,
    },

    /// Break outside loop.
    #[error("break outside of loop at {span}")]
    BreakOutsideLoop {
        /// Where the break occurred.
        span: Span,
    },

    /// Continue outside loop.
    #[error("continue outside of loop at {span}")]
    ContinueOutsideLoop {
        /// Where the continue occurred.
        span: Span,
    },

    /// Return outside function.
    #[error("return outside of function at {span}")]
    ReturnOutsideFunction {
        /// Where the return occurred.
        span: Span,
    },
}

impl WofError {
    /// Create a stack underflow error.
    #[inline]
    #[must_use]
    pub const fn stack_underflow(expected: usize, found: usize) -> Self {
        Self::StackUnderflow { expected, found }
    }

    /// Create a type mismatch error.
    #[inline]
    #[must_use]
    pub fn type_mismatch(expected: impl Into<String>, found: WofType) -> Self {
        Self::TypeMismatch {
            expected: expected.into(),
            found,
        }
    }

    /// Create a parse error with location.
    #[inline]
    #[must_use]
    pub fn parse(msg: impl Into<String>, span: Span) -> Self {
        Self::Parse {
            message: msg.into(),
            span,
        }
    }

    /// Create a parse error without location (legacy).
    #[inline]
    #[must_use]
    pub fn parse_simple(msg: impl Into<String>) -> Self {
        Self::Parse {
            message: msg.into(),
            span: Span::synthetic(),
        }
    }

    /// Create an IO error.
    #[inline]
    #[must_use]
    pub fn io(msg: impl Into<String>) -> Self {
        Self::Io(msg.into())
    }

    /// Create a plugin error.
    #[inline]
    #[must_use]
    pub fn plugin(msg: impl Into<String>) -> Self {
        Self::Plugin(msg.into())
    }

    /// Create a runtime error.
    #[inline]
    #[must_use]
    pub fn runtime(msg: impl Into<String>) -> Self {
        Self::Runtime(msg.into())
    }

    /// Create a runtime error with location.
    #[inline]
    #[must_use]
    pub fn runtime_at(msg: impl Into<String>, span: Span) -> Self {
        Self::RuntimeAt {
            message: msg.into(),
            span,
        }
    }

    /// Create an undefined variable error.
    #[inline]
    #[must_use]
    pub fn undefined_variable(name: impl Into<String>) -> Self {
        Self::UndefinedVariable { name: name.into() }
    }

    /// Create an undefined function error.
    #[inline]
    #[must_use]
    pub fn undefined_function(name: impl Into<String>) -> Self {
        Self::UndefinedFunction { name: name.into() }
    }

    /// Create an undefined label error.
    #[inline]
    #[must_use]
    pub fn undefined_label(name: impl Into<String>) -> Self {
        Self::UndefinedLabel { name: name.into() }
    }

    /// Get the span associated with this error, if any.
    #[must_use]
    pub fn span(&self) -> Option<Span> {
        match self {
            Self::Parse { span, .. }
            | Self::RuntimeAt { span, .. }
            | Self::UnclosedBlock { span }
            | Self::UnexpectedBlockClose { span }
            | Self::BreakOutsideLoop { span }
            | Self::ContinueOutsideLoop { span }
            | Self::ReturnOutsideFunction { span } => {
                if span.is_synthetic() {
                    None
                } else {
                    Some(*span)
                }
            }
            _ => None,
        }
    }

    /// Check if this is a recoverable error.
    #[must_use]
    pub const fn is_recoverable(&self) -> bool {
        !matches!(self, Self::Io(_) | Self::Plugin(_))
    }
}

impl From<std::io::Error> for WofError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<std::num::ParseIntError> for WofError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::Parse {
            message: err.to_string(),
            span: Span::synthetic(),
        }
    }
}

impl From<std::num::ParseFloatError> for WofError {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::Parse {
            message: err.to_string(),
            span: Span::synthetic(),
        }
    }
}

/// Extension trait for Result types to add context.
#[allow(dead_code)]
pub trait ResultExt<T> {
    /// Add context to an error.
    fn context(self, ctx: impl fmt::Display) -> Result<T>;

    /// Add lazy context to an error.
    fn with_context<F, C>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: fmt::Display;

    /// Attach a span to the error.
    fn at_span(self, span: Span) -> Result<T>;
}

impl<T> ResultExt<T> for Result<T> {
    fn context(self, ctx: impl fmt::Display) -> Result<T> {
        self.map_err(|e| WofError::Runtime(format!("{ctx}: {e}")))
    }

    fn with_context<F, C>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: fmt::Display,
    {
        self.map_err(|e| WofError::Runtime(format!("{}: {e}", f())))
    }

    fn at_span(self, span: Span) -> Result<T> {
        self.map_err(|e| WofError::runtime_at(e.to_string(), span))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = WofError::stack_underflow(2, 1);
        assert!(err.to_string().contains("expected at least 2"));

        let err = WofError::type_mismatch("integer", WofType::String);
        assert!(err.to_string().contains("expected integer"));
    }

    #[test]
    fn error_with_span() {
        let span = Span::new(10, 5, 100);
        let err = WofError::parse("unexpected token", span);
        assert!(err.to_string().contains("10:5"));
        assert_eq!(err.span(), Some(span));
    }

    #[test]
    fn recoverability() {
        assert!(WofError::DivisionByZero.is_recoverable());
        assert!(!WofError::Io("test".into()).is_recoverable());
    }
}

