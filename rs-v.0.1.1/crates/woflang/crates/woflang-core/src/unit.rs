//! Unit information for dimensional analysis.
//!
//! Woflang supports attaching units to numeric values for physical
//! computations and dimensional analysis.

use std::fmt;

/// Information about a unit attached to a value.
///
/// Units support basic scaling (e.g., kilometers vs meters) and
/// can be used for dimensional analysis in scientific computations.
#[derive(Clone, Debug, PartialEq)]
pub struct UnitInfo {
    /// Human-readable name of the unit (e.g., "m", "kg", "s").
    pub name: String,
    /// Scale factor relative to the base unit.
    pub scale: f64,
}

impl UnitInfo {
    /// Create a new unit with the given name and scale.
    #[inline]
    #[must_use]
    pub fn new(name: impl Into<String>, scale: f64) -> Self {
        Self {
            name: name.into(),
            scale,
        }
    }

    /// Create a base unit (scale = 1.0).
    #[inline]
    #[must_use]
    pub fn base(name: impl Into<String>) -> Self {
        Self::new(name, 1.0)
    }

    /// Check if two units are compatible (same base unit).
    #[inline]
    #[must_use]
    pub fn is_compatible(&self, other: &Self) -> bool {
        // Simple compatibility: same name after normalization
        self.name.to_lowercase() == other.name.to_lowercase()
    }

    /// Convert a value from this unit to another compatible unit.
    #[inline]
    #[must_use]
    pub fn convert(&self, value: f64, to: &Self) -> Option<f64> {
        if self.is_compatible(to) {
            Some(value * self.scale / to.scale)
        } else {
            None
        }
    }
}

impl fmt::Display for UnitInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for UnitInfo {
    fn default() -> Self {
        Self::base("1")
    }
}

/// Common SI units as constants.
#[allow(dead_code)]
pub mod si {
    use super::UnitInfo;

    /// Meter (base length unit).
    #[must_use]
    pub fn meter() -> UnitInfo {
        UnitInfo::base("m")
    }

    /// Kilometer.
    #[must_use]
    pub fn kilometer() -> UnitInfo {
        UnitInfo::new("km", 1000.0)
    }

    /// Centimeter.
    #[must_use]
    pub fn centimeter() -> UnitInfo {
        UnitInfo::new("cm", 0.01)
    }

    /// Kilogram (base mass unit).
    #[must_use]
    pub fn kilogram() -> UnitInfo {
        UnitInfo::base("kg")
    }

    /// Second (base time unit).
    #[must_use]
    pub fn second() -> UnitInfo {
        UnitInfo::base("s")
    }

    /// Kelvin (base temperature unit).
    #[must_use]
    pub fn kelvin() -> UnitInfo {
        UnitInfo::base("K")
    }

    /// Celsius.
    #[must_use]
    pub fn celsius() -> UnitInfo {
        UnitInfo::new("°C", 1.0) // Offset conversion handled separately
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_conversion() {
        let km = si::kilometer();
        let m = si::meter();

        assert!(km.is_compatible(&m));
        let result = km.convert(1.0, &m);
        assert!((result.unwrap() - 1000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn incompatible_units() {
        let m = si::meter();
        let kg = si::kilogram();

        assert!(!m.is_compatible(&kg));
        assert!(m.convert(1.0, &kg).is_none());
    }
}
