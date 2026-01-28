//! Exponential and logarithmic operations for Woflang.
//!
//! Provides exp, ln, log, log10, log2, pow, sqrt, cbrt, and related functions.

use woflang_core::{InterpreterContext, WofError, WofValue};
use woflang_runtime::Interpreter;

/// Register exponential/logarithmic operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // EXPONENTIALS
    // ═══════════════════════════════════════════════════════════════
    
    // e^x
    interp.register("exp", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.exp()));
        Ok(())
    });

    // 2^x
    interp.register("exp2", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.exp2()));
        Ok(())
    });

    // e^x - 1 (more accurate for small x)
    interp.register("expm1", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.exp_m1()));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // LOGARITHMS
    // ═══════════════════════════════════════════════════════════════
    
    // Natural log (ln)
    interp.register("ln", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        if x <= 0.0 {
            return Err(WofError::Runtime("ln: domain error, x must be > 0".into()));
        }
        interp.stack_mut().push(WofValue::double(x.ln()));
        Ok(())
    });

    // log is alias for ln
    interp.register("log", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        if x <= 0.0 {
            return Err(WofError::Runtime("log: domain error, x must be > 0".into()));
        }
        interp.stack_mut().push(WofValue::double(x.ln()));
        Ok(())
    });

    // Base-10 logarithm
    interp.register("log10", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        if x <= 0.0 {
            return Err(WofError::Runtime("log10: domain error, x must be > 0".into()));
        }
        interp.stack_mut().push(WofValue::double(x.log10()));
        Ok(())
    });

    // Base-2 logarithm
    interp.register("log2", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        if x <= 0.0 {
            return Err(WofError::Runtime("log2: domain error, x must be > 0".into()));
        }
        interp.stack_mut().push(WofValue::double(x.log2()));
        Ok(())
    });

    // ln(1 + x) (more accurate for small x)
    interp.register("ln1p", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        if x <= -1.0 {
            return Err(WofError::Runtime("ln1p: domain error, x must be > -1".into()));
        }
        interp.stack_mut().push(WofValue::double(x.ln_1p()));
        Ok(())
    });

    // Arbitrary base logarithm: log_b(x) where b is base
    // Stack: x b → log_b(x)
    interp.register("logb", |interp| {
        let base = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;
        if x <= 0.0 || base <= 0.0 || base == 1.0 {
            return Err(WofError::Runtime("logb: domain error".into()));
        }
        interp.stack_mut().push(WofValue::double(x.log(base)));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // POWERS AND ROOTS
    // ═══════════════════════════════════════════════════════════════
    
    // x^y (power)
    interp.register("pow", |interp| {
        let y = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.powf(y)));
        Ok(())
    });

    // Square root
    interp.register("√", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        if x < 0.0 {
            return Err(WofError::Runtime("√: domain error, x must be >= 0".into()));
        }
        interp.stack_mut().push(WofValue::double(x.sqrt()));
        Ok(())
    });

    interp.register("sqrt", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        if x < 0.0 {
            return Err(WofError::Runtime("sqrt: domain error, x must be >= 0".into()));
        }
        interp.stack_mut().push(WofValue::double(x.sqrt()));
        Ok(())
    });

    // Cube root (works for negative numbers)
    interp.register("∛", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.cbrt()));
        Ok(())
    });

    interp.register("cbrt", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.cbrt()));
        Ok(())
    });

    // Nth root: x^(1/n)
    // Stack: x n → x^(1/n)
    interp.register("nroot", |interp| {
        let n = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;
        if n == 0.0 {
            return Err(WofError::Runtime("nroot: n cannot be 0".into()));
        }
        interp.stack_mut().push(WofValue::double(x.powf(1.0 / n)));
        Ok(())
    });

    // Hypotenuse: sqrt(x^2 + y^2)
    interp.register("hypot", |interp| {
        let y = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.hypot(y)));
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Interpreter {
        let mut interp = Interpreter::new();
        register(&mut interp);
        interp
    }

    #[test]
    fn test_exp() {
        let mut interp = setup();
        interp.exec_line("0 exp").unwrap();
        let result = interp.stack().peek().unwrap().as_float().unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln() {
        let mut interp = setup();
        interp.exec_line("1 ln").unwrap();
        let result = interp.stack().peek().unwrap().as_float().unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt() {
        let mut interp = setup();
        interp.exec_line("4 sqrt").unwrap();
        let result = interp.stack().peek().unwrap().as_float().unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }
}
