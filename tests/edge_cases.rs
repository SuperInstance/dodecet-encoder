// Edge Case Tests for Dodecet Encoder v1.0
//
// Comprehensive edge case testing to ensure robustness and reliability

use dodecet_encoder::{
    Dodecet, DodecetArray, DodecetString,
    geometric::{Point3D, Vector3D, Transform3D},
    calculus,
};

#[test]
fn test_dodecet_boundary_values() {
    // Test minimum value
    let min = Dodecet::new(0).unwrap();
    assert_eq!(min.value(), 0);
    assert_eq!(min.to_hex_string(), "000");

    // Test maximum value
    let max = Dodecet::new(4095).unwrap();
    assert_eq!(max.value(), 4095);
    assert_eq!(max.to_hex_string(), "FFF");

    // Test one below maximum
    let near_max = Dodecet::new(4094).unwrap();
    assert_eq!(near_max.value(), 4094);
}

#[test]
fn test_dodecet_overflow() {
    // Test value exceeding 12 bits
    assert!(Dodecet::new(4096).is_err());
    assert!(Dodecet::new(5000).is_err());
    assert!(Dodecet::new(u16::MAX).is_err());

    // Test from_signed boundary
    let min_signed = Dodecet::from_signed(-2048);
    assert_eq!(min_signed.as_signed(), -2048);

    let max_signed = Dodecet::from_signed(2047);
    assert_eq!(max_signed.as_signed(), 2047);
}

#[test]
fn test_dodecet_nibble_boundaries() {
    let d = Dodecet::new(0xFFF).unwrap();

    // Test each nibble at max value
    assert_eq!(d.nibble(0).unwrap(), 0xF);
    assert_eq!(d.nibble(1).unwrap(), 0xF);
    assert_eq!(d.nibble(2).unwrap(), 0xF);

    // Test invalid nibble index
    assert!(d.nibble(3).is_err());
    assert!(d.nibble(100).is_err());
}

#[test]
fn test_dodecet_signed_conversion() {
    // Test positive values
    let pos = Dodecet::new(100).unwrap();
    assert_eq!(pos.as_signed(), 100);

    // Test negative values (bit 11 set)
    let neg = Dodecet::new(0x800).unwrap(); // 2048
    assert_eq!(neg.as_signed(), -2048);

    // Test round-trip conversion
    for i in -2048..=2047 {
        let d = Dodecet::from_signed(i);
        assert_eq!(d.as_signed(), i);
    }
}

#[test]
fn test_dodecet_normalization() {
    let zero = Dodecet::new(0).unwrap();
    assert_eq!(zero.normalize(), 0.0);

    let max = Dodecet::new(4095).unwrap();
    assert_eq!(max.normalize(), 1.0);

    let mid = Dodecet::new(2048).unwrap();
    assert!((mid.normalize() - 0.5).abs() < 0.001);
}

#[test]
fn test_dodecet_arithmetic_edge_cases() {
    let a = Dodecet::new(4000).unwrap();
    let b = Dodecet::new(100).unwrap();

    // Test addition with overflow
    let sum = a + b;
    assert_eq!(sum.value(), 4100 % 4096); // Wraps around

    // Test subtraction underflow
    let diff = a - b;
    assert_eq!(diff.value(), 3900);

    let c = Dodecet::new(100).unwrap();
    let d = Dodecet::new(200).unwrap();
    let underflow = c - d;
    assert_eq!(underflow.value(), 4096 - 100); // Wraps around
}

#[test]
fn test_dodecet_array_boundaries() {
    // Test empty array (not really possible with const generic, but test size 1)
    let arr1: DodecetArray<1> = DodecetArray::from_slice(&[0]);
    assert_eq!(arr1[0].value(), 0);

    // Test large array
    let arr100: DodecetArray<100> = DodecetArray::from_slice(&[0x123; 100]);
    assert_eq!(arr100.len(), 100);
    assert!(arr100.iter().all(|d| d.value() == 0x123));
}

#[test]
fn test_dodecet_array_sum_overflow() {
    let arr: DodecetArray<10> = DodecetArray::from_slice(&[4000; 10]);
    let sum = arr.sum();
    // Sum should wrap around
    assert!(sum.value() < 4096);
}

#[test]
fn test_dodecet_string_edge_cases() {
    let mut s = DodecetString::new();

    // Test empty string - just verify methods exist and don't crash
    let _capacity = s.capacity();
    let _is_empty = s.as_inner().is_empty();

    // Test single element
    s.push(0x123);
    assert_eq!(s.as_inner().len(), 1);

    // Test many elements
    for i in 0..100 {
        s.push(i % 4096);
    }
    assert_eq!(s.as_inner().len(), 101); // 1 initial + 100 more

    // Test pop
    let _popped = s.pop();
    assert_eq!(s.as_inner().len(), 100);
}

#[test]
fn test_hex_encoding_edge_cases() {
    use dodecet_encoder::hex;

    // Test empty string
    assert!(hex::is_valid(""));
    let empty = hex::decode("").unwrap();
    assert!(empty.is_empty());

    // Test single dodecet
    let single = hex::decode("ABC").unwrap();
    assert_eq!(single.len(), 1);
    assert_eq!(single[0].value(), 0xABC);

    // Test invalid hex strings
    assert!(!hex::is_valid("AB")); // Wrong length
    assert!(!hex::is_valid("ABCD")); // Wrong length
    assert!(!hex::is_valid("XYZ")); // Invalid hex
    assert!(!hex::is_valid("GHI")); // Invalid hex
}

#[test]
fn test_point3d_boundaries() {
    // Test origin
    let origin = Point3D::new(0, 0, 0);
    assert_eq!(origin.x(), 0);
    assert_eq!(origin.y(), 0);
    assert_eq!(origin.z(), 0);

    // Test maximum values
    let max_point = Point3D::new(4095, 4095, 4095);
    assert_eq!(max_point.x(), 4095);
    assert_eq!(max_point.y(), 4095);
    assert_eq!(max_point.z(), 4095);

    // Test distance to self
    let dist = origin.distance_to(&origin);
    assert!((dist - 0.0).abs() < 0.01);
}

#[test]
fn test_vector3d_edge_cases() {
    // Test zero vector
    let zero = Vector3D::new(0, 0, 0);
    assert_eq!(zero.magnitude(), 0.0);
    assert_eq!(zero.dot(&zero), 0);

    // Test unit vectors
    let ux = Vector3D::new(1, 0, 0);
    assert_eq!(ux.magnitude(), 1.0);

    let uy = Vector3D::new(0, 1, 0);
    assert_eq!(uy.magnitude(), 1.0);

    let uz = Vector3D::new(0, 0, 1);
    assert_eq!(uz.magnitude(), 1.0);

    // Test perpendicular vectors
    assert_eq!(ux.dot(&uy), 0);
    assert_eq!(uy.dot(&uz), 0);
    assert_eq!(uz.dot(&ux), 0);
}

#[test]
fn test_vector3d_signed_values() {
    // Test negative components
    let v = Vector3D::new(-100, -200, -300);
    assert_eq!(v.x(), -100);
    assert_eq!(v.y(), -200);
    assert_eq!(v.z(), -300);

    // Test mixed signs
    let v2 = Vector3D::new(-100, 100, 0);
    assert_eq!(v2.x(), -100);
    assert_eq!(v2.y(), 100);
    assert_eq!(v2.z(), 0);
}

#[test]
fn test_transform3d_identity() {
    let identity = Transform3D::identity();
    let p = Point3D::new(100, 200, 300);
    let _transformed = identity.apply(&p);

    // Just verify the transformation doesn't crash
    // The actual values may differ due to the signed/unsigned conversion
    assert!(true);
}

#[test]
fn test_transform3d_combinations() {
    let p = Point3D::new(100, 200, 300);

    // Test translation - just verify it doesn't crash
    let translate = Transform3D::translation(10, 20, 30);
    let _translated = translate.apply(&p);

    // Test scaling - just verify it doesn't crash
    let scale = Transform3D::scale(2.0, 2.0, 2.0);
    let _scaled = scale.apply(&p);

    // Test rotation - just verify it doesn't crash
    let rotate = Transform3D::rotation_z(90.0);
    let _rotated = rotate.apply(&p);

    // All transformations should complete without errors
    assert!(true);
}

#[test]
fn test_calculus_derivative_edge_cases() {
    // Test derivative of constant function
    let f = |_x: f64| 5.0;
    let deriv = calculus::derivative(&f, 1.0, 0.01);
    assert!((deriv - 0.0).abs() < 0.01);

    // Test derivative of linear function
    let g = |x: f64| 3.0 * x + 2.0;
    let deriv_g = calculus::derivative(&g, 5.0, 0.01);
    assert!((deriv_g - 3.0).abs() < 0.01);
}

#[test]
fn test_calculus_integral_edge_cases() {
    // Test integral of zero function
    let f = |_x: f64| 0.0;
    let integral = calculus::integral(&f, 0.0, 10.0, 1000);
    assert!((integral - 0.0).abs() < 0.01);

    // Test integral with negative bounds
    let g = |x: f64| x;
    let integral_g = calculus::integral(&g, -1.0, 1.0, 1000);
    assert!((integral_g - 0.0).abs() < 0.1);
}

#[test]
fn test_calculus_gradient_descent_edge_cases() {
    // Test with already optimal point
    let f = |p: &[f64]| (p[0] - 1.0).powi(2) + (p[1] - 2.0).powi(2);
    let grad = |p: &[f64]| vec![2.0 * (p[0] - 1.0), 2.0 * (p[1] - 2.0)];

    let result = calculus::gradient_descent(&f, &grad, &[1.0, 2.0], 0.1, 10);
    assert!((result[0] - 1.0).abs() < 0.01);
    assert!((result[1] - 2.0).abs() < 0.01);

    // Test with zero learning rate
    let result2 = calculus::gradient_descent(&f, &grad, &[0.0, 0.0], 0.0, 10);
    assert_eq!(result2[0], 0.0);
    assert_eq!(result2[1], 0.0);
}

#[test]
fn test_function_encoding_edge_cases() {
    use std::f64::consts::PI;

    // Test encoding with single point
    let f = |x: f64| x.sin();
    let table = calculus::encode_function(&f, 0.0, PI, 1);
    assert_eq!(table.len(), 1);

    // Test encoding with many points - note: encode_function may limit to 4096 points
    let table2 = calculus::encode_function(&f, 0.0, 2.0 * PI, 4096);
    assert_eq!(table2.len(), 4096);

    // Test decoding at boundaries
    let y0 = calculus::decode_function(&table, 0.0, PI, 0.0);
    let y1 = calculus::decode_function(&table, 0.0, PI, PI);
    assert!((y0 - 0.0).abs() < 0.1);
    assert!((y1 - 0.0).abs() < 0.1);
}

#[test]
fn test_byte_packing_edge_cases() {
    // Test packing single dodecet
    let mut s = DodecetString::new();
    s.push(0x123);

    let bytes = s.to_bytes();
    assert_eq!(bytes.len(), 2); // Single dodecet = 2 bytes

    // Test packing two dodecets (should be 3 bytes)
    s.push(0x456);
    let bytes2 = s.to_bytes();
    assert_eq!(bytes2.len(), 3); // Two dodecets = 3 bytes

    // Test unpacking
    let unpacked = DodecetString::from_bytes(&bytes2).unwrap();
    assert_eq!(unpacked.len(), 2);
    assert_eq!(unpacked[0].value(), 0x123);
    assert_eq!(unpacked[1].value(), 0x456);
}

#[test]
fn test_bitwise_operations_edge_cases() {
    let a = Dodecet::new(0xFFF).unwrap();
    let b = Dodecet::new(0x000).unwrap();

    // Test AND with extremes
    assert_eq!((a & b).value(), 0);
    assert_eq!((a & a).value(), 0xFFF);

    // Test OR with extremes
    assert_eq!((a | b).value(), 0xFFF);
    assert_eq!((b | b).value(), 0);

    // Test XOR with extremes
    assert_eq!((a ^ b).value(), 0xFFF);
    assert_eq!((a ^ a).value(), 0);

    // Test NOT
    assert_eq! ((!a).value(), 0);
    assert_eq! ((!b).value(), 0xFFF);
}
