//! # WofLang Analog Computing Module
//!
//! Bounded continuum arithmetic for the WofLang stack machine.
//!
//! ## Philosophy
//!
//! Traditional computing is digital: values overflow, wrap around, or produce
//! unexpected results at boundaries. **Analog mode** embraces bounded computation
//! where values saturate at limits, mimicking physical analog systems like
//! voltage rails in circuits or control signals in synthesizers.
//!
//! This isn't just a constraint—it's a fundamentally different computational
//! paradigm that's perfect for:
//!
//! - **Signal processing** (audio, control voltages, CV/gate)
//! - **Neural networks** (activation functions naturally saturate)
//! - **Physics simulations** (bounded physical quantities)
//! - **Game development** (health bars, percentage stats)
//! - **Control systems** (joystick input, motor control)
//!
//! ## Quick Start
//!
//! ```rust
//! use woflang_analog::{AnalogMode, AnalogConfig};
//! use woflang_analog::math::*;
//!
//! // Create a config for classic -100 to +100 range
//! let config = AnalogConfig::new(AnalogMode::Int201);
//!
//! // Normal math works as expected
//! assert_eq!(config.add(50.0, 30.0), 80.0);
//!
//! // But results saturate at boundaries!
//! assert_eq!(config.add(80.0, 50.0), 100.0);  // Would be 130, saturates to 100
//! assert_eq!(config.mul(50.0, 50.0), 100.0);  // Would be 2500, saturates to 100
//! ```
//!
//! ## Modes
//!
//! | Mode | Range | Use Case |
//! |------|-------|----------|
//! | `Int201` | [-100, +100] | General purpose, percentage-like |
//! | `Int2001` | [-1000, +1000] | Extended precision |
//! | `FloatUnit` | [-1.0, +1.0] | Normalized signals, neural nets |
//! | `FloatCustom` | [min, max] | User-defined domains |
//!
//! ## Global vs Local State
//!
//! The module supports both approaches:
//!
//! ```rust
//! use woflang_analog::{set_analog_mode, AnalogMode, clamp_analog};
//! use woflang_analog::math::analog_add;
//!
//! // Global state approach (thread-local)
//! set_analog_mode(AnalogMode::Int201);
//! let result = analog_add(80.0, 50.0);  // Uses global config
//!
//! // Local config approach (explicit)
//! use woflang_analog::AnalogConfig;
//! let config = AnalogConfig::new(AnalogMode::FloatUnit);
//! let result = config.add(0.8, 0.5);    // Uses local config
//! ```
//!
//! ## Feature Categories
//!
//! - **`mode`**: Core mode definitions, clamping, configuration
//! - **`math`**: Basic arithmetic (add, sub, mul, div, sqrt, pow, lerp)
//! - **`trig`**: Trigonometric functions with clamping (sin, cos, tan, atan2)
//! - **`linear`**: Linear algebra (dot product, magnitude, distance, normalize)
//! - **`ops`**: WofLang interpreter integration (opcodes 7000-7999)
//!
//! ## Example: Neural Network Activation
//!
//! ```rust
//! use woflang_analog::{AnalogMode, AnalogConfig};
//! use woflang_analog::trig::*;
//!
//! let config = AnalogConfig::new(AnalogMode::FloatUnit);
//!
//! // Neuron: weighted sum -> activation
//! let input = 0.7;
//! let weight = 1.5;
//! let bias = -0.3;
//!
//! let weighted = config.mul(input, weight);  // 0.7 * 1.5 = 1.0 (clamped from 1.05)
//! let biased = config.add(weighted, bias);   // 1.0 + (-0.3) = 0.7
//! let activated = config.tanh(biased);       // tanh(0.7) ≈ 0.604
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod linear;
pub mod math;
pub mod mode;
pub mod ops;
pub mod test_suite;
pub mod trig;

// Re-export primary types at crate root for convenience
pub use mode::{
    analog_max, analog_min, analog_status, clamp_analog, clamp_analog_rounded, get_analog_config,
    reset_analog_mode, set_analog_custom, set_analog_mode, AnalogConfig, AnalogMode,
};

/// Prelude module for convenient imports.
///
/// ```rust
/// use woflang_analog::prelude::*;
/// ```
pub mod prelude {
    pub use crate::linear::{
        analog_distance_2d, analog_distance_3d, analog_dot_2d, analog_dot_3d, analog_magnitude_2d,
        analog_magnitude_3d, analog_normalize_2d, analog_normalize_3d,
    };
    pub use crate::math::{
        analog_abs, analog_add, analog_div, analog_fma, analog_lerp, analog_mod, analog_mul,
        analog_neg, analog_pow, analog_sqrt, analog_sub,
    };
    pub use crate::mode::{
        analog_max, analog_min, analog_status, clamp_analog, get_analog_config, reset_analog_mode,
        set_analog_custom, set_analog_mode, AnalogConfig, AnalogMode,
    };
    pub use crate::trig::{
        analog_acos, analog_asin, analog_atan, analog_atan2, analog_cos, analog_exp,
        analog_gaussian, analog_ln, analog_relu, analog_sigmoid, analog_sin, analog_sinc,
        analog_softplus, analog_tan, analog_tanh, deg_to_rad, rad_to_deg, wrap_degrees,
        wrap_radians,
    };
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod integration_tests {
    use super::prelude::*;

    #[test]
    fn full_workflow_int201() {
        reset_analog_mode();
        set_analog_mode(AnalogMode::Int201);

        // Basic operations
        assert_eq!(analog_add(50.0, 30.0), 80.0);
        assert_eq!(analog_add(80.0, 50.0), 100.0); // Saturates

        // Trig
        let sin_half = analog_sin(std::f64::consts::FRAC_PI_6);
        assert!((sin_half - 0.5).abs() < 0.01);

        // Linear algebra
        assert_eq!(analog_dot_2d(3.0, 4.0, 1.0, 2.0), 11.0);
        assert_eq!(analog_magnitude_2d(3.0, 4.0), 5.0);
    }

    #[test]
    fn full_workflow_float_unit() {
        reset_analog_mode();
        set_analog_mode(AnalogMode::FloatUnit);

        // In float_unit mode, values clamp to [-1, 1]
        assert_eq!(analog_add(0.8, 0.5), 1.0); // Would be 1.3, saturates
        assert_eq!(analog_add(-0.8, -0.5), -1.0); // Would be -1.3, saturates

        // Neural network activation
        assert!(analog_tanh(0.0).abs() < f64::EPSILON);
        assert!(analog_sigmoid(0.0).abs() < 0.01); // Centered at 0 in our range
    }

    #[test]
    fn custom_mode() {
        reset_analog_mode();
        set_analog_custom(-5.0, 5.0);

        assert_eq!(analog_max(), 5.0);
        assert_eq!(analog_min(), -5.0);
        assert_eq!(clamp_analog(10.0), 5.0);
        assert_eq!(clamp_analog(-10.0), -5.0);
    }

    #[test]
    fn config_based_workflow() {
        let config = AnalogConfig::new(AnalogMode::Int201);

        // Using config methods
        assert_eq!(config.add(80.0, 50.0), 100.0);
        assert_eq!(config.mul(50.0, 50.0), 100.0);
        assert_eq!(config.lerp(-100.0, 100.0, 0.5), 0.0);

        // Deadzone
        assert_eq!(config.deadzone(5.0, 10.0), 0.0);
        assert_eq!(config.deadzone(15.0, 10.0), 15.0);

        // Vector ops
        let (nx, ny) = config.normalize_2d(3.0, 4.0);
        assert!((nx - 0.6).abs() < 0.01);
        assert!((ny - 0.8).abs() < 0.01);
    }

    #[test]
    fn synthesizer_use_case() {
        // Simulating a synthesizer control voltage scenario
        let config = AnalogConfig::new_custom(-5.0, 5.0); // ±5V like Eurorack

        // LFO generating a sine wave
        let lfo_phase = std::f64::consts::PI / 4.0; // 45 degrees
        let lfo_output = config.sin(lfo_phase);

        // Modulating a filter cutoff
        let base_cutoff = 0.0;
        let mod_depth = 2.0;
        let modulated = config.add(base_cutoff, config.mul(lfo_output, mod_depth));

        // Result stays within ±5V
        assert!(modulated >= -5.0 && modulated <= 5.0);
    }
}
