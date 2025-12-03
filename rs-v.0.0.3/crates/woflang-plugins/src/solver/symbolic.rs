//! Symbolic expression trees and differentiation.
//!
//! Provides a basic symbolic calculus system with expression trees
//! that can be differentiated with respect to a variable.
//!
//! ## Expression Types
//!
//! - `Const` - Constant value
//! - `Var` - Variable (x, y, etc.)
//! - `Add` - Addition
//! - `Mul` - Multiplication
//! - `Pow` - Power (x^n)
//! - `Sin`, `Cos` - Trigonometric functions
//! - `Ln` - Natural logarithm
//!
//! ## Operations
//!
//! - `symbolic_diff` - Differentiate an expression demo
//! - `sym_const`, `sym_var`, `sym_add`, `sym_mul` - Build expressions
//! - `sym_diff` - Differentiate top expression

use std::sync::{Mutex, OnceLock};
use woflang_core::WofValue;
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// EXPRESSION TREE
// ═══════════════════════════════════════════════════════════════════════════

/// A symbolic expression.
#[derive(Clone, Debug)]
pub enum Expr {
    /// Constant value
    Const(f64),
    /// Variable
    Var(String),
    /// Addition: lhs + rhs
    Add(Box<Expr>, Box<Expr>),
    /// Subtraction: lhs - rhs
    Sub(Box<Expr>, Box<Expr>),
    /// Multiplication: lhs * rhs
    Mul(Box<Expr>, Box<Expr>),
    /// Division: lhs / rhs
    Div(Box<Expr>, Box<Expr>),
    /// Power: base ^ exp
    Pow(Box<Expr>, Box<Expr>),
    /// Negation: -expr
    Neg(Box<Expr>),
    /// Sine
    Sin(Box<Expr>),
    /// Cosine
    Cos(Box<Expr>),
    /// Natural logarithm
    Ln(Box<Expr>),
    /// Exponential: e^x
    Exp(Box<Expr>),
}

impl Expr {
    /// Create a constant.
    pub fn constant(val: f64) -> Self {
        Expr::Const(val)
    }

    /// Create a variable.
    pub fn var(name: &str) -> Self {
        Expr::Var(name.to_string())
    }

    /// Convert to string representation.
    pub fn to_string(&self) -> String {
        match self {
            Expr::Const(v) => {
                if v.fract() == 0.0 && v.abs() < 1e10 {
                    format!("{}", *v as i64)
                } else {
                    format!("{:.6}", v).trim_end_matches('0').trim_end_matches('.').to_string()
                }
            }
            Expr::Var(name) => name.clone(),
            Expr::Add(l, r) => format!("({} + {})", l.to_string(), r.to_string()),
            Expr::Sub(l, r) => format!("({} - {})", l.to_string(), r.to_string()),
            Expr::Mul(l, r) => format!("({} * {})", l.to_string(), r.to_string()),
            Expr::Div(l, r) => format!("({} / {})", l.to_string(), r.to_string()),
            Expr::Pow(b, e) => format!("({}^{})", b.to_string(), e.to_string()),
            Expr::Neg(e) => format!("(-{})", e.to_string()),
            Expr::Sin(e) => format!("sin({})", e.to_string()),
            Expr::Cos(e) => format!("cos({})", e.to_string()),
            Expr::Ln(e) => format!("ln({})", e.to_string()),
            Expr::Exp(e) => format!("exp({})", e.to_string()),
        }
    }

    /// Differentiate with respect to a variable.
    pub fn diff(&self, var: &str) -> Expr {
        match self {
            // d/dx c = 0
            Expr::Const(_) => Expr::Const(0.0),

            // d/dx x = 1, d/dx y = 0 for y != x
            Expr::Var(name) => {
                if name == var {
                    Expr::Const(1.0)
                } else {
                    Expr::Const(0.0)
                }
            }

            // d/dx (f + g) = f' + g'
            Expr::Add(l, r) => Expr::Add(Box::new(l.diff(var)), Box::new(r.diff(var))),

            // d/dx (f - g) = f' - g'
            Expr::Sub(l, r) => Expr::Sub(Box::new(l.diff(var)), Box::new(r.diff(var))),

            // Product rule: d/dx (f * g) = f' * g + f * g'
            Expr::Mul(l, r) => Expr::Add(
                Box::new(Expr::Mul(Box::new(l.diff(var)), r.clone())),
                Box::new(Expr::Mul(l.clone(), Box::new(r.diff(var)))),
            ),

            // Quotient rule: d/dx (f / g) = (f' * g - f * g') / g²
            Expr::Div(l, r) => Expr::Div(
                Box::new(Expr::Sub(
                    Box::new(Expr::Mul(Box::new(l.diff(var)), r.clone())),
                    Box::new(Expr::Mul(l.clone(), Box::new(r.diff(var)))),
                )),
                Box::new(Expr::Pow(r.clone(), Box::new(Expr::Const(2.0)))),
            ),

            // Power rule: d/dx (f^n) = n * f^(n-1) * f'
            // (simplified for constant exponent)
            Expr::Pow(base, exp) => {
                // For constant exponent: n * f^(n-1) * f'
                // For variable exponent: use logarithmic differentiation
                match exp.as_ref() {
                    Expr::Const(n) => Expr::Mul(
                        Box::new(Expr::Const(*n)),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Pow(
                                base.clone(),
                                Box::new(Expr::Const(n - 1.0)),
                            )),
                            Box::new(base.diff(var)),
                        )),
                    ),
                    _ => {
                        // General case: d/dx (f^g) = f^g * (g' * ln(f) + g * f'/f)
                        Expr::Mul(
                            Box::new(Expr::Pow(base.clone(), exp.clone())),
                            Box::new(Expr::Add(
                                Box::new(Expr::Mul(
                                    Box::new(exp.diff(var)),
                                    Box::new(Expr::Ln(base.clone())),
                                )),
                                Box::new(Expr::Mul(
                                    exp.clone(),
                                    Box::new(Expr::Div(
                                        Box::new(base.diff(var)),
                                        base.clone(),
                                    )),
                                )),
                            )),
                        )
                    }
                }
            }

            // d/dx (-f) = -f'
            Expr::Neg(e) => Expr::Neg(Box::new(e.diff(var))),

            // d/dx sin(f) = cos(f) * f'
            Expr::Sin(e) => Expr::Mul(
                Box::new(Expr::Cos(e.clone())),
                Box::new(e.diff(var)),
            ),

            // d/dx cos(f) = -sin(f) * f'
            Expr::Cos(e) => Expr::Neg(Box::new(Expr::Mul(
                Box::new(Expr::Sin(e.clone())),
                Box::new(e.diff(var)),
            ))),

            // d/dx ln(f) = f' / f
            Expr::Ln(e) => Expr::Div(Box::new(e.diff(var)), e.clone()),

            // d/dx exp(f) = exp(f) * f'
            Expr::Exp(e) => Expr::Mul(
                Box::new(Expr::Exp(e.clone())),
                Box::new(e.diff(var)),
            ),
        }
    }

    /// Simplify the expression (basic simplifications).
    pub fn simplify(&self) -> Expr {
        match self {
            Expr::Add(l, r) => {
                let l = l.simplify();
                let r = r.simplify();
                match (&l, &r) {
                    (Expr::Const(0.0), _) => r,
                    (_, Expr::Const(0.0)) => l,
                    (Expr::Const(a), Expr::Const(b)) => Expr::Const(a + b),
                    _ => Expr::Add(Box::new(l), Box::new(r)),
                }
            }
            Expr::Sub(l, r) => {
                let l = l.simplify();
                let r = r.simplify();
                match (&l, &r) {
                    (_, Expr::Const(0.0)) => l,
                    (Expr::Const(a), Expr::Const(b)) => Expr::Const(a - b),
                    _ => Expr::Sub(Box::new(l), Box::new(r)),
                }
            }
            Expr::Mul(l, r) => {
                let l = l.simplify();
                let r = r.simplify();
                match (&l, &r) {
                    (Expr::Const(0.0), _) | (_, Expr::Const(0.0)) => Expr::Const(0.0),
                    (Expr::Const(1.0), _) => r,
                    (_, Expr::Const(1.0)) => l,
                    (Expr::Const(a), Expr::Const(b)) => Expr::Const(a * b),
                    _ => Expr::Mul(Box::new(l), Box::new(r)),
                }
            }
            Expr::Div(l, r) => {
                let l = l.simplify();
                let r = r.simplify();
                match (&l, &r) {
                    (Expr::Const(0.0), _) => Expr::Const(0.0),
                    (_, Expr::Const(1.0)) => l,
                    (Expr::Const(a), Expr::Const(b)) if *b != 0.0 => Expr::Const(a / b),
                    _ => Expr::Div(Box::new(l), Box::new(r)),
                }
            }
            Expr::Pow(b, e) => {
                let b = b.simplify();
                let e = e.simplify();
                match (&b, &e) {
                    (_, Expr::Const(0.0)) => Expr::Const(1.0),
                    (_, Expr::Const(1.0)) => b,
                    (Expr::Const(a), Expr::Const(n)) => Expr::Const(a.powf(*n)),
                    _ => Expr::Pow(Box::new(b), Box::new(e)),
                }
            }
            Expr::Neg(e) => {
                let e = e.simplify();
                match e {
                    Expr::Const(v) => Expr::Const(-v),
                    Expr::Neg(inner) => *inner,
                    _ => Expr::Neg(Box::new(e)),
                }
            }
            _ => self.clone(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// EXPRESSION STACK FOR BUILDING EXPRESSIONS
// ═══════════════════════════════════════════════════════════════════════════

fn expr_stack() -> &'static Mutex<Vec<Expr>> {
    static STACK: OnceLock<Mutex<Vec<Expr>>> = OnceLock::new();
    STACK.get_or_init(|| Mutex::new(Vec::new()))
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register symbolic operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // DEMO DIFFERENTIATION
    // ─────────────────────────────────────────────────────────────────────

    // Demo: differentiate x*x with respect to x
    interp.register("symbolic_diff", |interp| {
        // Build x * x
        let expr = Expr::Mul(
            Box::new(Expr::Var("x".to_string())),
            Box::new(Expr::Var("x".to_string())),
        );
        let deriv = expr.diff("x").simplify();

        println!();
        println!("[calculus] Expression: {}", expr.to_string());
        println!("[calculus] Derivative: {}", deriv.to_string());
        println!();

        // Push 1.0 as result indicator
        interp.stack_mut().push(WofValue::Float(1.0));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // EXPRESSION BUILDING
    // ─────────────────────────────────────────────────────────────────────

    // Push a constant onto expression stack
    // Stack: value → ()
    interp.register("sym_const", |interp| {
        let val = interp.stack_mut().pop()?.as_float()?;
        if let Ok(mut stack) = expr_stack().lock() {
            stack.push(Expr::Const(val));
            println!("[symbolic] Pushed constant: {}", val);
        }
        Ok(())
    });

    // Push a variable onto expression stack
    // Stack: "name" → ()
    interp.register("sym_var", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        if let Ok(mut stack) = expr_stack().lock() {
            stack.push(Expr::Var(name.clone()));
            println!("[symbolic] Pushed variable: {}", name);
        }
        Ok(())
    });

    // Add top two expressions
    // Expr stack: a b → (a + b)
    interp.register("sym_add", |_interp| {
        if let Ok(mut stack) = expr_stack().lock() {
            if stack.len() < 2 {
                println!("[symbolic] sym_add needs 2 expressions");
                return Ok(());
            }
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(Expr::Add(Box::new(a), Box::new(b)));
            println!("[symbolic] Added expressions");
        }
        Ok(())
    });

    // Multiply top two expressions
    // Expr stack: a b → (a * b)
    interp.register("sym_mul", |_interp| {
        if let Ok(mut stack) = expr_stack().lock() {
            if stack.len() < 2 {
                println!("[symbolic] sym_mul needs 2 expressions");
                return Ok(());
            }
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(Expr::Mul(Box::new(a), Box::new(b)));
            println!("[symbolic] Multiplied expressions");
        }
        Ok(())
    });

    // Power: base^exp
    interp.register("sym_pow", |_interp| {
        if let Ok(mut stack) = expr_stack().lock() {
            if stack.len() < 2 {
                println!("[symbolic] sym_pow needs 2 expressions");
                return Ok(());
            }
            let exp = stack.pop().unwrap();
            let base = stack.pop().unwrap();
            stack.push(Expr::Pow(Box::new(base), Box::new(exp)));
            println!("[symbolic] Power expression");
        }
        Ok(())
    });

    // Differentiate top expression
    // Stack: "var" → ()
    // Expr stack: expr → derivative
    interp.register("sym_diff", |interp| {
        let var = interp.stack_mut().pop()?.as_string()?;
        if let Ok(mut stack) = expr_stack().lock() {
            if stack.is_empty() {
                println!("[symbolic] sym_diff needs an expression");
                return Ok(());
            }
            let expr = stack.pop().unwrap();
            let deriv = expr.diff(&var).simplify();
            println!("[symbolic] d/d{} ({}) = {}", var, expr.to_string(), deriv.to_string());
            stack.push(deriv);
        }
        Ok(())
    });

    // Show top expression
    interp.register("sym_show", |_interp| {
        if let Ok(stack) = expr_stack().lock() {
            if stack.is_empty() {
                println!("[symbolic] Expression stack is empty");
            } else {
                println!("[symbolic] Top expression: {}", stack.last().unwrap().to_string());
            }
        }
        Ok(())
    });

    // Clear expression stack
    interp.register("sym_clear", |_interp| {
        if let Ok(mut stack) = expr_stack().lock() {
            stack.clear();
            println!("[symbolic] Expression stack cleared");
        }
        Ok(())
    });
}
