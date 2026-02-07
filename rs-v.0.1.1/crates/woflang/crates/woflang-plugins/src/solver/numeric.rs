//! Numeric equation solvers.
//!
//! Provides solvers for common equation types:
//! - Linear equations: ax + b = c
//! - Quadratic equations: ax² + bx + c = 0
//! - Systems of 2 linear equations
//!
//! ## Operations
//!
//! - `solve_linear` - Solve ax + b = c
//! - `solve_quadratic` - Solve ax² + bx + c = 0
//! - `solve_linear_2x2` - Solve 2x2 system of linear equations

use woflang_core::{WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// SOLVER IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Solve linear equation: ax + b = c => x = (c - b) / a
pub fn solve_linear(a: f64, b: f64, c: f64) -> Option<f64> {
    if a.abs() < 1e-12 {
        None
    } else {
        Some((c - b) / a)
    }
}

/// Result of quadratic solver.
#[derive(Debug)]
pub enum QuadraticResult {
    /// Two distinct real roots
    TwoReal(f64, f64),
    /// One repeated real root
    OneReal(f64),
    /// Complex conjugate roots: real ± imag*i
    Complex { real: f64, imag: f64 },
    /// Degenerate (a = 0)
    Degenerate,
}

/// Solve quadratic equation: ax² + bx + c = 0
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticResult {
    if a.abs() < 1e-12 {
        // Degenerates to linear: bx + c = 0
        if b.abs() < 1e-12 {
            return QuadraticResult::Degenerate;
        }
        return QuadraticResult::OneReal(-c / b);
    }

    let discriminant = b * b - 4.0 * a * c;

    if discriminant.abs() < 1e-12 {
        // One repeated root
        QuadraticResult::OneReal(-b / (2.0 * a))
    } else if discriminant > 0.0 {
        // Two distinct real roots
        let sqrt_disc = discriminant.sqrt();
        let x1 = (-b + sqrt_disc) / (2.0 * a);
        let x2 = (-b - sqrt_disc) / (2.0 * a);
        QuadraticResult::TwoReal(x1, x2)
    } else {
        // Complex conjugate roots
        let real = -b / (2.0 * a);
        let imag = (-discriminant).sqrt() / (2.0 * a);
        QuadraticResult::Complex { real, imag }
    }
}

/// Solve 2x2 system of linear equations:
/// a1*x + b1*y = c1
/// a2*x + b2*y = c2
/// Returns (x, y) if solvable
pub fn solve_linear_2x2(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> Option<(f64, f64)> {
    // Using Cramer's rule
    let det = a1 * b2 - a2 * b1;
    
    if det.abs() < 1e-12 {
        return None; // No unique solution
    }

    let x = (c1 * b2 - c2 * b1) / det;
    let y = (a1 * c2 - a2 * c1) / det;

    Some((x, y))
}

/// Newton-Raphson root finding for f(x) = 0
/// Given a function f and its derivative f', find root starting from x0
pub fn newton_raphson<F, G>(f: F, f_prime: G, x0: f64, max_iter: usize, tolerance: f64) -> Option<f64>
where
    F: Fn(f64) -> f64,
    G: Fn(f64) -> f64,
{
    let mut x = x0;
    
    for _ in 0..max_iter {
        let fx = f(x);
        let fpx = f_prime(x);
        
        if fpx.abs() < 1e-15 {
            return None; // Derivative too small
        }
        
        let x_new = x - fx / fpx;
        
        if (x_new - x).abs() < tolerance {
            return Some(x_new);
        }
        
        x = x_new;
    }
    
    Some(x) // Return best estimate
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register numeric solver operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // LINEAR SOLVER
    // ─────────────────────────────────────────────────────────────────────

    // Solve ax + b = c for x
    // Stack: a b c → x
    interp.register("solve_linear", |interp| {
        let c = interp.stack_mut().pop()?.as_double()?;
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;

        println!("[solver] Solving: {}x + {} = {}", a, b, c);

        match solve_linear(a, b, c) {
            Some(x) => {
                println!("[solver] Solution: x = {}", x);
                interp.stack_mut().push(WofValue::double(x));
            }
            None => {
                println!("[solver] No solution (a = 0)");
                interp.stack_mut().push(WofValue::nil());
            }
        }
        Ok(())
    });

    // Simpler version: solve ax = b for x
    // Stack: a b → x
    interp.register("solve_linear_simple", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;

        if a.abs() < 1e-12 {
            println!("[solver] Cannot solve: coefficient is zero");
            interp.stack_mut().push(WofValue::nil());
        } else {
            let x = b / a;
            println!("[solver] {}x = {} → x = {}", a, b, x);
            interp.stack_mut().push(WofValue::double(x));
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // QUADRATIC SOLVER
    // ─────────────────────────────────────────────────────────────────────

    // Solve ax² + bx + c = 0
    // Stack: a b c → x1 [x2]
    // For complex roots, pushes real and imag parts
    interp.register("solve_quadratic", |interp| {
        let c = interp.stack_mut().pop()?.as_double()?;
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;

        println!("[solver] Solving: {}x² + {}x + {} = 0", a, b, c);
        let discriminant = b * b - 4.0 * a * c;
        println!("[solver] Discriminant = {}", discriminant);

        match solve_quadratic(a, b, c) {
            QuadraticResult::TwoReal(x1, x2) => {
                println!("[solver] Two solutions:");
                println!("  x₁ = {}", x1);
                println!("  x₂ = {}", x2);
                interp.stack_mut().push(WofValue::double(x1));
                interp.stack_mut().push(WofValue::double(x2));
            }
            QuadraticResult::OneReal(x) => {
                println!("[solver] One solution: x = {}", x);
                interp.stack_mut().push(WofValue::double(x));
            }
            QuadraticResult::Complex { real, imag } => {
                println!("[solver] Complex solutions:");
                println!("  x = {} ± {}i", real, imag);
                // Push as string representation
                let result = format!("{} ± {}i", real, imag);
                interp.stack_mut().push(WofValue::string(result));
            }
            QuadraticResult::Degenerate => {
                println!("[solver] Degenerate equation (a = 0)");
                interp.stack_mut().push(WofValue::nil());
            }
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // 2x2 LINEAR SYSTEM
    // ─────────────────────────────────────────────────────────────────────

    // Solve 2x2 system:
    // a1*x + b1*y = c1
    // a2*x + b2*y = c2
    // Stack: a1 b1 c1 a2 b2 c2 → x y
    interp.register("solve_linear_2x2", |interp| {
        let c2 = interp.stack_mut().pop()?.as_double()?;
        let b2 = interp.stack_mut().pop()?.as_double()?;
        let a2 = interp.stack_mut().pop()?.as_double()?;
        let c1 = interp.stack_mut().pop()?.as_double()?;
        let b1 = interp.stack_mut().pop()?.as_double()?;
        let a1 = interp.stack_mut().pop()?.as_double()?;

        println!("[solver] Solving system:");
        println!("  {}x + {}y = {}", a1, b1, c1);
        println!("  {}x + {}y = {}", a2, b2, c2);

        match solve_linear_2x2(a1, b1, c1, a2, b2, c2) {
            Some((x, y)) => {
                println!("[solver] Solution: x = {}, y = {}", x, y);
                interp.stack_mut().push(WofValue::double(x));
                interp.stack_mut().push(WofValue::double(y));
            }
            None => {
                println!("[solver] No unique solution (parallel or coincident lines)");
                interp.stack_mut().push(WofValue::nil());
            }
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // NEWTON-RAPHSON (for polynomials)
    // ─────────────────────────────────────────────────────────────────────

    // Find root of x² - n = 0 (i.e., compute √n)
    // Stack: n → √n
    interp.register("newton_sqrt", |interp| {
        let n = interp.stack_mut().pop()?.as_double()?;

        if n < 0.0 {
            println!("[solver] Cannot compute square root of negative number");
            interp.stack_mut().push(WofValue::nil());
            return Ok(());
        }

        if n == 0.0 {
            interp.stack_mut().push(WofValue::double(0.0));
            return Ok(());
        }

        // f(x) = x² - n, f'(x) = 2x
        let f = |x: f64| x * x - n;
        let f_prime = |x: f64| 2.0 * x;

        match newton_raphson(f, f_prime, n / 2.0, 100, 1e-15) {
            Some(root) => {
                println!("[solver] √{} ≈ {}", n, root);
                interp.stack_mut().push(WofValue::double(root));
            }
            None => {
                println!("[solver] Newton-Raphson failed to converge");
                interp.stack_mut().push(WofValue::double(n.sqrt()));
            }
        }
        Ok(())
    });

    // Find cube root using Newton-Raphson
    // Stack: n → ∛n
    interp.register("newton_cbrt", |interp| {
        let n = interp.stack_mut().pop()?.as_double()?;

        // f(x) = x³ - n, f'(x) = 3x²
        let f = |x: f64| x * x * x - n;
        let f_prime = |x: f64| 3.0 * x * x;

        let x0 = if n >= 0.0 { n.cbrt() } else { -(-n).cbrt() };

        match newton_raphson(f, f_prime, x0, 100, 1e-15) {
            Some(root) => {
                println!("[solver] ∛{} ≈ {}", n, root);
                interp.stack_mut().push(WofValue::double(root));
            }
            None => {
                println!("[solver] Newton-Raphson failed to converge");
                interp.stack_mut().push(WofValue::double(n.cbrt()));
            }
        }
        Ok(())
    });
}
