//! Calculus operations for Woflang.
//!
//! Provides numerical differentiation (finite differences) and
//! basic integration helpers.

use woflang_core::{WofError, WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

/// Register calculus operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // NUMERICAL DIFFERENTIATION (Finite Differences)
    // ═══════════════════════════════════════════════════════════════
    
    // Central difference: df/dx ≈ (f(x+h) - f(x-h)) / (2h)
    // Stack: f(x+h) f(x-h) h → derivative
    interp.register("derivative_central", |interp| {
        let h = interp.stack_mut().pop()?.as_double()?;
        let f_minus = interp.stack_mut().pop()?.as_double()?;
        let f_plus = interp.stack_mut().pop()?.as_double()?;
        
        if h.abs() <= f64::EPSILON {
            return Err(WofError::Runtime("derivative_central: h too small".into()));
        }
        
        let deriv = (f_plus - f_minus) / (2.0 * h);
        interp.stack_mut().push(WofValue::double(deriv));
        Ok(())
    });

    // Forward difference: df/dx ≈ (f(x+h) - f(x)) / h
    // Stack: f(x+h) f(x) h → derivative
    interp.register("derivative_forward", |interp| {
        let h = interp.stack_mut().pop()?.as_double()?;
        let f_x = interp.stack_mut().pop()?.as_double()?;
        let f_plus = interp.stack_mut().pop()?.as_double()?;
        
        if h.abs() <= f64::EPSILON {
            return Err(WofError::Runtime("derivative_forward: h too small".into()));
        }
        
        let deriv = (f_plus - f_x) / h;
        interp.stack_mut().push(WofValue::double(deriv));
        Ok(())
    });

    // Backward difference: df/dx ≈ (f(x) - f(x-h)) / h
    // Stack: f(x) f(x-h) h → derivative
    interp.register("derivative_backward", |interp| {
        let h = interp.stack_mut().pop()?.as_double()?;
        let f_minus = interp.stack_mut().pop()?.as_double()?;
        let f_x = interp.stack_mut().pop()?.as_double()?;
        
        if h.abs() <= f64::EPSILON {
            return Err(WofError::Runtime("derivative_backward: h too small".into()));
        }
        
        let deriv = (f_x - f_minus) / h;
        interp.stack_mut().push(WofValue::double(deriv));
        Ok(())
    });

    // Second derivative (central): d²f/dx² ≈ (f(x+h) - 2f(x) + f(x-h)) / h²
    // Stack: f(x+h) f(x) f(x-h) h → second derivative
    interp.register("derivative2_central", |interp| {
        let h = interp.stack_mut().pop()?.as_double()?;
        let f_minus = interp.stack_mut().pop()?.as_double()?;
        let f_x = interp.stack_mut().pop()?.as_double()?;
        let f_plus = interp.stack_mut().pop()?.as_double()?;
        
        if h.abs() <= f64::EPSILON {
            return Err(WofError::Runtime("derivative2_central: h too small".into()));
        }
        
        let deriv2 = (f_plus - 2.0 * f_x + f_minus) / (h * h);
        interp.stack_mut().push(WofValue::double(deriv2));
        Ok(())
    });

    // Partial derivative glyph alias
    interp.register("∂", |interp| {
        // Same as derivative_central
        let h = interp.stack_mut().pop()?.as_double()?;
        let f_minus = interp.stack_mut().pop()?.as_double()?;
        let f_plus = interp.stack_mut().pop()?.as_double()?;
        
        if h.abs() <= f64::EPSILON {
            return Err(WofError::Runtime("∂: h too small".into()));
        }
        
        let deriv = (f_plus - f_minus) / (2.0 * h);
        interp.stack_mut().push(WofValue::double(deriv));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // SLOPE
    // ═══════════════════════════════════════════════════════════════
    
    // Slope between two points: (y2 - y1) / (x2 - x1)
    // Stack: y2 y1 x2 x1 → slope
    interp.register("slope", |interp| {
        let x1 = interp.stack_mut().pop()?.as_double()?;
        let x2 = interp.stack_mut().pop()?.as_double()?;
        let y1 = interp.stack_mut().pop()?.as_double()?;
        let y2 = interp.stack_mut().pop()?.as_double()?;
        
        let dx = x2 - x1;
        if dx.abs() <= f64::EPSILON {
            interp.stack_mut().push(WofValue::double(f64::INFINITY));
        } else {
            interp.stack_mut().push(WofValue::double((y2 - y1) / dx));
        }
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // NUMERICAL INTEGRATION
    // ═══════════════════════════════════════════════════════════════
    
    // Trapezoidal rule for single interval: (f(a) + f(b)) * h / 2
    // Stack: f(a) f(b) h → integral
    interp.register("trapezoid", |interp| {
        let h = interp.stack_mut().pop()?.as_double()?;
        let f_b = interp.stack_mut().pop()?.as_double()?;
        let f_a = interp.stack_mut().pop()?.as_double()?;
        
        let integral = (f_a + f_b) * h / 2.0;
        interp.stack_mut().push(WofValue::double(integral));
        Ok(())
    });

    // Simpson's rule for single interval: (f(a) + 4*f(m) + f(b)) * h / 6
    // Stack: f(a) f(m) f(b) h → integral
    interp.register("simpson", |interp| {
        let h = interp.stack_mut().pop()?.as_double()?;
        let f_b = interp.stack_mut().pop()?.as_double()?;
        let f_m = interp.stack_mut().pop()?.as_double()?;
        let f_a = interp.stack_mut().pop()?.as_double()?;
        
        let integral = (f_a + 4.0 * f_m + f_b) * h / 6.0;
        interp.stack_mut().push(WofValue::double(integral));
        Ok(())
    });

    // Integral glyph
    interp.register("∫", |interp| {
        // Same as trapezoid for simple case
        let h = interp.stack_mut().pop()?.as_double()?;
        let f_b = interp.stack_mut().pop()?.as_double()?;
        let f_a = interp.stack_mut().pop()?.as_double()?;
        
        let integral = (f_a + f_b) * h / 2.0;
        interp.stack_mut().push(WofValue::double(integral));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // SUMMATION
    // ═══════════════════════════════════════════════════════════════
    
    // Summation glyph - sum top n items on stack
    // Stack: x1 x2 ... xn n → sum
    interp.register("∑", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()? as usize;
        let mut sum = 0.0;
        
        for _ in 0..n {
            sum += interp.stack_mut().pop()?.as_double()?;
        }
        
        interp.stack_mut().push(WofValue::double(sum));
        Ok(())
    });

    // Product glyph - multiply top n items on stack
    // Stack: x1 x2 ... xn n → product
    interp.register("∏", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()? as usize;
        let mut product = 1.0;
        
        for _ in 0..n {
            product *= interp.stack_mut().pop()?.as_double()?;
        }
        
        interp.stack_mut().push(WofValue::double(product));
        Ok(())
    });
}
