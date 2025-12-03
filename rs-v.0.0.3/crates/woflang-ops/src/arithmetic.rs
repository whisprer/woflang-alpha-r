//! Core arithmetic operations.
//!
//! Provides basic mathematical operations on numeric stack values:
//!
//! | Operation | Stack Effect | Description |
//! |-----------|--------------|-------------|
//! | `+`       | (a b -- c)   | Addition |
//! | `-`       | (a b -- c)   | Subtraction |
//! | `*`       | (a b -- c)   | Multiplication |
//! | `/`       | (a b -- c)   | Division |
//! | `%`       | (a b -- c)   | Modulo |
//! | `neg`     | (a -- b)     | Negation |
//! | `abs`     | (a -- b)     | Absolute value |
//! | `min`     | (a b -- c)   | Minimum |
//! | `max`     | (a b -- c)   | Maximum |

use woflang_core::{InterpreterContext, Result, WofError, WofValue};
use woflang_runtime::Interpreter;

/// Register all arithmetic operations.
pub fn register(interp: &mut Interpreter) {
    interp.register("+", op_add);
    interp.register("-", op_sub);
    interp.register("*", op_mul);
    interp.register("/", op_div);
    interp.register("%", op_mod);
    interp.register("mod", op_mod);
    interp.register("neg", op_neg);
    interp.register("abs", op_abs);
    interp.register("min", op_min);
    interp.register("max", op_max);
    interp.register("inc", op_inc);
    interp.register("dec", op_dec);

    // Unicode aliases
    interp.register("×", op_mul);
    interp.register("÷", op_div);
}

fn op_add(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop()?;
    let a = interp.stack_mut().pop()?;

    // Integer arithmetic if both are integers
    let result = match (a.try_integer(), b.try_integer()) {
        (Some(a), Some(b)) => WofValue::integer(a.wrapping_add(b)),
        _ => WofValue::double(a.as_numeric()? + b.as_numeric()?),
    };

    interp.push(result);
    Ok(())
}

fn op_sub(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop()?;
    let a = interp.stack_mut().pop()?;

    let result = match (a.try_integer(), b.try_integer()) {
        (Some(a), Some(b)) => WofValue::integer(a.wrapping_sub(b)),
        _ => WofValue::double(a.as_numeric()? - b.as_numeric()?),
    };

    interp.push(result);
    Ok(())
}

fn op_mul(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop()?;
    let a = interp.stack_mut().pop()?;

    let result = match (a.try_integer(), b.try_integer()) {
        (Some(a), Some(b)) => WofValue::integer(a.wrapping_mul(b)),
        _ => WofValue::double(a.as_numeric()? * b.as_numeric()?),
    };

    interp.push(result);
    Ok(())
}

fn op_div(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_numeric()?;
    let a = interp.stack_mut().pop_numeric()?;

    if b == 0.0 {
        return Err(WofError::DivisionByZero);
    }

    interp.push(WofValue::double(a / b));
    Ok(())
}

fn op_mod(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop()?;
    let a = interp.stack_mut().pop()?;

    let result = match (a.try_integer(), b.try_integer()) {
        (Some(a), Some(b)) => {
            if b == 0 {
                return Err(WofError::DivisionByZero);
            }
            WofValue::integer(a % b)
        }
        _ => {
            let (a, b) = (a.as_numeric()?, b.as_numeric()?);
            if b == 0.0 {
                return Err(WofError::DivisionByZero);
            }
            WofValue::double(a % b)
        }
    };

    interp.push(result);
    Ok(())
}

fn op_neg(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop()?;

    let result = match a.try_integer() {
        Some(n) => WofValue::integer(-n),
        None => WofValue::double(-a.as_numeric()?),
    };

    interp.push(result);
    Ok(())
}

fn op_abs(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop()?;

    let result = match a.try_integer() {
        Some(n) => WofValue::integer(n.abs()),
        None => WofValue::double(a.as_numeric()?.abs()),
    };

    interp.push(result);
    Ok(())
}

fn op_min(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_numeric()?;
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.min(b)));
    Ok(())
}

fn op_max(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_numeric()?;
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.max(b)));
    Ok(())
}

fn op_inc(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop()?;

    let result = match a.try_integer() {
        Some(n) => WofValue::integer(n.wrapping_add(1)),
        None => WofValue::double(a.as_numeric()? + 1.0),
    };

    interp.push(result);
    Ok(())
}

fn op_dec(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop()?;

    let result = match a.try_integer() {
        Some(n) => WofValue::integer(n.wrapping_sub(1)),
        None => WofValue::double(a.as_numeric()? - 1.0),
    };

    interp.push(result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use woflang_core::InterpreterContext;

    fn make_interp() -> Interpreter {
        let mut interp = Interpreter::new();
        register(&mut interp);
        interp
    }

    #[test]
    fn test_add() {
        let mut interp = make_interp();
        interp.exec_line("5 3 +").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 8);
    }

    #[test]
    fn test_sub() {
        let mut interp = make_interp();
        interp.exec_line("10 4 -").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 6);
    }

    #[test]
    fn test_mul() {
        let mut interp = make_interp();
        interp.exec_line("6 7 *").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 42);
    }

    #[test]
    fn test_div() {
        let mut interp = make_interp();
        interp.exec_line("20 4 /").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_div_by_zero() {
        let mut interp = make_interp();
        let result = interp.exec_line("10 0 /");
        assert!(matches!(result, Err(WofError::DivisionByZero)));
    }

    #[test]
    fn test_mod() {
        let mut interp = make_interp();
        interp.exec_line("17 5 %").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 2);
    }

    #[test]
    fn test_neg() {
        let mut interp = make_interp();
        interp.exec_line("42 neg").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), -42);
    }

    #[test]
    fn test_abs() {
        let mut interp = make_interp();
        interp.exec_line("-42 abs").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 42);
    }

    #[test]
    fn test_float_arithmetic() {
        let mut interp = make_interp();
        interp.exec_line("3.5 2.5 +").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_unicode_mul() {
        let mut interp = make_interp();
        interp.exec_line("6 7 ×").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 42);
    }
}
