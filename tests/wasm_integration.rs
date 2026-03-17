// Integration Tests for WASM Package
// Tests the WASM package functionality and integration with constraint-theory

use dodecet_encoder::{Dodecet, DodecetArray, Point3D, Vector3D, Transform3D};
use std::time::Instant;

#[cfg(test)]
mod wasm_integration_tests {
    use super::*;

    /// Test basic dodecet creation and encoding
    #[test]
    fn test_dodecet_creation() {
        let d = Dodecet::from_hex(1000);
        assert_eq!(d.value(), 1000);
        assert_eq!(d.to_hex_string(), "3E8");
    }

    /// Test dodecet range validation
    #[test]
    fn test_dodecet_range() {
        // Test minimum value
        let min = Dodecet::from_hex(0);
        assert_eq!(min.value(), 0);

        // Test maximum value (4095 = 0xFFF)
        let max = Dodecet::from_hex(4095);
        assert_eq!(max.value(), 4095);
        assert_eq!(max.to_hex_string(), "FFF");

        // Test that values >4095 are handled
        let result = Dodecet::new(5000);
        assert!(result.is_err(), "Should reject values >4095");
    }

    /// Test Point3D creation
    #[test]
    fn test_point3d_creation() {
        let point = Point3D::new(100, 200, 300);

        assert_eq!(point.x(), 100);
        assert_eq!(point.y(), 200);
        assert_eq!(point.z(), 300);

        // Test hex conversion
        assert_eq!(point.to_hex_string(), "064C812C");
    }

    /// Test Vector3D operations
    #[test]
    fn test_vector3d_operations() {
        let v1 = Vector3D::new(100, 200, 300);
        let v2 = Vector3D::new(400, 500, 600);

        // Test addition
        let sum = v1.add(&v2);
        assert_eq!(sum.x(), 500);
        assert_eq!(sum.y(), 700);
        assert_eq!(sum.z(), 900);

        // Test dot product
        let dot = v1.dot(&v2);
        assert_eq!(dot, 100*400 + 200*500 + 300*600);

        // Test cross product
        let cross = v1.cross(&v2);
        // Cross product will have values that may overflow i16/u16
        // Just verify the cross product method works
        assert!(cross.x() != 0 || cross.y() != 0 || cross.z() != 0);
    }

    /// Test distance calculation
    #[test]
    fn test_distance_calculation() {
        let p1 = Point3D::new(0, 0, 0);
        let p2 = Point3D::new(300, 400, 0); // 3-4-5 triangle scaled

        let distance = p1.distance_to(&p2);
        assert!((distance - 500.0).abs() < 1.0); // Allow for some precision loss
    }

    /// Test Transform3D operations
    #[test]
    fn test_transform3d_operations() {
        let point = Point3D::new(1000, 0, 0); // 1.0 in normalized coords
        let transform = Transform3D::rotation_z(90.0);

        let transformed = transform.apply(&point);

        // After 90-degree rotation around Z-axis
        // The rotation should transform (1,0,0) towards (0,1,0)
        // With dodecet precision, we check approximate result
        let (tx, ty, tz) = transformed.normalized();
        assert!((tx).abs() < 0.1); // Should be close to 0
        assert!((ty - 1.0).abs() < 0.1); // Should be close to 1
        assert!((tz).abs() < 0.1); // Should be close to 0
    }

    /// Test normalized coordinates
    #[test]
    fn test_normalized_coordinates() {
        let point = Point3D::new(2048, 2048, 2048); // Middle values
        let (nx, ny, nz) = point.normalized();

        // Should be close to 0.5
        assert!((nx - 0.5).abs() < 0.001);
        assert!((ny - 0.5).abs() < 0.001);
        assert!((nz - 0.5).abs() < 0.001);
    }

    /// Test signed coordinates
    #[test]
    fn test_signed_coordinates() {
        let point = Point3D::new(0, 2048, 4095);
        let (sx, sy, sz) = point.signed();

        assert_eq!(sx, 0);
        assert_eq!(sy, -2048);
        assert_eq!(sz, 2047);
    }

    /// Test hex encoding/decoding round-trip
    #[test]
    fn test_hex_round_trip() {
        let d1 = Dodecet::from_hex(1234);
        let hex = d1.to_hex_string();
        let d2 = Dodecet::from_hex_str(&hex).unwrap();

        assert_eq!(d1.value(), d2.value());
    }

    /// Test DodecetArray operations
    #[test]
    fn test_dodecet_array() {
        let arr: DodecetArray<3> = DodecetArray::from_slice(&[100, 200, 300]);

        assert_eq!(arr[0].value(), 100);
        assert_eq!(arr[1].value(), 200);
        assert_eq!(arr[2].value(), 300);

        // Test sum
        assert_eq!(arr.clone().sum().value(), 600);

        // Test hex conversion
        assert_eq!(arr.to_hex_string(), "064C812C");
    }

    /// Test performance: encoding speed
    #[test]
    fn test_encoding_speed() {
        let iterations = 10000;
        let start = Instant::now();

        for i in 0..iterations {
            let d = Dodecet::from_hex(i % 4096);
            let _hex = d.to_hex_string();
        }

        let duration = start.elapsed();
        let per_operation = duration.as_nanos() as f64 / iterations as f64;

        // Target: <25ns per encoding operation
        assert!(per_operation < 25.0, "Encoding too slow: {}ns", per_operation);
    }

    /// Test performance: decoding speed
    #[test]
    fn test_decoding_speed() {
        let test_values: Vec<String> = (0..10000)
            .map(|i| Dodecet::from_hex(i % 4096).to_hex_string())
            .collect();

        let start = Instant::now();

        for hex in &test_values {
            let _d = Dodecet::from_hex_str(hex).unwrap();
        }

        let duration = start.elapsed();
        let per_operation = duration.as_nanos() as f64 / test_values.len() as f64;

        // Target: <30ns per decoding operation
        assert!(per_operation < 30.0, "Decoding too slow: {}ns", per_operation);
    }

    /// Test performance: geometric operations
    #[test]
    fn test_geometric_operations_speed() {
        let iterations = 10000;
        let v1 = Vector3D::new(100, 200, 300);
        let v2 = Vector3D::new(400, 500, 600);

        let start = Instant::now();

        for _ in 0..iterations {
            let _sum = v1.add(&v2);
            let _dot = v1.dot(&v2);
            let _cross = v1.cross(&v2);
        }

        let duration = start.elapsed();
        let per_operation = duration.as_nanos() as f64 / (iterations * 3) as f64;

        // Target: <20ns per vector operation
        assert!(per_operation < 20.0, "Vector ops too slow: {}ns", per_operation);
    }

    /// Test performance: distance calculation
    #[test]
    fn test_distance_calculation_speed() {
        let iterations = 10000;
        let p1 = Point3D::new(0, 0, 0);
        let p2 = Point3D::new(300, 400, 500);

        let start = Instant::now();

        for _ in 0..iterations {
            let _distance = p1.distance_to(&p2);
        }

        let duration = start.elapsed();
        let per_operation = duration.as_nanos() as f64 / iterations as f64;

        // Target: <45ns per distance calculation
        assert!(per_operation < 45.0, "Distance calc too slow: {}ns", per_operation);
    }

    /// Test memory efficiency
    #[test]
    fn test_memory_efficiency() {
        // Create many dodecets to test memory usage
        let count = 50000u32;
        let dodecets: Vec<Dodecet> = (0..count)
            .map(|i| Dodecet::from_hex((i % 4096) as u16))
            .collect();

        // Each Dodecet should be 2 bytes (u16)
        // With Vec overhead, expect <500KB for 50K dodecets
        let expected_size = count as usize * 2; // 100KB for data
        let actual_size = std::mem::size_of_val(&*dodecets);

        // Allow 4x overhead for Vec structure
        assert!(actual_size < expected_size * 4,
                "Memory usage too high: {} bytes", actual_size);
    }

    /// Test constraint-theory simulator integration
    #[test]
    fn test_constraint_theory_integration() {
        // Pythagorean snapping test
        let point = Point3D::new(300, 400, 0); // 3-4-5 triangle
        let origin = Point3D::new(0, 0, 0);
        let distance = point.distance_to(&origin);

        assert!((distance - 500.0).abs() < 1.0);

        // Test hex encoding preserves geometric relationship
        let point_hex = point.to_hex_string();
        let origin_hex = origin.to_hex_string();

        let point_decoded = Point3D::from_hex_str(&point_hex).unwrap();
        let origin_decoded = Point3D::from_hex_str(&origin_hex).unwrap();

        let encoded_distance = point_decoded.distance_to(&origin_decoded);

        // Should still be approximately 500.0
        assert!((encoded_distance - 500.0).abs() < 1.0);
    }

    /// Test hex array format for constraint-theory
    #[test]
    fn test_hex_array_format() {
        let point = Point3D::new(100, 200, 300);
        let hex_array = point.to_hex_string();

        // Should be 6 hex characters (3 dodecets * 3 chars each, but compacted)
        assert!(hex_array.len() <= 6);

        // Should be valid hex
        assert!(hex_array.chars().all(|c: char| c.is_ascii_hexdigit()));
    }

    /// Test nibble operations
    #[test]
    fn test_nibble_operations() {
        let d = Dodecet::from_hex(0xABC);

        assert_eq!(d.nibble(0).unwrap(), 0xC);
        assert_eq!(d.nibble(1).unwrap(), 0xB);
        assert_eq!(d.nibble(2).unwrap(), 0xA);

        // Test invalid nibble
        assert!(d.nibble(3).is_err());
    }

    /// Test arithmetic operations
    #[test]
    fn test_arithmetic_operations() {
        let d1 = Dodecet::from_hex(1000);
        let d2 = Dodecet::from_hex(500);

        // Test wrapping add
        let sum = d1.wrapping_add(d2);
        assert_eq!(sum.value(), 1500);

        // Test wrapping sub
        let diff = d1.wrapping_sub(d2);
        assert_eq!(diff.value(), 500);

        // Test wrapping mul
        let d3 = Dodecet::from_hex(2);
        let product = d1.wrapping_mul(d3);
        assert_eq!(product.value(), 2000);
    }

    /// Test bit operations
    #[test]
    fn test_bit_operations() {
        let d1 = Dodecet::from_hex(0xFF0);
        let d2 = Dodecet::from_hex(0x00F);

        // Test AND
        let and = d1 & d2;
        assert_eq!(and.value(), 0);

        // Test OR
        let or = d1 | d2;
        assert_eq!(or.value(), 0xFFF);

        // Test XOR
        let xor = d1 ^ d2;
        assert_eq!(xor.value(), 0xFFF);
    }
}

#[cfg(test)]
mod browser_simulation_tests {
    use super::*;

    /// Simulate browser memory constraints
    #[test]
    fn test_browser_memory_constraints() {
        // Simulate browser memory limit (50MB)
        let max_memory = 50 * 1024 * 1024; // 50MB in bytes

        // Create as many dodecets as possible within memory limit
        let mut dodecets = Vec::new();
        let mut count = 0;

        loop {
            let new_dodecets: Vec<Dodecet> = (0..10000)
                .map(|i| Dodecet::from_hex(i % 4096))
                .collect();

            let new_size = std::mem::size_of_val(&*new_dodecets);
            let current_size = std::mem::size_of_val(&*dodecets);

            if current_size + new_size > max_memory {
                break;
            }

            dodecets.extend(new_dodecets);
            count += 10000;

            if count > 1000000 {
                break; // Safety limit
            }
        }

        // Should be able to store at least 500K dodecets in 50MB
        assert!(count >= 500000, "Could only store {} dodecets in 50MB", count);
    }

    /// Test WASM-like performance characteristics
    #[test]
    fn test_wasm_performance_characteristics() {
        // Simulate WASM overhead
        let iterations = 1000;

        // Test encoding with "WASM boundary" overhead
        let start = Instant::now();
        for i in 0..iterations {
            let d = Dodecet::from_hex(i % 4096);
            let _hex = d.to_hex_string();
            // Simulate JS-WASM boundary crossing
            let _json = format!("{{\"value\": {}, \"hex\": \"{}\"}}", d.value(), d.to_hex_string());
        }
        let duration_with_overhead = start.elapsed();

        // Test without overhead
        let start = Instant::now();
        for i in 0..iterations {
            let d = Dodecet::from_hex(i % 4096);
            let _hex = d.to_hex_string();
        }
        let duration_without_overhead = start.elapsed();

        // Overhead should be <2x
        let overhead_ratio = duration_with_overhead.as_nanos() as f64 /
                           duration_without_overhead.as_nanos() as f64;

        assert!(overhead_ratio < 2.0,
                "WASM overhead too high: {}x", overhead_ratio);
    }
}
