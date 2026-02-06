//! Assertion and testing operations for Woflang.
//!
//! Provides assert, assert_eq, expect, and related testing helpers.

use woflang_core::{WofError, WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

/// Register assertion operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // ASSERTIONS
    // ═══════════════════════════════════════════════════════════════
    
    // Assert top is truthy (non-zero, non-empty)
    interp.register("assert", |interp| {
        let val = interp.stack_mut().pop()?;
        if !is_truthy(&val) {
            return Err(WofError::Runtime(format!("assertion failed: {:?}", val)));
        }
        Ok(())
    });

    // Assert top two values are equal
    interp.register("assert_eq", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        if !values_equal(&a, &b) {
            return Err(WofError::Runtime(format!(
                "assertion failed: {:?} != {:?}", a, b
            )));
        }
        Ok(())
    });

    // Assert top two values are not equal
    interp.register("assert_ne", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        if values_equal(&a, &b) {
            return Err(WofError::Runtime(format!(
                "assertion failed: {:?} == {:?}", a, b
            )));
        }
        Ok(())
    });

    // Assert a < b
    interp.register("assert_lt", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        if !(a < b) {
            return Err(WofError::Runtime(format!(
                "assertion failed: {} < {}", a, b
            )));
        }
        Ok(())
    });

    // Assert a <= b
    interp.register("assert_le", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        if !(a <= b) {
            return Err(WofError::Runtime(format!(
                "assertion failed: {} <= {}", a, b
            )));
        }
        Ok(())
    });

    // Assert a > b
    interp.register("assert_gt", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        if !(a > b) {
            return Err(WofError::Runtime(format!(
                "assertion failed: {} > {}", a, b
            )));
        }
        Ok(())
    });

    // Assert a >= b
    interp.register("assert_ge", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        if !(a >= b) {
            return Err(WofError::Runtime(format!(
                "assertion failed: {} >= {}", a, b
            )));
        }
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // EXPECT (soft assertions - print warning but continue)
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("expect", |interp| {
        let val = interp.stack_mut().pop()?;
        if !is_truthy(&val) {
            eprintln!("[WARN] expectation failed: {:?}", val);
        }
        Ok(())
    });

    interp.register("expect_eq", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        if !values_equal(&a, &b) {
            eprintln!("[WARN] expectation failed: {:?} != {:?}", a, b);
        }
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // APPROXIMATE EQUALITY (for floats)
    // ═══════════════════════════════════════════════════════════════
    
    // Assert approximately equal within epsilon
    // Stack: a b epsilon → (asserts |a - b| < epsilon)
    interp.register("assert_approx", |interp| {
        let eps = interp.stack_mut().pop()?.as_double()?;
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        if (a - b).abs() >= eps {
            return Err(WofError::Runtime(format!(
                "assertion failed: |{} - {}| = {} >= {}", a, b, (a - b).abs(), eps
            )));
        }
        Ok(())
    });

    // Check if approximately equal (push result)
    interp.register("approx_eq?", |interp| {
        let eps = interp.stack_mut().pop()?.as_double()?;
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        let result = if (a - b).abs() < eps { 1 } else { 0 };
        interp.stack_mut().push(WofValue::integer(result));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // FAIL AND UNREACHABLE
    // ═══════════════════════════════════════════════════════════════
    
    // Always fail with message
    interp.register("fail", |interp| {
        let msg = interp.stack_mut().pop()?;
        Err(WofError::Runtime(format!("fail: {}", msg)))
    });

    // Mark unreachable code
    interp.register("unreachable", |_interp| {
        Err(WofError::Runtime("unreachable code reached".into()))
    });

    // Panic with stack trace hint
    interp.register("panic", |interp| {
        let msg = interp.stack_mut().pop()?;
        Err(WofError::Runtime(format!("PANIC: {}", msg)))
    });
}

/// Check if a value is truthy.
fn is_truthy(val: &WofValue) -> bool {
    if val.is_nil() {
        return false;
    }
    if let Ok(i) = val.as_integer() {
        return i != 0;
    }
    if let Ok(f) = val.as_double() {
        return f != 0.0 && !f.is_nan();
    }
    if let Ok(s) = val.as_string() {
        return !s.is_empty();
    }
    true
}

/// Check if two values are equal.
fn values_equal(a: &WofValue, b: &WofValue) -> bool {
    // Try numeric comparison first
    if let (Ok(fa), Ok(fb)) = (a.as_double(), b.as_double()) {
        return (fa - fb).abs() < f64::EPSILON || (fa.is_nan() && fb.is_nan());
    }
    // Try string comparison
    if let (Ok(sa), Ok(sb)) = (a.as_string(), b.as_string()) {
        return sa == sb;
    }
    // Fall back to debug representation
    format!("{:?}", a) == format!("{:?}", b)
}
