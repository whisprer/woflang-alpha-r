//! Pretty error diagnostics for Woflang.
//!
//! The [`Diagnostic`] type wraps a [`WofError`] with source context so
//! errors can be rendered with the offending line, an underline arrow
//! pointing at the exact column, and optional filename information.
//!
//! # Example Output
//!
//! ```text
//! error: stack underflow: expected at least 2 value(s), found 1
//!   --> script.wof:3:5
//!    |
//!  3 |  10 +
//!    |     ^ here
//! ```

use crate::span::Span;
use crate::WofError;
use std::fmt;

/// A diagnostic wrapping an error with source context.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// The underlying error.
    pub error: WofError,
    /// The source line where the error occurred.
    pub source_line: Option<String>,
    /// The filename (if from a file).
    pub filename: Option<String>,
    /// The span of the token that caused the error.
    pub span: Option<Span>,
}

impl Diagnostic {
    /// Create a diagnostic from an error, attempting to extract its span.
    pub fn from_error(error: WofError) -> Self {
        let span = error.span();
        Self {
            error,
            source_line: None,
            filename: None,
            span,
        }
    }

    /// Attach source context (the full line of source where the error occurred).
    #[must_use]
    pub fn with_source(mut self, source: &str) -> Self {
        if let Some(span) = self.span {
            if !span.is_synthetic() {
                // Extract the line from source
                let line_num = span.line() as usize;
                if let Some(line) = source.lines().nth(line_num.saturating_sub(1)) {
                    self.source_line = Some(line.to_string());
                }
            }
        }
        self
    }

    /// Attach a source line directly (for single-line REPL input).
    #[must_use]
    pub fn with_source_line(mut self, line: impl Into<String>) -> Self {
        self.source_line = Some(line.into());
        self
    }

    /// Attach a filename.
    #[must_use]
    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    /// Attach a span (overrides the error's span).
    #[must_use]
    pub fn with_span(mut self, span: Span) -> Self {
        if !span.is_synthetic() {
            self.span = Some(span);
        }
        self
    }

    /// Render the diagnostic as a pretty string for terminal output.
    ///
    /// Uses ANSI escape codes for color when `use_color` is true.
    pub fn render(&self, use_color: bool) -> String {
        let mut out = String::new();

        // Error header
        let error_msg = self.error.to_string();
        if use_color {
            out.push_str(&format!("\x1b[1;31merror\x1b[0m\x1b[1m: {error_msg}\x1b[0m\n"));
        } else {
            out.push_str(&format!("error: {error_msg}\n"));
        }

        // Location line: --> file:line:col
        if let Some(span) = self.span {
            if !span.is_synthetic() {
                let location = if let Some(ref filename) = self.filename {
                    format!("{filename}:{}:{}", span.line(), span.column())
                } else {
                    format!("{}:{}", span.line(), span.column())
                };

                if use_color {
                    out.push_str(&format!("  \x1b[1;34m-->\x1b[0m {location}\n"));
                } else {
                    out.push_str(&format!("  --> {location}\n"));
                }
            }
        }

        // Source line with underline arrow
        if let Some(ref source_line) = self.source_line {
            if let Some(span) = self.span {
                if !span.is_synthetic() {
                    let line_num = span.line();
                    let col = span.column().saturating_sub(1) as usize;
                    let width = std::cmp::max(1, span.length() as usize);
                    let gutter_width = format!("{line_num}").len();

                    // Empty gutter line
                    if use_color {
                        out.push_str(&format!(
                            "  \x1b[1;34m{:>gutter_width$} |\x1b[0m\n",
                            ""
                        ));
                    } else {
                        out.push_str(&format!("  {:>gutter_width$} |\n", ""));
                    }

                    // Source line
                    if use_color {
                        out.push_str(&format!(
                            "  \x1b[1;34m{line_num} |\x1b[0m  {source_line}\n"
                        ));
                    } else {
                        out.push_str(&format!("  {line_num} |  {source_line}\n"));
                    }

                    // Underline arrow
                    let padding = " ".repeat(col);
                    let underline = "^".repeat(width);
                    if use_color {
                        out.push_str(&format!(
                            "  \x1b[1;34m{:>gutter_width$} |\x1b[0m  {padding}\x1b[1;31m{underline}\x1b[0m\n",
                            ""
                        ));
                    } else {
                        out.push_str(&format!(
                            "  {:>gutter_width$} |  {padding}{underline}\n",
                            ""
                        ));
                    }
                }
            }
        }

        out
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render(false))
    }
}

/// Extension trait to convert WofError into Diagnostic.
pub trait IntoDiagnostic {
    /// Convert into a diagnostic.
    fn into_diagnostic(self) -> Diagnostic;
}

impl IntoDiagnostic for WofError {
    fn into_diagnostic(self) -> Diagnostic {
        Diagnostic::from_error(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_renders_with_source() {
        let span = Span::with_length(1, 4, 3, 1);
        let err = WofError::runtime_at("stack underflow", span);
        let diag = Diagnostic::from_error(err)
            .with_source_line("10 + 20")
            .with_span(span);

        let rendered = diag.render(false);
        assert!(rendered.contains("error"));
        assert!(rendered.contains("1:4"));
        assert!(rendered.contains("10 + 20"));
        assert!(rendered.contains("^"));
    }

    #[test]
    fn diagnostic_renders_with_filename() {
        let span = Span::with_length(3, 5, 20, 3);
        let err = WofError::parse("unexpected token", span);
        let diag = Diagnostic::from_error(err)
            .with_source_line("  dup rot + foo")
            .with_filename("test.wof");

        let rendered = diag.render(false);
        assert!(rendered.contains("test.wof:3:5"));
    }

    #[test]
    fn diagnostic_without_span() {
        let err = WofError::Runtime("something broke".into());
        let diag = Diagnostic::from_error(err);

        let rendered = diag.render(false);
        assert!(rendered.contains("something broke"));
        assert!(!rendered.contains("-->"));
    }

    #[test]
    fn diagnostic_with_color() {
        let span = Span::with_length(1, 1, 0, 3);
        let err = WofError::runtime_at("test", span);
        let diag = Diagnostic::from_error(err)
            .with_source_line("abc def")
            .with_span(span);

        let rendered = diag.render(true);
        assert!(rendered.contains("\x1b[1;31m")); // Red color
        assert!(rendered.contains("\x1b[1;34m")); // Blue color
    }
}
