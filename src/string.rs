//! # DodecetString: Variable-length strings of dodecets
//!
//! Provides heap-allocated growable arrays for encoding sequences.

use crate::{Dodecet, DodecetError, Result};
use std::ops::{Deref, DerefMut};

/// A growable vector of dodecets
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::DodecetString;
///
/// let mut s = DodecetString::new();
/// s.push(0x123);
/// s.push(0x456);
/// assert_eq!(s.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DodecetString {
    data: Vec<Dodecet>,
}

impl DodecetString {
    /// Create a new empty dodecet string
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetString;
    ///
    /// let s = DodecetString::new();
    /// assert!(s.is_empty());
    /// ```
    pub fn new() -> Self {
        DodecetString { data: Vec::new() }
    }

    /// Create with capacity
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetString;
    ///
    /// let s = DodecetString::with_capacity(10);
    /// assert!(s.as_inner().capacity() >= 10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        DodecetString {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Create from a slice of u16 values
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetString;
    ///
    /// let s = DodecetString::from_slice(&[0x123, 0x456, 0x789]);
    /// assert_eq!(s.len(), 3);
    /// ```
    pub fn from_slice(values: &[u16]) -> Self {
        DodecetString {
            data: values.iter().map(|&v| Dodecet::from_hex(v)).collect(),
        }
    }

    /// Create from dodecets
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::{Dodecet, DodecetString};
    ///
    /// let s = DodecetString::from_dodecets(vec![
    ///     Dodecet::from_hex(0x123),
    ///     Dodecet::from_hex(0x456),
    /// ]);
    /// ```
    pub fn from_dodecets(dodecets: Vec<Dodecet>) -> Self {
        DodecetString { data: dodecets }
    }

    /// Push a dodecet value
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetString;
    ///
    /// let mut s = DodecetString::new();
    /// s.push(0xABC);
    /// assert_eq!(s[0].value(), 0xABC);
    /// ```
    pub fn push(&mut self, value: u16) {
        self.data.push(Dodecet::from_hex(value));
    }

    /// Push a dodecet
    pub fn push_dodecet(&mut self, dodecet: Dodecet) {
        self.data.push(dodecet);
    }

    /// Pop a dodecet
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetString;
    ///
    /// let mut s = DodecetString::from_slice(&[0x123, 0x456]);
    /// let d = s.pop().unwrap();
    /// assert_eq!(d.value(), 0x456);
    /// assert_eq!(s.len(), 1);
    /// ```
    pub fn pop(&mut self) -> Option<Dodecet> {
        self.data.pop()
    }

    /// Convert to hex string
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetString;
    ///
    /// let s = DodecetString::from_slice(&[0x123, 0x456, 0x789]);
    /// assert_eq!(s.to_hex_string(), "123456789");
    /// ```
    pub fn to_hex_string(&self) -> String {
        self.data
            .iter()
            .map(|d| d.to_hex_string())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Parse from hex string
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetString;
    ///
    /// let s = DodecetString::from_hex_str("123456789").unwrap();
    /// assert_eq!(s.len(), 3);
    /// assert_eq!(s[0].value(), 0x123);
    /// ```
    pub fn from_hex_str(s: &str) -> Result<Self> {
        if s.len() % 3 != 0 {
            return Err(DodecetError::InvalidHex);
        }

        let mut data = Vec::with_capacity(s.len() / 3);

        for chunk in s.as_bytes().chunks(3) {
            let chunk_str = std::str::from_utf8(chunk).unwrap();
            data.push(Dodecet::from_hex_str(chunk_str)?);
        }

        Ok(DodecetString { data })
    }

    /// Convert to bytes (lossy compression)
    ///
    /// Since 12 bits don't fit evenly in 8-bit bytes, this packs 2 dodecets into 3 bytes
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetString;
    ///
    /// let s = DodecetString::from_slice(&[0x123, 0x456]);
    /// let bytes = s.to_bytes();
    /// assert_eq!(bytes.len(), 3); // 2 dodecets = 3 bytes
    /// ```
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity((self.data.len() * 3 + 1) / 2);

        for chunk in self.data.chunks(2) {
            if chunk.len() == 2 {
                let d0 = chunk[0].value() as u32;
                let d1 = chunk[1].value() as u32;

                // Pack 2 dodecets (24 bits) into 3 bytes
                bytes.push(((d0 >> 4) & 0xFF) as u8);
                bytes.push((((d0 & 0x0F) << 4) | ((d1 >> 8) & 0x0F)) as u8);
                bytes.push((d1 & 0xFF) as u8);
            } else if chunk.len() == 1 {
                // Last odd dodecet
                let d0 = chunk[0].value() as u32;
                bytes.push(((d0 >> 4) & 0xFF) as u8);
                bytes.push(((d0 & 0x0F) << 4) as u8);
            }
        }

        bytes
    }

    /// Parse from bytes (unpacking)
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetString;
    ///
    /// let bytes = vec![0x12, 0x34, 0x56];
    /// let s = DodecetString::from_bytes(&bytes).unwrap();
    /// assert_eq!(s.len(), 2);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut data = Vec::new();

        let mut i = 0;
        while i + 2 < bytes.len() {
            let d0 = ((bytes[i] as u32) << 4) | ((bytes[i + 1] as u32) >> 4);
            let d1 = (((bytes[i + 1] as u32) & 0x0F) << 8) | (bytes[i + 2] as u32);

            data.push(Dodecet::from_hex(d0 as u16));
            data.push(Dodecet::from_hex(d1 as u16));

            i += 3;
        }

        // Handle remaining bytes
        if i + 1 < bytes.len() {
            let d0 = ((bytes[i] as u32) << 4) | ((bytes[i + 1] as u32) >> 4);
            data.push(Dodecet::from_hex(d0 as u16));
        } else if i < bytes.len() {
            let d0 = (bytes[i] as u32) << 4;
            data.push(Dodecet::from_hex(d0 as u16));
        }

        Ok(DodecetString { data })
    }

    /// Iterate over dodecets
    pub fn iter(&self) -> impl Iterator<Item = &Dodecet> {
        self.data.iter()
    }

    /// Get inner vector
    pub fn as_inner(&self) -> &Vec<Dodecet> {
        &self.data
    }

    /// Get mutable inner vector
    pub fn as_inner_mut(&mut self) -> &mut Vec<Dodecet> {
        &mut self.data
    }
}

impl Default for DodecetString {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for DodecetString {
    type Target = [Dodecet];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for DodecetString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl From<Vec<u16>> for DodecetString {
    fn from(values: Vec<u16>) -> Self {
        DodecetString::from_slice(&values)
    }
}

impl From<Vec<Dodecet>> for DodecetString {
    fn from(data: Vec<Dodecet>) -> Self {
        DodecetString { data }
    }
}

impl std::fmt::Display for DodecetString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, d) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", d)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let s = DodecetString::new();
        assert!(s.is_empty());

        let s = DodecetString::with_capacity(10);
        assert!(s.as_inner().capacity() >= 10);
    }

    #[test]
    fn test_push_pop() {
        let mut s = DodecetString::new();
        s.push(0x123);
        s.push(0x456);

        assert_eq!(s.len(), 2);
        assert_eq!(s[0].value(), 0x123);
        assert_eq!(s[1].value(), 0x456);

        let d = s.pop().unwrap();
        assert_eq!(d.value(), 0x456);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn test_from_slice() {
        let s = DodecetString::from_slice(&[0x123, 0x456, 0x789]);
        assert_eq!(s.len(), 3);
        assert_eq!(s[0].value(), 0x123);
        assert_eq!(s[1].value(), 0x456);
        assert_eq!(s[2].value(), 0x789);
    }

    #[test]
    fn test_hex_string() {
        let s = DodecetString::from_slice(&[0x123, 0x456, 0x789]);
        assert_eq!(s.to_hex_string(), "123456789");

        let s2 = DodecetString::from_hex_str("123456789").unwrap();
        assert_eq!(s2.len(), 3);
        assert_eq!(s2[0].value(), 0x123);
    }

    #[test]
    fn test_bytes_conversion() {
        let s = DodecetString::from_slice(&[0x123, 0x456]);
        let bytes = s.to_bytes();

        // 2 dodecets = 24 bits = 3 bytes
        assert_eq!(bytes.len(), 3);

        let s2 = DodecetString::from_bytes(&bytes).unwrap();
        assert_eq!(s2.len(), 2);
        assert_eq!(s2[0].value(), 0x123);
        assert_eq!(s2[1].value(), 0x456);
    }

    #[test]
    fn test_bytes_odd_length() {
        let s = DodecetString::from_slice(&[0x123, 0x456, 0x789]);
        let bytes = s.to_bytes();

        // 3 dodecets = 36 bits = 5 bytes (with padding)
        assert!(bytes.len() <= 5);

        let s2 = DodecetString::from_bytes(&bytes).unwrap();
        assert_eq!(s2.len(), 3);
    }

    #[test]
    fn test_display() {
        let s = DodecetString::from_slice(&[0x123, 0x456]);
        assert_eq!(format!("{}", s), "[0x123, 0x456]");
    }
}
