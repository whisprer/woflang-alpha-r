//! Tokenizer for Woflang source code.
//!
//! The tokenizer converts UTF-8 source text into a stream of tokens.
//! It supports:
//!
//! - Integers and floating-point literals
//! - Quoted strings
//! - Symbols and operators (including Unicode glyphs)
//! - Comments (lines starting with `#`)
//! - Source location tracking (line:column)
//!
//! ## Performance
//!
//! The tokenizer is designed for minimal allocation. It returns tokens
//! as borrowed string slices where possible, only allocating for string
//! literals that need unescaping.

use std::iter::Peekable;
use std::str::CharIndices;
use woflang_core::Span;

/// Token kinds recognized by the Woflang tokenizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// Integer literal (e.g., `42`, `-17`).
    Integer,
    /// Floating-point literal (e.g., `3.14`, `-0.5`).
    Float,
    /// Quoted string literal (e.g., `"hello"`).
    String,
    /// Identifier or symbol (e.g., `+`, `dup`, `|0⟩`).
    Symbol,
    /// Label definition (e.g., `:label`).
    Label,
    /// Label reference (e.g., `@label`).
    LabelRef,
    /// End of input.
    Eof,
}

/// A token from the source text.
#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    /// The kind of token.
    pub kind: TokenKind,
    /// The raw text of the token.
    pub text: &'a str,
    /// Source span (line, column, offset, length).
    pub span: Span,
}

impl<'a> Token<'a> {
    /// Create a new token.
    #[inline]
    const fn new(kind: TokenKind, text: &'a str, span: Span) -> Self {
        Self { kind, text, span }
    }

    /// Create an EOF token.
    #[inline]
    fn eof(offset: usize, line: u32, column: u32) -> Self {
        Self {
            kind: TokenKind::Eof,
            text: "",
            span: Span::with_length(line, column, offset as u32, 0),
        }
    }

    /// Get the byte offset (for backwards compatibility).
    #[inline]
    #[must_use]
    pub fn offset(&self) -> usize {
        self.span.offset() as usize
    }
}

/// Tokenizer state machine for Woflang source.
pub struct Tokenizer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    offset: usize,
    line: u32,
    column: u32,
    line_start: usize,
}

impl<'a> Tokenizer<'a> {
    /// Create a new tokenizer for the given source.
    #[inline]
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            offset: 0,
            line: 1,
            column: 1,
            line_start: 0,
        }
    }

    /// Get the current position as a Span.
    #[inline]
    #[allow(dead_code)]
    fn current_span(&self, length: u32) -> Span {
        Span::with_length(self.line, self.column, self.offset as u32, length)
    }

    /// Peek at the current character without consuming.
    #[inline]
    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    /// Consume the current character and advance.
    #[inline]
    fn advance(&mut self) -> Option<(usize, char)> {
        let result = self.chars.next();
        if let Some((idx, c)) = &result {
            self.offset = *idx;
            if *c == '\n' {
                self.line += 1;
                self.column = 1;
                self.line_start = idx + 1;
            } else {
                self.column += 1;
            }
        }
        result
    }

    /// Skip whitespace characters.
    #[inline]
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Check if a character can start a number.
    #[inline]
    fn is_number_start(c: char, next: Option<char>) -> bool {
        c.is_ascii_digit() || (c == '-' && next.is_some_and(|n| n.is_ascii_digit() || n == '.'))
    }

    /// Tokenize a number (integer or float).
    fn tokenize_number(&mut self, start: usize, start_line: u32, start_col: u32) -> Token<'a> {
        let mut has_dot = false;

        // Consume leading sign if present
        if self.peek_char() == Some('-') {
            self.advance();
        }

        // Consume digits and optional decimal point
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' && !has_dot {
                // Look ahead to ensure it's a decimal, not method call
                let mut lookahead = self.chars.clone();
                lookahead.next();
                if lookahead.peek().is_some_and(|(_, n)| n.is_ascii_digit()) {
                    has_dot = true;
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let end = self.chars.peek().map_or(self.source.len(), |&(i, _)| i);
        let text = &self.source[start..end];
        let kind = if has_dot {
            TokenKind::Float
        } else {
            TokenKind::Integer
        };

        let span = Span::with_length(start_line, start_col, start as u32, (end - start) as u32);
        Token::new(kind, text, span)
    }

    /// Tokenize a quoted string.
    fn tokenize_string(&mut self, start: usize, start_line: u32, start_col: u32) -> Token<'a> {
        // Skip opening quote
        self.advance();

        loop {
            match self.advance() {
                Some((_, '"')) => break,
                Some((_, '\\')) => {
                    // Skip escaped character
                    self.advance();
                }
                Some(_) => continue,
                None => break, // Unterminated string
            }
        }

        let end = self.chars.peek().map_or(self.source.len(), |&(i, _)| i);
        let span = Span::with_length(start_line, start_col, start as u32, (end - start) as u32);
        Token::new(TokenKind::String, &self.source[start..end], span)
    }

    /// Tokenize a symbol (identifier or operator).
    fn tokenize_symbol(&mut self, start: usize, start_line: u32, start_col: u32) -> Token<'a> {
        // Check for label definition (:name) or label reference (@name)
        let first_char = self.source[start..].chars().next();
        let is_label = first_char == Some(':');
        let is_label_ref = first_char == Some('@');
        
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() || c == '"' {
                break;
            }
            // Unicode-aware: keep consuming until whitespace or quote
            self.advance();
        }

        let end = self.chars.peek().map_or(self.source.len(), |&(i, _)| i);
        let text = &self.source[start..end];
        let span = Span::with_length(start_line, start_col, start as u32, (end - start) as u32);
        
        let kind = if is_label {
            TokenKind::Label
        } else if is_label_ref {
            TokenKind::LabelRef
        } else {
            TokenKind::Symbol
        };
        
        Token::new(kind, text, span)
    }

    /// Get the next token from the source.
    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        let Some(&(start, c)) = self.chars.peek() else {
            return Token::eof(self.source.len(), self.line, self.column);
        };

        let start_line = self.line;
        let start_col = self.column;

        // Comment: skip to end of line
        if c == '#' {
            while let Some(c) = self.peek_char() {
                self.advance();
                if c == '\n' {
                    break;
                }
            }
            return self.next_token();
        }

        // String literal
        if c == '"' {
            return self.tokenize_string(start, start_line, start_col);
        }

        // Number literal
        let next = {
            let mut lookahead = self.chars.clone();
            lookahead.next();
            lookahead.peek().map(|&(_, c)| c)
        };
        if Self::is_number_start(c, next) {
            return self.tokenize_number(start, start_line, start_col);
        }

        // Symbol (anything else)
        self.tokenize_symbol(start, start_line, start_col)
    }

    /// Tokenize the entire source into a vector.
    #[must_use]
    pub fn tokenize_all(mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token.kind == TokenKind::Eof {
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token.kind == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    }
}

/// Parse a token's text into a string value (handling escapes).
#[must_use]
pub fn parse_string_literal(text: &str) -> String {
    // Strip quotes
    let inner = if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
        &text[1..text.len() - 1]
    } else {
        text
    };

    let mut result = String::with_capacity(inner.len());
    let mut chars = inner.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_integers() {
        let tokens: Vec<_> = Tokenizer::new("42 -17 0").collect();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Integer);
        assert_eq!(tokens[0].text, "42");
        assert_eq!(tokens[0].span.line(), 1);
        assert_eq!(tokens[0].span.column(), 1);
        assert_eq!(tokens[1].text, "-17");
        assert_eq!(tokens[2].text, "0");
    }

    #[test]
    fn tokenize_floats() {
        let tokens: Vec<_> = Tokenizer::new("3.14 -0.5 1.0").collect();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Float);
        assert_eq!(tokens[0].text, "3.14");
    }

    #[test]
    fn tokenize_strings() {
        let tokens: Vec<_> = Tokenizer::new(r#""hello" "world""#).collect();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::String);
        assert_eq!(tokens[0].text, r#""hello""#);
    }

    #[test]
    fn tokenize_symbols() {
        let tokens: Vec<_> = Tokenizer::new("+ - dup |0⟩ π").collect();
        assert_eq!(tokens.len(), 5);
        assert!(tokens.iter().all(|t| t.kind == TokenKind::Symbol));
    }

    #[test]
    fn tokenize_labels() {
        let tokens: Vec<_> = Tokenizer::new(":start @end :loop").collect();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Label);
        assert_eq!(tokens[0].text, ":start");
        assert_eq!(tokens[1].kind, TokenKind::LabelRef);
        assert_eq!(tokens[1].text, "@end");
        assert_eq!(tokens[2].kind, TokenKind::Label);
    }

    #[test]
    fn tokenize_mixed() {
        let tokens: Vec<_> = Tokenizer::new("42 3.14 + \"result\"").collect();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Integer);
        assert_eq!(tokens[1].kind, TokenKind::Float);
        assert_eq!(tokens[2].kind, TokenKind::Symbol);
        assert_eq!(tokens[3].kind, TokenKind::String);
    }

    #[test]
    fn skip_comments() {
        let tokens: Vec<_> = Tokenizer::new("42 # this is a comment\n17").collect();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "42");
        assert_eq!(tokens[1].text, "17");
    }

    #[test]
    fn parse_string_escapes() {
        assert_eq!(parse_string_literal(r#""hello\nworld""#), "hello\nworld");
        assert_eq!(parse_string_literal(r#""tab\there""#), "tab\there");
        assert_eq!(parse_string_literal(r#""quote\"here""#), "quote\"here");
    }

    #[test]
    fn unicode_symbols() {
        let tokens: Vec<_> = Tokenizer::new("∧ ∨ ¬ → ↔").collect();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].text, "∧");
        assert_eq!(tokens[3].text, "→");
    }

    #[test]
    fn multiline_tracking() {
        let tokens: Vec<_> = Tokenizer::new("a\nb\nc").collect();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].span.line(), 1);
        assert_eq!(tokens[1].span.line(), 2);
        assert_eq!(tokens[2].span.line(), 3);
    }

    #[test]
    fn column_tracking() {
        let tokens: Vec<_> = Tokenizer::new("abc def ghi").collect();
        assert_eq!(tokens[0].span.column(), 1);
        assert_eq!(tokens[1].span.column(), 5);
        assert_eq!(tokens[2].span.column(), 9);
    }
}
