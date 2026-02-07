//! Analog linear algebra operations.
//!
//! Vector operations with automatic clamping for 2D and 3D computations.
//! Includes dot products, magnitudes, distances, normalization, and
//! angle calculations.
//!
//! # Examples
//!
//! ```
//! use woflang_analog::linear::*;
//! use woflang_analog::{AnalogMode, AnalogConfig};
//!
//! let config = AnalogConfig::new(AnalogMode::Int201);
//!
//! // Vector operations
//! let dot = config.dot_2d(3.0, 4.0, 1.0, 2.0);  // 3*1 + 4*2 = 11
//! let mag = config.magnitude_2d(3.0, 4.0);       // sqrt(9+16) = 5
//! ```

use crate::mode::{clamp_analog, get_analog_config, AnalogConfig};
use crate::trig::wrap_radians;

// ═══════════════════════════════════════════════════════════════════════════
// 2D VECTOR OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

impl AnalogConfig {
    /// 2D dot product with analog clamping.
    #[inline]
    #[must_use]
    pub fn dot_2d(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        self.clamp(x1 * x2 + y1 * y2)
    }

    /// 2D vector magnitude (length) with analog clamping.
    #[inline]
    #[must_use]
    pub fn magnitude_2d(&self, x: f64, y: f64) -> f64 {
        self.clamp(x.hypot(y))
    }

    /// 2D Euclidean distance with analog clamping.
    #[inline]
    #[must_use]
    pub fn distance_2d(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        let dx = x2 - x1;
        let dy = y2 - y1;
        self.magnitude_2d(dx, dy)
    }

    /// 2D Manhattan distance with analog clamping.
    #[inline]
    #[must_use]
    pub fn manhattan_2d(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        self.clamp((x2 - x1).abs() + (y2 - y1).abs())
    }

    /// Normalize a 2D vector, clamping components.
    ///
    /// Returns (0, 0) for zero-length vectors.
    #[must_use]
    pub fn normalize_2d(&self, x: f64, y: f64) -> (f64, f64) {
        let mag = x.hypot(y);
        if mag < f64::EPSILON {
            (0.0, 0.0)
        } else {
            (self.clamp(x / mag), self.clamp(y / mag))
        }
    }

    /// Angle between two 2D vectors in radians, wrapped.
    #[must_use]
    pub fn angle_between_2d(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        let dot = x1 * x2 + y1 * y2;
        let mag1 = x1.hypot(y1);
        let mag2 = x2.hypot(y2);

        if mag1 < f64::EPSILON || mag2 < f64::EPSILON {
            return 0.0;
        }

        let cos_theta = (dot / (mag1 * mag2)).clamp(-1.0, 1.0);
        wrap_radians(cos_theta.acos())
    }

    /// 2D cross product (returns scalar z-component).
    #[inline]
    #[must_use]
    pub fn cross_2d(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        self.clamp(x1 * y2 - y1 * x2)
    }

    /// Project vector (x1, y1) onto vector (x2, y2).
    #[must_use]
    pub fn project_2d(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
        let dot = x1 * x2 + y1 * y2;
        let mag_sq = x2 * x2 + y2 * y2;

        if mag_sq < f64::EPSILON {
            return (0.0, 0.0);
        }

        let scale = dot / mag_sq;
        (self.clamp(x2 * scale), self.clamp(y2 * scale))
    }

    /// Reflect vector (x, y) around normal (nx, ny).
    #[must_use]
    pub fn reflect_2d(&self, x: f64, y: f64, nx: f64, ny: f64) -> (f64, f64) {
        let dot = 2.0 * (x * nx + y * ny);
        (self.clamp(x - dot * nx), self.clamp(y - dot * ny))
    }

    /// Rotate a 2D vector by angle (radians).
    #[must_use]
    pub fn rotate_2d(&self, x: f64, y: f64, angle: f64) -> (f64, f64) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        (
            self.clamp(x * cos_a - y * sin_a),
            self.clamp(x * sin_a + y * cos_a),
        )
    }

    /// Linear interpolation between two 2D points.
    #[must_use]
    pub fn lerp_2d(&self, x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> (f64, f64) {
        let t_clamped = t.clamp(0.0, 1.0);
        (
            self.clamp(x1 + (x2 - x1) * t_clamped),
            self.clamp(y1 + (y2 - y1) * t_clamped),
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 3D VECTOR OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

impl AnalogConfig {
    /// 3D dot product with analog clamping.
    #[inline]
    #[must_use]
    pub fn dot_3d(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
        self.clamp(x1 * x2 + y1 * y2 + z1 * z2)
    }

    /// 3D vector magnitude with analog clamping.
    #[inline]
    #[must_use]
    pub fn magnitude_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        self.clamp((x * x + y * y + z * z).sqrt())
    }

    /// 3D Euclidean distance with analog clamping.
    #[inline]
    #[must_use]
    pub fn distance_3d(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dz = z2 - z1;
        self.magnitude_3d(dx, dy, dz)
    }

    /// 3D Manhattan distance with analog clamping.
    #[inline]
    #[must_use]
    pub fn manhattan_3d(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
        self.clamp((x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs())
    }

    /// Normalize a 3D vector, clamping components.
    #[must_use]
    pub fn normalize_3d(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let mag = (x * x + y * y + z * z).sqrt();
        if mag < f64::EPSILON {
            (0.0, 0.0, 0.0)
        } else {
            (
                self.clamp(x / mag),
                self.clamp(y / mag),
                self.clamp(z / mag),
            )
        }
    }

    /// 3D cross product with analog clamping.
    #[must_use]
    pub fn cross_3d(
        &self,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) -> (f64, f64, f64) {
        (
            self.clamp(y1 * z2 - z1 * y2),
            self.clamp(z1 * x2 - x1 * z2),
            self.clamp(x1 * y2 - y1 * x2),
        )
    }

    /// Project 3D vector onto another.
    #[must_use]
    pub fn project_3d(
        &self,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) -> (f64, f64, f64) {
        let dot = x1 * x2 + y1 * y2 + z1 * z2;
        let mag_sq = x2 * x2 + y2 * y2 + z2 * z2;

        if mag_sq < f64::EPSILON {
            return (0.0, 0.0, 0.0);
        }

        let scale = dot / mag_sq;
        (
            self.clamp(x2 * scale),
            self.clamp(y2 * scale),
            self.clamp(z2 * scale),
        )
    }

    /// Angle between two 3D vectors in radians.
    #[must_use]
    pub fn angle_between_3d(
        &self,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) -> f64 {
        let dot = x1 * x2 + y1 * y2 + z1 * z2;
        let mag1 = (x1 * x1 + y1 * y1 + z1 * z1).sqrt();
        let mag2 = (x2 * x2 + y2 * y2 + z2 * z2).sqrt();

        if mag1 < f64::EPSILON || mag2 < f64::EPSILON {
            return 0.0;
        }

        let cos_theta = (dot / (mag1 * mag2)).clamp(-1.0, 1.0);
        wrap_radians(cos_theta.acos())
    }

    /// Linear interpolation between two 3D points.
    #[must_use]
    pub fn lerp_3d(
        &self,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
        t: f64,
    ) -> (f64, f64, f64) {
        let t_clamped = t.clamp(0.0, 1.0);
        (
            self.clamp(x1 + (x2 - x1) * t_clamped),
            self.clamp(y1 + (y2 - y1) * t_clamped),
            self.clamp(z1 + (z2 - z1) * t_clamped),
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COORDINATE TRANSFORMATIONS
// ═══════════════════════════════════════════════════════════════════════════

impl AnalogConfig {
    /// Convert Cartesian (x, y) to polar (r, theta).
    #[must_use]
    pub fn cartesian_to_polar(&self, x: f64, y: f64) -> (f64, f64) {
        let r = self.clamp(x.hypot(y));
        let theta = wrap_radians(y.atan2(x));
        (r, theta)
    }

    /// Convert polar (r, theta) to Cartesian (x, y).
    #[must_use]
    pub fn polar_to_cartesian(&self, r: f64, theta: f64) -> (f64, f64) {
        (self.clamp(r * theta.cos()), self.clamp(r * theta.sin()))
    }

    /// Convert Cartesian (x, y, z) to spherical (r, theta, phi).
    #[must_use]
    pub fn cartesian_to_spherical(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let r = self.magnitude_3d(x, y, z);
        if r < f64::EPSILON {
            return (0.0, 0.0, 0.0);
        }
        let theta = wrap_radians(y.atan2(x));
        let phi = (z / r).clamp(-1.0, 1.0).acos();
        (r, theta, phi)
    }

    /// Convert spherical (r, theta, phi) to Cartesian (x, y, z).
    #[must_use]
    pub fn spherical_to_cartesian(&self, r: f64, theta: f64, phi: f64) -> (f64, f64, f64) {
        let sin_phi = phi.sin();
        (
            self.clamp(r * sin_phi * theta.cos()),
            self.clamp(r * sin_phi * theta.sin()),
            self.clamp(r * phi.cos()),
        )
    }

    /// Convert Cartesian (x, y, z) to cylindrical (r, theta, z).
    #[must_use]
    pub fn cartesian_to_cylindrical(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let (r, theta) = self.cartesian_to_polar(x, y);
        (r, theta, self.clamp(z))
    }

    /// Convert cylindrical (r, theta, z) to Cartesian (x, y, z).
    #[must_use]
    pub fn cylindrical_to_cartesian(&self, r: f64, theta: f64, z: f64) -> (f64, f64, f64) {
        let (x, y) = self.polar_to_cartesian(r, theta);
        (x, y, self.clamp(z))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GLOBAL STATE FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// 2D dot product using global configuration.
#[inline]
#[must_use]
pub fn analog_dot_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    clamp_analog(x1 * x2 + y1 * y2)
}

/// 3D dot product using global configuration.
#[inline]
#[must_use]
pub fn analog_dot_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    clamp_analog(x1 * x2 + y1 * y2 + z1 * z2)
}

/// 2D magnitude using global configuration.
#[inline]
#[must_use]
pub fn analog_magnitude_2d(x: f64, y: f64) -> f64 {
    clamp_analog(x.hypot(y))
}

/// 3D magnitude using global configuration.
#[inline]
#[must_use]
pub fn analog_magnitude_3d(x: f64, y: f64, z: f64) -> f64 {
    clamp_analog((x * x + y * y + z * z).sqrt())
}

/// 2D distance using global configuration.
#[inline]
#[must_use]
pub fn analog_distance_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    get_analog_config().distance_2d(x1, y1, x2, y2)
}

/// 3D distance using global configuration.
#[inline]
#[must_use]
pub fn analog_distance_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    get_analog_config().distance_3d(x1, y1, z1, x2, y2, z2)
}

/// Normalize 2D vector using global configuration.
#[must_use]
pub fn analog_normalize_2d(x: f64, y: f64) -> (f64, f64) {
    get_analog_config().normalize_2d(x, y)
}

/// Normalize 3D vector using global configuration.
#[must_use]
pub fn analog_normalize_3d(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    get_analog_config().normalize_3d(x, y, z)
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mode::{reset_analog_mode, AnalogMode};
    use std::f64::consts::FRAC_PI_2;

    fn setup() {
        reset_analog_mode();
    }

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn dot_product_2d() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // (3,4) · (1,2) = 3*1 + 4*2 = 11
        assert_eq!(config.dot_2d(3.0, 4.0, 1.0, 2.0), 11.0);

        // Perpendicular vectors have zero dot product
        assert_eq!(config.dot_2d(1.0, 0.0, 0.0, 1.0), 0.0);
    }

    #[test]
    fn magnitude_2d() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // 3-4-5 triangle
        assert_eq!(config.magnitude_2d(3.0, 4.0), 5.0);

        // Unit vectors
        assert!(approx_eq(config.magnitude_2d(1.0, 0.0), 1.0));
    }

    #[test]
    fn distance_2d() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // Distance from origin
        assert_eq!(config.distance_2d(0.0, 0.0, 3.0, 4.0), 5.0);

        // Same point
        assert_eq!(config.distance_2d(5.0, 5.0, 5.0, 5.0), 0.0);
    }

    #[test]
    fn normalize_2d() {
        setup();
        let config = AnalogConfig::new(AnalogMode::FloatUnit);

        let (nx, ny) = config.normalize_2d(3.0, 4.0);
        assert!(approx_eq(nx, 0.6));
        assert!(approx_eq(ny, 0.8));

        // Zero vector stays zero
        let (zx, zy) = config.normalize_2d(0.0, 0.0);
        assert_eq!((zx, zy), (0.0, 0.0));
    }

    #[test]
    fn cross_3d() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // i × j = k
        let (cx, cy, cz) = config.cross_3d(1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert!(approx_eq(cx, 0.0));
        assert!(approx_eq(cy, 0.0));
        assert!(approx_eq(cz, 1.0));
    }

    #[test]
    fn rotate_2d() {
        setup();
        let config = AnalogConfig::new(AnalogMode::FloatUnit);

        // Rotate (1, 0) by 90 degrees -> (0, 1)
        let (rx, ry) = config.rotate_2d(1.0, 0.0, FRAC_PI_2);
        assert!(approx_eq(rx, 0.0));
        assert!(approx_eq(ry, 1.0));
    }

    #[test]
    fn coordinate_conversions() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // Cartesian to polar and back
        let (r, theta) = config.cartesian_to_polar(3.0, 4.0);
        assert!(approx_eq(r, 5.0));

        let (x, y) = config.polar_to_cartesian(r, theta);
        assert!(approx_eq(x, 3.0));
        assert!(approx_eq(y, 4.0));
    }

    #[test]
    fn projection() {
        setup();
        let config = AnalogConfig::new(AnalogMode::Int201);

        // Project (3, 4) onto x-axis (1, 0) -> (3, 0)
        let (px, py) = config.project_2d(3.0, 4.0, 1.0, 0.0);
        assert!(approx_eq(px, 3.0));
        assert!(approx_eq(py, 0.0));
    }
}
