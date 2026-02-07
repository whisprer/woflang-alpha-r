//! Analog trigonometric and transcendental functions.
//!
//! All trigonometric functions automatically clamp their results to the
//! current analog mode's range. Note that some functions (like sin/cos)
//! naturally produce values in [-1, 1], which may be further clamped
//! depending on the mode.
//!
//! # Angle Handling
//!
//! Functions accept angles in radians. Use `deg_to_rad` and `rad_to_deg`
//! for conversion. Angle wrapping functions ensure values stay within
//! expected bounds.

use crate::mode::{clamp_analog, get_analog_config, AnalogConfig};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

/// Full circle in degrees.
pub const FULL_CIRCLE_DEG: f64 = 360.0;

/// Half circle in degrees.
pub const HALF_CIRCLE_DEG: f64 = 180.0;

// ═══════════════════════════════════════════════════════════════════════════
// CONFIG-BASED TRIGONOMETRY
// ═══════════════════════════════════════════════════════════════════════════

impl AnalogConfig {
    /// Sine function with analog clamping.
    #[inline]
    #[must_use]
    pub fn sin(&self, radians: f64) -> f64 {
        self.clamp(radians.sin())
    }

    /// Cosine function with analog clamping.
    #[inline]
    #[must_use]
    pub fn cos(&self, radians: f64) -> f64 {
        self.clamp(radians.cos())
    }

    /// Tangent function with analog clamping.
    ///
    /// Note: tan can produce very large values near π/2, which will
    /// be clamped to the mode's bounds.
    #[inline]
    #[must_use]
    pub fn tan(&self, radians: f64) -> f64 {
        self.clamp(radians.tan())
    }

    /// Arcsine function with analog clamping.
    ///
    /// Input is clamped to [-1, 1] before computation.
    #[inline]
    #[must_use]
    pub fn asin(&self, x: f64) -> f64 {
        let clamped_input = x.clamp(-1.0, 1.0);
        self.clamp(clamped_input.asin())
    }

    /// Arccosine function with analog clamping.
    ///
    /// Input is clamped to [-1, 1] before computation.
    #[inline]
    #[must_use]
    pub fn acos(&self, x: f64) -> f64 {
        let clamped_input = x.clamp(-1.0, 1.0);
        self.clamp(clamped_input.acos())
    }

    /// Arctangent function with analog clamping.
    #[inline]
    #[must_use]
    pub fn atan(&self, x: f64) -> f64 {
        self.clamp(x.atan())
    }

    /// Two-argument arctangent with analog clamping.
    ///
    /// Returns the angle in radians between the positive x-axis and
    /// the point (x, y).
    #[inline]
    #[must_use]
    pub fn atan2(&self, y: f64, x: f64) -> f64 {
        self.clamp(y.atan2(x))
    }

    /// Hyperbolic sine with analog clamping.
    #[inline]
    #[must_use]
    pub fn sinh(&self, x: f64) -> f64 {
        self.clamp(x.sinh())
    }

    /// Hyperbolic cosine with analog clamping.
    #[inline]
    #[must_use]
    pub fn cosh(&self, x: f64) -> f64 {
        self.clamp(x.cosh())
    }

    /// Hyperbolic tangent with analog clamping.
    ///
    /// Note: tanh naturally produces values in (-1, 1), making it
    /// particularly useful for neural network activations.
    #[inline]
    #[must_use]
    pub fn tanh(&self, x: f64) -> f64 {
        self.clamp(x.tanh())
    }

    /// Exponential function with analog clamping.
    #[inline]
    #[must_use]
    pub fn exp(&self, x: f64) -> f64 {
        self.clamp(x.exp())
    }

    /// Natural logarithm with analog clamping.
    ///
    /// Returns midpoint for non-positive inputs.
    #[inline]
    #[must_use]
    pub fn ln(&self, x: f64) -> f64 {
        if x <= 0.0 {
            self.midpoint()
        } else {
            self.clamp(x.ln())
        }
    }

    /// Base-10 logarithm with analog clamping.
    #[inline]
    #[must_use]
    pub fn log10(&self, x: f64) -> f64 {
        if x <= 0.0 {
            self.midpoint()
        } else {
            self.clamp(x.log10())
        }
    }

    /// Base-2 logarithm with analog clamping.
    #[inline]
    #[must_use]
    pub fn log2(&self, x: f64) -> f64 {
        if x <= 0.0 {
            self.midpoint()
        } else {
            self.clamp(x.log2())
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GLOBAL STATE TRIGONOMETRY
// ═══════════════════════════════════════════════════════════════════════════

/// Analog sine using global configuration.
#[inline]
#[must_use]
pub fn analog_sin(radians: f64) -> f64 {
    clamp_analog(radians.sin())
}

/// Analog cosine using global configuration.
#[inline]
#[must_use]
pub fn analog_cos(radians: f64) -> f64 {
    clamp_analog(radians.cos())
}

/// Analog tangent using global configuration.
#[inline]
#[must_use]
pub fn analog_tan(radians: f64) -> f64 {
    clamp_analog(radians.tan())
}

/// Analog arcsine using global configuration.
#[inline]
#[must_use]
pub fn analog_asin(x: f64) -> f64 {
    clamp_analog(x.clamp(-1.0, 1.0).asin())
}

/// Analog arccosine using global configuration.
#[inline]
#[must_use]
pub fn analog_acos(x: f64) -> f64 {
    clamp_analog(x.clamp(-1.0, 1.0).acos())
}

/// Analog arctangent using global configuration.
#[inline]
#[must_use]
pub fn analog_atan(x: f64) -> f64 {
    clamp_analog(x.atan())
}

/// Analog atan2 using global configuration.
#[inline]
#[must_use]
pub fn analog_atan2(y: f64, x: f64) -> f64 {
    clamp_analog(y.atan2(x))
}

/// Analog hyperbolic tangent using global configuration.
///
/// Particularly useful as a neural network activation function.
#[inline]
#[must_use]
pub fn analog_tanh(x: f64) -> f64 {
    clamp_analog(x.tanh())
}

/// Analog exponential using global configuration.
#[inline]
#[must_use]
pub fn analog_exp(x: f64) -> f64 {
    clamp_analog(x.exp())
}

/// Analog natural logarithm using global configuration.
#[inline]
#[must_use]
pub fn analog_ln(x: f64) -> f64 {
    if x <= 0.0 {
        clamp_analog(0.0)
    } else {
        clamp_analog(x.ln())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ANGLE CONVERSIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Convert degrees to radians.
#[inline]
#[must_use]
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / HALF_CIRCLE_DEG
}

/// Convert radians to degrees.
#[inline]
#[must_use]
pub fn rad_to_deg(radians: f64) -> f64 {
    radians * HALF_CIRCLE_DEG / PI
}

/// Wrap angle to [0, 2π) range.
#[inline]
#[must_use]
pub fn wrap_radians(radians: f64) -> f64 {
    let wrapped = radians % TAU;
    if wrapped < 0.0 {
        wrapped + TAU
    } else {
        wrapped
    }
}

/// Wrap angle to [-π, π) range (symmetric).
#[inline]
#[must_use]
pub fn wrap_radians_symmetric(radians: f64) -> f64 {
    let wrapped = wrap_radians(radians);
    if wrapped >= PI {
        wrapped - TAU
    } else {
        wrapped
    }
}

/// Wrap angle to [0, 360) degrees range.
#[inline]
#[must_use]
pub fn wrap_degrees(degrees: f64) -> f64 {
    let wrapped = degrees % FULL_CIRCLE_DEG;
    if wrapped < 0.0 {
        wrapped + FULL_CIRCLE_DEG
    } else {
        wrapped
    }
}

/// Wrap angle to [-180, 180) degrees range (symmetric).
#[inline]
#[must_use]
pub fn wrap_degrees_symmetric(degrees: f64) -> f64 {
    let wrapped = wrap_degrees(degrees);
    if wrapped >= HALF_CIRCLE_DEG {
        wrapped - FULL_CIRCLE_DEG
    } else {
        wrapped
    }
}

/// Analog-clamped wrap radians.
#[inline]
#[must_use]
pub fn analog_wrap_radians(radians: f64) -> f64 {
    clamp_analog(wrap_radians(radians))
}

/// Analog-clamped wrap degrees.
#[inline]
#[must_use]
pub fn analog_wrap_degrees(degrees: f64) -> f64 {
    clamp_analog(wrap_degrees(degrees))
}

// ═══════════════════════════════════════════════════════════════════════════
// SPECIAL ANALOG FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Sigmoid function: 1 / (1 + e^(-x)), scaled to analog range.
///
/// Output naturally falls in (0, 1), then scaled to current analog range.
#[must_use]
pub fn analog_sigmoid(x: f64) -> f64 {
    let config = get_analog_config();
    let sigmoid = 1.0 / (1.0 + (-x).exp());
    // Scale from (0,1) to analog range
    config.denormalize(sigmoid)
}

/// ReLU (Rectified Linear Unit): max(0, x), clamped.
#[inline]
#[must_use]
pub fn analog_relu(x: f64) -> f64 {
    let config = get_analog_config();
    config.clamp(x.max(0.0))
}

/// Leaky ReLU: x if x > 0, else alpha * x.
#[inline]
#[must_use]
pub fn analog_leaky_relu(x: f64, alpha: f64) -> f64 {
    let config = get_analog_config();
    if x > 0.0 {
        config.clamp(x)
    } else {
        config.clamp(alpha * x)
    }
}

/// Softplus: ln(1 + e^x), clamped.
#[inline]
#[must_use]
pub fn analog_softplus(x: f64) -> f64 {
    clamp_analog((1.0 + x.exp()).ln())
}

/// Gaussian function: e^(-x²), clamped.
#[inline]
#[must_use]
pub fn analog_gaussian(x: f64) -> f64 {
    clamp_analog((-x * x).exp())
}

/// Sinc function: sin(x)/x, clamped.
///
/// Returns 1 at x = 0.
#[inline]
#[must_use]
pub fn analog_sinc(x: f64) -> f64 {
    if x.abs() < f64::EPSILON {
        clamp_analog(1.0)
    } else {
        clamp_analog(x.sin() / x)
    }
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

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn basic_trig() {
        setup();
        let config = AnalogConfig::new(AnalogMode::FloatUnit);

        // sin and cos naturally in [-1, 1]
        assert!(config.sin(0.0).abs() < f64::EPSILON);
        assert!(approx_eq(config.cos(0.0), 1.0));
        assert!(approx_eq(config.sin(FRAC_PI_2), 1.0));
    }

    #[test]
    fn trig_clamping() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // sin/cos in [-1, 1], but we're in Int201 mode
        // so they'll just return the sin/cos values directly
        let s = config.sin(PI / 6.0); // 0.5
        assert!((s - 0.5).abs() < 0.01);
    }

    #[test]
    fn angle_conversions() {
        assert!(approx_eq(deg_to_rad(180.0), PI));
        assert!(approx_eq(rad_to_deg(PI), 180.0));
        assert!(approx_eq(deg_to_rad(360.0), TAU));
    }

    #[test]
    fn angle_wrapping() {
        assert!(approx_eq(wrap_radians(TAU + 1.0), 1.0));
        assert!(approx_eq(wrap_radians(-1.0), TAU - 1.0));
        assert!(approx_eq(wrap_degrees(400.0), 40.0));
        assert!(approx_eq(wrap_degrees(-30.0), 330.0));
    }

    #[test]
    fn symmetric_wrapping() {
        assert!(approx_eq(wrap_radians_symmetric(PI + 0.5), -PI + 0.5));
        assert!(approx_eq(wrap_degrees_symmetric(200.0), -160.0));
    }

    #[test]
    fn activation_functions() {
        setup();
        set_analog_mode(AnalogMode::FloatUnit);

        // Sigmoid at 0 is 0.5
        let sig = analog_sigmoid(0.0);
        assert!((sig - 0.0).abs() < 0.01); // Scaled to [-1, 1] range -> 0

        // ReLU
        assert_eq!(analog_relu(0.5), 0.5);
        assert_eq!(analog_relu(-0.5), 0.0);

        // tanh
        assert!(analog_tanh(0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn special_functions() {
        setup();

        // Sinc at 0 is 1
        assert!(approx_eq(analog_sinc(0.0), 1.0));

        // Gaussian at 0 is 1
        assert!(approx_eq(analog_gaussian(0.0), 1.0));
    }
}
