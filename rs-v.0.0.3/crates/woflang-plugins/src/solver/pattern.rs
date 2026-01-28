//! Pattern-based equation solver.
//!
//! Solves equations given as strings using pattern matching.
//!
//! ## Supported Forms
//!
//! - Linear: `"2x + 3 = 7"`, `"x - 5 = 10"`, `"-3x = 9"`
//! - Quadratic: `"x^2 - 5x + 6 = 0"`, `"2x^2 + 3x - 2 = 0"`
//!
//! ## Operations
//!
//! - `pattern_solve` - Parse and solve an equation string

use woflang_core::{InterpreterContext, WofValue};
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// PATTERN MATCHING SOLVER
// ═══════════════════════════════════════════════════════════════════════════

/// Result of linear equation solve.
struct LinearResult {
    ok: bool,
    x: f64,
}

/// Result of quadratic equation solve.
struct QuadraticResult {
    ok: bool,
    x1: f64,
    x2: f64,
    complex_roots: bool,
}

/// Solve linear equation: ax + b = c
fn solve_linear(a: f64, b: f64, c: f64) -> LinearResult {
    if a.abs() < 1e-12 {
        LinearResult { ok: false, x: 0.0 }
    } else {
        LinearResult {
            ok: true,
            x: (c - b) / a,
        }
    }
}

/// Solve quadratic equation: ax² + bx + c = 0
fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticResult {
    if a.abs() < 1e-12 {
        // Degenerates to linear
        let lin = solve_linear(b, c, 0.0);
        return QuadraticResult {
            ok: lin.ok,
            x1: lin.x,
            x2: lin.x,
            complex_roots: false,
        };
    }

    let disc = b * b - 4.0 * a * c;

    if disc < 0.0 {
        let real = -b / (2.0 * a);
        let imag = (-disc).sqrt() / (2.0 * a);
        QuadraticResult {
            ok: true,
            x1: real,
            x2: imag, // Store imag in x2 for complex case
            complex_roots: true,
        }
    } else {
        let sqrt_disc = disc.sqrt();
        QuadraticResult {
            ok: true,
            x1: (-b + sqrt_disc) / (2.0 * a),
            x2: (-b - sqrt_disc) / (2.0 * a),
            complex_roots: false,
        }
    }
}

/// Parse a coefficient string like "2", "-3", "", "+", "-"
fn parse_coeff(s: &str, allow_empty: bool) -> Option<f64> {
    let trimmed: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    
    if trimmed.is_empty() {
        return if allow_empty { Some(1.0) } else { None };
    }
    
    if trimmed == "+" {
        return Some(1.0);
    }
    
    if trimmed == "-" {
        return Some(-1.0);
    }
    
    trimmed.parse().ok()
}

/// Try to match and solve linear equation: ax + b = c
/// Patterns:
/// - "2x + 3 = 7"
/// - "x + 1 = 5"
/// - "-3x - 9 = 0"
fn try_linear_pattern(eq: &str) -> Option<String> {
    // Remove spaces and normalize
    let eq = eq.replace(" ", "");
    
    // Split by '='
    let parts: Vec<&str> = eq.split('=').collect();
    if parts.len() != 2 {
        return None;
    }
    
    let lhs = parts[0];
    let rhs = parts[1];
    
    // Parse RHS (should be a number)
    let c: f64 = rhs.parse().ok()?;
    
    // Parse LHS: look for 'x' terms and constants
    // Pattern: [coeff]x [+/-] [const]
    
    let mut a = 0.0;
    let mut b = 0.0;
    
    // Simple state machine parser
    let mut i = 0;
    let chars: Vec<char> = lhs.chars().collect();
    
    while i < chars.len() {
        let mut sign = 1.0;
        
        // Check for leading sign
        if chars[i] == '+' {
            sign = 1.0;
            i += 1;
        } else if chars[i] == '-' {
            sign = -1.0;
            i += 1;
        }
        
        // Collect coefficient
        let mut coeff_str = String::new();
        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
            coeff_str.push(chars[i]);
            i += 1;
        }
        
        // Check if this is an x term
        if i < chars.len() && chars[i] == 'x' {
            let coeff = if coeff_str.is_empty() {
                1.0
            } else {
                coeff_str.parse().unwrap_or(1.0)
            };
            a += sign * coeff;
            i += 1;
        } else if !coeff_str.is_empty() {
            // Constant term
            let val: f64 = coeff_str.parse().ok()?;
            b += sign * val;
        }
    }
    
    let sol = solve_linear(a, b, c);
    if sol.ok {
        Some(format!("x = {:.6}", sol.x))
    } else {
        None
    }
}

/// Try to match and solve quadratic equation: ax² + bx + c = 0
/// Patterns:
/// - "x^2 - 5x + 6 = 0"
/// - "2x^2 + 3x - 2 = 0"
fn try_quadratic_pattern(eq: &str) -> Option<String> {
    let eq = eq.replace(" ", "");
    
    // Must end with "= 0" (or "=0")
    if !eq.ends_with("=0") {
        return None;
    }
    
    // Must contain "x^2"
    if !eq.contains("x^2") {
        return None;
    }
    
    let lhs = &eq[..eq.len() - 2]; // Remove "=0"
    
    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;
    
    // Simple parser for ax^2 + bx + c
    let chars: Vec<char> = lhs.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let mut sign = 1.0;
        
        // Check for leading sign
        if chars[i] == '+' {
            sign = 1.0;
            i += 1;
        } else if chars[i] == '-' {
            sign = -1.0;
            i += 1;
        }
        
        // Collect coefficient
        let mut coeff_str = String::new();
        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
            coeff_str.push(chars[i]);
            i += 1;
        }
        
        // Check term type
        if i + 2 < chars.len() && &lhs[i..i+3] == "x^2" {
            // x² term
            let coeff = if coeff_str.is_empty() { 1.0 } else { coeff_str.parse().unwrap_or(1.0) };
            a += sign * coeff;
            i += 3;
        } else if i < chars.len() && chars[i] == 'x' {
            // x term
            let coeff = if coeff_str.is_empty() { 1.0 } else { coeff_str.parse().unwrap_or(1.0) };
            b += sign * coeff;
            i += 1;
        } else if !coeff_str.is_empty() {
            // Constant term
            let val: f64 = coeff_str.parse().ok()?;
            c += sign * val;
        }
    }
    
    let sol = solve_quadratic(a, b, c);
    if !sol.ok {
        return None;
    }
    
    if sol.complex_roots {
        Some(format!("x = {} ± {}i", sol.x1, sol.x2))
    } else if (sol.x1 - sol.x2).abs() < 1e-9 {
        Some(format!("x = {:.6}", sol.x1))
    } else {
        Some(format!("x = {:.6}, x = {:.6}", sol.x1, sol.x2))
    }
}

/// Solve an equation given as a string.
pub fn pattern_solve(eq: &str) -> String {
    // Try linear first
    if let Some(solution) = try_linear_pattern(eq) {
        return solution;
    }
    
    // Try quadratic
    if let Some(solution) = try_quadratic_pattern(eq) {
        return solution;
    }
    
    format!(
        "pattern_solve: no recognised pattern in equation '{}'. \
         Supported forms include ax + b = c and ax^2 + bx + c = 0.",
        eq
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register pattern solver operations.
pub fn register(interp: &mut Interpreter) {
    // Solve equation from string
    // Stack: "equation" → "solution"
    interp.register("pattern_solve", |interp| {
        let eq = interp.stack_mut().pop()?.as_string()?;
        let solution = pattern_solve(&eq);
        
        println!("[pattern_solve] Input: {}", eq);
        println!("[pattern_solve] Result: {}", solution);
        
        interp.stack_mut().push(WofValue::string(solution));
        Ok(())
    });

    // Quick solve for "ax + b = c" format
    // Stack: "equation" → x
    interp.register("quick_solve", |interp| {
        let eq = interp.stack_mut().pop()?.as_string()?;
        
        if let Some(solution) = try_linear_pattern(&eq) {
            // Extract the numeric value
            if let Some(x_str) = solution.strip_prefix("x = ") {
                if let Ok(x) = x_str.parse::<f64>() {
                    interp.stack_mut().push(WofValue::Float(x));
                    return Ok(());
                }
            }
        }
        
        if let Some(solution) = try_quadratic_pattern(&eq) {
            interp.stack_mut().push(WofValue::string(solution));
            return Ok(());
        }
        
        println!("[quick_solve] Could not solve: {}", eq);
        interp.stack_mut().push(WofValue::Nil);
        Ok(())
    });

    // Evaluate polynomial at a point
    // Stack: a0 a1 ... an n x → f(x)
    // Evaluates a0 + a1*x + a2*x² + ... + an*x^n
    interp.register("poly_eval", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        let n = interp.stack_mut().pop()?.as_int()? as usize;
        
        let mut coeffs = Vec::with_capacity(n + 1);
        for _ in 0..=n {
            coeffs.push(interp.stack_mut().pop()?.as_float()?);
        }
        coeffs.reverse();
        
        // Horner's method
        let mut result = coeffs[n];
        for i in (0..n).rev() {
            result = result * x + coeffs[i];
        }
        
        interp.stack_mut().push(WofValue::Float(result));
        Ok(())
    });
}
