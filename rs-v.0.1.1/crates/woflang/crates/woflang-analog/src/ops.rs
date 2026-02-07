//! WofLang interpreter integration for analog operations.
//!
//! This module provides opcode handlers that integrate analog mode
//! with the WofLang virtual machine.
//!
//! # Opcode Ranges
//!
//! Analog operations use opcode range 7000-7999:
//!
//! - 7000-7009: Mode control
//! - 7010-7029: Basic math
//! - 7030-7049: Extended math / trig
//! - 7050-7069: Linear algebra 2D
//! - 7070-7089: Linear algebra 3D
//! - 7090-7099: Coordinate transforms

use crate::linear;
use crate::math;
use crate::mode::{
    analog_max, analog_min, analog_status, clamp_analog, get_analog_config, reset_analog_mode,
    set_analog_custom, set_analog_mode, AnalogConfig, AnalogMode,
};
use crate::trig;
use woflang_core::{WofError, WofType, WofValue};

/// Result type for analog operations.
pub type AnalogResult<T> = Result<T, WofError>;

// ═══════════════════════════════════════════════════════════════════════════
// VALUE EXTRACTION HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Extract a numeric value from WofValue, converting if needed.
#[inline]
fn to_f64(value: &WofValue) -> AnalogResult<f64> {
    value.as_numeric()
}

/// Create an analog-clamped float WofValue.
#[inline]
fn analog_value(v: f64) -> WofValue {
    WofValue::double(clamp_analog(v))
}

/// Create an analog-clamped integer WofValue (for integer modes).
#[inline]
fn analog_int_value(v: f64) -> WofValue {
    let config = get_analog_config();
    if config.is_integer_mode() {
        WofValue::integer(config.clamp_rounded(v) as i64)
    } else {
        WofValue::double(config.clamp(v))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MODE CONTROL OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Opcode 7000: Get analog mode status as string.
pub fn op_analog_status() -> WofValue {
    WofValue::string(analog_status())
}

/// Opcode 7001: Set mode to INT_201 (-100 to +100).
pub fn op_analog_mode_int201() {
    set_analog_mode(AnalogMode::Int201);
}

/// Opcode 7002: Set mode to INT_2001 (-1000 to +1000).
pub fn op_analog_mode_int2001() {
    set_analog_mode(AnalogMode::Int2001);
}

/// Opcode 7003: Set mode to FLOAT_UNIT (-1.0 to +1.0).
pub fn op_analog_mode_float_unit() {
    set_analog_mode(AnalogMode::FloatUnit);
}

/// Opcode 7004: Set custom float mode (pops min, max from stack).
pub fn op_analog_mode_custom(min: &WofValue, max: &WofValue) -> AnalogResult<()> {
    let min_val = to_f64(min)?;
    let max_val = to_f64(max)?;
    if min_val >= max_val {
        return Err(WofError::runtime("analog custom mode: min must be < max"));
    }
    set_analog_custom(min_val, max_val);
    Ok(())
}

/// Opcode 7005: Reset to default mode (INT_201).
pub fn op_analog_reset() {
    reset_analog_mode();
}

/// Opcode 7006: Get current minimum value.
pub fn op_analog_min() -> WofValue {
    analog_value(analog_min())
}

/// Opcode 7007: Get current maximum value.
pub fn op_analog_max() -> WofValue {
    analog_value(analog_max())
}

/// Opcode 7008: Get range span (max - min).
pub fn op_analog_range() -> WofValue {
    let config = get_analog_config();
    analog_value(config.range())
}

/// Opcode 7009: Check if integer mode.
pub fn op_analog_is_int() -> WofValue {
    WofValue::boolean(get_analog_config().is_integer_mode())
}

// ═══════════════════════════════════════════════════════════════════════════
// BASIC MATH OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Opcode 7010: Analog clamp value.
pub fn op_analog_clamp(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_int_value(to_f64(value)?))
}

/// Opcode 7011: Analog add.
pub fn op_analog_add(a: &WofValue, b: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_int_value(math::analog_add(to_f64(a)?, to_f64(b)?)))
}

/// Opcode 7012: Analog subtract.
pub fn op_analog_sub(a: &WofValue, b: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_int_value(math::analog_sub(to_f64(a)?, to_f64(b)?)))
}

/// Opcode 7013: Analog multiply.
pub fn op_analog_mul(a: &WofValue, b: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_int_value(math::analog_mul(to_f64(a)?, to_f64(b)?)))
}

/// Opcode 7014: Analog divide.
pub fn op_analog_div(a: &WofValue, b: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_int_value(math::analog_div(to_f64(a)?, to_f64(b)?)))
}

/// Opcode 7015: Analog modulo.
pub fn op_analog_mod(a: &WofValue, b: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_int_value(math::analog_mod(to_f64(a)?, to_f64(b)?)))
}

/// Opcode 7016: Analog negate.
pub fn op_analog_neg(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_int_value(math::analog_neg(to_f64(value)?)))
}

/// Opcode 7017: Analog absolute value.
pub fn op_analog_abs(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_int_value(math::analog_abs(to_f64(value)?)))
}

/// Opcode 7018: Analog square root.
pub fn op_analog_sqrt(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(math::analog_sqrt(to_f64(value)?)))
}

/// Opcode 7019: Analog power.
pub fn op_analog_pow(base: &WofValue, exp: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(math::analog_pow(to_f64(base)?, to_f64(exp)?)))
}

/// Opcode 7020: Analog linear interpolation.
pub fn op_analog_lerp(a: &WofValue, b: &WofValue, t: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_int_value(math::analog_lerp(
        to_f64(a)?,
        to_f64(b)?,
        to_f64(t)?,
    )))
}

/// Opcode 7021: Apply deadzone.
pub fn op_analog_deadzone(value: &WofValue, threshold: &WofValue) -> AnalogResult<WofValue> {
    let config = get_analog_config();
    Ok(analog_int_value(
        config.deadzone(to_f64(value)?, to_f64(threshold)?),
    ))
}

/// Opcode 7022: Remap value between ranges.
pub fn op_analog_remap(
    value: &WofValue,
    from_min: &WofValue,
    from_max: &WofValue,
    to_min: &WofValue,
    to_max: &WofValue,
) -> AnalogResult<WofValue> {
    let config = get_analog_config();
    Ok(analog_int_value(config.remap(
        to_f64(value)?,
        to_f64(from_min)?,
        to_f64(from_max)?,
        to_f64(to_min)?,
        to_f64(to_max)?,
    )))
}

/// Opcode 7023: Normalize to [0, 1] based on current range.
pub fn op_analog_normalize(value: &WofValue) -> AnalogResult<WofValue> {
    let config = get_analog_config();
    Ok(analog_value(config.normalize(to_f64(value)?)))
}

/// Opcode 7024: Denormalize from [0, 1] to current range.
pub fn op_analog_denormalize(value: &WofValue) -> AnalogResult<WofValue> {
    let config = get_analog_config();
    Ok(analog_int_value(config.denormalize(to_f64(value)?)))
}

// ═══════════════════════════════════════════════════════════════════════════
// TRIGONOMETRIC OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Opcode 7030: Analog sine.
pub fn op_analog_sin(radians: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_sin(to_f64(radians)?)))
}

/// Opcode 7031: Analog cosine.
pub fn op_analog_cos(radians: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_cos(to_f64(radians)?)))
}

/// Opcode 7032: Analog tangent.
pub fn op_analog_tan(radians: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_tan(to_f64(radians)?)))
}

/// Opcode 7033: Analog arcsine.
pub fn op_analog_asin(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_asin(to_f64(value)?)))
}

/// Opcode 7034: Analog arccosine.
pub fn op_analog_acos(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_acos(to_f64(value)?)))
}

/// Opcode 7035: Analog arctangent.
pub fn op_analog_atan(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_atan(to_f64(value)?)))
}

/// Opcode 7036: Analog atan2.
pub fn op_analog_atan2(y: &WofValue, x: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_atan2(to_f64(y)?, to_f64(x)?)))
}

/// Opcode 7037: Analog hyperbolic tangent.
pub fn op_analog_tanh(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_tanh(to_f64(value)?)))
}

/// Opcode 7038: Analog exponential.
pub fn op_analog_exp(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_exp(to_f64(value)?)))
}

/// Opcode 7039: Analog natural logarithm.
pub fn op_analog_ln(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_ln(to_f64(value)?)))
}

/// Opcode 7040: Degrees to radians.
pub fn op_deg_to_rad(degrees: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::deg_to_rad(to_f64(degrees)?)))
}

/// Opcode 7041: Radians to degrees.
pub fn op_rad_to_deg(radians: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::rad_to_deg(to_f64(radians)?)))
}

/// Opcode 7042: Wrap radians to [0, 2π).
pub fn op_wrap_radians(radians: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::wrap_radians(to_f64(radians)?)))
}

/// Opcode 7043: Wrap degrees to [0, 360).
pub fn op_wrap_degrees(degrees: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::wrap_degrees(to_f64(degrees)?)))
}

/// Opcode 7044: Analog sigmoid.
pub fn op_analog_sigmoid(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_sigmoid(to_f64(value)?)))
}

/// Opcode 7045: Analog ReLU.
pub fn op_analog_relu(value: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(trig::analog_relu(to_f64(value)?)))
}

// ═══════════════════════════════════════════════════════════════════════════
// LINEAR ALGEBRA OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Opcode 7050: 2D dot product.
pub fn op_dot_2d(
    x1: &WofValue,
    y1: &WofValue,
    x2: &WofValue,
    y2: &WofValue,
) -> AnalogResult<WofValue> {
    Ok(analog_value(linear::analog_dot_2d(
        to_f64(x1)?,
        to_f64(y1)?,
        to_f64(x2)?,
        to_f64(y2)?,
    )))
}

/// Opcode 7051: 2D magnitude.
pub fn op_magnitude_2d(x: &WofValue, y: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(linear::analog_magnitude_2d(
        to_f64(x)?,
        to_f64(y)?,
    )))
}

/// Opcode 7052: 2D distance.
pub fn op_distance_2d(
    x1: &WofValue,
    y1: &WofValue,
    x2: &WofValue,
    y2: &WofValue,
) -> AnalogResult<WofValue> {
    Ok(analog_value(linear::analog_distance_2d(
        to_f64(x1)?,
        to_f64(y1)?,
        to_f64(x2)?,
        to_f64(y2)?,
    )))
}

/// Opcode 7053: 2D normalize - returns (nx, ny) as tuple... or pushes twice.
pub fn op_normalize_2d(x: &WofValue, y: &WofValue) -> AnalogResult<(WofValue, WofValue)> {
    let (nx, ny) = linear::analog_normalize_2d(to_f64(x)?, to_f64(y)?);
    Ok((analog_value(nx), analog_value(ny)))
}

/// Opcode 7060: 3D dot product.
pub fn op_dot_3d(
    x1: &WofValue,
    y1: &WofValue,
    z1: &WofValue,
    x2: &WofValue,
    y2: &WofValue,
    z2: &WofValue,
) -> AnalogResult<WofValue> {
    Ok(analog_value(linear::analog_dot_3d(
        to_f64(x1)?,
        to_f64(y1)?,
        to_f64(z1)?,
        to_f64(x2)?,
        to_f64(y2)?,
        to_f64(z2)?,
    )))
}

/// Opcode 7061: 3D magnitude.
pub fn op_magnitude_3d(x: &WofValue, y: &WofValue, z: &WofValue) -> AnalogResult<WofValue> {
    Ok(analog_value(linear::analog_magnitude_3d(
        to_f64(x)?,
        to_f64(y)?,
        to_f64(z)?,
    )))
}

/// Opcode 7062: 3D distance.
pub fn op_distance_3d(
    x1: &WofValue,
    y1: &WofValue,
    z1: &WofValue,
    x2: &WofValue,
    y2: &WofValue,
    z2: &WofValue,
) -> AnalogResult<WofValue> {
    Ok(analog_value(linear::analog_distance_3d(
        to_f64(x1)?,
        to_f64(y1)?,
        to_f64(z1)?,
        to_f64(x2)?,
        to_f64(y2)?,
        to_f64(z2)?,
    )))
}

/// Opcode 7063: 3D normalize.
pub fn op_normalize_3d(
    x: &WofValue,
    y: &WofValue,
    z: &WofValue,
) -> AnalogResult<(WofValue, WofValue, WofValue)> {
    let (nx, ny, nz) = linear::analog_normalize_3d(to_f64(x)?, to_f64(y)?, to_f64(z)?);
    Ok((analog_value(nx), analog_value(ny), analog_value(nz)))
}

// ═══════════════════════════════════════════════════════════════════════════
// COORDINATE TRANSFORM OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Opcode 7090: Cartesian to polar.
pub fn op_cartesian_to_polar(x: &WofValue, y: &WofValue) -> AnalogResult<(WofValue, WofValue)> {
    let config = get_analog_config();
    let (r, theta) = config.cartesian_to_polar(to_f64(x)?, to_f64(y)?);
    Ok((analog_value(r), analog_value(theta)))
}

/// Opcode 7091: Polar to Cartesian.
pub fn op_polar_to_cartesian(r: &WofValue, theta: &WofValue) -> AnalogResult<(WofValue, WofValue)> {
    let config = get_analog_config();
    let (x, y) = config.polar_to_cartesian(to_f64(r)?, to_f64(theta)?);
    Ok((analog_value(x), analog_value(y)))
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        reset_analog_mode();
    }

    #[test]
    fn mode_operations() {
        setup();

        op_analog_mode_int201();
        assert_eq!(analog_min(), -100.0);
        assert_eq!(analog_max(), 100.0);

        op_analog_mode_float_unit();
        assert_eq!(analog_min(), -1.0);
        assert_eq!(analog_max(), 1.0);
    }

    #[test]
    fn math_operations() {
        setup();
        op_analog_mode_int201();

        let a = WofValue::double(80.0);
        let b = WofValue::double(50.0);

        // Should saturate at 100
        let result = op_analog_add(&a, &b).unwrap();
        assert_eq!(result.as_double().unwrap(), 100.0);
    }

    #[test]
    fn clamp_operation() {
        setup();
        op_analog_mode_int201();

        let val = WofValue::double(150.0);
        let clamped = op_analog_clamp(&val).unwrap();
        assert_eq!(clamped.as_double().unwrap(), 100.0);
    }

    #[test]
    fn trig_operations() {
        setup();
        op_analog_mode_float_unit();

        let zero = WofValue::double(0.0);
        let sin_0 = op_analog_sin(&zero).unwrap();
        assert!(sin_0.as_double().unwrap().abs() < f64::EPSILON);

        let cos_0 = op_analog_cos(&zero).unwrap();
        assert!((cos_0.as_double().unwrap() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn linear_operations() {
        setup();
        op_analog_mode_int201();

        let x1 = WofValue::double(3.0);
        let y1 = WofValue::double(4.0);
        let x2 = WofValue::double(1.0);
        let y2 = WofValue::double(2.0);

        // Dot product: 3*1 + 4*2 = 11
        let dot = op_dot_2d(&x1, &y1, &x2, &y2).unwrap();
        assert_eq!(dot.as_double().unwrap(), 11.0);

        // Magnitude of (3, 4) = 5
        let mag = op_magnitude_2d(&x1, &y1).unwrap();
        assert_eq!(mag.as_double().unwrap(), 5.0);
    }
}
