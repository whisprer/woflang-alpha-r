//! WofLang Analog Computing Core
//!
//! Values exist on a bounded continuum, auto-clamped to the active range.
//! This is the philosophical heart of WofLang - not binary, but analog.
//!
//! Modes:
//! - INT_201:     -100 to +100 (classic analog, like a VU meter)
//! - INT_2001:    -1000 to +1000 (finer resolution)
//! - FLOAT_UNIT:  -1.0 to +1.0 (normalized signal)
//! - FLOAT_CUSTOM: user-defined range

use std::sync::atomic::{AtomicU8, AtomicU32, Ordering};
use std::f64::consts::{PI, TAU};

/// Analog computing modes - each defines a bounded continuum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AnalogMode {
    /// -100 to +100 inclusive (classic VU meter style)
    Int201 = 0,
    /// -1000 to +1000 inclusive (finer resolution)
    Int2001 = 1,
    /// -1.0 to +1.0 float (normalized signal)
    FloatUnit = 2,
    /// User-defined float range
    FloatCustom = 3,
}

impl Default for AnalogMode {
    fn default() -> Self {
        AnalogMode::Int201
    }
}

impl From<u8> for AnalogMode {
    fn from(v: u8) -> Self {
        match v {
            0 => AnalogMode::Int201,
            1 => AnalogMode::Int2001,
            2 => AnalogMode::FloatUnit,
            3 => AnalogMode::FloatCustom,
            _ => AnalogMode::Int201,
        }
    }
}

// Global analog state using atomics for thread safety
static ANALOG_MODE: AtomicU8 = AtomicU8::new(0); // Int201
static CUSTOM_MIN_BITS: AtomicU32 = AtomicU32::new(0xBF800000); // -1.0f32
static CUSTOM_MAX_BITS: AtomicU32 = AtomicU32::new(0x3F800000); // 1.0f32

/// Get the current analog mode
pub fn get_mode() -> AnalogMode {
    AnalogMode::from(ANALOG_MODE.load(Ordering::Relaxed))
}

/// Set the analog mode
pub fn set_mode(mode: AnalogMode) {
    ANALOG_MODE.store(mode as u8, Ordering::Relaxed);
}

/// Set a custom float range (only used when mode is FloatCustom)
pub fn set_custom_range(min: f64, max: f64) -> Result<(), &'static str> {
    if min >= max {
        return Err("Custom range must have min < max");
    }
    CUSTOM_MIN_BITS.store((min as f32).to_bits(), Ordering::Relaxed);
    CUSTOM_MAX_BITS.store((max as f32).to_bits(), Ordering::Relaxed);
    Ok(())
}

/// Get the minimum value for the current analog mode
pub fn analog_min() -> f64 {
    match get_mode() {
        AnalogMode::Int201 => -100.0,
        AnalogMode::Int2001 => -1000.0,
        AnalogMode::FloatUnit => -1.0,
        AnalogMode::FloatCustom => {
            f32::from_bits(CUSTOM_MIN_BITS.load(Ordering::Relaxed)) as f64
        }
    }
}

/// Get the maximum value for the current analog mode
pub fn analog_max() -> f64 {
    match get_mode() {
        AnalogMode::Int201 => 100.0,
        AnalogMode::Int2001 => 1000.0,
        AnalogMode::FloatUnit => 1.0,
        AnalogMode::FloatCustom => {
            f32::from_bits(CUSTOM_MAX_BITS.load(Ordering::Relaxed)) as f64
        }
    }
}

/// Get the span (range) of the current analog mode
pub fn analog_span() -> f64 {
    analog_max() - analog_min()
}

/// Check if current mode is integer-based
pub fn is_integer_mode() -> bool {
    matches!(get_mode(), AnalogMode::Int201 | AnalogMode::Int2001)
}

// ============================================================================
// CLAMPING - The heart of analog computing
// ============================================================================

/// Clamp a value to the current analog range
#[inline]
pub fn clamp(value: f64) -> f64 {
    let min = analog_min();
    let max = analog_max();
    value.max(min).min(max)
}

/// Clamp a value to a specific range (for temporary calculations)
#[inline]
pub fn clamp_to(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

/// Clamp to the -1.0 to 1.0 range (common for trig results)
#[inline]
pub fn clamp_unit(value: f64) -> f64 {
    value.max(-1.0).min(1.0)
}

// ============================================================================
// BASIC ANALOG MATH - All operations auto-clamp
// ============================================================================

/// Analog addition: a + b, clamped to analog range
#[inline]
pub fn add(a: f64, b: f64) -> f64 {
    clamp(a + b)
}

/// Analog subtraction: a - b, clamped to analog range
#[inline]
pub fn sub(a: f64, b: f64) -> f64 {
    clamp(a - b)
}

/// Analog multiplication: a * b, clamped to analog range
#[inline]
pub fn mul(a: f64, b: f64) -> f64 {
    clamp(a * b)
}

/// Analog division: a / b, clamped to analog range
/// Division by zero returns 0.0 (safe fallback)
#[inline]
pub fn div(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        0.0 // Safe fallback for division by void
    } else {
        clamp(a / b)
    }
}

/// Analog modulo: a % b, clamped to analog range
#[inline]
pub fn modulo(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        0.0
    } else {
        clamp(a % b)
    }
}

/// Analog negation: -a, clamped to analog range
#[inline]
pub fn negate(a: f64) -> f64 {
    clamp(-a)
}

/// Analog absolute value: |a|, clamped to analog range
#[inline]
pub fn abs(a: f64) -> f64 {
    clamp(a.abs())
}

/// Analog minimum of two values
#[inline]
pub fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

/// Analog maximum of two values
#[inline]
pub fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

// ============================================================================
// EXTENDED MATH - Trig, exponentials, all clamped
// ============================================================================

/// Analog square root (negative inputs return 0)
#[inline]
pub fn sqrt(a: f64) -> f64 {
    if a < 0.0 {
        0.0 // Safe fallback for sqrt of negative
    } else {
        clamp(a.sqrt())
    }
}

/// Analog power: base^exponent, clamped
#[inline]
pub fn pow(base: f64, exponent: f64) -> f64 {
    let result = base.powf(exponent);
    if result.is_nan() || result.is_infinite() {
        if result.is_sign_positive() || result.is_nan() {
            analog_max()
        } else {
            analog_min()
        }
    } else {
        clamp(result)
    }
}

/// Analog sine
#[inline]
pub fn sin(a: f64) -> f64 {
    clamp(a.sin())
}

/// Analog cosine
#[inline]
pub fn cos(a: f64) -> f64 {
    clamp(a.cos())
}

/// Analog tangent (clamped to avoid infinity)
#[inline]
pub fn tan(a: f64) -> f64 {
    let result = a.tan();
    if result.is_infinite() || result.is_nan() {
        if result.is_sign_positive() || result.is_nan() {
            analog_max()
        } else {
            analog_min()
        }
    } else {
        clamp(result)
    }
}

/// Analog arcsine (input clamped to [-1, 1])
#[inline]
pub fn asin(a: f64) -> f64 {
    let clamped_input = clamp_unit(a);
    clamp(clamped_input.asin())
}

/// Analog arccosine (input clamped to [-1, 1])
#[inline]
pub fn acos(a: f64) -> f64 {
    let clamped_input = clamp_unit(a);
    clamp(clamped_input.acos())
}

/// Analog arctangent
#[inline]
pub fn atan(a: f64) -> f64 {
    clamp(a.atan())
}

/// Analog atan2(y, x)
#[inline]
pub fn atan2(y: f64, x: f64) -> f64 {
    clamp(y.atan2(x))
}

/// Analog hyperbolic sine
#[inline]
pub fn sinh(a: f64) -> f64 {
    clamp(a.sinh())
}

/// Analog hyperbolic cosine
#[inline]
pub fn cosh(a: f64) -> f64 {
    clamp(a.cosh())
}

/// Analog hyperbolic tangent (naturally bounded to [-1, 1])
#[inline]
pub fn tanh(a: f64) -> f64 {
    clamp(a.tanh())
}

/// Analog natural logarithm (negative/zero inputs return min)
#[inline]
pub fn ln(a: f64) -> f64 {
    if a <= 0.0 {
        analog_min()
    } else {
        clamp(a.ln())
    }
}

/// Analog log base 10
#[inline]
pub fn log10(a: f64) -> f64 {
    if a <= 0.0 {
        analog_min()
    } else {
        clamp(a.log10())
    }
}

/// Analog log base 2
#[inline]
pub fn log2(a: f64) -> f64 {
    if a <= 0.0 {
        analog_min()
    } else {
        clamp(a.log2())
    }
}

/// Analog exponential e^a
#[inline]
pub fn exp(a: f64) -> f64 {
    let result = a.exp();
    if result.is_infinite() {
        analog_max()
    } else {
        clamp(result)
    }
}

// ============================================================================
// ANGLE UTILITIES
// ============================================================================

/// Degrees to radians conversion
#[inline]
pub fn deg_to_rad(degrees: f64) -> f64 {
    clamp(degrees * PI / 180.0)
}

/// Radians to degrees conversion
#[inline]
pub fn rad_to_deg(radians: f64) -> f64 {
    clamp(radians * 180.0 / PI)
}

/// Wrap radians to [0, 2π)
#[inline]
pub fn wrap_radians(radians: f64) -> f64 {
    let mut wrapped = radians % TAU;
    if wrapped < 0.0 {
        wrapped += TAU;
    }
    clamp(wrapped)
}

/// Wrap degrees to [0, 360)
#[inline]
pub fn wrap_degrees(degrees: f64) -> f64 {
    let mut wrapped = degrees % 360.0;
    if wrapped < 0.0 {
        wrapped += 360.0;
    }
    clamp(wrapped)
}

/// Wrap radians to [-π, π)
#[inline]
pub fn wrap_radians_symmetric(radians: f64) -> f64 {
    let mut wrapped = radians % TAU;
    if wrapped > PI {
        wrapped -= TAU;
    } else if wrapped < -PI {
        wrapped += TAU;
    }
    clamp(wrapped)
}

/// Wrap degrees to [-180, 180)
#[inline]
pub fn wrap_degrees_symmetric(degrees: f64) -> f64 {
    let mut wrapped = degrees % 360.0;
    if wrapped > 180.0 {
        wrapped -= 360.0;
    } else if wrapped < -180.0 {
        wrapped += 360.0;
    }
    clamp(wrapped)
}

// ============================================================================
// REMAP AND SCALE UTILITIES
// ============================================================================

/// Remap a value from one range to another, with analog clamping
pub fn remap(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    if (from_max - from_min).abs() < f64::EPSILON {
        return clamp(to_min);
    }
    let normalized = (value - from_min) / (from_max - from_min);
    let result = to_min + normalized * (to_max - to_min);
    clamp(result)
}

/// Apply a deadzone - values within threshold from zero become zero
pub fn deadzone(value: f64, threshold: f64) -> f64 {
    if value.abs() < threshold {
        0.0
    } else {
        clamp(value)
    }
}

/// Smooth step (cubic Hermite interpolation)
pub fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = clamp_to((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    clamp(t * t * (3.0 - 2.0 * t))
}

/// Linear interpolation between a and b
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    clamp(a + (b - a) * clamp_to(t, 0.0, 1.0))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamping_int201() {
        set_mode(AnalogMode::Int201);
        assert_eq!(clamp(150.0), 100.0);
        assert_eq!(clamp(-150.0), -100.0);
        assert_eq!(clamp(50.0), 50.0);
    }

    #[test]
    fn test_clamping_float_unit() {
        set_mode(AnalogMode::FloatUnit);
        assert_eq!(clamp(1.5), 1.0);
        assert_eq!(clamp(-1.5), -1.0);
        assert_eq!(clamp(0.5), 0.5);
    }

    #[test]
    fn test_analog_add() {
        set_mode(AnalogMode::Int201);
        assert_eq!(add(80.0, 50.0), 100.0); // Clamped!
        assert_eq!(add(30.0, 20.0), 50.0);  // Normal
    }

    #[test]
    fn test_analog_div_by_zero() {
        set_mode(AnalogMode::Int201);
        assert_eq!(div(50.0, 0.0), 0.0); // Safe fallback
    }

    #[test]
    fn test_wrap_degrees() {
        set_mode(AnalogMode::Int2001); // Allow 0-360 range
        assert!((wrap_degrees(450.0) - 90.0).abs() < 0.001);
        assert!((wrap_degrees(-90.0) - 270.0).abs() < 0.001);
    }
}
