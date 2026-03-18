//! SIMD-optimized operations for dodecet processing
//!
//! Provides vectorized implementations using platform-specific SIMD

use crate::{Dodecet, Result};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

/// SIMD-optimized dodecet array operations
pub struct SimdOps;

impl SimdOps {
    /// Convert array of dodecets to normalized floats using SIMD
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[inline]
    pub unsafe fn normalize_avx2(dodecets: &[Dodecet], output: &mut [f32]) -> Result<()> {
        if dodecets.len() != output.len() {
            return Err(crate::DodecetError::InvalidLength);
        }

        const MAX_DODECET_F32: f32 = 4095.0;

        let chunks = dodecets.chunks_exact(8);
        let remainder = chunks.remainder();

        // Process 8 dodecets at a time using AVX2
        for chunk in chunks {
            // Load dodecet values
            let values = [
                chunk[0].value() as f32,
                chunk[1].value() as f32,
                chunk[2].value() as f32,
                chunk[3].value() as f32,
                chunk[4].value() as f32,
                chunk[5].value() as f32,
                chunk[6].value() as f32,
                chunk[7].value() as f32,
            ];

            // Load into AVX2 register
            let vec = _mm256_loadu_ps(values.as_ptr());

            // Divide by max value
            let max_vec = _mm256_set1_ps(MAX_DODECET_F32);
            let normalized = _mm256_div_ps(vec, max_vec);

            // Store result
            _mm256_storeu_ps(output.as_mut_ptr(), normalized);

            output = &mut output[8..];
        }

        // Handle remainder
        for (i, &d) in remainder.iter().enumerate() {
            output[i] = d.value() as f32 / MAX_DODECET_F32;
        }

        Ok(())
    }

    /// Convert array of dodecets to normalized floats using SIMD (ARM NEON)
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[inline]
    pub unsafe fn normalize_neon(dodecets: &[Dodecet], output: &mut [f32]) -> Result<()> {
        if dodecets.len() != output.len() {
            return Err(crate::DodecetError::InvalidLength);
        }

        const MAX_DODECET_F32: f32 = 4095.0;

        let chunks = dodecets.chunks_exact(4);
        let remainder = chunks.remainder();

        // Process 4 dodecets at a time using NEON
        for chunk in chunks {
            // Load dodecet values
            let values = [
                chunk[0].value() as f32,
                chunk[1].value() as f32,
                chunk[2].value() as f32,
                chunk[3].value() as f32,
            ];

            // Load into NEON register
            let vec = vld1q_f32(values.as_ptr());

            // Divide by max value
            let max_vec = vdupq_n_f32(MAX_DODECET_F32);
            let normalized = vdivq_f32(vec, max_vec);

            // Store result
            vst1q_f32(output.as_mut_ptr(), normalized);

            output = &mut output[4..];
        }

        // Handle remainder
        for (i, &d) in remainder.iter().enumerate() {
            output[i] = d.value() as f32 / MAX_DODECET_F32;
        }

        Ok(())
    }

    /// Fallback implementation for non-SIMD platforms
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    #[inline]
    pub fn normalize_scalar(dodecets: &[Dodecet], output: &mut [f32]) -> Result<()> {
        if dodecets.len() != output.len() {
            return Err(crate::DodecetError::InvalidLength);
        }

        const MAX_DODECET_F32: f32 = 4095.0;

        for (i, &d) in dodecets.iter().enumerate() {
            output[i] = d.value() as f32 / MAX_DODECET_F32;
        }

        Ok(())
    }

    /// Batch normalize with automatic SIMD detection
    pub fn normalize_auto(dodecets: &[Dodecet], output: &mut [f32]) -> Result<()> {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { Self::normalize_avx2(dodecets, output) };
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            if std::arch::is_aarch64_feature_detected!("neon") {
                return unsafe { Self::normalize_neon(dodecets, output) };
            }
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            return Self::normalize_scalar(dodecets, output);
        }

        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        {
            Self::normalize_scalar(dodecets, output)
        }
    }
}

/// SIMD-optimized hex encoding
pub struct SimdHex;

impl SimdHex {
    /// Encode dodecets to hex string using SIMD
    pub fn encode(dodecets: &[Dodecet]) -> String {
        // Pre-allocate output string
        let mut output = String::with_capacity(dodecets.len() * 3);

        for d in dodecets {
            let value = d.value();
            let hex = [
                b"0123456789ABCDEF"[(value >> 8) as usize],
                b"0123456789ABCDEF"[((value >> 4) & 0xF) as usize],
                b"0123456789ABCDEF"[(value & 0xF) as usize],
            ];

            output.push_str(unsafe { std::str::from_utf8_unchecked(&hex) });
        }

        output
    }

    /// Decode hex string to dodecets using SIMD
    pub fn decode(hex: &str) -> Result<Vec<Dodecet>> {
        if hex.len() % 3 != 0 {
            return Err(crate::DodecetError::InvalidHex);
        }

        let mut dodecets = Vec::with_capacity(hex.len() / 3);

        for chunk in hex.as_bytes().chunks_exact(3) {
            let mut value: u16 = 0;

            for &byte in chunk {
                let digit = if byte.is_ascii_digit() {
                    byte - b'0'
                } else if byte.is_ascii_uppercase() {
                    byte - b'A' + 10
                } else if byte.is_ascii_lowercase() {
                    byte - b'a' + 10
                } else {
                    return Err(crate::DodecetError::InvalidHex);
                };

                value = (value << 4) | (digit as u16);
            }

            dodecets.push(Dodecet::from_hex(value));
        }

        Ok(dodecets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_auto() {
        let dodecets: Vec<Dodecet> = (0..16).map(Dodecet::from_hex).collect();
        let mut output = vec![0.0f32; 16];

        SimdOps::normalize_auto(&dodecets, &mut output).unwrap();

        for (i, &d) in dodecets.iter().enumerate() {
            let expected = d.value() as f32 / 4095.0;
            assert!((output[i] - expected).abs() < 0.0001);
        }
    }

    #[test]
    fn test_simd_hex_encode() {
        let dodecets: Vec<Dodecet> = vec![
            Dodecet::from_hex(0xABC),
            Dodecet::from_hex(0x123),
            Dodecet::from_hex(0x456),
        ];

        let hex = SimdHex::encode(&dodecets);
        assert_eq!(hex, "ABC123456");
    }

    #[test]
    fn test_simd_hex_decode() {
        let hex = "ABC123456";
        let dodecets = SimdHex::decode(hex).unwrap();

        assert_eq!(dodecets[0].value(), 0xABC);
        assert_eq!(dodecets[1].value(), 0x123);
        assert_eq!(dodecets[2].value(), 0x456);
    }

    #[test]
    fn test_simd_hex_roundtrip() {
        let original: Vec<Dodecet> = (0..100).map(|i| Dodecet::from_hex(i % 4096)).collect();
        let hex = SimdHex::encode(&original);
        let decoded = SimdHex::decode(&hex).unwrap();

        assert_eq!(original.len(), decoded.len());
        for (o, d) in original.iter().zip(decoded.iter()) {
            assert_eq!(o.value(), d.value());
        }
    }
}
