//! Stack abstraction for the Woflang virtual machine.
//!
//! The [`WofStack`] type provides a type-safe wrapper around a vector
//! of [`WofValue`]s with additional methods for batch operations and
//! inspection.

use crate::{Result, WofError, WofValue};
use std::fmt;

/// A type-safe stack for Woflang values.
///
/// The stack grows upward: the "top" is the last element.
///
/// # Performance
///
/// The underlying storage is a `Vec<WofValue>`. For SIMD batch
/// processing, use [`WofStack::as_slice`] to obtain a contiguous
/// slice of values.
#[derive(Clone, Default)]
pub struct WofStack {
    inner: Vec<WofValue>,
}

impl WofStack {
    /// Create a new empty stack.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Create a stack with pre-allocated capacity.
    #[inline]
    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: Vec::with_capacity(cap),
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // BASIC OPERATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Push a value onto the stack.
    #[inline]
    pub fn push(&mut self, value: WofValue) {
        self.inner.push(value);
    }

    /// Pop a value from the stack.
    ///
    /// Returns an error if the stack is empty.
    #[inline]
    pub fn pop(&mut self) -> Result<WofValue> {
        self.inner
            .pop()
            .ok_or_else(|| WofError::stack_underflow(1, 0))
    }

    /// Pop a value, converting it to an integer.
    #[inline]
    pub fn pop_integer(&mut self) -> Result<i64> {
        self.pop()?.as_integer()
    }

    /// Pop a value, converting it to a double.
    #[inline]
    pub fn pop_double(&mut self) -> Result<f64> {
        self.pop()?.as_double()
    }

    /// Pop a value, extracting it as a numeric f64.
    #[inline]
    pub fn pop_numeric(&mut self) -> Result<f64> {
        self.pop()?.as_numeric()
    }

    /// Pop a value, extracting it as a string.
    #[inline]
    pub fn pop_string(&mut self) -> Result<String> {
        self.pop()?.as_str().map(String::from)
    }

    /// Pop a value, extracting it as a boolean.
    #[inline]
    pub fn pop_bool(&mut self) -> Result<bool> {
        Ok(self.pop()?.as_bool())
    }

    /// Peek at the top value without removing it.
    #[inline]
    pub fn peek(&self) -> Result<&WofValue> {
        self.inner
            .last()
            .ok_or_else(|| WofError::stack_underflow(1, 0))
    }

    /// Peek at the value at offset from top (0 = top).
    #[inline]
    pub fn peek_at(&self, offset: usize) -> Result<&WofValue> {
        let len = self.inner.len();
        if offset >= len {
            return Err(WofError::stack_underflow(offset + 1, len));
        }
        // SAFETY: offset < len, so len - 1 - offset is valid
        Ok(&self.inner[len - 1 - offset])
    }

    // ═══════════════════════════════════════════════════════════════
    // STACK MANIPULATION
    // ═══════════════════════════════════════════════════════════════

    /// Duplicate the top value.
    #[inline]
    pub fn dup(&mut self) -> Result<()> {
        let top = self.peek()?.clone();
        self.push(top);
        Ok(())
    }

    /// Drop the top value.
    #[inline]
    pub fn drop(&mut self) -> Result<()> {
        self.pop().map(|_| ())
    }

    /// Swap the top two values.
    #[inline]
    pub fn swap(&mut self) -> Result<()> {
        let len = self.inner.len();
        if len < 2 {
            return Err(WofError::stack_underflow(2, len));
        }
        self.inner.swap(len - 1, len - 2);
        Ok(())
    }

    /// Rotate the top three values: (a b c -- b c a)
    #[inline]
    pub fn rot(&mut self) -> Result<()> {
        let len = self.inner.len();
        if len < 3 {
            return Err(WofError::stack_underflow(3, len));
        }
        // Rotate: move bottom of 3 to top
        let a = self.inner.remove(len - 3);
        self.inner.push(a);
        Ok(())
    }

    /// Duplicate the second value: (a b -- a b a)
    #[inline]
    pub fn over(&mut self) -> Result<()> {
        let val = self.peek_at(1)?.clone();
        self.push(val);
        Ok(())
    }

    /// Clear all values from the stack.
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    // ═══════════════════════════════════════════════════════════════
    // INSPECTION
    // ═══════════════════════════════════════════════════════════════

    /// Get the number of values on the stack.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the stack is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Check if the stack has at least `n` values.
    #[inline]
    #[must_use]
    pub fn has(&self, n: usize) -> bool {
        self.inner.len() >= n
    }

    /// Get a slice of all values (bottom to top).
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[WofValue] {
        &self.inner
    }

    /// Get a mutable slice of all values.
    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [WofValue] {
        &mut self.inner
    }

    /// Iterate over values from bottom to top.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &WofValue> {
        self.inner.iter()
    }

    /// Iterate over values from top to bottom.
    #[inline]
    pub fn iter_rev(&self) -> impl Iterator<Item = &WofValue> {
        self.inner.iter().rev()
    }

    // ═══════════════════════════════════════════════════════════════
    // BATCH OPERATIONS (SIMD-READY)
    // ═══════════════════════════════════════════════════════════════

    /// Pop multiple values at once.
    ///
    /// Returns values in pop order (top of stack is index 0).
    #[inline]
    pub fn pop_n(&mut self, n: usize) -> Result<Vec<WofValue>> {
        let len = self.inner.len();
        if len < n {
            return Err(WofError::stack_underflow(n, len));
        }
        let values: Vec<_> = self.inner.drain((len - n)..).rev().collect();
        Ok(values)
    }

    /// Push multiple values at once.
    #[inline]
    pub fn push_all(&mut self, values: impl IntoIterator<Item = WofValue>) {
        self.inner.extend(values);
    }

    /// Pop all numeric values and return as f64 slice.
    ///
    /// Useful for SIMD batch processing.
    pub fn drain_numerics(&mut self) -> Result<Vec<f64>> {
        let mut result = Vec::with_capacity(self.inner.len());
        for val in self.inner.drain(..) {
            result.push(val.as_numeric()?);
        }
        Ok(result)
    }
}

impl fmt::Debug for WofStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WofStack")
            .field("len", &self.inner.len())
            .field("values", &self.inner)
            .finish()
    }
}

impl fmt::Display for WofStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack[{}]: ", self.inner.len())?;
        if self.inner.is_empty() {
            write!(f, "(empty)")?;
        } else {
            for (i, val) in self.inner.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{val}")?;
            }
        }
        Ok(())
    }
}

impl FromIterator<WofValue> for WofStack {
    fn from_iter<T: IntoIterator<Item = WofValue>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl Extend<WofValue> for WofStack {
    fn extend<T: IntoIterator<Item = WofValue>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_roundtrip() {
        let mut stack = WofStack::new();
        stack.push(WofValue::integer(42));
        stack.push(WofValue::double(3.14));

        assert_eq!(stack.len(), 2);
        assert!((stack.pop_numeric().unwrap() - 3.14).abs() < f64::EPSILON);
        assert_eq!(stack.pop_integer().unwrap(), 42);
        assert!(stack.is_empty());
    }

    #[test]
    fn dup_operation() {
        let mut stack = WofStack::new();
        stack.push(WofValue::integer(5));
        stack.dup().unwrap();

        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop_integer().unwrap(), 5);
        assert_eq!(stack.pop_integer().unwrap(), 5);
    }

    #[test]
    fn swap_operation() {
        let mut stack = WofStack::new();
        stack.push(WofValue::integer(1));
        stack.push(WofValue::integer(2));
        stack.swap().unwrap();

        assert_eq!(stack.pop_integer().unwrap(), 1);
        assert_eq!(stack.pop_integer().unwrap(), 2);
    }

    #[test]
    fn underflow_error() {
        let mut stack = WofStack::new();
        let result = stack.pop();
        assert!(matches!(result, Err(WofError::StackUnderflow { .. })));
    }

    #[test]
    fn pop_n_batch() {
        let mut stack = WofStack::new();
        stack.push(WofValue::integer(1));
        stack.push(WofValue::integer(2));
        stack.push(WofValue::integer(3));

        let values = stack.pop_n(2).unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0].as_integer().unwrap(), 3); // Top first
        assert_eq!(values[1].as_integer().unwrap(), 2);
        assert_eq!(stack.len(), 1);
    }
}
