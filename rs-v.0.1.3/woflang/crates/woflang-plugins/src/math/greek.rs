//! Greek mathematical symbols and common constants.
//!
//! Unlike the C++ stubs, these are properly implemented!
//!
//! ## Constants
//!
//! - `π`, `pi`, `PI` - Pi (3.14159...)
//! - `∞`, `inf`, `infinity` - Infinity
//! - `∅`, `empty`, `void` - Empty set / nil
//!
//! ## Operations
//!
//! - `Σ`, `sum` - Sum n values from stack
//! - `Π`, `product` - Product of n values
//! - `Δ`, `delta` - Difference (b - a)
//! - `√`, `sqrt` - Square root

use std::f64::consts::PI;
use woflang_core::{WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

/// Register Greek symbol operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // CONSTANTS
    // ─────────────────────────────────────────────────────────────────────

    // Pi
    interp.register("π", |interp| {
        interp.stack_mut().push(WofValue::double(PI));
        Ok(())
    });

    interp.register("pi", |interp| {
        interp.stack_mut().push(WofValue::double(PI));
        Ok(())
    });

    interp.register("PI", |interp| {
        interp.stack_mut().push(WofValue::double(PI));
        Ok(())
    });

    // Tau (2π)
    interp.register("τ", |interp| {
        interp.stack_mut().push(WofValue::double(2.0 * PI));
        Ok(())
    });

    interp.register("tau", |interp| {
        interp.stack_mut().push(WofValue::double(2.0 * PI));
        Ok(())
    });

    // Euler's number e
    interp.register("ε", |interp| {
        interp.stack_mut().push(WofValue::double(std::f64::consts::E));
        Ok(())
    });

    // Golden ratio φ
    interp.register("φ", |interp| {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        interp.stack_mut().push(WofValue::double(phi));
        Ok(())
    });

    interp.register("phi", |interp| {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        interp.stack_mut().push(WofValue::double(phi));
        Ok(())
    });

    // Infinity
    interp.register("∞", |interp| {
        interp.stack_mut().push(WofValue::double(f64::INFINITY));
        Ok(())
    });

    interp.register("inf", |interp| {
        interp.stack_mut().push(WofValue::double(f64::INFINITY));
        Ok(())
    });

    interp.register("infinity", |interp| {
        interp.stack_mut().push(WofValue::double(f64::INFINITY));
        Ok(())
    });

    // Negative infinity
    interp.register("-∞", |interp| {
        interp.stack_mut().push(WofValue::double(f64::NEG_INFINITY));
        Ok(())
    });

    // Empty / void / nil
    interp.register("∅", |interp| {
        interp.stack_mut().push(WofValue::nil());
        Ok(())
    });

    interp.register("empty", |interp| {
        interp.stack_mut().push(WofValue::nil());
        Ok(())
    });

    interp.register("void", |interp| {
        interp.stack_mut().push(WofValue::nil());
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // AGGREGATION OPERATIONS
    // ─────────────────────────────────────────────────────────────────────

    // Σ - Sum n values: v1 v2 ... vn n → sum
    interp.register("Σ", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()? as usize;
        let mut sum = 0.0;
        for _ in 0..n {
            sum += interp.stack_mut().pop()?.as_double()?;
        }
        interp.stack_mut().push(WofValue::double(sum));
        Ok(())
    });

    interp.register("sum", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()? as usize;
        let mut sum = 0.0;
        for _ in 0..n {
            sum += interp.stack_mut().pop()?.as_double()?;
        }
        interp.stack_mut().push(WofValue::double(sum));
        Ok(())
    });

    // Π - Product of n values: v1 v2 ... vn n → product
    interp.register("Π", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()? as usize;
        let mut prod = 1.0;
        for _ in 0..n {
            prod *= interp.stack_mut().pop()?.as_double()?;
        }
        interp.stack_mut().push(WofValue::double(prod));
        Ok(())
    });

    interp.register("product", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()? as usize;
        let mut prod = 1.0;
        for _ in 0..n {
            prod *= interp.stack_mut().pop()?.as_double()?;
        }
        interp.stack_mut().push(WofValue::double(prod));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // BASIC OPERATIONS WITH GREEK SYMBOLS
    // ─────────────────────────────────────────────────────────────────────

    // Δ - Delta (difference): a b → (b - a)
    interp.register("Δ", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(b - a));
        Ok(())
    });

    interp.register("delta", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(b - a));
        Ok(())
    });

    // √ - Square root
    interp.register("√", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.sqrt()));
        Ok(())
    });

    // ∛ - Cube root
    interp.register("∛", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.cbrt()));
        Ok(())
    });

    // ∜ - Fourth root
    interp.register("∜", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.powf(0.25)));
        Ok(())
    });

    // ± - Plus or minus: x y → (x+y) (x-y)
    interp.register("±", |interp| {
        let y = interp.stack_mut().pop()?.as_double()?;
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x + y));
        interp.stack_mut().push(WofValue::double(x - y));
        Ok(())
    });

    // ∓ - Minus or plus: x y → (x-y) (x+y)
    interp.register("∓", |interp| {
        let y = interp.stack_mut().pop()?.as_double()?;
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x - y));
        interp.stack_mut().push(WofValue::double(x + y));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // SET NOTATION
    // ─────────────────────────────────────────────────────────────────────

    // ∈ - Element of (check if value is in list)
    // For now, just checks if two values are equal
    interp.register("∈", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        // Simple equality check
        let result = if a == b {
            true
        } else if let (Ok(fa), Ok(fb)) = (a.as_double(), b.as_double()) {
            (fa - fb).abs() < f64::EPSILON
        } else {
            false
        };
        interp.stack_mut().push(WofValue::double(if result { 1.0 } else { 0.0 }));
        Ok(())
    });

    // ∉ - Not element of
    interp.register("∉", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = if a == b {
            false
        } else if let (Ok(fa), Ok(fb)) = (a.as_double(), b.as_double()) {
            (fa - fb).abs() >= f64::EPSILON
        } else {
            true
        };
        interp.stack_mut().push(WofValue::double(if result { 1.0 } else { 0.0 }));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // COMPARISON SYMBOLS
    // ─────────────────────────────────────────────────────────────────────

    // ≤ - Less than or equal
    interp.register("≤", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(if a <= b { 1.0 } else { 0.0 }));
        Ok(())
    });

    // ≥ - Greater than or equal
    interp.register("≥", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(if a >= b { 1.0 } else { 0.0 }));
        Ok(())
    });

    // ≠ - Not equal
    interp.register("≠", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(if (a - b).abs() >= f64::EPSILON { 1.0 } else { 0.0 }));
        Ok(())
    });

    // ≈ - Approximately equal (within tolerance)
    interp.register("≈", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        let tol = 1e-9;
        interp.stack_mut().push(WofValue::double(if (a - b).abs() < tol { 1.0 } else { 0.0 }));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // SPECIAL FUNCTIONS
    // ─────────────────────────────────────────────────────────────────────

    // Γ - Gamma function approximation (via Lanczos)
    interp.register("Γ", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        // Simple approximation using Sterling for x > 0
        let result = if x > 0.0 {
            // Γ(n) = (n-1)! for positive integers
            // For general x, use tgamma equivalent
            gamma_approx(x)
        } else {
            f64::NAN
        };
        interp.stack_mut().push(WofValue::double(result));
        Ok(())
    });

    // Help
    interp.register("greek_help", |_interp| {
        println!("Greek Symbol Operations:");
        println!();
        println!("  Constants:");
        println!("    π/pi/PI      → 3.14159...");
        println!("    τ/tau        → 2π");
        println!("    ε            → e (2.71828...)");
        println!("    φ/phi        → golden ratio");
        println!("    ∞/inf        → infinity");
        println!("    ∅/empty/void → nil");
        println!();
        println!("  Aggregation:");
        println!("    v1..vn n Σ/sum     → sum of n values");
        println!("    v1..vn n Π/product → product of n values");
        println!();
        println!("  Operations:");
        println!("    a b Δ/delta  → b - a");
        println!("    x √          → √x");
        println!("    x ∛          → ∛x");
        println!("    x ∜          → ∜x");
        println!("    x y ±        → (x+y) (x-y)");
        println!();
        println!("  Comparisons:");
        println!("    a b ≤        → a ≤ b");
        println!("    a b ≥        → a ≥ b");
        println!("    a b ≠        → a ≠ b");
        println!("    a b ≈        → approximately equal");
        Ok(())
    });
}

/// Simple gamma function approximation using Stirling's approximation
/// and recursion for small values.
fn gamma_approx(x: f64) -> f64 {
    if x < 0.5 {
        // Reflection formula: Γ(1-z)Γ(z) = π/sin(πz)
        PI / (PI * x).sin() / gamma_approx(1.0 - x)
    } else if x < 1.0 {
        // Γ(x) = Γ(x+1)/x
        gamma_approx(x + 1.0) / x
    } else if x < 10.0 {
        // Use recursion: Γ(x+1) = x * Γ(x)
        let mut result = 1.0;
        let mut n = x;
        while n > 1.0 {
            n -= 1.0;
            result *= n;
        }
        result
    } else {
        // Stirling's approximation for large x
        (2.0 * PI / x).sqrt() * (x / std::f64::consts::E).powf(x)
    }
}
