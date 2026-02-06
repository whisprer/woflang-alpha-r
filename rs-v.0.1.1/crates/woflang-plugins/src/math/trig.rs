//! Trigonometric operations for Woflang.
//!
//! Provides sin, cos, tan, their inverses, hyperbolic variants,
//! and degree/radian conversion utilities.

use woflang_core::{WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

/// Register trigonometric operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // CONSTANTS
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("π", |interp| {
        interp.stack_mut().push(WofValue::double(std::f64::consts::PI));
        Ok(())
    });
    
    interp.register("τ", |interp| {
        interp.stack_mut().push(WofValue::double(std::f64::consts::TAU));
        Ok(())
    });
    
    interp.register("ℯ", |interp| {
        interp.stack_mut().push(WofValue::double(std::f64::consts::E));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // BASIC TRIG (radians)
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("sin", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.sin()));
        Ok(())
    });

    interp.register("cos", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.cos()));
        Ok(())
    });

    interp.register("tan", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.tan()));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // INVERSE TRIG
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("asin", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.asin()));
        Ok(())
    });

    interp.register("acos", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.acos()));
        Ok(())
    });

    interp.register("atan", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.atan()));
        Ok(())
    });

    interp.register("atan2", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        let y = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(y.atan2(x)));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // HYPERBOLIC
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("sinh", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.sinh()));
        Ok(())
    });

    interp.register("cosh", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.cosh()));
        Ok(())
    });

    interp.register("tanh", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.tanh()));
        Ok(())
    });

    // Inverse hyperbolic
    interp.register("asinh", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.asinh()));
        Ok(())
    });

    interp.register("acosh", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.acosh()));
        Ok(())
    });

    interp.register("atanh", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(x.atanh()));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // DEGREE/RADIAN CONVERSION
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("deg→rad", |interp| {
        let deg = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(deg.to_radians()));
        Ok(())
    });

    interp.register("rad→deg", |interp| {
        let rad = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(rad.to_degrees()));
        Ok(())
    });

    // ASCII aliases
    interp.register("deg2rad", |interp| {
        let deg = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(deg.to_radians()));
        Ok(())
    });

    interp.register("rad2deg", |interp| {
        let rad = interp.stack_mut().pop()?.as_double()?;
        interp.stack_mut().push(WofValue::double(rad.to_degrees()));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // SINCOS (returns both sin and cos)
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("sincos", |interp| {
        let x = interp.stack_mut().pop()?.as_double()?;
        let (sin_x, cos_x) = x.sin_cos();
        interp.stack_mut().push(WofValue::double(sin_x));
        interp.stack_mut().push(WofValue::double(cos_x));
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn setup() -> Interpreter {
        let mut interp = Interpreter::new();
        register(&mut interp);
        interp
    }

    #[test]
    fn test_sin() {
        let mut interp = setup();
        interp.exec_line("0 sin").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_cos() {
        let mut interp = setup();
        interp.exec_line("0 cos").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_pi() {
        let mut interp = setup();
        interp.exec_line("π").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - PI).abs() < 1e-10);
    }
}
