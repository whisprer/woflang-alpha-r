//! Analog Mode - Core definitions for bounded continuum computing.
//!
//! WofLang's analog mode represents values on a continuous spectrum
//! with automatic clamping at the boundaries. This creates a fundamentally
//! different computing paradigm where overflow becomes saturation.
//!
//! # Philosophy
//!
//! Traditional computing is digital - values can overflow, wrap around,
//! or produce unexpected results at boundaries. Analog mode embraces
//! bounded computation where values saturate at limits, mimicking
//! physical analog systems like voltage rails in circuits.
//!
//! # Modes
//!
//! - `Int201`: Integer-like mode with range [-100, +100] (201 values)
//! - `Int2001`: Extended integer mode with range [-1000, +1000]
//! - `FloatUnit`: Normalized float mode with range [-1.0, +1.0]
//! - `FloatCustom`: User-defined float range
//!
//! # Example
//!
//! ```
//! use woflang_analog::{AnalogMode, AnalogConfig, clamp_analog};
//!
//! // Set up classic -100 to +100 mode
//! let config = AnalogConfig::new(AnalogMode::Int201);
//!
//! // Values saturate at boundaries
//! assert_eq!(config.clamp(150.0), 100.0);   // Saturates at max
//! assert_eq!(config.clamp(-200.0), -100.0); // Saturates at min
//! assert_eq!(config.clamp(42.0), 42.0);     // Within range, unchanged
//! ```

use std::cell::RefCell;
use std::fmt;

/// Analog computing mode variants.
///
/// Each mode defines a different value range for bounded computation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum AnalogMode {
    /// Range: [-100, +100] inclusive (201 discrete values)
    /// 
    /// Classic WofLang analog mode. Ideal for percentage-like values,
    /// control signals, and general-purpose bounded arithmetic.
    #[default]
    Int201 = 0,

    /// Range: [-1000, +1000] inclusive (2001 discrete values)
    ///
    /// Extended precision mode for applications needing more headroom.
    Int2001 = 1,

    /// Range: [-1.0, +1.0] floating point
    ///
    /// Normalized mode ideal for signal processing, neural networks,
    /// and mathematical operations requiring unit scaling.
    FloatUnit = 2,

    /// User-defined float range [custom_min, custom_max]
    ///
    /// Maximum flexibility for domain-specific applications.
    FloatCustom = 3,
}

impl AnalogMode {
    /// Returns true if this mode uses integer-like semantics.
    #[inline]
    #[must_use]
    pub const fn is_integer_mode(self) -> bool {
        matches!(self, Self::Int201 | Self::Int2001)
    }

    /// Returns the minimum value for this mode (assuming default custom range).
    #[inline]
    #[must_use]
    pub const fn default_min(self) -> f64 {
        match self {
            Self::Int201 => -100.0,
            Self::Int2001 => -1000.0,
            Self::FloatUnit => -1.0,
            Self::FloatCustom => -1.0, // Default custom range
        }
    }

    /// Returns the maximum value for this mode (assuming default custom range).
    #[inline]
    #[must_use]
    pub const fn default_max(self) -> f64 {
        match self {
            Self::Int201 => 100.0,
            Self::Int2001 => 1000.0,
            Self::FloatUnit => 1.0,
            Self::FloatCustom => 1.0, // Default custom range
        }
    }

    /// Returns the range span (max - min) for this mode.
    #[inline]
    #[must_use]
    pub const fn default_range(self) -> f64 {
        self.default_max() - self.default_min()
    }
}

impl fmt::Display for AnalogMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int201 => write!(f, "INT_201 [-100, +100]"),
            Self::Int2001 => write!(f, "INT_2001 [-1000, +1000]"),
            Self::FloatUnit => write!(f, "FLOAT_UNIT [-1.0, +1.0]"),
            Self::FloatCustom => write!(f, "FLOAT_CUSTOM"),
        }
    }
}

/// Configuration for analog computing mode.
///
/// Holds the current mode and custom range (if applicable).
/// All analog operations use this configuration to determine
/// value bounds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalogConfig {
    /// The current analog mode
    pub mode: AnalogMode,
    /// Custom minimum (only used in FloatCustom mode)
    pub custom_min: f64,
    /// Custom maximum (only used in FloatCustom mode)
    pub custom_max: f64,
}

impl Default for AnalogConfig {
    fn default() -> Self {
        Self {
            mode: AnalogMode::Int201,
            custom_min: -1.0,
            custom_max: 1.0,
        }
    }
}

impl AnalogConfig {
    /// Create a new configuration with the given mode.
    ///
    /// For `FloatCustom` mode, use `new_custom()` instead.
    #[inline]
    #[must_use]
    pub const fn new(mode: AnalogMode) -> Self {
        Self {
            mode,
            custom_min: -1.0,
            custom_max: 1.0,
        }
    }

    /// Create a custom float mode with specified bounds.
    ///
    /// # Panics
    ///
    /// Panics if `min >= max`.
    #[must_use]
    pub fn new_custom(min: f64, max: f64) -> Self {
        assert!(min < max, "Custom analog range must have min < max");
        Self {
            mode: AnalogMode::FloatCustom,
            custom_min: min,
            custom_max: max,
        }
    }

    /// Get the minimum value for the current mode.
    #[inline]
    #[must_use]
    pub const fn min(&self) -> f64 {
        match self.mode {
            AnalogMode::Int201 => -100.0,
            AnalogMode::Int2001 => -1000.0,
            AnalogMode::FloatUnit => -1.0,
            AnalogMode::FloatCustom => self.custom_min,
        }
    }

    /// Get the maximum value for the current mode.
    #[inline]
    #[must_use]
    pub const fn max(&self) -> f64 {
        match self.mode {
            AnalogMode::Int201 => 100.0,
            AnalogMode::Int2001 => 1000.0,
            AnalogMode::FloatUnit => 1.0,
            AnalogMode::FloatCustom => self.custom_max,
        }
    }

    /// Get the range span (max - min).
    #[inline]
    #[must_use]
    pub const fn range(&self) -> f64 {
        self.max() - self.min()
    }

    /// Get the midpoint of the range.
    #[inline]
    #[must_use]
    pub const fn midpoint(&self) -> f64 {
        (self.min() + self.max()) / 2.0
    }

    /// Returns true if this mode uses integer-like semantics.
    #[inline]
    #[must_use]
    pub const fn is_integer_mode(&self) -> bool {
        self.mode.is_integer_mode()
    }

    /// Clamp a value to the current mode's range.
    ///
    /// This is the core analog operation - values saturate at boundaries
    /// rather than overflowing or wrapping.
    ///
    /// # Examples
    ///
    /// ```
    /// use woflang_analog::{AnalogMode, AnalogConfig};
    ///
    /// let config = AnalogConfig::new(AnalogMode::Int201);
    /// assert_eq!(config.clamp(150.0), 100.0);
    /// assert_eq!(config.clamp(-150.0), -100.0);
    /// assert_eq!(config.clamp(50.0), 50.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn clamp(&self, value: f64) -> f64 {
        value.clamp(self.min(), self.max())
    }

    /// Clamp and optionally round for integer modes.
    #[inline]
    #[must_use]
    pub fn clamp_rounded(&self, value: f64) -> f64 {
        let clamped = self.clamp(value);
        if self.is_integer_mode() {
            clamped.round()
        } else {
            clamped
        }
    }

    /// Remap a value from one range to another, clamping the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use woflang_analog::{AnalogMode, AnalogConfig};
    ///
    /// let config = AnalogConfig::new(AnalogMode::Int201);
    ///
    /// // Map 0.5 from [0,1] to [-100,100]
    /// let result = config.remap(0.5, 0.0, 1.0, -100.0, 100.0);
    /// assert_eq!(result, 0.0);
    /// ```
    #[must_use]
    pub fn remap(&self, value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
        let from_range = from_max - from_min;
        if from_range.abs() < f64::EPSILON {
            return self.clamp(to_min);
        }
        let t = (value - from_min) / from_range;
        let remapped = to_min + t * (to_max - to_min);
        self.clamp(remapped)
    }

    /// Apply a deadzone - values within the threshold become zero.
    ///
    /// Useful for joystick input, noise rejection, etc.
    ///
    /// # Examples
    ///
    /// ```
    /// use woflang_analog::{AnalogMode, AnalogConfig};
    ///
    /// let config = AnalogConfig::new(AnalogMode::Int201);
    ///
    /// assert_eq!(config.deadzone(5.0, 10.0), 0.0);   // Within deadzone
    /// assert_eq!(config.deadzone(15.0, 10.0), 15.0); // Outside deadzone
    /// ```
    #[inline]
    #[must_use]
    pub fn deadzone(&self, value: f64, threshold: f64) -> f64 {
        let clamped = self.clamp(value);
        if clamped.abs() < threshold.abs() {
            0.0
        } else {
            clamped
        }
    }

    /// Normalize a value to the [0, 1] range based on current bounds.
    #[inline]
    #[must_use]
    pub fn normalize(&self, value: f64) -> f64 {
        let clamped = self.clamp(value);
        (clamped - self.min()) / self.range()
    }

    /// Convert a normalized [0, 1] value back to the current range.
    #[inline]
    #[must_use]
    pub fn denormalize(&self, normalized: f64) -> f64 {
        let value = self.min() + normalized.clamp(0.0, 1.0) * self.range();
        self.clamp(value)
    }

    /// Get a status string describing the current configuration.
    #[must_use]
    pub fn status(&self) -> String {
        format!(
            "Analog Mode: {}\n  Range: [{}, {}]\n  Span: {}\n  Integer: {}",
            self.mode,
            self.min(),
            self.max(),
            self.range(),
            self.is_integer_mode()
        )
    }
}

impl fmt::Display for AnalogConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mode {
            AnalogMode::FloatCustom => {
                write!(f, "FLOAT_CUSTOM [{}, {}]", self.custom_min, self.custom_max)
            }
            _ => write!(f, "{}", self.mode),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// THREAD-LOCAL STATE
// ═══════════════════════════════════════════════════════════════════════════

thread_local! {
    /// Global analog configuration for the current thread.
    static ANALOG_STATE: RefCell<AnalogConfig> = RefCell::new(AnalogConfig::default());
}

/// Get the current analog configuration.
#[must_use]
pub fn get_analog_config() -> AnalogConfig {
    ANALOG_STATE.with(|state| *state.borrow())
}

/// Set the analog mode (for non-custom modes).
pub fn set_analog_mode(mode: AnalogMode) {
    ANALOG_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.mode = mode;
    });
}

/// Set a custom float mode with specified bounds.
///
/// # Panics
///
/// Panics if `min >= max`.
pub fn set_analog_custom(min: f64, max: f64) {
    assert!(min < max, "Custom analog range must have min < max");
    ANALOG_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.mode = AnalogMode::FloatCustom;
        s.custom_min = min;
        s.custom_max = max;
    });
}

/// Reset analog mode to default (Int201).
pub fn reset_analog_mode() {
    ANALOG_STATE.with(|state| {
        *state.borrow_mut() = AnalogConfig::default();
    });
}

/// Get the current minimum value.
#[inline]
#[must_use]
pub fn analog_min() -> f64 {
    get_analog_config().min()
}

/// Get the current maximum value.
#[inline]
#[must_use]
pub fn analog_max() -> f64 {
    get_analog_config().max()
}

/// Clamp a value using the current global configuration.
///
/// This is the primary interface for most analog operations.
#[inline]
#[must_use]
pub fn clamp_analog(value: f64) -> f64 {
    get_analog_config().clamp(value)
}

/// Clamp and round (for integer modes) using the current global configuration.
#[inline]
#[must_use]
pub fn clamp_analog_rounded(value: f64) -> f64 {
    get_analog_config().clamp_rounded(value)
}

/// Get a status string for the current global configuration.
#[must_use]
pub fn analog_status() -> String {
    get_analog_config().status()
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_defaults() {
        assert_eq!(AnalogMode::Int201.default_min(), -100.0);
        assert_eq!(AnalogMode::Int201.default_max(), 100.0);
        assert_eq!(AnalogMode::FloatUnit.default_range(), 2.0);
    }

    #[test]
    fn config_clamping() {
        let config = AnalogConfig::new(AnalogMode::Int201);
        
        assert_eq!(config.clamp(50.0), 50.0);
        assert_eq!(config.clamp(150.0), 100.0);
        assert_eq!(config.clamp(-150.0), -100.0);
        assert_eq!(config.clamp(0.0), 0.0);
    }

    #[test]
    fn custom_mode() {
        let config = AnalogConfig::new_custom(-5.0, 5.0);
        
        assert_eq!(config.min(), -5.0);
        assert_eq!(config.max(), 5.0);
        assert_eq!(config.clamp(10.0), 5.0);
        assert_eq!(config.clamp(-10.0), -5.0);
    }

    #[test]
    fn deadzone() {
        let config = AnalogConfig::new(AnalogMode::Int201);
        
        assert_eq!(config.deadzone(5.0, 10.0), 0.0);
        assert_eq!(config.deadzone(-5.0, 10.0), 0.0);
        assert_eq!(config.deadzone(15.0, 10.0), 15.0);
        assert_eq!(config.deadzone(-15.0, 10.0), -15.0);
    }

    #[test]
    fn normalization() {
        let config = AnalogConfig::new(AnalogMode::Int201);
        
        assert!((config.normalize(0.0) - 0.5).abs() < f64::EPSILON);
        assert!((config.normalize(100.0) - 1.0).abs() < f64::EPSILON);
        assert!((config.normalize(-100.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn remap() {
        let config = AnalogConfig::new(AnalogMode::Int201);
        
        // Map 0.5 from [0, 1] to [-100, 100]
        let result = config.remap(0.5, 0.0, 1.0, -100.0, 100.0);
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn global_state() {
        reset_analog_mode();
        assert_eq!(get_analog_config().mode, AnalogMode::Int201);
        
        set_analog_mode(AnalogMode::FloatUnit);
        assert_eq!(get_analog_config().mode, AnalogMode::FloatUnit);
        
        set_analog_custom(-10.0, 10.0);
        assert_eq!(get_analog_config().mode, AnalogMode::FloatCustom);
        assert_eq!(analog_min(), -10.0);
        assert_eq!(analog_max(), 10.0);
        
        reset_analog_mode();
    }

    #[test]
    fn is_integer_mode() {
        assert!(AnalogMode::Int201.is_integer_mode());
        assert!(AnalogMode::Int2001.is_integer_mode());
        assert!(!AnalogMode::FloatUnit.is_integer_mode());
        assert!(!AnalogMode::FloatCustom.is_integer_mode());
    }
}
