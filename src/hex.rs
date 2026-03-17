//! # Hex encoding/decoding utilities
//!
//! Efficient bidirectional conversion between dodecets and hex strings.

use crate::{Dodecet, DodecetError, Result};

/// Encode a slice of dodecets to a hex string
///
/// Each dodecet becomes 3 hex characters.
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::{Dodecet, hex};
///
/// let dodecets = vec![Dodecet::from_hex(0x123), Dodecet::from_hex(0x456)];
/// let hex_str = hex::encode(&dodecets);
/// assert_eq!(hex_str, "123456");
/// ```
pub fn encode(dodecets: &[Dodecet]) -> String {
    dodecets
        .iter()
        .map(|d| d.to_hex_string())
        .collect::<Vec<_>>()
        .join("")
}

/// Decode a hex string to a vector of dodecets
///
/// # Errors
/// Returns `DodecetError::InvalidHex` if:
/// - String length is not a multiple of 3
/// - Contains non-hex characters
/// - Any value > 4095
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::hex;
///
/// let hex_str = "123456789";
/// let dodecets = hex::decode(hex_str).unwrap();
/// assert_eq!(dodecets.len(), 3);
/// assert_eq!(dodecets[0].value(), 0x123);
/// ```
pub fn decode(s: &str) -> Result<Vec<Dodecet>> {
    if !s.len().is_multiple_of(3) {
        return Err(DodecetError::InvalidHex);
    }

    s.as_bytes()
        .chunks(3)
        .map(|chunk| {
            let chunk_str = std::str::from_utf8(chunk).map_err(|_| DodecetError::InvalidHex)?;
            Dodecet::from_hex_str(chunk_str)
        })
        .collect()
}

/// Encode a single dodecet to 3 hex characters
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::{Dodecet, hex};
///
/// let d = Dodecet::from_hex(0xAB0);
/// assert_eq!(hex::encode_dodecet(d), "AB0");
/// ```
pub fn encode_dodecet(d: Dodecet) -> String {
    d.to_hex_string()
}

/// Decode 3 hex characters to a single dodecet
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::{Dodecet, hex};
///
/// let d = hex::decode_dodecet("AB0").unwrap();
/// assert_eq!(d.value(), 0xAB0);
/// ```
pub fn decode_dodecet(s: &str) -> Result<Dodecet> {
    if s.len() != 3 {
        return Err(DodecetError::InvalidHex);
    }
    Dodecet::from_hex_str(s)
}

/// Validate a hex string for dodecet encoding
///
/// Returns true if the string is valid for decoding.
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::hex;
///
/// assert!(hex::is_valid("123456"));
/// assert!(!hex::is_valid("12345")); // Not multiple of 3
/// assert!(!hex::is_valid("GHI")); // Non-hex characters
/// ```
pub fn is_valid(s: &str) -> bool {
    if !s.len().is_multiple_of(3) {
        return false;
    }

    s.chars()
        .all(|c| c.is_ascii_hexdigit() && c.is_ascii_alphanumeric())
}

/// Format a hex string with spacing for readability
///
/// Adds a space every 3 characters (every dodecet).
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::hex;
///
/// let formatted = hex::format_spaced("123456789");
/// assert_eq!(formatted, "123 456 789");
/// ```
pub fn format_spaced(s: &str) -> String {
    s.as_bytes()
        .chunks(3)
        .map(std::str::from_utf8)
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| DodecetError::InvalidHex)
        .unwrap()
        .join(" ")
}

/// Remove spaces from a spaced hex string
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::hex;
///
/// let unspaced = hex::remove_spaces("123 456 789");
/// assert_eq!(unspaced, "123456789");
/// ```
pub fn remove_spaces(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Convert hex string to uppercase
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::hex;
///
/// assert_eq!(hex::to_uppercase("abc"), "ABC");
/// ```
pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

/// Convert hex string to lowercase
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::hex;
///
/// assert_eq!(hex::to_lowercase("ABC"), "abc");
/// ```
pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

/// Calculate the number of dodecets in a hex string
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::hex;
///
/// assert_eq!(hex::dodecet_count("123456789"), 3);
/// ```
pub fn dodecet_count(s: &str) -> usize {
    s.len() / 3
}

/// Create a hex editor-friendly view
///
/// Shows offset, hex values, and ASCII representation.
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::hex;
///
/// let view = hex::hex_view("123456789ABCDEF");
/// // Output includes offset, hex, and ASCII
/// ```
pub fn hex_view(s: &str) -> String {
    let mut view = String::new();
    let dodecets = decode(s).unwrap_or_default();

    for (i, d) in dodecets.iter().enumerate() {
        let offset = i * 3;
        let hex_val = format!("{:03X}", d.value());
        let ascii = dodecet_to_ascii(d);

        view.push_str(&format!("{:08X}  {}  |{}|\n", offset, hex_val, ascii));
    }

    view
}

/// Convert a dodecet to an ASCII representation
///
/// Maps each nibble to a character.
fn dodecet_to_ascii(d: &Dodecet) -> String {
    let n0 = d.nibble(0).unwrap();
    let n1 = d.nibble(1).unwrap();
    let n2 = d.nibble(2).unwrap();

    let to_char = |n: u8| -> char {
        if (0x20..=0x7E).contains(&n) {
            n as char
        } else {
            '.'
        }
    };

    format!("{}{}{}", to_char(n2), to_char(n1), to_char(n0))
}

/// Compare two hex strings for equality (case-insensitive)
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::hex;
///
/// assert!(hex::equal_ignore_case("ABC123", "abc123"));
/// ```
pub fn equal_ignore_case(a: &str, b: &str) -> bool {
    a.to_lowercase() == b.to_lowercase()
}

/// XOR two hex strings of equal length
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::hex;
///
/// let result = hex::xor("FFF", "123").unwrap();
/// assert_eq!(result, "EDC");
/// ```
pub fn xor(a: &str, b: &str) -> Result<String> {
    if a.len() != b.len() {
        return Err(DodecetError::InvalidHex);
    }

    let d1 = decode(a)?;
    let d2 = decode(b)?;

    let result: Vec<Dodecet> = d1
        .iter()
        .zip(d2.iter())
        .map(|(d1, d2)| d1.xor(*d2))
        .collect();

    Ok(encode(&result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let dodecets = vec![Dodecet::from_hex(0x123), Dodecet::from_hex(0x456)];
        let hex_str = encode(&dodecets);
        assert_eq!(hex_str, "123456");

        let decoded = decode(&hex_str).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].value(), 0x123);
        assert_eq!(decoded[1].value(), 0x456);
    }

    #[test]
    fn test_encode_decode_dodecet() {
        let d = Dodecet::from_hex(0xAB0);
        assert_eq!(encode_dodecet(d), "AB0");

        let d2 = decode_dodecet("AB0").unwrap();
        assert_eq!(d2.value(), 0xAB0);
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("123456"));
        assert!(!is_valid("12345")); // Not multiple of 3
        assert!(is_valid("")); // Empty string is valid
    }

    #[test]
    fn test_format_spaced() {
        assert_eq!(format_spaced("123456789"), "123 456 789");
    }

    #[test]
    fn test_remove_spaces() {
        assert_eq!(remove_spaces("123 456 789"), "123456789");
    }

    #[test]
    fn test_uppercase_lowercase() {
        assert_eq!(to_uppercase("abc"), "ABC");
        assert_eq!(to_lowercase("ABC"), "abc");
    }

    #[test]
    fn test_dodecet_count() {
        assert_eq!(dodecet_count("123456789"), 3);
        assert_eq!(dodecet_count(""), 0);
    }

    #[test]
    fn test_equal_ignore_case() {
        assert!(equal_ignore_case("ABC123", "abc123"));
        assert!(!equal_ignore_case("ABC123", "ABC124"));
    }

    #[test]
    fn test_xor() {
        let result = xor("FFF", "123").unwrap();
        assert_eq!(result, "EDC");

        let result = xor("000", "000").unwrap();
        assert_eq!(result, "000");
    }

    #[test]
    fn test_invalid_hex() {
        assert!(decode("GHI").is_err());
        assert!(decode("12345").is_err());
    }

    #[test]
    fn test_hex_view() {
        let view = hex_view("123456");
        assert!(view.contains("123"));
        assert!(view.contains("456"));
    }
}
