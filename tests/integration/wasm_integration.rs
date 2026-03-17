// Integration Tests for WASM Package
// Tests the WASM package functionality and integration with constraint-theory

use dodecet_encoder::{Dodecet, Point3D, Vector3D, Transform3D};
use std::time::{Duration, Instant};

#[cfg(test)]
mod wasm_integration_tests {
    use super::*;

    /// Test basic dodecet creation and encoding
    #[test]
    fn test_dodecet_creation() {
        let d = Dodecet::new(1000);
        assert_eq!(d.value(), 1000);
        assert_eq!(d.to_hex(), "3E8");
    }

    /// Test dodecet range validation
    #[test]
    fn test_dodecet_range() {
        // Test minimum value
        let min = Dodecet::new(0);
        assert_eq!(min.value(), 0);

        // Test maximum value (4095 = 0xFFF)
        let max = Dodecet::new(4095);
        assert_eq!(max.value(), 4095);
        assert_eq!(max.to_hex(), "FFF");

        // Test overflow handling
        let overflow = Dodecet::new(5000);
        assert_eq!(overflow.value(), 5000 % 4096);
    }

    /// Test Point3D encoding
    #[test]
    fn test_point3d_encoding() {
        let point = Point3D::new(100.0, 200.0, 300.0);
        let dodecets = point.to_dodecets();

        assert_eq!(dodecets.len(), 3);
        assert!(dodecets[0].value() < 4096);
        assert!(dodecets[1].value() < 4096);
        assert!(dodecets[2].value() < 4096);
    }

    /// Test Vector3D operations
    #[test]
    fn test_vector3d_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);

        // Test addition
        let sum = v1.add(&v2);
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);

        // Test dot product
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0);

        // Test cross product
        let cross = v1.cross(&v2);
        assert_eq!(cross.x, -3.0);
        assert_eq!(cross.y, 6.0);
        assert_eq!(cross.z, -3.0);
    }

    /// Test distance calculation
    #[test]
    fn test_distance_calculation() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 0.0);

        let distance = p1.distance_to(&p2);
        assert!((distance - 5.0).abs() < 0.001);

        // Test with dodecet encoding
        let p1_dodecets = p1.to_dodecets();
        let p2_dodecets = p2.to_dodecets();
        let p1_decoded = Point3D::from_dodecets(&p1_dodecets);
        let p2_decoded = Point3D::from_dodecets(&p2_dodecets);

        let encoded_distance = p1_decoded.distance_to(&p2_decoded);
        assert!((encoded_distance - 5.0).abs() < 1.0); // Allow for encoding loss
    }

    /// Test Transform3D operations
    #[test]
    fn test_transform3d_operations() {
        let point = Point3D::new(1.0, 0.0, 0.0);
        let transform = Transform3D::rotation_z(90.0);

        let transformed = transform.apply(&point);

        // After 90-degree rotation around Z-axis, (1,0,0) becomes (0,1,0)
        assert!((transformed.x - 0.0).abs() < 0.01);
        assert!((transformed.y - 1.0).abs() < 0.01);
        assert!((transformed.z - 0.0).abs() < 0.01);
    }

    /// Test encoding precision
    #[test]
    fn test_encoding_precision() {
        let original = Point3D::new(256.5, 512.75, 1024.25);
        let dodecets = original.to_dodecets();
        let decoded = Point3D::from_dodecets(&dodecets);

        // Check that encoding maintains reasonable precision
        let error_x = (original.x - decoded.x).abs();
        let error_y = (original.y - decoded.y).abs();
        let error_z = (original.z - decoded.z).abs();

        // With 12-bit encoding, expect <1% error for typical values
        assert!(error_x / original.x < 0.01);
        assert!(error_y / original.y < 0.01);
        assert!(error_z / original.z < 0.01);
    }

    /// Test hex encoding/decoding round-trip
    #[test]
    fn test_hex_round_trip() {
        let d1 = Dodecet::new(1234);
        let hex = d1.to_hex();
        let d2 = Dodecet::from_hex(&hex).unwrap();

        assert_eq!(d1.value(), d2.value());
    }

    /// Test array operations
    #[test]
    fn test_array_operations() {
        let dodecets = vec![
            Dodecet::new(100),
            Dodecet::new(200),
            Dodecet::new(300),
        ];

        let hex_array = Dodecet::array_to_hex(&dodecets);
        let decoded = Dodecet::array_from_hex(&hex_array).unwrap();

        assert_eq!(dodecets.len(), decoded.len());
        for (original, decoded_dodecet) in dodecets.iter().zip(decoded.iter()) {
            assert_eq!(original.value(), decoded_dodecet.value());
        }
    }

    /// Test performance: encoding speed
    #[test]
    fn test_encoding_speed() {
        let iterations = 10000;
        let start = Instant::now();

        for i in 0..iterations {
            let d = Dodecet::new(i % 4096);
            let _hex = d.to_hex();
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
            .map(|i| Dodecet::new(i % 4096).to_hex())
            .collect();

        let start = Instant::now();

        for hex in &test_values {
            let _d = Dodecet::from_hex(hex).unwrap();
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
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);

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
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 5.0);

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
        let dodecets: Vec<Dodecet> = (0..100000)
            .map(|i| Dodecet::new(i % 4096))
            .collect();

        // Each Dodecet should be 2 bytes (u16)
        // With Vec overhead, expect <500KB for 100K dodecets
        let expected_size = 100000 * 2; // 200KB for data
        let actual_size = std::mem::size_of_val(&*dodecets);

        // Allow 4x overhead for Vec structure
        assert!(actual_size < expected_size * 4,
                "Memory usage too high: {} bytes", actual_size);
    }

    /// Test constraint-theory simulator integration
    #[test]
    fn test_constraint_theory_integration() {
        // Pythagorean snapping test
        let point = Point3D::new(3.0, 4.0, 0.0);
        let origin = Point3D::new(0.0, 0.0, 0.0);
        let distance = point.distance_to(&origin);

        assert!((distance - 5.0).abs() < 0.001);

        // Test encoding preserves geometric relationship
        let point_encoded = point.to_dodecets();
        let origin_encoded = origin.to_dodecets();
        let point_decoded = Point3D::from_dodecets(&point_encoded);
        let origin_decoded = Point3D::from_dodecets(&origin_encoded);

        let encoded_distance = point_decoded.distance_to(&origin_decoded);

        // Should still be approximately 5.0
        assert!((encoded_distance - 5.0).abs() < 1.0);
    }

    /// Test hex array format for constraint-theory
    #[test]
    fn test_hex_array_format() {
        let point = Point3D::new(100.0, 200.0, 300.0);
        let dodecets = point.to_dodecets();
        let hex_array = Dodecet::array_to_hex(&dodecets);

        // Should be 6 hex characters (3 dodecets * 2 chars each)
        assert_eq!(hex_array.len(), 6);

        // Should be valid hex
        assert!(hex_array.chars().all(|c| c.is_ascii_hexdigit()));
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
                .map(|i| Dodecet::new(i % 4096))
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
            let d = Dodecet::new(i % 4096);
            let _hex = d.to_hex();
            // Simulate JS-WASM boundary crossing
            let _json = format!("{{\"value\": {}, \"hex\": \"{}\"}}", d.value(), d.to_hex());
        }
        let duration_with_overhead = start.elapsed();

        // Test without overhead
        let start = Instant::now();
        for i in 0..iterations {
            let d = Dodecet::new(i % 4096);
            let _hex = d.to_hex();
        }
        let duration_without_overhead = start.elapsed();

        // Overhead should be <2x
        let overhead_ratio = duration_with_overhead.as_nanos() as f64 /
                           duration_without_overhead.as_nanos() as f64;

        assert!(overhead_ratio < 2.0,
                "WASM overhead too high: {}x", overhead_ratio);
    }
}
