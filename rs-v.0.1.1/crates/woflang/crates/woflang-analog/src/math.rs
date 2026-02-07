//! Analog arithmetic operations with automatic clamping.
//!
//! All operations in this module automatically clamp results to the
//! current analog mode's range. This creates saturation semantics
//! instead of overflow/underflow.
//!
//! # Examples
//!
//! ```
//! use woflang_analog::{AnalogConfig, AnalogMode};
//! use woflang_analog::math::*;
//!
//! let config = AnalogConfig::new(AnalogMode::Int201);
//!
//! // Normal operations work as expected
//! assert_eq!(config.add(50.0, 30.0), 80.0);
//! assert_eq!(config.sub(50.0, 30.0), 20.0);
//!
//! // But results saturate at boundaries
//! assert_eq!(config.add(80.0, 50.0), 100.0);  // Saturates at +100
//! assert_eq!(config.sub(-80.0, 50.0), -100.0); // Saturates at -100
//! ```

use crate::mode::{clamp_analog, get_analog_config, AnalogConfig};

// ═══════════════════════════════════════════════════════════════════════════
// CONFIG-BASED OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

impl AnalogConfig {
    /// Analog addition with automatic clamping.
    #[inline]
    #[must_use]
    pub fn add(&self, a: f64, b: f64) -> f64 {
        self.clamp(a + b)
    }

    /// Analog subtraction with automatic clamping.
    #[inline]
    #[must_use]
    pub fn sub(&self, a: f64, b: f64) -> f64 {
        self.clamp(a - b)
    }

    /// Analog multiplication with automatic clamping.
    #[inline]
    #[must_use]
    pub fn mul(&self, a: f64, b: f64) -> f64 {
        self.clamp(a * b)
    }

    /// Analog division with automatic clamping.
    ///
    /// Division by zero returns the midpoint (typically 0).
    #[inline]
    #[must_use]
    pub fn div(&self, a: f64, b: f64) -> f64 {
        if b.abs() < f64::EPSILON {
            self.midpoint() // Safe fallback for div by zero
        } else {
            self.clamp(a / b)
        }
    }

    /// Analog modulo with automatic clamping.
    #[inline]
    #[must_use]
    pub fn modulo(&self, a: f64, b: f64) -> f64 {
        if b.abs() < f64::EPSILON {
            self.clamp(a)
        } else {
            self.clamp(a % b)
        }
    }

    /// Analog power with automatic clamping.
    #[inline]
    #[must_use]
    pub fn pow(&self, base: f64, exp: f64) -> f64 {
        self.clamp(base.powf(exp))
    }

    /// Analog negation with automatic clamping.
    #[inline]
    #[must_use]
    pub fn neg(&self, a: f64) -> f64 {
        self.clamp(-a)
    }

    /// Analog absolute value with automatic clamping.
    #[inline]
    #[must_use]
    pub fn abs(&self, a: f64) -> f64 {
        self.clamp(a.abs())
    }

    /// Analog square root with automatic clamping.
    ///
    /// Negative inputs return the midpoint (typically 0).
    #[inline]
    #[must_use]
    pub fn sqrt(&self, a: f64) -> f64 {
        if a < 0.0 {
            self.midpoint() // Safe fallback for negative sqrt
        } else {
            self.clamp(a.sqrt())
        }
    }

    /// Analog minimum of two values.
    #[inline]
    #[must_use]
    pub fn min_of(&self, a: f64, b: f64) -> f64 {
        self.clamp(a.min(b))
    }

    /// Analog maximum of two values.
    #[inline]
    #[must_use]
    pub fn max_of(&self, a: f64, b: f64) -> f64 {
        self.clamp(a.max(b))
    }

    /// Linear interpolation between two values, clamped.
    ///
    /// When `t = 0`, returns `a`. When `t = 1`, returns `b`.
    #[inline]
    #[must_use]
    pub fn lerp(&self, a: f64, b: f64, t: f64) -> f64 {
        let t_clamped = t.clamp(0.0, 1.0);
        self.clamp(a + (b - a) * t_clamped)
    }

    /// Inverse linear interpolation - find t where lerp would produce value.
    #[inline]
    #[must_use]
    pub fn inverse_lerp(&self, a: f64, b: f64, value: f64) -> f64 {
        let range = b - a;
        if range.abs() < f64::EPSILON {
            0.0
        } else {
            ((value - a) / range).clamp(0.0, 1.0)
        }
    }

    /// Smoothstep interpolation (Hermite).
    #[inline]
    #[must_use]
    pub fn smoothstep(&self, edge0: f64, edge1: f64, x: f64) -> f64 {
        let t = self.inverse_lerp(edge0, edge1, x);
        let smooth = t * t * (3.0 - 2.0 * t);
        self.clamp(self.min() + smooth * self.range())
    }

    /// Sign function: returns -max, 0, or +max.
    #[inline]
    #[must_use]
    pub fn sign(&self, a: f64) -> f64 {
        if a > f64::EPSILON {
            self.max()
        } else if a < -f64::EPSILON {
            self.min()
        } else {
            self.midpoint()
        }
    }

    /// Fused multiply-add: (a * b) + c, clamped.
    #[inline]
    #[must_use]
    pub fn fma(&self, a: f64, b: f64, c: f64) -> f64 {
        self.clamp(a.mul_add(b, c))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GLOBAL STATE OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Analog addition using global configuration.
#[inline]
#[must_use]
pub fn analog_add(a: f64, b: f64) -> f64 {
    clamp_analog(a + b)
}

/// Analog subtraction using global configuration.
#[inline]
#[must_use]
pub fn analog_sub(a: f64, b: f64) -> f64 {
    clamp_analog(a - b)
}

/// Analog multiplication using global configuration.
#[inline]
#[must_use]
pub fn analog_mul(a: f64, b: f64) -> f64 {
    clamp_analog(a * b)
}

/// Analog division using global configuration.
#[inline]
#[must_use]
pub fn analog_div(a: f64, b: f64) -> f64 {
    if b.abs() < f64::EPSILON {
        clamp_analog(0.0)
    } else {
        clamp_analog(a / b)
    }
}

/// Analog modulo using global configuration.
#[inline]
#[must_use]
pub fn analog_mod(a: f64, b: f64) -> f64 {
    if b.abs() < f64::EPSILON {
        clamp_analog(a)
    } else {
        clamp_analog(a % b)
    }
}

/// Analog power using global configuration.
#[inline]
#[must_use]
pub fn analog_pow(base: f64, exp: f64) -> f64 {
    clamp_analog(base.powf(exp))
}

/// Analog negation using global configuration.
#[inline]
#[must_use]
pub fn analog_neg(a: f64) -> f64 {
    clamp_analog(-a)
}

/// Analog absolute value using global configuration.
#[inline]
#[must_use]
pub fn analog_abs(a: f64) -> f64 {
    clamp_analog(a.abs())
}

/// Analog square root using global configuration.
#[inline]
#[must_use]
pub fn analog_sqrt(a: f64) -> f64 {
    if a < 0.0 {
        clamp_analog(0.0)
    } else {
        clamp_analog(a.sqrt())
    }
}

/// Analog linear interpolation using global configuration.
#[inline]
#[must_use]
pub fn analog_lerp(a: f64, b: f64, t: f64) -> f64 {
    get_analog_config().lerp(a, b, t)
}

/// Analog fused multiply-add using global configuration.
#[inline]
#[must_use]
pub fn analog_fma(a: f64, b: f64, c: f64) -> f64 {
    clamp_analog(a.mul_add(b, c))
}

// ═══════════════════════════════════════════════════════════════════════════
// BATCH OPERATIONS (for future SIMD optimization)
// ═══════════════════════════════════════════════════════════════════════════

/// Batch add: adds corresponding elements, clamping each result.
#[must_use]
pub fn batch_add(a: &[f64], b: &[f64]) -> Vec<f64> {
    let config = get_analog_config();
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| config.add(*x, *y))
        .collect()
}

/// Batch multiply: multiplies corresponding elements, clamping each result.
#[must_use]
pub fn batch_mul(a: &[f64], b: &[f64]) -> Vec<f64> {
    let config = get_analog_config();
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| config.mul(*x, *y))
        .collect()
}

/// Batch clamp: clamps all values in a slice.
#[must_use]
pub fn batch_clamp(values: &[f64]) -> Vec<f64> {
    let config = get_analog_config();
    values.iter().map(|x| config.clamp(*x)).collect()
}

/// Batch scale: multiplies all values by a scalar, clamping each.
#[must_use]
pub fn batch_scale(values: &[f64], scalar: f64) -> Vec<f64> {
    let config = get_analog_config();
    values.iter().map(|x| config.mul(*x, scalar)).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mode::{reset_analog_mode, set_analog_mode, AnalogMode};

    fn setup() {
        reset_analog_mode();
    }

    #[test]
    fn basic_arithmetic() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        assert_eq!(config.add(50.0, 30.0), 80.0);
        assert_eq!(config.sub(50.0, 30.0), 20.0);
        assert_eq!(config.mul(5.0, 10.0), 50.0);
        assert_eq!(config.div(100.0, 2.0), 50.0);
    }

    #[test]
    fn saturation() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // Addition saturation
        assert_eq!(config.add(80.0, 50.0), 100.0);
        assert_eq!(config.add(-80.0, -50.0), -100.0);

        // Multiplication saturation
        assert_eq!(config.mul(50.0, 50.0), 100.0); // 2500 -> saturates to 100
        assert_eq!(config.mul(-50.0, 50.0), -100.0);
    }

    #[test]
    fn division_by_zero() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // Div by zero returns midpoint (0 for symmetric ranges)
        assert_eq!(config.div(50.0, 0.0), 0.0);
    }

    #[test]
    fn sqrt_negative() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // Sqrt of negative returns midpoint
        assert_eq!(config.sqrt(-25.0), 0.0);
        assert_eq!(config.sqrt(25.0), 5.0);
    }

    #[test]
    fn lerp() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        assert_eq!(config.lerp(-100.0, 100.0, 0.0), -100.0);
        assert_eq!(config.lerp(-100.0, 100.0, 1.0), 100.0);
        assert_eq!(config.lerp(-100.0, 100.0, 0.5), 0.0);
    }

    #[test]
    fn global_functions() {
        setup();
        set_analog_mode(AnalogMode::Int201);

        assert_eq!(analog_add(80.0, 50.0), 100.0);
        assert_eq!(analog_mul(50.0, 50.0), 100.0);
        assert_eq!(analog_neg(50.0), -50.0);
    }

    #[test]
    fn batch_operations() {
        setup();
        set_analog_mode(AnalogMode::Int201);

        let a = vec![10.0, 20.0, 80.0];
        let b = vec![5.0, 10.0, 50.0];

        let result = batch_add(&a, &b);
        assert_eq!(result, vec![15.0, 30.0, 100.0]); // Last one saturates

        let scaled = batch_scale(&a, 2.0);
        assert_eq!(scaled, vec![20.0, 40.0, 100.0]); // Last one saturates
    }

    #[test]
    fn fma() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // 10 * 5 + 20 = 70
        assert_eq!(config.fma(10.0, 5.0, 20.0), 70.0);

        // 50 * 50 + 0 = 2500 -> saturates to 100
        assert_eq!(config.fma(50.0, 50.0, 0.0), 100.0);
    }
}
