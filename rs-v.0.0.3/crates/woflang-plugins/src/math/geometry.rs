//! 2D geometric transforms and coordinate conversions.
//!
//! ## Operations
//!
//! - `translate2d` - Translate a point (x y dx dy → x' y')
//! - `scale2d` - Scale a point (x y sx sy → x' y')
//! - `rotate2d_rad` - Rotate by radians (x y theta → x' y')
//! - `rotate2d_deg` - Rotate by degrees (x y theta → x' y')
//! - `cart_to_polar` - Cartesian to polar (x y → r theta)
//! - `polar_to_cart` - Polar to cartesian (r theta → x y)

use std::f64::consts::PI;
use woflang_core::WofValue;
use woflang_runtime::Interpreter;

/// Register all geometry operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // 2D TRANSFORMS
    // ─────────────────────────────────────────────────────────────────────

    // Translate: x y dx dy → x+dx y+dy
    interp.register("translate2d", |interp| {
        let dy = interp.stack_mut().pop()?.as_float()?;
        let dx = interp.stack_mut().pop()?.as_float()?;
        let y = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;

        interp.stack_mut().push(WofValue::Float(x + dx));
        interp.stack_mut().push(WofValue::Float(y + dy));
        Ok(())
    });

    // Scale: x y sx sy → x*sx y*sy
    interp.register("scale2d", |interp| {
        let sy = interp.stack_mut().pop()?.as_float()?;
        let sx = interp.stack_mut().pop()?.as_float()?;
        let y = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;

        interp.stack_mut().push(WofValue::Float(x * sx));
        interp.stack_mut().push(WofValue::Float(y * sy));
        Ok(())
    });

    // Rotate by radians: x y theta → x' y'
    interp.register("rotate2d_rad", |interp| {
        let theta = interp.stack_mut().pop()?.as_float()?;
        let y = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;

        let c = theta.cos();
        let s = theta.sin();

        let x_new = x * c - y * s;
        let y_new = x * s + y * c;

        interp.stack_mut().push(WofValue::Float(x_new));
        interp.stack_mut().push(WofValue::Float(y_new));
        Ok(())
    });

    // Rotate by degrees: x y theta_deg → x' y'
    interp.register("rotate2d_deg", |interp| {
        let theta_deg = interp.stack_mut().pop()?.as_float()?;
        let y = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;

        let theta = theta_deg * (PI / 180.0);
        let c = theta.cos();
        let s = theta.sin();

        let x_new = x * c - y * s;
        let y_new = x * s + y * c;

        interp.stack_mut().push(WofValue::Float(x_new));
        interp.stack_mut().push(WofValue::Float(y_new));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // COORDINATE CONVERSIONS
    // ─────────────────────────────────────────────────────────────────────

    // Cartesian to polar: x y → r theta_rad
    interp.register("cart_to_polar", |interp| {
        let y = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;

        let r = (x * x + y * y).sqrt();
        let theta = y.atan2(x);

        interp.stack_mut().push(WofValue::Float(r));
        interp.stack_mut().push(WofValue::Float(theta));
        Ok(())
    });

    // Polar to cartesian: r theta_rad → x y
    interp.register("polar_to_cart", |interp| {
        let theta = interp.stack_mut().pop()?.as_float()?;
        let r = interp.stack_mut().pop()?.as_float()?;

        let x = r * theta.cos();
        let y = r * theta.sin();

        interp.stack_mut().push(WofValue::Float(x));
        interp.stack_mut().push(WofValue::Float(y));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // VECTOR OPERATIONS
    // ─────────────────────────────────────────────────────────────────────

    // Vector magnitude: x y → |v|
    interp.register("vec2_mag", |interp| {
        let y = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::Float((x * x + y * y).sqrt()));
        Ok(())
    });

    // Dot product: x1 y1 x2 y2 → x1*x2 + y1*y2
    interp.register("vec2_dot", |interp| {
        let y2 = interp.stack_mut().pop()?.as_float()?;
        let x2 = interp.stack_mut().pop()?.as_float()?;
        let y1 = interp.stack_mut().pop()?.as_float()?;
        let x1 = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::Float(x1 * x2 + y1 * y2));
        Ok(())
    });

    // Normalize: x y → x/|v| y/|v|
    interp.register("vec2_normalize", |interp| {
        let y = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;
        let mag = (x * x + y * y).sqrt();

        if mag > f64::EPSILON {
            interp.stack_mut().push(WofValue::Float(x / mag));
            interp.stack_mut().push(WofValue::Float(y / mag));
        } else {
            interp.stack_mut().push(WofValue::Float(0.0));
            interp.stack_mut().push(WofValue::Float(0.0));
        }
        Ok(())
    });

    // Distance between two points: x1 y1 x2 y2 → distance
    interp.register("vec2_dist", |interp| {
        let y2 = interp.stack_mut().pop()?.as_float()?;
        let x2 = interp.stack_mut().pop()?.as_float()?;
        let y1 = interp.stack_mut().pop()?.as_float()?;
        let x1 = interp.stack_mut().pop()?.as_float()?;

        let dx = x2 - x1;
        let dy = y2 - y1;
        interp.stack_mut().push(WofValue::Float((dx * dx + dy * dy).sqrt()));
        Ok(())
    });

    // Lerp (linear interpolation): x1 y1 x2 y2 t → x y
    interp.register("vec2_lerp", |interp| {
        let t = interp.stack_mut().pop()?.as_float()?;
        let y2 = interp.stack_mut().pop()?.as_float()?;
        let x2 = interp.stack_mut().pop()?.as_float()?;
        let y1 = interp.stack_mut().pop()?.as_float()?;
        let x1 = interp.stack_mut().pop()?.as_float()?;

        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);

        interp.stack_mut().push(WofValue::Float(x));
        interp.stack_mut().push(WofValue::Float(y));
        Ok(())
    });
}
