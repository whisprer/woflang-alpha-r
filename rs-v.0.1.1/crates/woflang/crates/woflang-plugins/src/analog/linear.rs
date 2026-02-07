//! WofLang Analog Linear Algebra
//!
//! Dot products, magnitudes, cross products, projections - all analog clamped.

use super::core::{clamp, sqrt};

// ============================================================================
// DOT PRODUCTS
// ============================================================================

/// 2D dot product: x1*x2 + y1*y2, clamped
#[inline]
pub fn dot_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    clamp(x1 * x2 + y1 * y2)
}

/// 3D dot product: x1*x2 + y1*y2 + z1*z2, clamped
#[inline]
pub fn dot_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    clamp(x1 * x2 + y1 * y2 + z1 * z2)
}

/// 4D dot product, clamped
#[inline]
pub fn dot_4d(x1: f64, y1: f64, z1: f64, w1: f64, x2: f64, y2: f64, z2: f64, w2: f64) -> f64 {
    clamp(x1 * x2 + y1 * y2 + z1 * z2 + w1 * w2)
}

/// N-dimensional dot product from slices
pub fn dot_nd(a: &[f64], b: &[f64]) -> f64 {
    let sum: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    clamp(sum)
}

// ============================================================================
// MAGNITUDES (Vector Lengths)
// ============================================================================

/// 2D magnitude (vector length): sqrt(x² + y²), clamped
#[inline]
pub fn magnitude_2d(x: f64, y: f64) -> f64 {
    sqrt(x * x + y * y)
}

/// 3D magnitude (vector length): sqrt(x² + y² + z²), clamped
#[inline]
pub fn magnitude_3d(x: f64, y: f64, z: f64) -> f64 {
    sqrt(x * x + y * y + z * z)
}

/// 4D magnitude, clamped
#[inline]
pub fn magnitude_4d(x: f64, y: f64, z: f64, w: f64) -> f64 {
    sqrt(x * x + y * y + z * z + w * w)
}

/// N-dimensional magnitude from slice
pub fn magnitude_nd(v: &[f64]) -> f64 {
    let sum_sq: f64 = v.iter().map(|x| x * x).sum();
    sqrt(sum_sq)
}

/// Squared magnitude 2D (avoids sqrt when not needed)
#[inline]
pub fn magnitude_sq_2d(x: f64, y: f64) -> f64 {
    clamp(x * x + y * y)
}

/// Squared magnitude 3D (avoids sqrt when not needed)
#[inline]
pub fn magnitude_sq_3d(x: f64, y: f64, z: f64) -> f64 {
    clamp(x * x + y * y + z * z)
}

// ============================================================================
// DISTANCE
// ============================================================================

/// 2D Euclidean distance, clamped
#[inline]
pub fn distance_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    magnitude_2d(x2 - x1, y2 - y1)
}

/// 3D Euclidean distance, clamped
#[inline]
pub fn distance_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    magnitude_3d(x2 - x1, y2 - y1, z2 - z1)
}

/// Squared distance 2D (avoids sqrt)
#[inline]
pub fn distance_sq_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    magnitude_sq_2d(x2 - x1, y2 - y1)
}

/// Squared distance 3D (avoids sqrt)
#[inline]
pub fn distance_sq_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    magnitude_sq_3d(x2 - x1, y2 - y1, z2 - z1)
}

/// Manhattan distance 2D
#[inline]
pub fn manhattan_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    clamp((x2 - x1).abs() + (y2 - y1).abs())
}

/// Manhattan distance 3D
#[inline]
pub fn manhattan_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    clamp((x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs())
}

/// Chebyshev distance 2D (max of absolute differences)
#[inline]
pub fn chebyshev_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    clamp((x2 - x1).abs().max((y2 - y1).abs()))
}

/// Chebyshev distance 3D
#[inline]
pub fn chebyshev_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    clamp((x2 - x1).abs().max((y2 - y1).abs()).max((z2 - z1).abs()))
}

// ============================================================================
// NORMALIZATION
// ============================================================================

/// Normalize a 2D vector to unit length
/// Returns (0, 0) if input is zero vector
pub fn normalize_2d(x: f64, y: f64) -> (f64, f64) {
    let mag = magnitude_2d(x, y);
    if mag == 0.0 {
        (0.0, 0.0)
    } else {
        (clamp(x / mag), clamp(y / mag))
    }
}

/// Normalize a 3D vector to unit length
/// Returns (0, 0, 0) if input is zero vector
pub fn normalize_3d(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let mag = magnitude_3d(x, y, z);
    if mag == 0.0 {
        (0.0, 0.0, 0.0)
    } else {
        (clamp(x / mag), clamp(y / mag), clamp(z / mag))
    }
}

/// Normalize a 4D vector to unit length
pub fn normalize_4d(x: f64, y: f64, z: f64, w: f64) -> (f64, f64, f64, f64) {
    let mag = magnitude_4d(x, y, z, w);
    if mag == 0.0 {
        (0.0, 0.0, 0.0, 0.0)
    } else {
        (clamp(x / mag), clamp(y / mag), clamp(z / mag), clamp(w / mag))
    }
}

// ============================================================================
// CROSS PRODUCT
// ============================================================================

/// 2D cross product (returns scalar - the z component of 3D cross with z=0)
/// Useful for determining left/right orientation
#[inline]
pub fn cross_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    clamp(x1 * y2 - y1 * x2)
}

/// 3D cross product
pub fn cross_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> (f64, f64, f64) {
    (
        clamp(y1 * z2 - z1 * y2),
        clamp(z1 * x2 - x1 * z2),
        clamp(x1 * y2 - y1 * x2),
    )
}

// ============================================================================
// PROJECTION
// ============================================================================

/// Project vector a onto vector b (2D)
/// Returns the component of a in the direction of b
pub fn project_2d(ax: f64, ay: f64, bx: f64, by: f64) -> (f64, f64) {
    let b_mag_sq = magnitude_sq_2d(bx, by);
    if b_mag_sq == 0.0 {
        return (0.0, 0.0);
    }
    let dot = dot_2d(ax, ay, bx, by);
    let scalar = dot / b_mag_sq;
    (clamp(scalar * bx), clamp(scalar * by))
}

/// Project vector a onto vector b (3D)
pub fn project_3d(
    ax: f64, ay: f64, az: f64,
    bx: f64, by: f64, bz: f64,
) -> (f64, f64, f64) {
    let b_mag_sq = magnitude_sq_3d(bx, by, bz);
    if b_mag_sq == 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let dot = dot_3d(ax, ay, az, bx, by, bz);
    let scalar = dot / b_mag_sq;
    (clamp(scalar * bx), clamp(scalar * by), clamp(scalar * bz))
}

/// Reflect vector v around normal n (2D)
/// Assumes n is normalized
pub fn reflect_2d(vx: f64, vy: f64, nx: f64, ny: f64) -> (f64, f64) {
    let dot = dot_2d(vx, vy, nx, ny);
    (
        clamp(vx - 2.0 * dot * nx),
        clamp(vy - 2.0 * dot * ny),
    )
}

/// Reflect vector v around normal n (3D)
/// Assumes n is normalized
pub fn reflect_3d(vx: f64, vy: f64, vz: f64, nx: f64, ny: f64, nz: f64) -> (f64, f64, f64) {
    let dot = dot_3d(vx, vy, vz, nx, ny, nz);
    (
        clamp(vx - 2.0 * dot * nx),
        clamp(vy - 2.0 * dot * ny),
        clamp(vz - 2.0 * dot * nz),
    )
}

// ============================================================================
// ANGLE BETWEEN VECTORS
// ============================================================================

/// Angle between two 2D vectors in radians
pub fn angle_between_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dot = dot_2d(x1, y1, x2, y2);
    let mag1 = magnitude_2d(x1, y1);
    let mag2 = magnitude_2d(x2, y2);
    
    if mag1 == 0.0 || mag2 == 0.0 {
        return 0.0;
    }
    
    // Clamp to [-1, 1] to handle floating point errors
    let cos_theta = (dot / (mag1 * mag2)).max(-1.0).min(1.0);
    clamp(cos_theta.acos())
}

/// Angle between two 3D vectors in radians
pub fn angle_between_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    let dot = dot_3d(x1, y1, z1, x2, y2, z2);
    let mag1 = magnitude_3d(x1, y1, z1);
    let mag2 = magnitude_3d(x2, y2, z2);
    
    if mag1 == 0.0 || mag2 == 0.0 {
        return 0.0;
    }
    
    let cos_theta = (dot / (mag1 * mag2)).max(-1.0).min(1.0);
    clamp(cos_theta.acos())
}

// ============================================================================
// COORDINATE CONVERSION
// ============================================================================

/// Convert 2D Cartesian to polar coordinates (r, theta)
pub fn cartesian_to_polar(x: f64, y: f64) -> (f64, f64) {
    let r = magnitude_2d(x, y);
    let theta = clamp(y.atan2(x));
    (r, theta)
}

/// Convert polar to 2D Cartesian coordinates
pub fn polar_to_cartesian(r: f64, theta: f64) -> (f64, f64) {
    (clamp(r * theta.cos()), clamp(r * theta.sin()))
}

/// Convert 3D Cartesian to spherical coordinates (r, theta, phi)
/// theta: azimuthal angle (in xy-plane from x-axis)
/// phi: polar angle (from z-axis)
pub fn cartesian_to_spherical(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let r = magnitude_3d(x, y, z);
    if r == 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let theta = clamp(y.atan2(x));
    let phi = clamp((z / r).acos());
    (r, theta, phi)
}

/// Convert spherical to 3D Cartesian coordinates
pub fn spherical_to_cartesian(r: f64, theta: f64, phi: f64) -> (f64, f64, f64) {
    let sin_phi = phi.sin();
    (
        clamp(r * sin_phi * theta.cos()),
        clamp(r * sin_phi * theta.sin()),
        clamp(r * phi.cos()),
    )
}

/// Convert 3D Cartesian to cylindrical coordinates (rho, phi, z)
pub fn cartesian_to_cylindrical(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let rho = magnitude_2d(x, y);
    let phi = clamp(y.atan2(x));
    (rho, phi, clamp(z))
}

/// Convert cylindrical to 3D Cartesian coordinates
pub fn cylindrical_to_cartesian(rho: f64, phi: f64, z: f64) -> (f64, f64, f64) {
    (
        clamp(rho * phi.cos()),
        clamp(rho * phi.sin()),
        clamp(z),
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analog::core::{set_mode, AnalogMode};

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.0001
    }

    #[test]
    fn test_dot_2d() {
        set_mode(AnalogMode::Int201);
        assert!(approx_eq(dot_2d(1.0, 0.0, 0.0, 1.0), 0.0)); // Perpendicular
        assert!(approx_eq(dot_2d(1.0, 0.0, 1.0, 0.0), 1.0)); // Parallel
    }

    #[test]
    fn test_magnitude_2d() {
        set_mode(AnalogMode::Int201);
        assert!(approx_eq(magnitude_2d(3.0, 4.0), 5.0)); // 3-4-5 triangle
    }

    #[test]
    fn test_normalize_2d() {
        set_mode(AnalogMode::FloatUnit);
        let (x, y) = normalize_2d(3.0, 4.0);
        assert!(approx_eq(x, 0.6));
        assert!(approx_eq(y, 0.8));
    }

    #[test]
    fn test_cross_3d() {
        set_mode(AnalogMode::Int201);
        // x cross y = z
        let (x, y, z) = cross_3d(1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert!(approx_eq(x, 0.0));
        assert!(approx_eq(y, 0.0));
        assert!(approx_eq(z, 1.0));
    }

    #[test]
    fn test_distance_2d() {
        set_mode(AnalogMode::Int201);
        assert!(approx_eq(distance_2d(0.0, 0.0, 3.0, 4.0), 5.0));
    }
}
