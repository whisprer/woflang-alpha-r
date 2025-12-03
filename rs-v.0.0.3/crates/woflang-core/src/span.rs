//! Source location tracking for error reporting.
//!
//! The [`Span`] type tracks the position of tokens and expressions
//! within source code, enabling precise error messages with line and
//! column information.

use std::fmt;

/// A source location span.
///
/// Spans track the position of a syntactic element within source code.
/// They are designed to be lightweight (16 bytes) and copyable.
///
/// # Example
///
/// ```
/// use woflang_core::Span;
///
/// let span = Span::new(1, 5, 4);
/// assert_eq!(span.line(), 1);
/// assert_eq!(span.column(), 5);
/// assert_eq!(span.offset(), 4);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    /// 1-indexed line number.
    line: u32,
    /// 1-indexed column number (in Unicode codepoints, not bytes).
    column: u32,
    /// Byte offset from start of source.
    offset: u32,
    /// Length in bytes.
    length: u32,
}

impl Span {
    /// Create a new span at the given position.
    ///
    /// # Arguments
    ///
    /// * `line` - 1-indexed line number
    /// * `column` - 1-indexed column number
    /// * `offset` - Byte offset from start of source
    #[must_use]
    pub const fn new(line: u32, column: u32, offset: u32) -> Self {
        Self {
            line,
            column,
            offset,
            length: 0,
        }
    }

    /// Create a span with a specific length.
    #[must_use]
    pub const fn with_length(line: u32, column: u32, offset: u32, length: u32) -> Self {
        Self {
            line,
            column,
            offset,
            length,
        }
    }

    /// Create a dummy span for synthetic tokens.
    #[must_use]
    pub const fn synthetic() -> Self {
        Self {
            line: 0,
            column: 0,
            offset: 0,
            length: 0,
        }
    }

    /// Check if this is a synthetic (non-source) span.
    #[must_use]
    pub const fn is_synthetic(&self) -> bool {
        self.line == 0
    }

    /// Get the 1-indexed line number.
    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// Get the 1-indexed column number.
    #[must_use]
    pub const fn column(&self) -> u32 {
        self.column
    }

    /// Get the byte offset from source start.
    #[must_use]
    pub const fn offset(&self) -> u32 {
        self.offset
    }

    /// Get the length in bytes.
    #[must_use]
    pub const fn length(&self) -> u32 {
        self.length
    }

    /// Create a span covering from this span to another.
    #[must_use]
    pub fn to(self, end: Self) -> Self {
        let new_length = if end.offset >= self.offset {
            end.offset - self.offset + end.length
        } else {
            self.length
        };
        Self {
            length: new_length,
            ..self
        }
    }

    /// Extend this span by the given number of bytes.
    #[must_use]
    pub const fn extend(self, extra_bytes: u32) -> Self {
        Self {
            length: self.length + extra_bytes,
            ..self
        }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_synthetic() {
            write!(f, "Span(<synthetic>)")
        } else {
            write!(f, "Span({}:{})", self.line, self.column)
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_synthetic() {
            write!(f, "<unknown>")
        } else {
            write!(f, "{}:{}", self.line, self.column)
        }
    }
}

/// Trait for types that have a source span.
pub trait Spanned {
    /// Get the span of this element.
    fn span(&self) -> Span;
}

impl Spanned for Span {
    fn span(&self) -> Span {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_display() {
        let span = Span::new(10, 5, 100);
        assert_eq!(format!("{span}"), "10:5");
    }

    #[test]
    fn synthetic_span() {
        let span = Span::synthetic();
        assert!(span.is_synthetic());
        assert_eq!(format!("{span}"), "<unknown>");
    }

    #[test]
    fn span_to() {
        let start = Span::with_length(1, 1, 0, 3);
        let end = Span::with_length(1, 10, 9, 5);
        let combined = start.to(end);
        assert_eq!(combined.offset(), 0);
        assert_eq!(combined.length(), 14); // 9 - 0 + 5
    }
}
