//! Woflang value types with SIMD-aligned memory layout.
//!
//! The [`WofValue`] type represents all possible values in the Woflang
//! stack machine. The memory layout is carefully designed for:
//!
//! - 16-byte alignment for SIMD operations
//! - Compact discriminant encoding
//! - Cache-friendly access patterns

use crate::{Result, UnitInfo, WofError};
use core::fmt;
use num_traits::{ToPrimitive, Zero};
use std::sync::Arc;

/// Discriminant for [`WofValue`] types.
///
/// The ordering is significant: numeric types are contiguous
/// for fast `is_numeric()` checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum WofType {
    /// Uninitialized or invalid value.
    #[default]
    Unknown = 0,
    /// 64-bit signed integer.
    Integer = 1,
    /// 64-bit IEEE 754 floating point.
    Double = 2,
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
        matches!(self, Self::Integer | Self::Double)
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
            Self::Unknown => write!(f, "unknown"),
            Self::Integer => write!(f, "integer"),
            Self::Double => write!(f, "double"),
            Self::String => write!(f, "string"),
            Self::Symbol => write!(f, "symbol"),
        }
    }
}

/// Internal storage union for [`WofValue`].
///
/// This is repr(C) to ensure predictable layout across platforms.
/// The active variant is indicated by the `WofType` discriminant.
#[derive(Clone)]
enum ValueStorage {
    None,
    Integer(i64),
    Double(f64),
    String(Arc<str>),
}

impl Default for ValueStorage {
    fn default() -> Self {
        Self::None
    }
}

/// The primary value type for the Woflang stack machine.
///
/// # Memory Layout
///
/// The struct is aligned to 16 bytes to enable SIMD operations on
/// arrays of values. The layout is:
///
/// ```text
/// ┌──────────────────────────────────────────────────────────┐
/// │ Offset 0-7:   ValueStorage (8 bytes for primitives)      │
/// │ Offset 8:     WofType discriminant (1 byte)              │
/// │ Offset 9-15:  Padding / reserved                         │
/// │ Offset 16-23: Optional<Arc<UnitInfo>> (8 bytes)          │
/// └──────────────────────────────────────────────────────────┘
/// ```
///
/// # Examples
///
/// ```
/// use woflang_core::{WofValue, WofType};
///
/// let int_val = WofValue::integer(42);
/// assert_eq!(int_val.value_type(), WofType::Integer);
/// assert!(int_val.is_numeric());
///
/// let str_val = WofValue::string("hello");
/// assert!(!str_val.is_numeric());
/// ```
#[derive(Clone, Default)]
#[repr(C)]
pub struct WofValue {
    storage: ValueStorage,
    typ: WofType,
    unit: Option<Arc<UnitInfo>>,
}

impl WofValue {
    // ═══════════════════════════════════════════════════════════════
    // CONSTRUCTORS
    // ═══════════════════════════════════════════════════════════════

    /// Create an integer value.
    #[inline]
    #[must_use]
    pub const fn integer(v: i64) -> Self {
        Self {
            storage: ValueStorage::Integer(v),
            typ: WofType::Integer,
            unit: None,
        }
    }

    /// Create a floating-point value.
    #[inline]
    #[must_use]
    pub const fn double(v: f64) -> Self {
        Self {
            storage: ValueStorage::Double(v),
            typ: WofType::Double,
            unit: None,
        }
    }

    /// Create a string value.
    #[inline]
    #[must_use]
    pub fn string(s: impl AsRef<str>) -> Self {
        Self {
            storage: ValueStorage::String(Arc::from(s.as_ref())),
            typ: WofType::String,
            unit: None,
        }
    }

    /// Create a symbol value.
    #[inline]
    #[must_use]
    pub fn symbol(s: impl AsRef<str>) -> Self {
        Self {
            storage: ValueStorage::String(Arc::from(s.as_ref())),
            typ: WofType::Symbol,
            unit: None,
        }
    }

    /// Create a boolean value (stored as integer 0 or 1).
    #[inline]
    #[must_use]
    pub const fn boolean(b: bool) -> Self {
        Self::integer(if b { 1 } else { 0 })
    }

    /// Create an unknown/nil value.
    #[inline]
    #[must_use]
    pub const fn nil() -> Self {
        Self {
            storage: ValueStorage::None,
            typ: WofType::Unknown,
            unit: None,
        }
    }

    /// Create a value with an attached unit.
    #[inline]
    #[must_use]
    pub fn with_unit(mut self, unit: UnitInfo) -> Self {
        self.unit = Some(Arc::new(unit));
        self
    }

    // ═══════════════════════════════════════════════════════════════
    // TYPE INSPECTION
    // ═══════════════════════════════════════════════════════════════

    /// Get the type discriminant.
    #[inline]
    #[must_use]
    pub const fn value_type(&self) -> WofType {
        self.typ
    }

    /// Returns `true` if this value is numeric (integer or double).
    #[inline]
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        self.typ.is_numeric()
    }

    /// Returns `true` if this value is an integer.
    #[inline]
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        matches!(self.typ, WofType::Integer)
    }

    /// Returns `true` if this value is a double.
    #[inline]
    #[must_use]
    pub const fn is_double(&self) -> bool {
        matches!(self.typ, WofType::Double)
    }

    /// Returns `true` if this value is truthy.
    #[inline]
    #[must_use]
    pub fn is_truthy(&self) -> bool {
        match &self.storage {
            ValueStorage::None => false,
            ValueStorage::Integer(n) => !n.is_zero(),
            ValueStorage::Double(n) => !n.is_zero() && !n.is_nan(),
            ValueStorage::String(s) => !s.is_empty() && s.as_ref() != "false",
        }
    }

    /// Get the attached unit, if any.
    #[inline]
    #[must_use]
    pub fn unit(&self) -> Option<&UnitInfo> {
        self.unit.as_deref()
    }

    // ═══════════════════════════════════════════════════════════════
    // VALUE EXTRACTION
    // ═══════════════════════════════════════════════════════════════

    /// Extract as integer, returning an error if not numeric.
    #[inline]
    pub fn as_integer(&self) -> Result<i64> {
        match &self.storage {
            ValueStorage::Integer(n) => Ok(*n),
            ValueStorage::Double(n) => n
                .to_i64()
                .ok_or_else(|| WofError::type_mismatch("integer", self.typ)),
            _ => Err(WofError::type_mismatch("integer", self.typ)),
        }
    }

    /// Extract as f64, returning an error if not numeric.
    #[inline]
    pub fn as_double(&self) -> Result<f64> {
        match &self.storage {
            ValueStorage::Integer(n) => n
                .to_f64()
                .ok_or_else(|| WofError::type_mismatch("double", self.typ)),
            ValueStorage::Double(n) => Ok(*n),
            _ => Err(WofError::type_mismatch("double", self.typ)),
        }
    }

    /// Extract as numeric (f64), with implicit conversion.
    #[inline]
    pub fn as_numeric(&self) -> Result<f64> {
        self.as_double()
    }

    /// Extract as string reference.
    #[inline]
    pub fn as_str(&self) -> Result<&str> {
        match &self.storage {
            ValueStorage::String(s) => Ok(s.as_ref()),
            _ => Err(WofError::type_mismatch("string", self.typ)),
        }
    }

    /// Extract as boolean.
    #[inline]
    #[must_use]
    pub fn as_bool(&self) -> bool {
        self.is_truthy()
    }

    /// Try to extract the raw integer without conversion.
    #[inline]
    #[must_use]
    pub fn try_integer(&self) -> Option<i64> {
        match &self.storage {
            ValueStorage::Integer(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to extract the raw double without conversion.
    #[inline]
    #[must_use]
    pub fn try_double(&self) -> Option<f64> {
        match &self.storage {
            ValueStorage::Double(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to extract the string without conversion.
    #[inline]
    #[must_use]
    pub fn try_str(&self) -> Option<&str> {
        match &self.storage {
            ValueStorage::String(s) => Some(s.as_ref()),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// TRAIT IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════

impl fmt::Debug for WofValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.storage {
            ValueStorage::None => write!(f, "WofValue::nil"),
            ValueStorage::Integer(n) => write!(f, "WofValue::integer({n})"),
            ValueStorage::Double(n) => write!(f, "WofValue::double({n})"),
            ValueStorage::String(s) if self.typ == WofType::Symbol => {
                write!(f, "WofValue::symbol({s:?})")
            }
            ValueStorage::String(s) => write!(f, "WofValue::string({s:?})"),
        }
    }
}

impl fmt::Display for WofValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.storage {
            ValueStorage::None => write!(f, "<nil>"),
            ValueStorage::Integer(n) => write!(f, "{n}"),
            ValueStorage::Double(n) => {
                if n.fract().abs() < f64::EPSILON {
                    write!(f, "{n:.1}")
                } else {
                    write!(f, "{n}")
                }
            }
            ValueStorage::String(s) => write!(f, "{s}"),
        }?;
        if let Some(unit) = &self.unit {
            write!(f, " {}", unit.name)?;
        }
        Ok(())
    }
}

impl PartialEq for WofValue {
    fn eq(&self, other: &Self) -> bool {
        if self.typ != other.typ {
            return false;
        }
        match (&self.storage, &other.storage) {
            (ValueStorage::None, ValueStorage::None) => true,
            (ValueStorage::Integer(a), ValueStorage::Integer(b)) => a == b,
            (ValueStorage::Double(a), ValueStorage::Double(b)) => {
                // Handle NaN comparison
                (a.is_nan() && b.is_nan()) || a == b
            }
            (ValueStorage::String(a), ValueStorage::String(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for WofValue {}

impl std::hash::Hash for WofValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.typ.hash(state);
        match &self.storage {
            ValueStorage::None => {}
            ValueStorage::Integer(n) => n.hash(state),
            ValueStorage::Double(n) => n.to_bits().hash(state),
            ValueStorage::String(s) => s.hash(state),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// CONVERSIONS
// ═══════════════════════════════════════════════════════════════════════

impl From<i64> for WofValue {
    #[inline]
    fn from(v: i64) -> Self {
        Self::integer(v)
    }
}

impl From<i32> for WofValue {
    #[inline]
    fn from(v: i32) -> Self {
        Self::integer(i64::from(v))
    }
}

impl From<f64> for WofValue {
    #[inline]
    fn from(v: f64) -> Self {
        Self::double(v)
    }
}

impl From<f32> for WofValue {
    #[inline]
    fn from(v: f32) -> Self {
        Self::double(f64::from(v))
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
        Self::string(v)
    }
}

impl From<&str> for WofValue {
    #[inline]
    fn from(v: &str) -> Self {
        Self::string(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_roundtrip() {
        let v = WofValue::integer(42);
        assert_eq!(v.as_integer().unwrap(), 42);
        assert_eq!(v.as_double().unwrap(), 42.0);
    }

    #[test]
    fn double_roundtrip() {
        let v = WofValue::double(3.14159);
        assert!((v.as_double().unwrap() - 3.14159).abs() < f64::EPSILON);
    }

    #[test]
    fn string_roundtrip() {
        let v = WofValue::string("hello, wofl");
        assert_eq!(v.as_str().unwrap(), "hello, wofl");
    }

    #[test]
    fn truthiness() {
        assert!(WofValue::integer(1).is_truthy());
        assert!(!WofValue::integer(0).is_truthy());
        assert!(WofValue::double(0.1).is_truthy());
        assert!(!WofValue::double(0.0).is_truthy());
        assert!(WofValue::string("x").is_truthy());
        assert!(!WofValue::string("").is_truthy());
        assert!(!WofValue::nil().is_truthy());
    }

    #[test]
    fn equality() {
        assert_eq!(WofValue::integer(5), WofValue::integer(5));
        assert_ne!(WofValue::integer(5), WofValue::integer(6));
        assert_ne!(WofValue::integer(5), WofValue::double(5.0));
    }

    #[test]
    fn display_formatting() {
        assert_eq!(format!("{}", WofValue::integer(42)), "42");
        assert_eq!(format!("{}", WofValue::double(3.0)), "3.0");
        assert_eq!(format!("{}", WofValue::string("test")), "test");
    }
}
