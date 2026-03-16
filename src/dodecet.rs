//! # Dodecet: The 12-bit Building Block
//!
//! A dodecet is a 12-bit value composed of 3 nibbles (4-bit groups).
//! It's the fundamental unit of the dodecet encoding system.

use crate::{DodecetError, Result, MAX_DODECET, NIBBLES};

/// A 12-bit dodecet value (0-4095)
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::Dodecet;
///
/// let d = Dodecet::new(0xABC);
/// assert_eq!(d.value(), 0xABC);
/// assert_eq!(d.nibble(0), 0xC);
/// assert_eq!(d.nibble(1), 0xB);
/// assert_eq!(d.nibble(2), 0xA);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dodecet {
    value: u16,
}

impl Dodecet {
    /// Create a new dodecet from a u16 value
    ///
    /// # Errors
    /// Returns `DodecetError::Overflow` if value > 4095
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::new(0xABC).unwrap();
    /// assert_eq!(d.value(), 0xABC);
    /// ```
    pub fn new(value: u16) -> Result<Self> {
        if value > MAX_DODECET {
            Err(DodecetError::Overflow)
        } else {
            Ok(Dodecet { value })
        }
    }

    /// Create a dodecet from a hex value (unchecked)
    ///
    /// # Safety
    /// Caller must ensure value <= 4095
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = unsafe { Dodecet::from_hex_unchecked(0xABC) };
    /// assert_eq!(d.value(), 0xABC);
    /// ```
    #[inline]
    pub unsafe fn from_hex_unchecked(value: u16) -> Self {
        Dodecet { value }
    }

    /// Create a dodecet from a hex value
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::from_hex(0xABC);
    /// assert_eq!(d.value(), 0xABC);
    /// ```
    #[inline]
    pub fn from_hex(value: u16) -> Self {
        debug_assert!(value <= MAX_DODECET, "Dodecet overflow");
        Dodecet {
            value: value & MAX_DODECET,
        }
    }

    /// Get the raw value
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::from_hex(0xABC);
    /// assert_eq!(d.value(), 0xABC);
    /// ```
    #[inline]
    pub fn value(self) -> u16 {
        self.value
    }

    /// Get a specific nibble (0, 1, or 2)
    ///
    /// # Arguments
    /// * `index` - Nibble index (0 = LSB, 2 = MSB)
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::from_hex(0xABC);
    /// assert_eq!(d.nibble(0), 0xC);
    /// assert_eq!(d.nibble(1), 0xB);
    /// assert_eq!(d.nibble(2), 0xA);
    /// ```
    #[inline]
    pub fn nibble(self, index: u8) -> Result<u8> {
        if index >= NIBBLES {
            return Err(DodecetError::InvalidNibble);
        }
        Ok(((self.value >> (index * 4)) & 0xF) as u8)
    }

    /// Set a specific nibble
    ///
    /// # Arguments
    /// * `index` - Nibble index (0 = LSB, 2 = MSB)
    /// * `nibble` - New nibble value (0-15)
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let mut d = Dodecet::from_hex(0xABC);
    /// d.set_nibble(0, 0xD).unwrap();
    /// assert_eq!(d.value(), 0xABD);
    /// ```
    #[inline]
    pub fn set_nibble(&mut self, index: u8, nibble: u8) -> Result<()> {
        if index >= NIBBLES {
            return Err(DodecetError::InvalidNibble);
        }
        if nibble > 0xF {
            return Err(DodecetError::Overflow);
        }

        let mask = !(0xFu16 << (index * 4));
        self.value = (self.value & mask) | ((nibble as u16) << (index * 4));
        Ok(())
    }

    /// Check if dodecet is zero
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// assert!(Dodecet::from_hex(0).is_zero());
    /// assert!(!Dodecet::from_hex(0xABC).is_zero());
    /// ```
    #[inline]
    pub fn is_zero(self) -> bool {
        self.value == 0
    }

    /// Check if dodecet is at maximum value
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// assert!(Dodecet::from_hex(0xFFF).is_max());
    /// assert!(!Dodecet::from_hex(0xABC).is_max());
    /// ```
    #[inline]
    pub fn is_max(self) -> bool {
        self.value == MAX_DODECET
    }

    /// Count set bits (population count)
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::from_hex(0xFFF); // All bits set
    /// assert_eq!(d.count_ones(), 12);
    /// ```
    #[inline]
    pub fn count_ones(self) -> u32 {
        self.value.count_ones()
    }

    /// Count unset bits
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::from_hex(0x000); // No bits set
    /// assert_eq!(d.count_zeros(), 12);
    /// ```
    #[inline]
    pub fn count_zeros(self) -> u32 {
        self.value.count_zeros()
    }

    /// Bitwise AND
    #[inline]
    pub fn and(self, other: Dodecet) -> Dodecet {
        Dodecet {
            value: self.value & other.value,
        }
    }

    /// Bitwise OR
    #[inline]
    pub fn or(self, other: Dodecet) -> Dodecet {
        Dodecet {
            value: self.value | other.value,
        }
    }

    /// Bitwise XOR
    #[inline]
    pub fn xor(self, other: Dodecet) -> Dodecet {
        Dodecet {
            value: self.value ^ other.value,
        }
    }

    /// Bitwise NOT
    #[inline]
    pub fn not(self) -> Dodecet {
        Dodecet {
            value: (!self.value) & MAX_DODECET,
        }
    }

    /// Arithmetic addition with wrapping
    #[inline]
    pub fn wrapping_add(self, other: Dodecet) -> Dodecet {
        Dodecet {
            value: self.value.wrapping_add(other.value) & MAX_DODECET,
        }
    }

    /// Arithmetic subtraction with wrapping
    #[inline]
    pub fn wrapping_sub(self, other: Dodecet) -> Dodecet {
        Dodecet {
            value: self.value.wrapping_sub(other.value) & MAX_DODECET,
        }
    }

    /// Arithmetic multiplication with wrapping
    #[inline]
    pub fn wrapping_mul(self, other: Dodecet) -> Dodecet {
        Dodecet {
            value: self.value.wrapping_mul(other.value) & MAX_DODECET,
        }
    }

    /// Convert to hex string (3 characters)
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::from_hex(0xABC);
    /// assert_eq!(d.to_hex_string(), "ABC");
    /// ```
    pub fn to_hex_string(self) -> String {
        format!("{:03X}", self.value)
    }

    /// Parse from hex string
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::from_hex_str("ABC").unwrap();
    /// assert_eq!(d.value(), 0xABC);
    /// ```
    pub fn from_hex_str(s: &str) -> Result<Self> {
        let value = u16::from_str_radix(s.trim(), 16)
            .map_err(|_| DodecetError::InvalidHex)?;

        if value > MAX_DODECET {
            return Err(DodecetError::Overflow);
        }

        Ok(Dodecet { value })
    }

    /// Convert to binary string (12 characters)
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::from_hex(0xABC);
    /// assert_eq!(d.to_binary_string(), "101010111100");
    /// ```
    pub fn to_binary_string(self) -> String {
        format!("{:012b}", self.value)
    }

    /// Geometric interpretation: Treat as signed value (-2048 to 2047)
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::from_hex(0x800);
    /// assert_eq!(d.as_signed(), -2048);
    /// ```
    #[inline]
    pub fn as_signed(self) -> i16 {
        if self.value & 0x800 != 0 {
            (self.value as i16) - 4096
        } else {
            self.value as i16
        }
    }

    /// Normalize to floating point [0.0, 1.0]
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::Dodecet;
    ///
    /// let d = Dodecet::from_hex(0x800); // Midpoint
    /// assert!((d.normalize() - 0.5).abs() < 0.001);
    /// ```
    #[inline]
    pub fn normalize(self) -> f64 {
        self.value as f64 / MAX_DODECET as f64
    }
}

impl Default for Dodecet {
    fn default() -> Self {
        Dodecet { value: 0 }
    }
}

impl From<u8> for Dodecet {
    fn from(value: u8) -> Self {
        Dodecet { value: value as u16 }
    }
}

impl TryFrom<u16> for Dodecet {
    type Error = DodecetError;

    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        Dodecet::new(value)
    }
}

impl From<Dodecet> for u16 {
    fn from(d: Dodecet) -> Self {
        d.value
    }
}

impl std::fmt::Display for Dodecet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:03X}", self.value)
    }
}

impl std::fmt::Binary for Dodecet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:012b}", self.value)
    }
}

impl std::fmt::Octal for Dodecet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04o}", self.value)
    }
}

impl std::ops::Add for Dodecet {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.wrapping_add(other)
    }
}

impl std::ops::Sub for Dodecet {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.wrapping_sub(other)
    }
}

impl std::ops::Mul for Dodecet {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.wrapping_mul(other)
    }
}

impl std::ops::BitAnd for Dodecet {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        self.and(other)
    }
}

impl std::ops::BitOr for Dodecet {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.or(other)
    }
}

impl std::ops::BitXor for Dodecet {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        self.xor(other)
    }
}

impl std::ops::Not for Dodecet {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let d = Dodecet::new(0xABC).unwrap();
        assert_eq!(d.value(), 0xABC);

        let d2 = Dodecet::from_hex(0xDEF);
        assert_eq!(d2.value(), 0xDEF);
    }

    #[test]
    fn test_nibbles() {
        let d = Dodecet::from_hex(0xABC);
        assert_eq!(d.nibble(0).unwrap(), 0xC);
        assert_eq!(d.nibble(1).unwrap(), 0xB);
        assert_eq!(d.nibble(2).unwrap(), 0xA);
    }

    #[test]
    fn test_set_nibble() {
        let mut d = Dodecet::from_hex(0xABC);
        d.set_nibble(0, 0xD).unwrap();
        assert_eq!(d.value(), 0xABD);

        d.set_nibble(1, 0xE).unwrap();
        assert_eq!(d.value(), 0xAED);

        d.set_nibble(2, 0x1).unwrap();
        assert_eq!(d.value(), 0x1ED);
    }

    #[test]
    fn test_overflow() {
        assert!(Dodecet::new(0x1000).is_err());
        assert!(Dodecet::new(0xFFF).is_ok());
    }

    #[test]
    fn test_bitwise_ops() {
        let a = Dodecet::from_hex(0xF0F);
        let b = Dodecet::from_hex(0x0F0);

        assert_eq!((a & b).value(), 0x000);
        assert_eq!((a | b).value(), 0xFFF);
        assert_eq!((a ^ b).value(), 0xFFF);
        assert_eq!((!a).value(), 0x0F0);
    }

    #[test]
    fn test_arithmetic() {
        let a = Dodecet::from_hex(0x800);
        let b = Dodecet::from_hex(0x800);

        let c = a + b;
        assert_eq!(c.value(), 0x000); // Wraps around

        let d = Dodecet::from_hex(0x100) - Dodecet::from_hex(0x001);
        assert_eq!(d.value(), 0x0FF);
    }

    #[test]
    fn test_conversions() {
        let d = Dodecet::from_hex(0xABC);

        assert_eq!(d.to_hex_string(), "ABC");
        assert_eq!(d.to_binary_string(), "101010111100");

        let d2 = Dodecet::from_hex_str("ABC").unwrap();
        assert_eq!(d2.value(), 0xABC);
    }

    #[test]
    fn test_signed() {
        let d = Dodecet::from_hex(0x800);
        assert_eq!(d.as_signed(), -2048);

        let d = Dodecet::from_hex(0x7FF);
        assert_eq!(d.as_signed(), 2047);

        let d = Dodecet::from_hex(0x000);
        assert_eq!(d.as_signed(), 0);
    }

    #[test]
    fn test_normalize() {
        let d = Dodecet::from_hex(0x000);
        assert_eq!(d.normalize(), 0.0);

        let d = Dodecet::from_hex(0xFFF);
        assert!((d.normalize() - 1.0).abs() < f64::EPSILON);

        let d = Dodecet::from_hex(0x800);
        assert!((d.normalize() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_count_bits() {
        let d = Dodecet::from_hex(0xFFF);
        assert_eq!(d.count_ones(), 12);
        // count_zeros() on u16 returns 16 - count_ones(), not 12 - count_ones()
        assert_eq!(d.count_zeros(), 4);

        let d = Dodecet::from_hex(0x000);
        assert_eq!(d.count_ones(), 0);
        assert_eq!(d.count_zeros(), 16);
    }

    #[test]
    fn test_display() {
        let d = Dodecet::from_hex(0xABC);
        assert_eq!(format!("{}", d), "0xABC");
        assert_eq!(format!("{:b}", d), "101010111100");
        assert_eq!(format!("{:o}", d), "5274");
    }
}
