//! Basic mathematical operations for Woflang.
//!
//! Provides floor, ceil, round, abs, sign, min, max, clamp, and related functions.
//! Note: Core arithmetic (+, -, *, /, %) is in woflang-ops.

use woflang_core::WofValue;
use woflang_runtime::Interpreter;

/// Register basic math operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // ROUNDING
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("floor", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.floor()));
        Ok(())
    });

    interp.register("⌊", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.floor()));
        Ok(())
    });

    interp.register("ceil", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.ceil()));
        Ok(())
    });

    interp.register("⌈", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.ceil()));
        Ok(())
    });

    interp.register("round", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.round()));
        Ok(())
    });

    interp.register("trunc", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.trunc()));
        Ok(())
    });

    interp.register("fract", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.fract()));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // SIGN AND ABSOLUTE VALUE
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("abs", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.abs()));
        Ok(())
    });

    interp.register("sign", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        let s = if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { 0.0 };
        interp.stack_mut().push(WofValue::double(s));
        Ok(())
    });

    interp.register("signum", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.signum()));
        Ok(())
    });

    interp.register("copysign", |interp| {
        let sign = interp.stack_mut().pop()?.as_float()?;
        let mag = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(mag.copysign(sign)));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // MIN/MAX/CLAMP
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("min", |interp| {
        let b = interp.stack_mut().pop()?.as_float()?;
        let a = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(a.min(b)));
        Ok(())
    });

    interp.register("max", |interp| {
        let b = interp.stack_mut().pop()?.as_float()?;
        let a = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(a.max(b)));
        Ok(())
    });

    // clamp: x lo hi → x clamped to [lo, hi]
    interp.register("clamp", |interp| {
        let hi = interp.stack_mut().pop()?.as_float()?;
        let lo = interp.stack_mut().pop()?.as_float()?;
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(x.clamp(lo, hi)));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // MODULAR ARITHMETIC
    // ═══════════════════════════════════════════════════════════════
    
    // Euclidean remainder (always positive)
    interp.register("rem_euclid", |interp| {
        let b = interp.stack_mut().pop()?.as_float()?;
        let a = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(a.rem_euclid(b)));
        Ok(())
    });

    // Euclidean division
    interp.register("div_euclid", |interp| {
        let b = interp.stack_mut().pop()?.as_float()?;
        let a = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(a.div_euclid(b)));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // SPECIAL VALUES
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("∞", |interp| {
        interp.stack_mut().push(WofValue::double(f64::INFINITY));
        Ok(())
    });

    interp.register("inf", |interp| {
        interp.stack_mut().push(WofValue::double(f64::INFINITY));
        Ok(())
    });

    interp.register("-∞", |interp| {
        interp.stack_mut().push(WofValue::double(f64::NEG_INFINITY));
        Ok(())
    });

    interp.register("nan", |interp| {
        interp.stack_mut().push(WofValue::double(f64::NAN));
        Ok(())
    });

    interp.register("is_nan", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::integer(if x.is_nan() { 1 } else { 0 }));
        Ok(())
    });

    interp.register("is_inf", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::integer(if x.is_infinite() { 1 } else { 0 }));
        Ok(())
    });

    interp.register("is_finite", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::integer(if x.is_finite() { 1 } else { 0 }));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // INTERPOLATION
    // ═══════════════════════════════════════════════════════════════
    
    // Linear interpolation: a b t → a + (b - a) * t
    interp.register("lerp", |interp| {
        let t = interp.stack_mut().pop()?.as_float()?;
        let b = interp.stack_mut().pop()?.as_float()?;
        let a = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::double(a + (b - a) * t));
        Ok(())
    });

    // Inverse lerp: a b x → (x - a) / (b - a)
    interp.register("invlerp", |interp| {
        let x = interp.stack_mut().pop()?.as_float()?;
        let b = interp.stack_mut().pop()?.as_float()?;
        let a = interp.stack_mut().pop()?.as_float()?;
        let denom = b - a;
        if denom == 0.0 {
            interp.stack_mut().push(WofValue::double(0.0));
        } else {
            interp.stack_mut().push(WofValue::double((x - a) / denom));
        }
        Ok(())
    });
}
