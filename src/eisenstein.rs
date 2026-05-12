//! Eisenstein Constraint Module for Dodecet Encoder
//!
//! Integrates the Weyl group folding (S₃) with dodecet 12-bit constraint state encoding.
//!
//! ## Layout
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │              DODECET (12 bits)               │
//! ├──────────────┬──────────────┬───────────────┤
//! │  Nibble 2    │  Nibble 1    │  Nibble 0     │
//! │  Bits 11-8   │  Bits 7-4    │  Bits 3-0     │
//! │              │              │               │
//! │  CONSTRAINT  │  DIRECTION   │  CHIRALITY    │
//! │  LEVEL       │  IN CELL     │  + SAFETY     │
//! │              │              │               │
//! │  0 = on snap │  0-15 = 22.5°│  bits 0-2:    │
//! │  15 = at ρ   │  azimuth     │   chamber 0-5 │
//! │              │              │  bit 3:       │
//! │  Right-skewed│  Uniform     │   safe/crit   │
//! │  (70% ≥ 8)  │  (all equal) │  (0.70 = crit)│
//! └──────────────┴──────────────┴───────────────┘
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use dodecet_encoder::eisenstein::{EisensteinConstraint, SnapResult};
//!
//! let constraint = EisensteinConstraint::new();
//! let result = constraint.snap(1.5, 2.3);
//! println!("Dodecet: 0x{:03X}", result.dodecet);
//! println!("Error: {:.4} / ρ = {:.4}", result.error, result.error_normalized);
//! println!("Chamber: {} ({})", result.chamber, result.parity_str());
//! println!("Safe: {}", result.is_safe());
//! ```

const SQRT_3: f64 = 1.7320508075688772;

/// Covering radius of A₂ lattice: ρ = 1/√3
pub const COVERING_RADIUS: f64 = 1.0 / SQRT_3;

/// Voronoi cell area of A₂: A = √3/2
pub const CELL_AREA: f64 = SQRT_3 / 2.0;

/// ω = e^{2πi/3} = -1/2 + i√3/2
const OMEGA_RE: f64 = -0.5;
const OMEGA_IM: f64 = SQRT_3 / 2.0;

/// |W(A₂)| = |S₃| = 6
pub const WEYL_ORDER: usize = 6;

/// Square-root funnel threshold: ρ/2 (safe if below)
pub const SAFE_THRESHOLD: f64 = COVERING_RADIUS / 2.0;

/// The 6 Weyl chambers of S₃ (sorted descending permutations of (0,1,2))
const WEYL_PERMS: [(usize, usize, usize); 6] = [
    (0, 1, 2), // chamber 0: even
    (0, 2, 1), // chamber 1: odd
    (1, 0, 2), // chamber 2: even
    (1, 2, 0), // chamber 3: odd
    (2, 0, 1), // chamber 4: even
    (2, 1, 0), // chamber 5: even
];

/// Even chambers have positive parity (reached by rotations)
const EVEN_CHAMBERS: [usize; 3] = [0, 2, 5];
/// Odd chambers have negative parity (reached by reflections)
const ODD_CHAMBERS: [usize; 3] = [1, 3, 4];

/// Result of Eisenstein constraint snap
#[derive(Debug, Clone, Copy)]
pub struct SnapResult {
    /// The 12-bit dodecet encoding the full constraint state
    pub dodecet: u16,
    /// Nearest Eisenstein integer (a, b) where a + bω is the snap point
    pub snap_a: i32,
    pub snap_b: i32,
    /// Snap error (distance from input to snap point)
    pub error: f64,
    /// Error normalized to covering radius [0, 1]
    pub error_normalized: f64,
    /// Error quantized to 16 levels (nibble 2)
    pub error_level: u8,
    /// Azimuthal angle quantized to 16 levels (nibble 1)
    pub angle_level: u8,
    /// Weyl chamber index 0-5
    pub chamber: u8,
    /// Parity: +1 for even chambers, -1 for odd
    pub parity: i8,
    /// Whether the point is within the safe zone (error < ρ/2)
    pub is_safe: bool,
}

impl SnapResult {
    /// Format dodecet as hex string (3 chars)
    pub fn to_hex(&self) -> String {
        format!("{:03X}", self.dodecet)
    }

    /// Get parity as string
    pub fn parity_str(&self) -> &'static str {
        if self.parity > 0 {
            "even (+)"
        } else {
            "odd (-)"
        }
    }

    /// Deadband funnel position: √(error/ρ)
    /// 0.0 = on snap, 1.0 = at boundary
    /// Square-root because CDF = πr²/A → r = √(A·P/π)
    pub fn funnel_position(&self) -> f64 {
        (self.error / COVERING_RADIUS).sqrt()
    }

    /// Precision feeling Φ = 1/δ where δ is the deadband
    /// Higher Φ = more constrained
    pub fn precision_feeling(&self) -> f64 {
        if self.error > 0.0 {
            1.0 / self.error
        } else {
            f64::INFINITY
        }
    }

    /// Decode dodecet back to components
    pub fn decode_dodecet(dodecet: u16) -> (u8, u8, u8, bool) {
        let err_level = ((dodecet >> 8) & 0xF) as u8;
        let angle_level = ((dodecet >> 4) & 0xF) as u8;
        let chamber_byte = (dodecet & 0xF) as u8;
        let chamber = chamber_byte & 0x7;
        let safe = (chamber_byte >> 3) & 1 == 0;
        (err_level, angle_level, chamber, safe)
    }

    /// CDF prediction: P(d < r) = πr²/A
    /// Given this error level, what fraction of points have LESS error?
    pub fn cdf_below(&self) -> f64 {
        std::f64::consts::PI * self.error * self.error / CELL_AREA
    }
}

/// Eisenstein constraint checker
pub struct EisensteinConstraint {
    /// Deadband funnel width (0 to 1.0). Controls how tight the constraint is.
    /// Square-root funnel: δ(t) = ρ·√(1-t)
    pub funnel_width: f64,
}

impl Default for EisensteinConstraint {
    fn default() -> Self {
        Self::new()
    }
}

impl EisensteinConstraint {
    pub fn new() -> Self {
        EisensteinConstraint { funnel_width: 1.0 }
    }

    pub fn with_funnel(mut self, width: f64) -> Self {
        self.funnel_width = width.clamp(0.0, 1.0);
        self
    }

    /// Snap a point to the nearest Eisenstein integer using 9-candidate Voronoi search.
    ///
    /// This is the core constraint check: map ℝ² → A₂ → dodecet.
    pub fn snap(&self, x: f64, y: f64) -> SnapResult {
        // Convert to Eisenstein coordinates
        let a_f = x - y * OMEGA_RE / OMEGA_IM;
        let b_f = y / OMEGA_IM;

        let a0 = a_f.round() as i32;
        let b0 = b_f.round() as i32;

        // 9-candidate Voronoi search (guaranteed covering radius)
        let mut best_a = a0;
        let mut best_b = b0;
        let mut best_err = f64::MAX;

        for da in -1..=1i32 {
            for db in -1..=1i32 {
                let ca = a0 + da;
                let cb = b0 + db;
                let cx = ca as f64 + cb as f64 * OMEGA_RE;
                let cy = cb as f64 * OMEGA_IM;
                let err = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
                if err < best_err {
                    best_a = ca;
                    best_b = cb;
                    best_err = err;
                }
            }
        }

        // Classify into Weyl chamber
        let chamber = Self::classify_chamber(x, y);
        let parity = if EVEN_CHAMBERS.contains(&{ chamber }) {
            1
        } else {
            -1
        };

        // Quantize error to 16 levels (nibble 2)
        let err_norm = (best_err / COVERING_RADIUS).min(1.0);
        let err_level = (err_norm * 15.0).round() as u8;

        // Quantize angle to 16 levels (nibble 1)
        let dx = x - (best_a as f64 + best_b as f64 * OMEGA_RE);
        let dy = y - (best_b as f64 * OMEGA_IM);
        let angle_level = if dx != 0.0 || dy != 0.0 {
            let angle = dy.atan2(dx);
            let norm = (angle + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
            (norm * 16.0).floor() as u8 % 16
        } else {
            0
        };

        // Safety flag: safe if error < ρ/2
        let is_safe = best_err < SAFE_THRESHOLD;
        let safe_bit: u8 = if is_safe { 0 } else { 1 };
        let chamber_byte = (safe_bit << 3) | (chamber as u8 & 0x7);

        // Pack dodecet
        let dodecet =
            ((err_level as u16) << 8) | ((angle_level as u16) << 4) | (chamber_byte as u16);

        SnapResult {
            dodecet,
            snap_a: best_a,
            snap_b: best_b,
            error: best_err,
            error_normalized: err_norm,
            error_level: err_level,
            angle_level,
            chamber: chamber as u8,
            parity,
            is_safe,
        }
    }

    /// Classify a point into one of 6 Weyl chambers by sorting barycentric coords.
    fn classify_chamber(x: f64, y: f64) -> usize {
        let b1 = x - y * OMEGA_RE / OMEGA_IM;
        let b2 = y / OMEGA_IM;
        let b3 = -(b1 + b2);

        let vals = [b1, b2, b3];
        let indices = [0usize, 1, 2];
        let mut sorted = indices;
        sorted.sort_by(|&a, &b| {
            vals[b]
                .partial_cmp(&vals[a])
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let perm = (sorted[0], sorted[1], sorted[2]);
        WEYL_PERMS.iter().position(|&p| p == perm).unwrap_or(0)
    }

    /// Merge multiple constraint states (fleet consensus).
    ///
    /// Pessimistic on error, majority vote on chirality.
    pub fn merge(&self, results: &[SnapResult]) -> SnapResult {
        if results.is_empty() {
            return self.snap(0.0, 0.0);
        }
        if results.len() == 1 {
            return results[0];
        }

        // Error: take max (pessimistic)
        let max_err = results
            .iter()
            .map(|r| r.error)
            .fold(f64::NEG_INFINITY, f64::max);
        let err_level = (max_err / COVERING_RADIUS * 15.0).round() as u8;

        // Angle: majority vote on angle level
        let mut angle_votes = [0u16; 16];
        for r in results {
            angle_votes[r.angle_level as usize] += 1;
        }
        let angle_level = angle_votes
            .iter()
            .enumerate()
            .max_by_key(|(_, &v)| v)
            .map(|(i, _)| i as u8)
            .unwrap_or(0);

        // Chirality: majority vote
        let mut chamber_votes = [0u16; 6];
        for r in results {
            chamber_votes[r.chamber as usize] += 1;
        }
        let chamber = chamber_votes
            .iter()
            .enumerate()
            .max_by_key(|(_, &v)| v)
            .map(|(i, _)| i as u8)
            .unwrap_or(0);

        // Safety: all must be safe for merged to be safe
        let all_safe = results.iter().all(|r| r.is_safe);
        let safe_bit: u8 = if all_safe { 0 } else { 1 };
        let chamber_byte = (safe_bit << 3) | (chamber & 0x7);

        let dodecet =
            ((err_level as u16) << 8) | ((angle_level as u16) << 4) | (chamber_byte as u16);

        let parity = if EVEN_CHAMBERS.contains(&(chamber as usize)) {
            1
        } else {
            -1
        };

        SnapResult {
            dodecet,
            snap_a: 0, // merged result doesn't have a single snap point
            snap_b: 0,
            error: max_err,
            error_normalized: max_err / COVERING_RADIUS,
            error_level: err_level,
            angle_level,
            chamber,
            parity,
            is_safe: all_safe,
        }
    }

    /// Square-root deadband funnel: δ(t) = ρ·√(1-t)
    ///
    /// At t=0: δ=ρ (wide, starting)
    /// At t=0.5: δ=ρ/√2 ≈ 0.707ρ
    /// At t=1.0: δ=0 (narrow, snapped)
    pub fn deadband(&self, t: f64) -> f64 {
        COVERING_RADIUS * (1.0 - t).sqrt().max(0.0)
    }

    /// Check if a point satisfies the constraint at the current funnel width.
    pub fn check(&self, x: f64, y: f64) -> ConstraintVerdict {
        let result = self.snap(x, y);
        let threshold = self.deadband(self.funnel_width);

        if result.error <= threshold {
            ConstraintVerdict::Satisfied(result)
        } else {
            ConstraintVerdict::Violated(result)
        }
    }
}

/// Result of constraint check
#[derive(Debug)]
pub enum ConstraintVerdict {
    Satisfied(SnapResult),
    Violated(SnapResult),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covering_radius() {
        assert!((COVERING_RADIUS - 1.0 / SQRT_3).abs() < 1e-10);
        assert!((COVERING_RADIUS - 0.577350269).abs() < 1e-6);
    }

    #[test]
    fn test_snap_origin() {
        let ec = EisensteinConstraint::new();
        let result = ec.snap(0.0, 0.0);
        assert_eq!(result.snap_a, 0);
        assert_eq!(result.snap_b, 0);
        assert!(result.error < 1e-10);
        assert!(result.is_safe);
        assert_eq!(result.error_level, 0);
    }

    #[test]
    fn test_snap_near_lattice_point() {
        let ec = EisensteinConstraint::new();
        // Snap (1, 0) should snap to (1, 0) in Eisenstein
        let result = ec.snap(1.0, 0.0);
        assert_eq!(result.snap_a, 1);
        assert_eq!(result.snap_b, 0);
        assert!(result.error < 0.01);
    }

    #[test]
    fn test_covering_radius_never_exceeded() {
        let ec = EisensteinConstraint::new();
        for _ in 0..10000 {
            let x = rand_float(-10.0, 10.0);
            let y = rand_float(-10.0, 10.0);
            let result = ec.snap(x, y);
            assert!(
                result.error <= COVERING_RADIUS + 1e-6,
                "Error {:.6} exceeds ρ {:.6} at ({:.2}, {:.2})",
                result.error,
                COVERING_RADIUS,
                x,
                y
            );
        }
    }

    #[test]
    fn test_dodecet_roundtrip() {
        let ec = EisensteinConstraint::new();
        let result = ec.snap(1.5, 2.3);
        let (err, angle, ch, safe) = SnapResult::decode_dodecet(result.dodecet);
        assert_eq!(err, result.error_level);
        assert_eq!(angle, result.angle_level);
        assert_eq!(ch, result.chamber);
        assert_eq!(safe, result.is_safe);
    }

    #[test]
    fn test_weyl_invariance() {
        let ec = EisensteinConstraint::new();
        let mut errors_by_chamber: [Vec<f64>; 6] = Default::default();
        for _ in 0..10000 {
            let x = rand_float(-5.0, 5.0);
            let y = rand_float(-5.0, 5.0);
            let result = ec.snap(x, y);
            errors_by_chamber[result.chamber as usize].push(result.error);
        }
        let means: Vec<f64> = errors_by_chamber
            .iter()
            .filter(|v| !v.is_empty())
            .map(|v| v.iter().sum::<f64>() / v.len() as f64)
            .collect();
        let max_spread = means.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
            - means.iter().cloned().fold(f64::INFINITY, f64::min);
        // Snap error should be Weyl-invariant (spread < 5%)
        assert!(
            max_spread / means[0] < 0.05,
            "Chamber means too spread: {:?}",
            means
        );
    }

    #[test]
    fn test_merge_pessimistic() {
        let ec = EisensteinConstraint::new();
        let r1 = ec.snap(0.1, 0.1); // small error
        let r2 = ec.snap(0.5, 0.3); // larger error
        let merged = ec.merge(&[r1, r2]);
        // Merged error should be the maximum
        assert!(merged.error >= r1.error - 1e-6);
        assert!(merged.error >= r2.error - 1e-6);
    }

    #[test]
    fn test_deadband_funnel() {
        let ec = EisensteinConstraint::new();
        assert!((ec.deadband(0.0) - COVERING_RADIUS).abs() < 1e-10);
        assert!(ec.deadband(0.5) < COVERING_RADIUS);
        assert!(ec.deadband(0.5) > 0.0);
        assert!((ec.deadband(1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_right_skew() {
        let ec = EisensteinConstraint::new();
        let mut high_count = 0;
        let total = 10000;
        for _ in 0..total {
            let x = rand_float(-5.0, 5.0);
            let y = rand_float(-5.0, 5.0);
            let result = ec.snap(x, y);
            if result.error_level >= 8 {
                high_count += 1;
            }
        }
        // Right-skew: majority should be at high error levels
        assert!(
            high_count as f64 / total as f64 > 0.60,
            "Expected >60% at levels 8-15, got {:.1}%",
            high_count as f64 / total as f64 * 100.0
        );
    }

    fn rand_float(min: f64, max: f64) -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        static mut SEED: u64 = 0;
        unsafe {
            if SEED == 0 {
                SEED = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64;
            }
            SEED = SEED
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let x = SEED;
            min + (max - min) * ((x >> 33) as f64 / (1u64 << 31) as f64)
        }
    }
}
