//! Equation solver module for Woflang.
//!
//! Provides symbolic and numeric equation solving capabilities.
//!
//! ## Submodules
//!
//! - `symbolic` - Symbolic expression trees and differentiation
//! - `numeric` - Linear and quadratic numeric solvers
//! - `pattern` - Pattern-based string equation solver
//! - `simplify` - Expression simplification rules
//!
//! ## Quick Reference
//!
//! ### Numeric Solvers
//! ```text
//! a b c solve_linear      # Solve ax + b = c → x
//! a b c solve_quadratic   # Solve ax² + bx + c = 0 → x1 [x2]
//! a1 b1 c1 a2 b2 c2 solve_linear_2x2  # Solve 2x2 system
//! ```
//!
//! ### Pattern Solver
//! ```text
//! "2x + 3 = 7" pattern_solve  # → "x = 2"
//! "x^2 - 5x + 6 = 0" pattern_solve  # → "x = 3, x = 2"
//! ```
//!
//! ### Symbolic Calculus
//! ```text
//! symbolic_diff  # Demo: differentiate x*x
//! 2 sym_const "x" sym_var sym_mul  # Build 2*x
//! "x" sym_diff  # Differentiate w.r.t. x
//! ```
//!
//! ### Simplification
//! ```text
//! "x" "x" "+" simplify_sum  # X + X → 2 * X
//! "x" 1 "*" simplify_mul_one  # X * 1 → X
//! ```

mod symbolic;
mod numeric;
mod pattern;
mod simplify;

use woflang_core::InterpreterContext;
use woflang_runtime::Interpreter;

/// Register all solver operations.
pub fn register(interp: &mut Interpreter) {
    symbolic::register(interp);
    numeric::register(interp);
    pattern::register(interp);
    simplify::register(interp);

    // Help command
    interp.register("solver_help", |_interp| {
        println!("Equation Solver Operations:");
        println!();
        println!("  Numeric Solvers:");
        println!("    a b c solve_linear         # ax + b = c → x");
        println!("    a b solve_linear_simple    # ax = b → x");
        println!("    a b c solve_quadratic      # ax² + bx + c = 0");
        println!("    ... solve_linear_2x2       # 2x2 system");
        println!("    n newton_sqrt              # √n via Newton-Raphson");
        println!("    n newton_cbrt              # ∛n via Newton-Raphson");
        println!();
        println!("  Pattern Solver (string equations):");
        println!("    \"2x + 3 = 7\" pattern_solve     # → \"x = 2\"");
        println!("    \"x^2 - 5x + 6 = 0\" pattern_solve");
        println!("    \"equation\" quick_solve         # → numeric result");
        println!();
        println!("  Symbolic Calculus:");
        println!("    symbolic_diff       # Demo differentiation");
        println!("    val sym_const       # Push constant");
        println!("    \"x\" sym_var         # Push variable");
        println!("    sym_add sym_mul     # Combine expressions");
        println!("    \"x\" sym_diff        # Differentiate");
        println!("    sym_show sym_clear  # Manage expression stack");
        println!();
        println!("  Simplification Rules:");
        println!("    simplify_sum        # X + X → 2 * X");
        println!("    simplify_mul_one    # X * 1 → X");
        println!("    simplify_mul_zero   # X * 0 → 0");
        println!("    simplify_add_zero   # X + 0 → X");
        println!("    simplify_power      # X^0 → 1, X^1 → X");
        println!();
        println!("  Logic:");
        println!("    a b iff             # Biconditional (a ↔ b)");
        Ok(())
    });
}
