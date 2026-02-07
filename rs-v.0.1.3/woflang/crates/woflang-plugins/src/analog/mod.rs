//! Analog Computing Plugin Registration
//!
//! Wires the `woflang-analog` crate's operations into the WofLang interpreter.
//! All ops live in the 7000-7999 opcode space and use bounded continuum arithmetic
//! where values saturate at boundaries instead of overflowing.
//!
//! # Glyph Design
//!
//! Each operation has both a Unicode glyph and an ASCII alias:
//!
//! | Glyph | ASCII | Description |
//! |-------|-------|-------------|
//! | `⊞`   | `a+`  | Analog add |
//! | `⊟`   | `a-`  | Analog sub |
//! | `⊠`   | `a*`  | Analog mul |
//! | `⊘`   | `a/`  | Analog div |
//! | `≋`   | `a~`  | Analog status |
//! | `⌇`   | `a.clamp` | Clamp to range |
//!
//! # Stack Effects
//!
//! Operations follow standard stack convention:
//! - Unary: `( a -- result )`
//! - Binary: `( a b -- result )` where `a` is deeper, `b` on top
//! - Ternary: `( a b c -- result )` e.g., lerp takes `(start end t)`

use woflang_analog::ops;
use woflang_core::InterpreterContext;
use woflang_runtime::Interpreter;

/// Register all analog computing operations with the interpreter.
pub fn register(interp: &mut Interpreter) {
    register_mode_control(interp);
    register_basic_math(interp);
    register_trig(interp);
    register_linear_2d(interp);
    register_linear_3d(interp);
    register_coordinate(interp);
}

// ═══════════════════════════════════════════════════════════════════════════
// MODE CONTROL (7000-7009)
// ═══════════════════════════════════════════════════════════════════════════

fn register_mode_control(interp: &mut Interpreter) {
    // 7000: Status — push mode description string
    for name in ["≋", "a.status", "analog_status"] {
        interp.register(name, |ctx| {
            ctx.push(ops::op_analog_status());
            Ok(())
        });
    }

    // 7001: Set INT_201 mode (-100 to +100)
    for name in ["a.201", "analog_201", "analog_int201"] {
        interp.register(name, |_ctx| {
            ops::op_analog_mode_int201();
            Ok(())
        });
    }

    // 7002: Set INT_2001 mode (-1000 to +1000)
    for name in ["a.2001", "analog_2001", "analog_int2001"] {
        interp.register(name, |_ctx| {
            ops::op_analog_mode_int2001();
            Ok(())
        });
    }

    // 7003: Set FLOAT_UNIT mode (-1.0 to +1.0)
    for name in ["a.unit", "analog_unit", "analog_float"] {
        interp.register(name, |_ctx| {
            ops::op_analog_mode_float_unit();
            Ok(())
        });
    }

    // 7004: Set custom mode ( min max -- )
    for name in ["a.custom", "analog_custom"] {
        interp.register(name, |ctx| {
            let max = ctx.pop()?;
            let min = ctx.pop()?;
            ops::op_analog_mode_custom(&min, &max)?;
            Ok(())
        });
    }

    // 7005: Reset to default mode
    for name in ["a.reset", "analog_reset"] {
        interp.register(name, |_ctx| {
            ops::op_analog_reset();
            Ok(())
        });
    }

    // 7006: Push current minimum ( -- min )
    for name in ["a.min", "analog_min"] {
        interp.register(name, |ctx| {
            ctx.push(ops::op_analog_min());
            Ok(())
        });
    }

    // 7007: Push current maximum ( -- max )
    for name in ["a.max", "analog_max"] {
        interp.register(name, |ctx| {
            ctx.push(ops::op_analog_max());
            Ok(())
        });
    }

    // 7008: Push range span ( -- range )
    for name in ["a.range", "analog_range"] {
        interp.register(name, |ctx| {
            ctx.push(ops::op_analog_range());
            Ok(())
        });
    }

    // 7009: Check if integer mode ( -- bool )
    for name in ["a.int?", "analog_is_int"] {
        interp.register(name, |ctx| {
            ctx.push(ops::op_analog_is_int());
            Ok(())
        });
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BASIC MATH (7010-7029)
// ═══════════════════════════════════════════════════════════════════════════

fn register_basic_math(interp: &mut Interpreter) {
    // 7010: Clamp to analog range ( a -- clamped )
    for name in ["⌇", "a.clamp", "analog_clamp"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_clamp(&a)?);
            Ok(())
        });
    }

    // 7011: Analog add ( a b -- a+b )
    for name in ["⊞", "a+", "analog_add"] {
        interp.register(name, |ctx| {
            let b = ctx.pop()?;
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_add(&a, &b)?);
            Ok(())
        });
    }

    // 7012: Analog subtract ( a b -- a-b )
    for name in ["⊟", "a-", "analog_sub"] {
        interp.register(name, |ctx| {
            let b = ctx.pop()?;
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_sub(&a, &b)?);
            Ok(())
        });
    }

    // 7013: Analog multiply ( a b -- a*b )
    for name in ["⊠", "a*", "analog_mul"] {
        interp.register(name, |ctx| {
            let b = ctx.pop()?;
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_mul(&a, &b)?);
            Ok(())
        });
    }

    // 7014: Analog divide ( a b -- a/b )
    for name in ["⊘", "a/", "analog_div"] {
        interp.register(name, |ctx| {
            let b = ctx.pop()?;
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_div(&a, &b)?);
            Ok(())
        });
    }

    // 7015: Analog modulo ( a b -- a%b )
    for name in ["a%", "analog_mod"] {
        interp.register(name, |ctx| {
            let b = ctx.pop()?;
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_mod(&a, &b)?);
            Ok(())
        });
    }

    // 7016: Analog negate ( a -- -a )
    for name in ["a.neg", "analog_neg"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_neg(&a)?);
            Ok(())
        });
    }

    // 7017: Analog absolute value ( a -- |a| )
    for name in ["a.abs", "analog_abs"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_abs(&a)?);
            Ok(())
        });
    }

    // 7018: Analog square root ( a -- √a )
    for name in ["a.sqrt", "analog_sqrt"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_sqrt(&a)?);
            Ok(())
        });
    }

    // 7019: Analog power ( base exp -- base^exp )
    for name in ["a.pow", "analog_pow"] {
        interp.register(name, |ctx| {
            let exp = ctx.pop()?;
            let base = ctx.pop()?;
            ctx.push(ops::op_analog_pow(&base, &exp)?);
            Ok(())
        });
    }

    // 7020: Analog lerp ( start end t -- interpolated )
    for name in ["a.lerp", "analog_lerp"] {
        interp.register(name, |ctx| {
            let t = ctx.pop()?;
            let b = ctx.pop()?;
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_lerp(&a, &b, &t)?);
            Ok(())
        });
    }

    // 7021: Deadzone ( value threshold -- result )
    for name in ["a.dead", "analog_deadzone"] {
        interp.register(name, |ctx| {
            let threshold = ctx.pop()?;
            let value = ctx.pop()?;
            ctx.push(ops::op_analog_deadzone(&value, &threshold)?);
            Ok(())
        });
    }

    // 7022: Remap ( value from_min from_max to_min to_max -- result )
    for name in ["a.remap", "analog_remap"] {
        interp.register(name, |ctx| {
            let to_max = ctx.pop()?;
            let to_min = ctx.pop()?;
            let from_max = ctx.pop()?;
            let from_min = ctx.pop()?;
            let value = ctx.pop()?;
            ctx.push(ops::op_analog_remap(&value, &from_min, &from_max, &to_min, &to_max)?);
            Ok(())
        });
    }

    // 7023: Normalize to [0,1] ( value -- normalized )
    for name in ["a.norm", "analog_normalize"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_normalize(&a)?);
            Ok(())
        });
    }

    // 7024: Denormalize from [0,1] ( normalized -- value )
    for name in ["a.denorm", "analog_denormalize"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_denormalize(&a)?);
            Ok(())
        });
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRIGONOMETRIC OPERATIONS (7030-7049)
// ═══════════════════════════════════════════════════════════════════════════

fn register_trig(interp: &mut Interpreter) {
    // 7030: Analog sine ( radians -- sin )
    for name in ["a.sin", "analog_sin"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_sin(&a)?);
            Ok(())
        });
    }

    // 7031: Analog cosine ( radians -- cos )
    for name in ["a.cos", "analog_cos"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_cos(&a)?);
            Ok(())
        });
    }

    // 7032: Analog tangent ( radians -- tan )
    for name in ["a.tan", "analog_tan"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_tan(&a)?);
            Ok(())
        });
    }

    // 7033: Analog arcsine ( value -- radians )
    for name in ["a.asin", "analog_asin"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_asin(&a)?);
            Ok(())
        });
    }

    // 7034: Analog arccosine ( value -- radians )
    for name in ["a.acos", "analog_acos"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_acos(&a)?);
            Ok(())
        });
    }

    // 7035: Analog arctangent ( value -- radians )
    for name in ["a.atan", "analog_atan"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_atan(&a)?);
            Ok(())
        });
    }

    // 7036: Analog atan2 ( y x -- radians )
    for name in ["a.atan2", "analog_atan2"] {
        interp.register(name, |ctx| {
            let x = ctx.pop()?;
            let y = ctx.pop()?;
            ctx.push(ops::op_analog_atan2(&y, &x)?);
            Ok(())
        });
    }

    // 7037: Analog tanh ( value -- tanh )
    for name in ["a.tanh", "analog_tanh"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_tanh(&a)?);
            Ok(())
        });
    }

    // 7038: Analog exp ( value -- e^value )
    for name in ["a.exp", "analog_exp"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_exp(&a)?);
            Ok(())
        });
    }

    // 7039: Analog ln ( value -- ln(value) )
    for name in ["a.ln", "analog_ln"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_ln(&a)?);
            Ok(())
        });
    }

    // 7040: Degrees to radians ( deg -- rad )
    for name in ["a.d2r", "deg_to_rad"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_deg_to_rad(&a)?);
            Ok(())
        });
    }

    // 7041: Radians to degrees ( rad -- deg )
    for name in ["a.r2d", "rad_to_deg"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_rad_to_deg(&a)?);
            Ok(())
        });
    }

    // 7042: Wrap radians to [0, 2π) ( rad -- wrapped )
    for name in ["a.wrapr", "wrap_radians"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_wrap_radians(&a)?);
            Ok(())
        });
    }

    // 7043: Wrap degrees to [0, 360) ( deg -- wrapped )
    for name in ["a.wrapd", "wrap_degrees"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_wrap_degrees(&a)?);
            Ok(())
        });
    }

    // 7044: Sigmoid activation ( value -- sigmoid )
    for name in ["a.sigmoid", "analog_sigmoid"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_sigmoid(&a)?);
            Ok(())
        });
    }

    // 7045: ReLU activation ( value -- relu )
    for name in ["a.relu", "analog_relu"] {
        interp.register(name, |ctx| {
            let a = ctx.pop()?;
            ctx.push(ops::op_analog_relu(&a)?);
            Ok(())
        });
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LINEAR ALGEBRA 2D (7050-7059)
// ═══════════════════════════════════════════════════════════════════════════

fn register_linear_2d(interp: &mut Interpreter) {
    // 7050: 2D dot product ( x1 y1 x2 y2 -- dot )
    for name in ["a.dot2", "analog_dot2d"] {
        interp.register(name, |ctx| {
            let y2 = ctx.pop()?;
            let x2 = ctx.pop()?;
            let y1 = ctx.pop()?;
            let x1 = ctx.pop()?;
            ctx.push(ops::op_dot_2d(&x1, &y1, &x2, &y2)?);
            Ok(())
        });
    }

    // 7051: 2D magnitude ( x y -- mag )
    for name in ["a.mag2", "analog_mag2d"] {
        interp.register(name, |ctx| {
            let y = ctx.pop()?;
            let x = ctx.pop()?;
            ctx.push(ops::op_magnitude_2d(&x, &y)?);
            Ok(())
        });
    }

    // 7052: 2D distance ( x1 y1 x2 y2 -- dist )
    for name in ["a.dist2", "analog_dist2d"] {
        interp.register(name, |ctx| {
            let y2 = ctx.pop()?;
            let x2 = ctx.pop()?;
            let y1 = ctx.pop()?;
            let x1 = ctx.pop()?;
            ctx.push(ops::op_distance_2d(&x1, &y1, &x2, &y2)?);
            Ok(())
        });
    }

    // 7053: 2D normalize ( x y -- nx ny )
    for name in ["a.norm2", "analog_norm2d"] {
        interp.register(name, |ctx| {
            let y = ctx.pop()?;
            let x = ctx.pop()?;
            let (nx, ny) = ops::op_normalize_2d(&x, &y)?;
            ctx.push(nx);
            ctx.push(ny);
            Ok(())
        });
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LINEAR ALGEBRA 3D (7060-7069)
// ═══════════════════════════════════════════════════════════════════════════

fn register_linear_3d(interp: &mut Interpreter) {
    // 7060: 3D dot product ( x1 y1 z1 x2 y2 z2 -- dot )
    for name in ["a.dot3", "analog_dot3d"] {
        interp.register(name, |ctx| {
            let z2 = ctx.pop()?;
            let y2 = ctx.pop()?;
            let x2 = ctx.pop()?;
            let z1 = ctx.pop()?;
            let y1 = ctx.pop()?;
            let x1 = ctx.pop()?;
            ctx.push(ops::op_dot_3d(&x1, &y1, &z1, &x2, &y2, &z2)?);
            Ok(())
        });
    }

    // 7061: 3D magnitude ( x y z -- mag )
    for name in ["a.mag3", "analog_mag3d"] {
        interp.register(name, |ctx| {
            let z = ctx.pop()?;
            let y = ctx.pop()?;
            let x = ctx.pop()?;
            ctx.push(ops::op_magnitude_3d(&x, &y, &z)?);
            Ok(())
        });
    }

    // 7062: 3D distance ( x1 y1 z1 x2 y2 z2 -- dist )
    for name in ["a.dist3", "analog_dist3d"] {
        interp.register(name, |ctx| {
            let z2 = ctx.pop()?;
            let y2 = ctx.pop()?;
            let x2 = ctx.pop()?;
            let z1 = ctx.pop()?;
            let y1 = ctx.pop()?;
            let x1 = ctx.pop()?;
            ctx.push(ops::op_distance_3d(&x1, &y1, &z1, &x2, &y2, &z2)?);
            Ok(())
        });
    }

    // 7063: 3D normalize ( x y z -- nx ny nz )
    for name in ["a.norm3", "analog_norm3d"] {
        interp.register(name, |ctx| {
            let z = ctx.pop()?;
            let y = ctx.pop()?;
            let x = ctx.pop()?;
            let (nx, ny, nz) = ops::op_normalize_3d(&x, &y, &z)?;
            ctx.push(nx);
            ctx.push(ny);
            ctx.push(nz);
            Ok(())
        });
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COORDINATE TRANSFORMS (7090-7099)
// ═══════════════════════════════════════════════════════════════════════════

fn register_coordinate(interp: &mut Interpreter) {
    // 7090: Cartesian to polar ( x y -- r theta )
    for name in ["a.c2p", "analog_cart2pol"] {
        interp.register(name, |ctx| {
            let y = ctx.pop()?;
            let x = ctx.pop()?;
            let (r, theta) = ops::op_cartesian_to_polar(&x, &y)?;
            ctx.push(r);
            ctx.push(theta);
            Ok(())
        });
    }

    // 7091: Polar to Cartesian ( r theta -- x y )
    for name in ["a.p2c", "analog_pol2cart"] {
        interp.register(name, |ctx| {
            let theta = ctx.pop()?;
            let r = ctx.pop()?;
            let (x, y) = ops::op_polar_to_cartesian(&r, &theta)?;
            ctx.push(x);
            ctx.push(y);
            Ok(())
        });
    }
}
