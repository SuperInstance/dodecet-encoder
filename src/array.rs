//! # DodecetArray: Fixed-size arrays of dodecets
//!
//! Provides fixed-size arrays optimized for geometric operations.

use crate::Dodecet;
use std::ops::{Deref, DerefMut};

/// A fixed-size array of dodecets
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::{Dodecet, DodecetArray};
///
/// let arr = DodecetArray::<3>::from_slice(&[0x123, 0x456, 0x789]);
/// assert_eq!(arr[0], Dodecet::from_hex(0x123));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DodecetArray<const N: usize> {
    data: [Dodecet; N],
}

impl<const N: usize> DodecetArray<N> {
    /// Create a new dodecet array initialized to zeros
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetArray;
    ///
    /// let arr: DodecetArray<3> = DodecetArray::new();
    /// assert!(arr.iter().all(|d| d.is_zero()));
    /// ```
    pub fn new() -> Self
    where
        [Dodecet; N]: Sized,
    {
        DodecetArray {
            data: [Dodecet::from_hex(0); N],
        }
    }

    /// Create from a slice of u16 values
    ///
    /// # Panics
    /// Panics if slice length != N or any value > 4095
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetArray;
    ///
    /// let arr = DodecetArray::<3>::from_slice(&[0x123, 0x456, 0x789]);
    /// ```
    pub fn from_slice(values: &[u16]) -> Self
    where
        [Dodecet; N]: Sized,
    {
        assert_eq!(values.len(), N, "Slice length must match array size");

        let data: [Dodecet; N] = values
            .iter()
            .map(|&v| Dodecet::from_hex(v))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        DodecetArray { data }
    }

    /// Create from dodecets
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::{Dodecet, DodecetArray};
    ///
    /// let arr = DodecetArray::<3>::from_dodecets([
    ///     Dodecet::from_hex(0x123),
    ///     Dodecet::from_hex(0x456),
    ///     Dodecet::from_hex(0x789),
    /// ]);
    /// ```
    pub fn from_dodecets(data: [Dodecet; N]) -> Self {
        DodecetArray { data }
    }

    /// Get inner array
    pub fn as_inner(&self) -> &[Dodecet; N] {
        &self.data
    }

    /// Get mutable inner array
    pub fn as_inner_mut(&mut self) -> &mut [Dodecet; N] {
        &mut self.data
    }

    /// Convert to hex string (concatenated)
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetArray;
    ///
    /// let arr = DodecetArray::<2>::from_slice(&[0x123, 0x456]);
    /// assert_eq!(arr.to_hex_string(), "123456");
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
    /// use dodecet_encoder::DodecetArray;
    ///
    /// let arr = DodecetArray::<2>::from_hex_str("123456").unwrap();
    /// assert_eq!(arr[0].value(), 0x123);
    /// assert_eq!(arr[1].value(), 0x456);
    /// ```
    pub fn from_hex_str(s: &str) -> crate::Result<Self>
    where
        [Dodecet; N]: Sized,
    {
        let expected_len = N * 3;
        if s.len() != expected_len {
            return Err(crate::DodecetError::InvalidHex);
        }

        let mut data = [Dodecet::from_hex(0); N];

        for (i, chunk) in s.as_bytes().chunks(3).enumerate() {
            let chunk_str = std::str::from_utf8(chunk).unwrap();
            data[i] = Dodecet::from_hex_str(chunk_str)?;
        }

        Ok(DodecetArray { data })
    }

    /// Map each dodecet through a function
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetArray;
    ///
    /// let arr = DodecetArray::<3>::from_slice(&[0x100, 0x200, 0x300]);
    /// let doubled = arr.map(|d| Dodecet::from_hex(d.value() * 2));
    /// assert_eq!(doubled[0].value(), 0x200);
    /// ```
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnMut(Dodecet) -> Dodecet,
    {
        DodecetArray {
            data: self.data.map(f),
        }
    }

    /// Zip with another array
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetArray;
    ///
    /// let a = DodecetArray::<3>::from_slice(&[0x100, 0x200, 0x300]);
    /// let b = DodecetArray::<3>::from_slice(&[0x001, 0x002, 0x003]);
    /// let sum = a.zip_map(b, |x, y| x + y);
    /// assert_eq!(sum[0].value(), 0x101);
    /// ```
    pub fn zip_map<F>(self, other: Self, mut f: F) -> Self
    where
        F: FnMut(Dodecet, Dodecet) -> Dodecet,
    {
        DodecetArray {
            data: self
                .data
                .into_iter()
                .zip(other.data)
                .map(|(a, b)| f(a, b))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    /// Sum all dodecets
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetArray;
    ///
    /// let arr = DodecetArray::<3>::from_slice(&[0x100, 0x200, 0x300]);
    /// assert_eq!(arr.sum().value(), 0x600);
    /// ```
    pub fn sum(self) -> Dodecet {
        self.data.into_iter().fold(Dodecet::from_hex(0), |acc, d| acc + d)
    }

    /// Get the average
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::DodecetArray;
    ///
    /// let arr = DodecetArray::<3>::from_slice(&[0x000, 0x003, 0x006]);
    /// let avg = arr.average();
    /// assert_eq!(avg.value(), 0x003);
    /// ```
    pub fn average(&self) -> Dodecet {
        let sum: u32 = self.data.iter().map(|d| d.value() as u32).sum();
        Dodecet::from_hex((sum / N as u32) as u16)
    }
}

impl<const N: usize> Default for DodecetArray<N>
where
    [Dodecet; N]: Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Deref for DodecetArray<N> {
    type Target = [Dodecet; N];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<const N: usize> DerefMut for DodecetArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<const N: usize> From<[u16; N]> for DodecetArray<N> {
    fn from(values: [u16; N]) -> Self {
        DodecetArray::from_slice(&values)
    }
}

impl<const N: usize> From<[Dodecet; N]> for DodecetArray<N> {
    fn from(data: [Dodecet; N]) -> Self {
        DodecetArray { data }
    }
}

impl<const N: usize> std::fmt::Display for DodecetArray<N> {
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
        let arr: DodecetArray<3> = DodecetArray::new();
        assert_eq!(arr.len(), 3);
        assert!(arr.iter().all(|d| d.is_zero()));
    }

    #[test]
    fn test_from_slice() {
        let arr = DodecetArray::<3>::from_slice(&[0x123, 0x456, 0x789]);
        assert_eq!(arr[0].value(), 0x123);
        assert_eq!(arr[1].value(), 0x456);
        assert_eq!(arr[2].value(), 0x789);
    }

    #[test]
    fn test_indexing() {
        let mut arr = DodecetArray::<3>::from_slice(&[0x123, 0x456, 0x789]);
        assert_eq!(arr[0].value(), 0x123);

        arr[1] = Dodecet::from_hex(0xABC);
        assert_eq!(arr[1].value(), 0xABC);
    }

    #[test]
    fn test_hex_string() {
        let arr = DodecetArray::<2>::from_slice(&[0x123, 0x456]);
        assert_eq!(arr.to_hex_string(), "123456");

        let arr2 = DodecetArray::<2>::from_hex_str("123456").unwrap();
        assert_eq!(arr2[0].value(), 0x123);
        assert_eq!(arr2[1].value(), 0x456);
    }

    #[test]
    fn test_map() {
        let arr = DodecetArray::<3>::from_slice(&[0x100, 0x200, 0x300]);
        let doubled = arr.map(|d| Dodecet::from_hex(d.value() * 2));
        assert_eq!(doubled[0].value(), 0x200);
        assert_eq!(doubled[1].value(), 0x400);
        assert_eq!(doubled[2].value(), 0x600);
    }

    #[test]
    fn test_zip_map() {
        let a = DodecetArray::<3>::from_slice(&[0x100, 0x200, 0x300]);
        let b = DodecetArray::<3>::from_slice(&[0x001, 0x002, 0x003]);
        let sum = a.zip_map(b, |x, y| x + y);
        assert_eq!(sum[0].value(), 0x101);
        assert_eq!(sum[1].value(), 0x202);
        assert_eq!(sum[2].value(), 0x303);
    }

    #[test]
    fn test_sum() {
        let arr = DodecetArray::<3>::from_slice(&[0x100, 0x200, 0x300]);
        assert_eq!(arr.sum().value(), 0x600);
    }

    #[test]
    fn test_average() {
        let arr = DodecetArray::<3>::from_slice(&[0x000, 0x003, 0x006]);
        assert_eq!(arr.average().value(), 0x003);
    }

    #[test]
    fn test_display() {
        let arr = DodecetArray::<3>::from_slice(&[0x123, 0x456, 0x789]);
        assert_eq!(format!("{}", arr), "[0x123, 0x456, 0x789]");
    }
}
