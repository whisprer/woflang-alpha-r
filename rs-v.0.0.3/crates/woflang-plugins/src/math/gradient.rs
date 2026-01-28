//! Numerical gradient and Hessian computation.
//!
//! Uses central finite differences for 2D functions.
//!
//! ## Operations
//!
//! - `grad2_central` - 2D gradient via central differences
//! - `hess2_central` - 2D Hessian via central differences
//! - `diff_forward` - Forward difference
//! - `diff_backward` - Backward difference
//! - `diff_central` - Central difference
//! - `diff_second` - Second derivative

use woflang_core::{InterpreterContext, WofError, WofValue};
use woflang_runtime::Interpreter;

/// Register all gradient/differentiation operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // 2D GRADIENT
    // ─────────────────────────────────────────────────────────────────────

    // 2D gradient via central differences
    // Stack: f(x-h,y) f(x+h,y) f(x,y-h) f(x,y+h) h → grad_x grad_y
    interp.register("grad2_central", |interp| {
        let h = interp.stack_mut().pop()?.as_float()?;
        let f_x_yph = interp.stack_mut().pop()?.as_float()?;  // f(x, y+h)
        let f_x_ymh = interp.stack_mut().pop()?.as_float()?;  // f(x, y-h)
        let f_xph_y = interp.stack_mut().pop()?.as_float()?;  // f(x+h, y)
        let f_xmh_y = interp.stack_mut().pop()?.as_float()?;  // f(x-h, y)

        if h.abs() < f64::EPSILON {
            return Err(WofError::Runtime("grad2_central: step h must be non-zero".to_string()));
        }

        let gx = (f_xph_y - f_xmh_y) / (2.0 * h);
        let gy = (f_x_yph - f_x_ymh) / (2.0 * h);

        interp.stack_mut().push(WofValue::Float(gx));
        interp.stack_mut().push(WofValue::Float(gy));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // 2D HESSIAN
    // ─────────────────────────────────────────────────────────────────────

    // 2D Hessian via central differences
    // Stack: f(x-h,y-h) f(x-h,y) f(x-h,y+h) f(x,y-h) f(x,y) f(x,y+h)
    //        f(x+h,y-h) f(x+h,y) f(x+h,y+h) h
    // → f_xx f_yy f_xy
    interp.register("hess2_central", |interp| {
        let h = interp.stack_mut().pop()?.as_float()?;
        let f_xph_yph = interp.stack_mut().pop()?.as_float()?;  // f(x+h, y+h)
        let f_xph_y = interp.stack_mut().pop()?.as_float()?;    // f(x+h, y)
        let f_xph_ymh = interp.stack_mut().pop()?.as_float()?;  // f(x+h, y-h)
        let f_x_yph = interp.stack_mut().pop()?.as_float()?;    // f(x, y+h)
        let f_x_y = interp.stack_mut().pop()?.as_float()?;      // f(x, y)
        let f_x_ymh = interp.stack_mut().pop()?.as_float()?;    // f(x, y-h)
        let f_xmh_yph = interp.stack_mut().pop()?.as_float()?;  // f(x-h, y+h)
        let f_xmh_y = interp.stack_mut().pop()?.as_float()?;    // f(x-h, y)
        let f_xmh_ymh = interp.stack_mut().pop()?.as_float()?;  // f(x-h, y-h)

        if h.abs() < f64::EPSILON {
            return Err(WofError::Runtime("hess2_central: step h must be non-zero".to_string()));
        }

        let h2 = h * h;
        let invh2 = 1.0 / h2;

        let f_xx = (f_xph_y - 2.0 * f_x_y + f_xmh_y) * invh2;
        let f_yy = (f_x_yph - 2.0 * f_x_y + f_x_ymh) * invh2;
        let f_xy = (f_xph_yph - f_xph_ymh - f_xmh_yph + f_xmh_ymh) / (4.0 * h2);

        interp.stack_mut().push(WofValue::Float(f_xx));
        interp.stack_mut().push(WofValue::Float(f_yy));
        interp.stack_mut().push(WofValue::Float(f_xy));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // 1D FINITE DIFFERENCES
    // ─────────────────────────────────────────────────────────────────────

    // Forward difference: f(x) f(x+h) h → f'(x)
    // Formula: (f(x+h) - f(x)) / h
    interp.register("diff_forward", |interp| {
        let h = interp.stack_mut().pop()?.as_float()?;
        let f_xph = interp.stack_mut().pop()?.as_float()?;
        let f_x = interp.stack_mut().pop()?.as_float()?;

        if h.abs() < f64::EPSILON {
            return Err(WofError::Runtime("diff_forward: step h must be non-zero".to_string()));
        }

        let deriv = (f_xph - f_x) / h;
        interp.stack_mut().push(WofValue::Float(deriv));
        Ok(())
    });

    // Backward difference: f(x-h) f(x) h → f'(x)
    // Formula: (f(x) - f(x-h)) / h
    interp.register("diff_backward", |interp| {
        let h = interp.stack_mut().pop()?.as_float()?;
        let f_x = interp.stack_mut().pop()?.as_float()?;
        let f_xmh = interp.stack_mut().pop()?.as_float()?;

        if h.abs() < f64::EPSILON {
            return Err(WofError::Runtime("diff_backward: step h must be non-zero".to_string()));
        }

        let deriv = (f_x - f_xmh) / h;
        interp.stack_mut().push(WofValue::Float(deriv));
        Ok(())
    });

    // Central difference: f(x-h) f(x+h) h → f'(x)
    // Formula: (f(x+h) - f(x-h)) / (2h)
    interp.register("diff_central", |interp| {
        let h = interp.stack_mut().pop()?.as_float()?;
        let f_xph = interp.stack_mut().pop()?.as_float()?;
        let f_xmh = interp.stack_mut().pop()?.as_float()?;

        if h.abs() < f64::EPSILON {
            return Err(WofError::Runtime("diff_central: step h must be non-zero".to_string()));
        }

        let deriv = (f_xph - f_xmh) / (2.0 * h);
        interp.stack_mut().push(WofValue::Float(deriv));
        Ok(())
    });

    // Second derivative: f(x-h) f(x) f(x+h) h → f''(x)
    // Formula: (f(x+h) - 2f(x) + f(x-h)) / h²
    interp.register("diff_second", |interp| {
        let h = interp.stack_mut().pop()?.as_float()?;
        let f_xph = interp.stack_mut().pop()?.as_float()?;
        let f_x = interp.stack_mut().pop()?.as_float()?;
        let f_xmh = interp.stack_mut().pop()?.as_float()?;

        if h.abs() < f64::EPSILON {
            return Err(WofError::Runtime("diff_second: step h must be non-zero".to_string()));
        }

        let second = (f_xph - 2.0 * f_x + f_xmh) / (h * h);
        interp.stack_mut().push(WofValue::Float(second));
        Ok(())
    });

    // Dot-notation aliases for the diff operations
    interp.register("diff.forward", |interp| {
        let h = interp.stack_mut().pop()?.as_float()?;
        let f_xph = interp.stack_mut().pop()?.as_float()?;
        let f_x = interp.stack_mut().pop()?.as_float()?;
        if h.abs() < f64::EPSILON {
            return Err(WofError::Runtime("diff.forward: step h must be non-zero".to_string()));
        }
        interp.stack_mut().push(WofValue::Float((f_xph - f_x) / h));
        Ok(())
    });

    interp.register("diff.backward", |interp| {
        let h = interp.stack_mut().pop()?.as_float()?;
        let f_x = interp.stack_mut().pop()?.as_float()?;
        let f_xmh = interp.stack_mut().pop()?.as_float()?;
        if h.abs() < f64::EPSILON {
            return Err(WofError::Runtime("diff.backward: step h must be non-zero".to_string()));
        }
        interp.stack_mut().push(WofValue::Float((f_x - f_xmh) / h));
        Ok(())
    });

    interp.register("diff.central", |interp| {
        let h = interp.stack_mut().pop()?.as_float()?;
        let f_xph = interp.stack_mut().pop()?.as_float()?;
        let f_xmh = interp.stack_mut().pop()?.as_float()?;
        if h.abs() < f64::EPSILON {
            return Err(WofError::Runtime("diff.central: step h must be non-zero".to_string()));
        }
        interp.stack_mut().push(WofValue::Float((f_xph - f_xmh) / (2.0 * h)));
        Ok(())
    });
}
