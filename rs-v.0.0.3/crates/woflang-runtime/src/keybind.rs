//! Keybinding alias system for Woflang.
//!
//! Allows users to define short ASCII aliases for Unicode glyphs,
//! making the language more accessible without a specialized keyboard.
//!
//! # Examples
//!
//! ```
//! use woflang_runtime::KeyBindings;
//!
//! let mut kb = KeyBindings::new();
//! kb.bind("df", "∂");
//! kb.bind("int", "∫");
//! kb.bind("sum", "∑");
//!
//! assert_eq!(kb.resolve("df"), Some("∂"));
//! assert_eq!(kb.resolve("unknown"), None);
//! ```

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

/// Manages keybinding aliases (e.g., "df" → "∂").
#[derive(Debug, Clone, Default)]
pub struct KeyBindings {
    /// Alias → glyph mappings.
    bindings: HashMap<String, String>,
}

impl KeyBindings {
    /// Create a new empty keybinding manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Create a keybinding manager with default math/logic bindings.
    #[must_use]
    pub fn with_defaults() -> Self {
        let mut kb = Self::new();
        
        // Control flow
        kb.bind("if", "若");
        kb.bind("then", "則");
        kb.bind("else", "或");
        kb.bind("elif", "另");
        kb.bind("ret", "至");
        kb.bind("fn", "⊕");
        kb.bind("call", "巡");
        kb.bind("loop", "⟳");
        kb.bind("times", "⨯");
        kb.bind("break", "🛑");
        kb.bind("continue", "↻");
        
        // Blocks
        kb.bind("{", "⺆");
        kb.bind("}", "⺘");
        kb.bind("begin", "⺆");
        kb.bind("end", "⺘");
        
        // Variables
        kb.bind("let", "字");
        kb.bind("get", "読");
        kb.bind("set", "支");
        
        // Math operators
        kb.bind("df", "∂");      // Partial derivative
        kb.bind("int", "∫");     // Integral
        kb.bind("sum", "∑");     // Summation
        kb.bind("prod", "∏");    // Product
        kb.bind("sqrt", "√");    // Square root
        kb.bind("inf", "∞");     // Infinity
        kb.bind("pi", "π");      // Pi
        kb.bind("tau", "τ");     // Tau
        kb.bind("phi", "φ");     // Phi (golden ratio)
        kb.bind("euler", "ℯ");   // Euler's number
        
        // Logic
        kb.bind("and", "∧");
        kb.bind("or", "∨");
        kb.bind("not", "¬");
        kb.bind("xor", "⊻");
        kb.bind("implies", "→");
        kb.bind("iff", "↔");
        kb.bind("forall", "∀");
        kb.bind("exists", "∃");
        
        // Comparison
        kb.bind("eq", "＝");
        kb.bind("ne", "≠");
        kb.bind("lt", "＜");
        kb.bind("gt", "＞");
        kb.bind("le", "≤");
        kb.bind("ge", "≥");
        
        // Sets
        kb.bind("in", "∈");
        kb.bind("notin", "∉");
        kb.bind("subset", "⊂");
        kb.bind("supset", "⊃");
        kb.bind("union", "∪");
        kb.bind("intersect", "∩");
        kb.bind("empty", "∅");
        
        // Greek letters (commonly used)
        kb.bind("alpha", "α");
        kb.bind("beta", "β");
        kb.bind("gamma", "γ");
        kb.bind("delta", "δ");
        kb.bind("epsilon", "ε");
        kb.bind("theta", "θ");
        kb.bind("lambda", "λ");
        kb.bind("mu", "μ");
        kb.bind("sigma", "σ");
        kb.bind("omega", "ω");
        
        // Quantum
        kb.bind("ket0", "|0⟩");
        kb.bind("ket1", "|1⟩");
        kb.bind("bra0", "⟨0|");
        kb.bind("bra1", "⟨1|");
        
        kb
    }

    /// Bind an alias to a glyph.
    pub fn bind(&mut self, alias: impl Into<String>, glyph: impl Into<String>) {
        self.bindings.insert(alias.into(), glyph.into());
    }

    /// Remove a binding.
    pub fn unbind(&mut self, alias: &str) -> bool {
        self.bindings.remove(alias).is_some()
    }

    /// Resolve an alias to its glyph.
    #[must_use]
    pub fn resolve(&self, alias: &str) -> Option<&str> {
        self.bindings.get(alias).map(|s| s.as_str())
    }

    /// Check if an alias is bound.
    #[must_use]
    pub fn is_bound(&self, alias: &str) -> bool {
        self.bindings.contains_key(alias)
    }

    /// Get all bindings as (alias, glyph) pairs.
    #[must_use]
    pub fn all(&self) -> Vec<(&str, &str)> {
        let mut pairs: Vec<_> = self.bindings
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        pairs.sort_by(|a, b| a.0.cmp(b.0));
        pairs
    }

    /// Get the number of bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Check if there are no bindings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Clear all bindings.
    pub fn clear(&mut self) {
        self.bindings.clear();
    }

    /// Expand all aliases in a line of code.
    ///
    /// Performs a single pass, replacing each token that matches an alias.
    #[must_use]
    pub fn expand_line(&self, line: &str) -> String {
        let mut result = String::with_capacity(line.len());
        let mut chars = line.chars().peekable();
        
        while chars.peek().is_some() {
            // Skip whitespace
            if chars.peek().map_or(false, |c| c.is_whitespace()) {
                result.push(chars.next().unwrap());
                continue;
            }
            
            // Try to match an alias
            let start = result.len();
            let mut token = String::new();
            
            // Collect alphanumeric token
            while chars.peek().map_or(false, |c| c.is_alphanumeric() || *c == '_') {
                token.push(chars.next().unwrap());
            }
            
            if token.is_empty() {
                // Not alphanumeric, just copy the char
                if let Some(c) = chars.next() {
                    result.push(c);
                }
            } else if let Some(glyph) = self.resolve(&token) {
                // Found a binding, expand it
                result.push_str(glyph);
            } else {
                // No binding, keep original
                result.push_str(&token);
            }
        }
        
        result
    }

    // ═══════════════════════════════════════════════════════════════
    // PERSISTENCE
    // ═══════════════════════════════════════════════════════════════

    /// Get the default bindings file path (~/.wofbinds).
    #[must_use]
    pub fn default_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".wofbinds"))
    }

    /// Load bindings from a file.
    ///
    /// File format: one binding per line as `alias glyph` or `alias=glyph`.
    pub fn load(&mut self, path: &PathBuf) -> io::Result<usize> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut count = 0;

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse "alias glyph" or "alias=glyph"
            let parts: Vec<&str> = if line.contains('=') {
                line.splitn(2, '=').collect()
            } else {
                line.splitn(2, char::is_whitespace).collect()
            };

            if parts.len() == 2 {
                let alias = parts[0].trim();
                let glyph = parts[1].trim();
                if !alias.is_empty() && !glyph.is_empty() {
                    self.bind(alias, glyph);
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    /// Save bindings to a file.
    pub fn save(&self, path: &PathBuf) -> io::Result<()> {
        let mut file = File::create(path)?;
        
        writeln!(file, "# Woflang keybindings")?;
        writeln!(file, "# Format: alias glyph")?;
        writeln!(file)?;

        for (alias, glyph) in self.all() {
            writeln!(file, "{} {}", alias, glyph)?;
        }

        Ok(())
    }

    /// Load from default path if it exists.
    pub fn load_default(&mut self) -> io::Result<usize> {
        if let Some(path) = Self::default_path() {
            if path.exists() {
                return self.load(&path);
            }
        }
        Ok(0)
    }

    /// Save to default path.
    pub fn save_default(&self) -> io::Result<()> {
        if let Some(path) = Self::default_path() {
            self.save(&path)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "home directory not found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_and_resolve() {
        let mut kb = KeyBindings::new();
        kb.bind("df", "∂");
        
        assert_eq!(kb.resolve("df"), Some("∂"));
        assert_eq!(kb.resolve("unknown"), None);
    }

    #[test]
    fn unbind() {
        let mut kb = KeyBindings::new();
        kb.bind("df", "∂");
        
        assert!(kb.unbind("df"));
        assert!(!kb.unbind("df")); // Already removed
        assert_eq!(kb.resolve("df"), None);
    }

    #[test]
    fn expand_line() {
        let mut kb = KeyBindings::new();
        kb.bind("df", "∂");
        kb.bind("int", "∫");
        
        assert_eq!(kb.expand_line("df x"), "∂ x");
        assert_eq!(kb.expand_line("int df"), "∫ ∂");
        assert_eq!(kb.expand_line("unknown"), "unknown");
    }

    #[test]
    fn expand_preserves_structure() {
        let mut kb = KeyBindings::new();
        kb.bind("if", "若");
        kb.bind("begin", "⺆");
        kb.bind("end", "⺘");
        
        let input = "10 5 > if begin \"yes\" print end";
        let expected = "10 5 > 若 ⺆ \"yes\" print ⺘";
        assert_eq!(kb.expand_line(input), expected);
    }

    #[test]
    fn defaults_include_common_bindings() {
        let kb = KeyBindings::with_defaults();
        
        assert_eq!(kb.resolve("if"), Some("若"));
        assert_eq!(kb.resolve("df"), Some("∂"));
        assert_eq!(kb.resolve("and"), Some("∧"));
        assert_eq!(kb.resolve("pi"), Some("π"));
    }

    #[test]
    fn all_sorted() {
        let mut kb = KeyBindings::new();
        kb.bind("zebra", "z");
        kb.bind("alpha", "a");
        kb.bind("beta", "b");
        
        let all = kb.all();
        assert_eq!(all[0].0, "alpha");
        assert_eq!(all[1].0, "beta");
        assert_eq!(all[2].0, "zebra");
    }
}
