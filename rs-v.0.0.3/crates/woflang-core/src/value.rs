//! Woflang value types.
//!
//! The [`WofValue`] enum represents all possible values in the Woflang
//! stack machine. It's a standard Rust tagged union optimized for
//! pattern matching and ergonomic use.

use crate::{Result, WofError};
use std::fmt;
use std::sync::Arc;

/// Type discriminant for [`WofValue`].
///
/// Useful for error messages and type checking without matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum WofType {
    /// Nil/undefined value.
    #[default]
    Nil = 0,
    /// 64-bit signed integer.
    Integer = 1,
    /// 64-bit IEEE 754 floating point.
    Float = 2,
    /// Heap-allocated UTF-8 string.
    String = 3,
    /// Interned symbol (identifier).
    Symbol = 4,
}

impl WofType {
    /// Returns `true` if this type represents a numeric value.
    #[inline]
    #[must_use]
    pub const fn is_numeric(self) -> bool {
        matches!(self, Self::Integer | Self::Float)
    }

    /// Returns `true` if this type represents a string-like value.
    #[inline]
    #[must_use]
    pub const fn is_string_like(self) -> bool {
        matches!(self, Self::String | Self::Symbol)
    }
}

impl fmt::Display for WofType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Integer => write!(f, "integer"),
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Symbol => write!(f, "symbol"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// WOFVALUE - THE CORE VALUE TYPE
// ═══════════════════════════════════════════════════════════════════════════

/// The primary value type for the Woflang stack machine.
///
/// This is a standard Rust enum, enabling idiomatic pattern matching:
///
/// ```
/// use woflang_core::WofValue;
///
/// let val = WofValue::Float(3.14);
///
/// match val {
///     WofValue::Integer(n) => println!("int: {n}"),
///     WofValue::Float(f) => println!("float: {f}"),
///     WofValue::String(s) => println!("string: {s}"),
///     WofValue::Symbol(s) => println!("symbol: {s}"),
///     WofValue::Nil => println!("nil"),
/// }
/// ```
#[derive(Clone)]
pub enum WofValue {
    /// Nil/undefined/uninitialized value.
    Nil,
    /// 64-bit signed integer. Booleans are represented as 0/1.
    Integer(i64),
    /// 64-bit IEEE 754 floating point.
    Float(f64),
    /// Heap-allocated UTF-8 string. Uses `Arc` for cheap cloning.
    String(Arc<str>),
    /// Interned symbol/identifier. Uses `Arc` for cheap cloning.
    Symbol(Arc<str>),
}

impl Default for WofValue {
    fn default() -> Self {
        Self::Nil
    }
}

impl WofValue {
    // ═══════════════════════════════════════════════════════════════
    // CONSTRUCTORS (lowercase, idiomatic)
    // ═══════════════════════════════════════════════════════════════

    /// Create an integer value.
    #[inline]
    #[must_use]
    pub const fn integer(v: i64) -> Self {
        Self::Integer(v)
    }

    /// Create a floating-point value.
    #[inline]
    #[must_use]
    pub const fn float(v: f64) -> Self {
        Self::Float(v)
    }

    /// Create a floating-point value (alias for `float`).
    #[inline]
    #[must_use]
    pub const fn double(v: f64) -> Self {
        Self::Float(v)
    }

    /// Create a string value.
    #[inline]
    #[must_use]
    pub fn string(s: impl AsRef<str>) -> Self {
        Self::String(Arc::from(s.as_ref()))
    }

    /// Create a symbol value.
    #[inline]
    #[must_use]
    pub fn symbol(s: impl AsRef<str>) -> Self {
        Self::Symbol(Arc::from(s.as_ref()))
    }

    /// Create a boolean value (stored as integer 0 or 1).
    #[inline]
    #[must_use]
    pub const fn boolean(b: bool) -> Self {
        Self::Integer(if b { 1 } else { 0 })
    }

    /// Create a nil value.
    #[inline]
    #[must_use]
    pub const fn nil() -> Self {
        Self::Nil
    }

    // ═══════════════════════════════════════════════════════════════
    // TYPE INSPECTION
    // ═══════════════════════════════════════════════════════════════

    /// Get the type discriminant.
    #[inline]
    #[must_use]
    pub const fn value_type(&self) -> WofType {
        match self {
            Self::Nil => WofType::Nil,
            Self::Integer(_) => WofType::Integer,
            Self::Float(_) => WofType::Float,
            Self::String(_) => WofType::String,
            Self::Symbol(_) => WofType::Symbol,
        }
    }

    /// Returns `true` if this value is numeric (integer or float).
    #[inline]
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(self, Self::Integer(_) | Self::Float(_))
    }

    /// Returns `true` if this value is an integer.
    #[inline]
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    /// Returns `true` if this value is a float.
    #[inline]
    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Alias for `is_float`.
    #[inline]
    #[must_use]
    pub const fn is_double(&self) -> bool {
        self.is_float()
    }

    /// Returns `true` if this value is nil.
    #[inline]
    #[must_use]
    pub const fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    /// Returns `true` if this value is a string.
    #[inline]
    #[must_use]
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns `true` if this value is a symbol.
    #[inline]
    #[must_use]
    pub const fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }

    /// Returns `true` if this value is truthy.
    ///
    /// - `Nil` → false
    /// - `Integer(0)` → false
    /// - `Float(0.0)` or `Float(NaN)` → false
    /// - Empty string → false
    /// - Everything else → true
    #[inline]
    #[must_use]
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Integer(n) => *n != 0,
            Self::Float(f) => *f != 0.0 && !f.is_nan(),
            Self::String(s) | Self::Symbol(s) => !s.is_empty(),
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // VALUE EXTRACTION
    // ═══════════════════════════════════════════════════════════════

    /// Extract as integer, returning an error if not numeric.
    #[inline]
    pub fn as_integer(&self) -> Result<i64> {
        match self {
            Self::Integer(n) => Ok(*n),
            Self::Float(f) => {
                if f.is_finite() {
                    Ok(*f as i64)
                } else {
                    Err(WofError::type_mismatch("integer", self.value_type()))
                }
            }
            _ => Err(WofError::type_mismatch("integer", self.value_type())),
        }
    }

    /// Alias for `as_integer`.
    #[inline]
    pub fn as_int(&self) -> Result<i64> {
        self.as_integer()
    }

    /// Extract as f64, returning an error if not numeric.
    #[inline]
    pub fn as_float(&self) -> Result<f64> {
        match self {
            Self::Integer(n) => Ok(*n as f64),
            Self::Float(f) => Ok(*f),
            _ => Err(WofError::type_mismatch("float", self.value_type())),
        }
    }

    /// Alias for `as_float`.
    #[inline]
    pub fn as_double(&self) -> Result<f64> {
        self.as_float()
    }

    /// Alias for `as_float`.
    #[inline]
    pub fn as_numeric(&self) -> Result<f64> {
        self.as_float()
    }

    /// Extract as string reference.
    #[inline]
    pub fn as_str(&self) -> Result<&str> {
        match self {
            Self::String(s) | Self::Symbol(s) => Ok(s.as_ref()),
            _ => Err(WofError::type_mismatch("string", self.value_type())),
        }
    }

    /// Extract as owned String.
    #[inline]
    pub fn as_string(&self) -> Result<String> {
        self.as_str().map(String::from)
    }

    /// Extract as boolean (truthiness).
    #[inline]
    #[must_use]
    pub fn as_bool(&self) -> bool {
        self.is_truthy()
    }

    /// Try to extract raw integer without conversion.
    #[inline]
    #[must_use]
    pub const fn try_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to extract raw float without conversion.
    #[inline]
    #[must_use]
    pub const fn try_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Alias for `try_float`.
    #[inline]
    #[must_use]
    pub const fn try_double(&self) -> Option<f64> {
        self.try_float()
    }

    /// Try to extract string without conversion.
    #[inline]
    #[must_use]
    pub fn try_str(&self) -> Option<&str> {
        match self {
            Self::String(s) | Self::Symbol(s) => Some(s.as_ref()),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════

impl fmt::Debug for WofValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "Nil"),
            Self::Integer(n) => write!(f, "Integer({n})"),
            Self::Float(n) => write!(f, "Float({n})"),
            Self::String(s) => write!(f, "String({s:?})"),
            Self::Symbol(s) => write!(f, "Symbol({s:?})"),
        }
    }
}

impl fmt::Display for WofValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Integer(n) => write!(f, "{n}"),
            Self::Float(n) => {
                if n.fract().abs() < f64::EPSILON {
                    write!(f, "{n:.1}")
                } else {
                    write!(f, "{n}")
                }
            }
            Self::String(s) | Self::Symbol(s) => write!(f, "{s}"),
        }
    }
}

impl PartialEq for WofValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => {
                (a.is_nan() && b.is_nan()) || a == b
            }
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Symbol(a), Self::Symbol(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for WofValue {}

impl std::hash::Hash for WofValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Nil => {}
            Self::Integer(n) => n.hash(state),
            Self::Float(f) => f.to_bits().hash(state),
            Self::String(s) | Self::Symbol(s) => s.hash(state),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CONVERSIONS
// ═══════════════════════════════════════════════════════════════════════════

impl From<i64> for WofValue {
    #[inline]
    fn from(v: i64) -> Self {
        Self::Integer(v)
    }
}

impl From<i32> for WofValue {
    #[inline]
    fn from(v: i32) -> Self {
        Self::Integer(i64::from(v))
    }
}

impl From<f64> for WofValue {
    #[inline]
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}

impl From<f32> for WofValue {
    #[inline]
    fn from(v: f32) -> Self {
        Self::Float(f64::from(v))
    }
}

impl From<bool> for WofValue {
    #[inline]
    fn from(v: bool) -> Self {
        Self::boolean(v)
    }
}

impl From<String> for WofValue {
    #[inline]
    fn from(v: String) -> Self {
        Self::String(Arc::from(v.as_str()))
    }
}

impl From<&str> for WofValue {
    #[inline]
    fn from(v: &str) -> Self {
        Self::String(Arc::from(v))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_matching_works() {
        let v = WofValue::Float(2.718);
        let result = match v {
            WofValue::Integer(n) => format!("int: {n}"),
            WofValue::Float(f) => format!("float: {f}"),
            _ => "other".to_string(),
        };
        assert!(result.starts_with("float:"));
    }

    #[test]
    fn direct_construction() {
        // These should all work - enum variants as constructors
        let _ = WofValue::Integer(42);
        let _ = WofValue::Float(3.14);
        let _ = WofValue::Nil;
    }

    #[test]
    fn method_construction() {
        let _ = WofValue::integer(42);
        let _ = WofValue::float(3.14);
        let _ = WofValue::double(3.14);
        let _ = WofValue::string("hello");
        let _ = WofValue::symbol("x");
        let _ = WofValue::boolean(true);
        let _ = WofValue::nil();
    }

    #[test]
    fn truthiness() {
        assert!(WofValue::Integer(1).is_truthy());
        assert!(!WofValue::Integer(0).is_truthy());
        assert!(WofValue::Float(0.1).is_truthy());
        assert!(!WofValue::Float(0.0).is_truthy());
        assert!(!WofValue::Nil.is_truthy());
    }
}
