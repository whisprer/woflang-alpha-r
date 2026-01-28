//! Expression simplification rules.
//!
//! Provides stack-based simplification rules for symbolic expressions.
//!
//! ## Rules
//!
//! - `simplify_sum` - X X + → 2 X *
//! - `simplify_mul_one` - X 1 * → X, 1 X * → X
//! - `simplify_mul_zero` - X 0 * → 0, 0 X * → 0
//! - `simplify_add_zero` - X 0 + → X, 0 X + → X

use woflang_core::{InterpreterContext, WofValue};
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Check if a value is a symbol/string.
fn is_symbol(v: &WofValue) -> bool {
    matches!(v, WofValue::String(_))
}

/// Check if a value is an integer.
fn is_integer(v: &WofValue) -> bool {
    matches!(v, WofValue::Integer(_))
}

/// Check if a value is zero.
fn is_zero(v: &WofValue) -> bool {
    match v {
        WofValue::Integer(0) => true,
        WofValue::Float(f) if *f == 0.0 => true,
        _ => false,
    }
}

/// Check if a value is one.
fn is_one(v: &WofValue) -> bool {
    match v {
        WofValue::Integer(1) => true,
        WofValue::Float(f) if (*f - 1.0).abs() < f64::EPSILON => true,
        _ => false,
    }
}

/// Get string content from a value.
fn as_string(v: &WofValue) -> Option<String> {
    match v {
        WofValue::String(s) => Some(s.to_string()),
        WofValue::Symbol(s) => Some(s.to_string()),
        _ => None,
    }
}

/// Check if two values represent the same symbol.
fn same_symbol(a: &WofValue, b: &WofValue) -> bool {
    match (a, b) {
        (WofValue::String(s1), WofValue::String(s2)) => s1 == s2,
        (WofValue::Symbol(s1), WofValue::Symbol(s2)) => s1 == s2,
        _ => false,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register simplification rules.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // SIMPLIFY SUM: X X + → 2 X *
    // ─────────────────────────────────────────────────────────────────────

    // Stack: X X "+" → 2 X "*"
    interp.register("simplify_sum", |interp| {
        if interp.stack().len() < 3 {
            return Ok(());
        }

        let c = interp.stack_mut().pop()?; // operator
        let b = interp.stack_mut().pop()?; // rhs
        let a = interp.stack_mut().pop()?; // lhs

        // Check for pattern: X X "+"
        let matches = is_symbol(&a)
            && is_symbol(&b)
            && same_symbol(&a, &b)
            && as_string(&c).map(|s| s == "+").unwrap_or(false);

        if matches {
            // X X + → 2 X *
            interp.stack_mut().push(WofValue::integer(2));
            interp.stack_mut().push(a);
            interp.stack_mut().push(WofValue::string("*".to_string()));
            println!("[simplify] X + X → 2 * X");
        } else {
            // No match, restore original
            interp.stack_mut().push(a);
            interp.stack_mut().push(b);
            interp.stack_mut().push(c);
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // SIMPLIFY MUL ONE: X 1 * → X, 1 X * → X
    // ─────────────────────────────────────────────────────────────────────

    interp.register("simplify_mul_one", |interp| {
        if interp.stack().len() < 3 {
            return Ok(());
        }

        let c = interp.stack_mut().pop()?; // operator
        let b = interp.stack_mut().pop()?; // rhs
        let a = interp.stack_mut().pop()?; // lhs

        // Check if operator is "*"
        let is_mul = as_string(&c).map(|s| s == "*").unwrap_or(false);

        if is_mul {
            // X * 1 → X
            if is_one(&b) {
                interp.stack_mut().push(a);
                println!("[simplify] X * 1 → X");
                return Ok(());
            }

            // 1 * X → X
            if is_one(&a) {
                interp.stack_mut().push(b);
                println!("[simplify] 1 * X → X");
                return Ok(());
            }
        }

        // No match, restore
        interp.stack_mut().push(a);
        interp.stack_mut().push(b);
        interp.stack_mut().push(c);
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // SIMPLIFY MUL ZERO: X 0 * → 0, 0 X * → 0
    // ─────────────────────────────────────────────────────────────────────

    interp.register("simplify_mul_zero", |interp| {
        if interp.stack().len() < 3 {
            return Ok(());
        }

        let c = interp.stack_mut().pop()?;
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;

        let is_mul = as_string(&c).map(|s| s == "*").unwrap_or(false);

        if is_mul && (is_zero(&a) || is_zero(&b)) {
            interp.stack_mut().push(WofValue::integer(0));
            println!("[simplify] X * 0 → 0");
            return Ok(());
        }

        // No match, restore
        interp.stack_mut().push(a);
        interp.stack_mut().push(b);
        interp.stack_mut().push(c);
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // SIMPLIFY ADD ZERO: X 0 + → X, 0 X + → X
    // ─────────────────────────────────────────────────────────────────────

    interp.register("simplify_add_zero", |interp| {
        if interp.stack().len() < 3 {
            return Ok(());
        }

        let c = interp.stack_mut().pop()?;
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;

        let is_add = as_string(&c).map(|s| s == "+").unwrap_or(false);

        if is_add {
            if is_zero(&b) {
                interp.stack_mut().push(a);
                println!("[simplify] X + 0 → X");
                return Ok(());
            }

            if is_zero(&a) {
                interp.stack_mut().push(b);
                println!("[simplify] 0 + X → X");
                return Ok(());
            }
        }

        // No match, restore
        interp.stack_mut().push(a);
        interp.stack_mut().push(b);
        interp.stack_mut().push(c);
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // SIMPLIFY POWER: X 0 ^ → 1, X 1 ^ → X
    // ─────────────────────────────────────────────────────────────────────

    interp.register("simplify_power", |interp| {
        if interp.stack().len() < 3 {
            return Ok(());
        }

        let c = interp.stack_mut().pop()?;
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;

        let is_pow = as_string(&c).map(|s| s == "^").unwrap_or(false);

        if is_pow {
            // X^0 → 1
            if is_zero(&b) {
                interp.stack_mut().push(WofValue::integer(1));
                println!("[simplify] X^0 → 1");
                return Ok(());
            }

            // X^1 → X
            if is_one(&b) {
                interp.stack_mut().push(a);
                println!("[simplify] X^1 → X");
                return Ok(());
            }
        }

        // No match, restore
        interp.stack_mut().push(a);
        interp.stack_mut().push(b);
        interp.stack_mut().push(c);
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // SIMPLIFY ALL: Try all rules
    // ─────────────────────────────────────────────────────────────────────

    // Note: This is a placeholder - full implementation would need expression trees
    interp.register("simplify", |_interp| {
        println!("[simplify] Use specific rules: simplify_sum, simplify_mul_one, etc.");
        println!("           Or use symbolic expressions with sym_* operations");
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // IFF (Logical biconditional)
    // ─────────────────────────────────────────────────────────────────────

    // Stack: a b → (a ↔ b)
    interp.register("iff", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;

        let a_bool = match &a {
            WofValue::Integer(n) => *n != 0,
            WofValue::Float(f) => *f != 0.0,
            WofValue::String(s) => !s.is_empty(),
            WofValue::Symbol(s) => !s.is_empty(),
            WofValue::Nil => false,
        };

        let b_bool = match &b {
            WofValue::Integer(n) => *n != 0,
            WofValue::Float(f) => *f != 0.0,
            WofValue::String(s) => !s.is_empty(),
            WofValue::Symbol(s) => !s.is_empty(),
            WofValue::Nil => false,
        };

        let result = a_bool == b_bool;
        interp.stack_mut().push(WofValue::Float(if result { 1.0 } else { 0.0 }));
        Ok(())
    });
}
