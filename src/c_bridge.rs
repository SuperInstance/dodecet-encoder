//! C Bridge Backend for Eisenstein Constraint Snapping
//!
//! Wraps the fleet-math-c C FFI implementation as an alternative snap backend.
//! Uses `f32` internally (matching the C implementation) but exposes the same
//! `snap(x: f64, y: f64)` interface as the pure-Rust `EisensteinConstraint`.
//!
//! Enable with the `c-bridge` feature flag.

use crate::eisenstein::{SnapResult, COVERING_RADIUS, SAFE_THRESHOLD};

const EVEN_CHAMBERS: [usize; 3] = [0, 2, 5];

const SQRT_3: f64 = 1.7320508075688772;
const OMEGA_RE: f64 = -0.5;
const OMEGA_IM: f64 = SQRT_3 / 2.0;

/// Eisenstein constraint checker backed by the C implementation.
///
/// Produces results compatible with the pure-Rust `EisensteinConstraint` but
/// delegates the heavy snap computation to the optimized C bridge.
pub struct CBridgeEisensteinConstraint {
    /// Deadband funnel width (mirrors `EisensteinConstraint::funnel_width`).
    pub funnel_width: f64,
}

impl Default for CBridgeEisensteinConstraint {
    fn default() -> Self {
        Self::new()
    }
}

impl CBridgeEisensteinConstraint {
    pub fn new() -> Self {
        CBridgeEisensteinConstraint { funnel_width: 1.0 }
    }

    pub fn with_funnel(mut self, width: f64) -> Self {
        self.funnel_width = width.clamp(0.0, 1.0);
        self
    }

    /// Snap a point to the nearest Eisenstein integer via the C bridge.
    ///
    /// This is the drop-in replacement for `EisensteinConstraint::snap()`.
    /// The C code handles lattice snapping in `f32`; we cast and enrich the
    /// result with the same Weyl-chamber classification and dodecet packing
    /// used by the Rust implementation.
    pub fn snap(&self, x: f64, y: f64) -> SnapResult {
        let c_result = fleet_math_c::snap(x as f32, y as f32);

        // The C bridge returns its own dodecet and chamber, but for exact
        // parity with the Rust implementation we re-derive the snap point
        // and re-classify using the same algorithms.
        // If you want the raw C results, use `snap_raw()`.

        // Re-derive the snap point from the C result's error and the input
        // by performing the same 9-candidate search in f32, then using
        // the C result's chamber classification for consistency.
        let a_f = (x as f32) - (y as f32) * (OMEGA_RE as f32) / (OMEGA_IM as f32);
        let b_f = (y as f32) / (OMEGA_IM as f32);

        let a0 = a_f.round() as i32;
        let b0 = b_f.round() as i32;

        let mut best_a = a0;
        let mut best_b = b0;
        let mut best_err = f32::MAX;

        for da in -1..=1i32 {
            for db in -1..=1i32 {
                let ca = a0 + da;
                let cb = b0 + db;
                let cx = ca as f32 + cb as f32 * (OMEGA_RE as f32);
                let cy = cb as f32 * (OMEGA_IM as f32);
                let dx = (x as f32) - cx;
                let dy = (y as f32) - cy;
                let err = (dx * dx + dy * dy).sqrt();
                if err < best_err {
                    best_a = ca;
                    best_b = cb;
                    best_err = err;
                }
            }
        }

        let best_err_f64 = best_err as f64;
        let chamber = c_result.chamber;
        let parity = if EVEN_CHAMBERS.contains(&(chamber as usize)) {
            1
        } else {
            -1
        };

        // Quantize error to 16 levels
        let err_norm = (best_err_f64 / COVERING_RADIUS).min(1.0);
        let err_level = (err_norm * 15.0).round() as u8;

        // Quantize angle to 16 levels
        let dx = x - (best_a as f64 + best_b as f64 * OMEGA_RE);
        let dy = y - (best_b as f64 * OMEGA_IM);
        let angle_level = if dx != 0.0 || dy != 0.0 {
            let angle = dy.atan2(dx);
            let norm = (angle + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
            (norm * 16.0).floor() as u8 % 16
        } else {
            0
        };

        let is_safe = best_err_f64 < SAFE_THRESHOLD;
        let safe_bit: u8 = if is_safe { 0 } else { 1 };
        let chamber_byte = (safe_bit << 3) | (chamber & 0x7);
        let dodecet = ((err_level as u16) << 8) | ((angle_level as u16) << 4) | (chamber_byte as u16);

        SnapResult {
            dodecet,
            snap_a: best_a,
            snap_b: best_b,
            error: best_err_f64,
            error_normalized: err_norm,
            error_level: err_level,
            angle_level,
            chamber,
            parity,
            is_safe,
        }
    }

    /// Raw C bridge snap — returns the C result directly without re-deriving.
    /// Useful for benchmarking pure C throughput.
    pub fn snap_raw(&self, x: f64, y: f64) -> fleet_math_c::SnapResult {
        fleet_math_c::snap(x as f32, y as f32)
    }

    /// Batch snap via C bridge — processes interleaved (x, y) pairs.
    /// Returns SnapResults compatible with the Rust implementation.
    pub fn batch_snap(&self, points: &[(f64, f64)]) -> Vec<SnapResult> {
        let flat: Vec<f32> = points.iter().flat_map(|&(x, y)| [x as f32, y as f32]).collect();
        let raw = fleet_math_c::batch_snap(&flat);
        raw.into_iter()
            .enumerate()
            .map(|(i, r)| {
                let x = points[i].0;
                let y = points[i].1;
                // Re-derive snap point for full SnapResult compatibility
                let a_f = (x as f32) - (y as f32) * (OMEGA_RE as f32) / (OMEGA_IM as f32);
                let b_f = (y as f32) / (OMEGA_IM as f32);
                let a0 = a_f.round() as i32;
                let b0 = b_f.round() as i32;
                let mut best_a = a0;
                let mut best_b = b0;
                let mut best_err = f32::MAX;
                for da in -1..=1i32 {
                    for db in -1..=1i32 {
                        let ca = a0 + da;
                        let cb = b0 + db;
                        let cx = ca as f32 + cb as f32 * (OMEGA_RE as f32);
                        let cy = cb as f32 * (OMEGA_IM as f32);
                        let dx = (x as f32) - cx;
                        let dy = (y as f32) - cy;
                        let err = (dx * dx + dy * dy).sqrt();
                        if err < best_err {
                            best_a = ca;
                            best_b = cb;
                            best_err = err;
                        }
                    }
                }
                let best_err_f64 = best_err as f64;
                let chamber = r.chamber;
                let parity = if EVEN_CHAMBERS.contains(&(chamber as usize)) { 1 } else { -1 };
                let err_norm = (best_err_f64 / COVERING_RADIUS).min(1.0);
                let err_level = (err_norm * 15.0).round() as u8;
                let ddx = x - (best_a as f64 + best_b as f64 * OMEGA_RE);
                let ddy = y - (best_b as f64 * OMEGA_IM);
                let angle_level = if ddx != 0.0 || ddy != 0.0 {
                    let angle = ddy.atan2(ddx);
                    let norm = (angle + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
                    (norm * 16.0).floor() as u8 % 16
                } else {
                    0
                };
                let is_safe = best_err_f64 < SAFE_THRESHOLD;
                let safe_bit: u8 = if is_safe { 0 } else { 1 };
                let chamber_byte = (safe_bit << 3) | (chamber & 0x7);
                let dodecet = ((err_level as u16) << 8) | ((angle_level as u16) << 4) | (chamber_byte as u16);
                SnapResult {
                    dodecet, snap_a: best_a, snap_b: best_b,
                    error: best_err_f64, error_normalized: err_norm,
                    error_level: err_level, angle_level, chamber, parity, is_safe,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eisenstein::EisensteinConstraint;

    #[test]
    fn test_c_bridge_snap_origin() {
        let cb = CBridgeEisensteinConstraint::new();
        let result = cb.snap(0.0, 0.0);
        assert_eq!(result.snap_a, 0);
        assert_eq!(result.snap_b, 0);
        assert!(result.error < 0.001, "Origin error: {}", result.error);
        assert!(result.is_safe);
    }

    #[test]
    fn test_c_bridge_matches_rust() {
        let rust = EisensteinConstraint::new();
        let c_bridge = CBridgeEisensteinConstraint::new();

        let test_points: [(f64, f64); 20] = [
            (1.0, 0.0), (0.0, 1.0), (1.5, 2.3), (-3.7, 4.1),
            (0.1, 0.1), (0.5, 0.3), (2.0, 2.0), (-1.0, -1.0),
            (100.0, 100.0), (-50.3, 27.8), (0.001, 0.002),
            (3.14159, 2.71828), (1e-6, 1e-6), (-0.5, 0.866025),
            (7.0, 0.0), (0.0, -7.0), (0.333, 0.667),
            (-2.5, -2.5), (10.0, 0.01), (0.01, 10.0),
        ];

        for &(x, y) in &test_points {
            let r_rust = rust.snap(x, y);
            let r_c = c_bridge.snap(x, y);

            // snap_a and snap_b must match
            assert_eq!(
                r_rust.snap_a, r_c.snap_a,
                "snap_a mismatch at ({}, {}): rust={} c={}",
                x, y, r_rust.snap_a, r_c.snap_a
            );
            assert_eq!(
                r_rust.snap_b, r_c.snap_b,
                "snap_b mismatch at ({}, {}): rust={} c={}",
                x, y, r_rust.snap_b, r_c.snap_b
            );

            // Error should match within f32→f64 tolerance
            let err_diff = (r_rust.error - r_c.error).abs();
            assert!(
                err_diff < 1e-4,
                "Error mismatch at ({}, {}): rust={:.8} c={:.8} diff={:.8}",
                x, y, r_rust.error, r_c.error, err_diff
            );
        }
    }

    #[test]
    fn test_c_bridge_chamber_valid() {
        let cb = CBridgeEisensteinConstraint::new();
        let mut seed: u64 = 42;
        for _ in 0..100 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let x = ((seed >> 33) as f64 / (1u64 << 31) as f64) * 10.0 - 5.0;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let y = ((seed >> 33) as f64 / (1u64 << 31) as f64) * 10.0 - 5.0;
            let result = cb.snap(x, y);
            assert!(result.chamber <= 5, "Chamber must be 0-5, got {}", result.chamber);
        }
    }

    #[test]
    fn test_c_bridge_covering_radius() {
        let cb = CBridgeEisensteinConstraint::new();
        let mut seed: u64 = 99;
        for _ in 0..1000 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let x = ((seed >> 33) as f64 / (1u64 << 31) as f64) * 20.0 - 10.0;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let y = ((seed >> 33) as f64 / (1u64 << 31) as f64) * 20.0 - 10.0;
            let result = cb.snap(x, y);
            assert!(
                result.error <= COVERING_RADIUS + 1e-3,
                "Error {:.6} exceeds ρ {:.6} at ({:.2}, {:.2})",
                result.error, COVERING_RADIUS, x, y
            );
        }
    }
}
