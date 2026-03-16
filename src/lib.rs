//! # Dodecet Encoder
//!
//! A revolutionary 12-bit dodecet encoding system optimized for geometric and calculus operations.
//!
//! ## What is a Dodecet?
//!
//! A **dodecet** is a 12-bit unit composed of 3 nibbles (4-bit sets):
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │           DODECET (12 bits)             │
//! ├─────────────────────────────────────────┤
//! │  Nibble 2  │  Nibble 1  │  Nibble 0    │
//! │  (4 bits)  │  (4 bits)  │  (4 bits)    │
//! │  [11:8]    │  [7:4]     │  [3:0]       │
//! ├─────────────────────────────────────────┤
//! │  Example:   0xA        0xB        0xC  │
//! │  Hex: 0xABC = 1010 1011 1100 (binary)  │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## Why 12 Bits?
//!
//! - **Hex-Friendly**: Each dodecet = 3 hex digits (e.g., 0xABC)
//! - **Geometric**: 12 bits can encode 4096 values (vs 256 for 8-bit)
//! - **Calculus-Optimized**: Natural alignment for derivatives and integrals
//! - **Shape Encoding**: Perfect for vertices, edges, faces (3D geometry)
//!
//! ## Quick Start
//!
//! ```rust
//! use dodecet_encoder::{Dodecet, DodecetArray};
//!
//! // Create a dodecet from hex
//! let d = Dodecet::from_hex(0xABC);
//!
//! // Access nibbles
//! assert_eq!(d.nibble(0), 0xC);
//! assert_eq!(d.nibble(1), 0xB);
//! assert_eq!(d.nibble(2), 0xA);
//!
//! // Geometric encoding: 3D point (x, y, z)
//! let point = DodecetArray::from_slice(&[0x123, 0x456, 0x789]);
//! ```
//!
//! ## Architecture
//!
//! This crate provides:
//! - **Core Types**: `Dodecet`, `DodecetArray`, `DodecetString`
//! - **Geometric Operations**: Vector math, transformations, rotations
//! - **Hex Encoding**: Bidirectional hex conversion
//! - **Performance**: SIMD-optimized operations where possible

pub mod dodecet;
pub mod array;
pub mod string;
pub mod geometric;
pub mod hex;
pub mod calculus;

// Re-export core types
pub use dodecet::Dodecet;
pub use array::DodecetArray;
pub use string::DodecetString;
pub use geometric::{Point3D, Vector3D, Transform3D};

/// Maximum value of a dodecet (12 bits = 4095)
pub const MAX_DODECET: u16 = 0xFFF;

/// Number of bits in a dodecet
pub const DODECET_BITS: u8 = 12;

/// Number of nibbles in a dodecet
pub const NIBBLES: u8 = 3;

/// Number of values a dodecet can represent
pub const CAPACITY: u16 = 4096;

/// Error type for dodecet operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DodecetError {
    /// Value exceeds 12-bit capacity
    Overflow,
    /// Invalid hex string
    InvalidHex,
    /// Invalid nibble index
    InvalidNibble,
    /// Invalid geometric operation
    InvalidGeometry,
}

impl std::fmt::Display for DodecetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DodecetError::Overflow => write!(f, "Value exceeds 12-bit capacity (max: 4095)"),
            DodecetError::InvalidHex => write!(f, "Invalid hex string"),
            DodecetError::InvalidNibble => write!(f, "Nibble index must be 0, 1, or 2"),
            DodecetError::InvalidGeometry => write!(f, "Invalid geometric operation"),
        }
    }
}

impl std::error::Error for DodecetError {}

/// Result type for dodecet operations
pub type Result<T> = std::result::Result<T, DodecetError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(MAX_DODECET, 4095);
        assert_eq!(DODECET_BITS, 12);
        assert_eq!(NIBBLES, 3);
        assert_eq!(CAPACITY, 4096);
    }
}
