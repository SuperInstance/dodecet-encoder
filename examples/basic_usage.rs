//! Basic usage examples for dodecet-encoder
//!
//! Run with: cargo run --example basic_usage

use dodecet_encoder::{Dodecet, DodecetArray, DodecetString, hex};

fn main() {
    println!("=== Dodecet Encoder - Basic Usage ===\n");

    // Creating dodecets
    println!("1. Creating Dodecets:");
    let d1 = Dodecet::new(0xABC).unwrap();
    let d2 = Dodecet::from_hex(0x123);
    println!("   d1 = {}", d1);
    println!("   d2 = {}\n", d2);

    // Accessing nibbles
    println!("2. Accessing Nibbles:");
    println!("   {} nibble(0) = {}", d1, d1.nibble(0).unwrap());
    println!("   {} nibble(1) = {}", d1, d1.nibble(1).unwrap());
    println!("   {} nibble(2) = {}\n", d1, d1.nibble(2).unwrap());

    // Bitwise operations
    println!("3. Bitwise Operations:");
    let and = d1 & d2;
    let or = d1 | d2;
    let xor = d1 ^ d2;
    println!("   {} & {} = {}", d1, d2, and);
    println!("   {} | {} = {}", d1, d2, or);
    println!("   {} ^ {} = {}\n", d1, d2, xor);

    // Arithmetic operations
    println!("4. Arithmetic Operations:");
    let sum = d1 + d2;
    let diff = d1 - d2;
    let product = d1 * d2;
    println!("   {} + {} = {}", d1, d2, sum);
    println!("   {} - {} = {}", d1, d2, diff);
    println!("   {} * {} = {}\n", d1, d2, product);

    // Conversions
    println!("5. Conversions:");
    println!("   to_hex_string() = {}", d1.to_hex_string());
    println!("   to_binary_string() = {}", d1.to_binary_string());
    println!("   as_signed() = {}", d1.as_signed());
    println!("   normalize() = {}\n", d1.normalize());

    // Arrays
    println!("6. Dodecet Arrays:");
    let arr = DodecetArray::<3>::from_slice(&[0x123, 0x456, 0x789]);
    println!("   Array: {}", arr);
    println!("   Sum: {}", arr.clone().sum());
    println!("   Average: {}\n", arr.average());

    // Strings
    println!("7. Dodecet Strings:");
    let mut s = DodecetString::new();
    s.push(0x123);
    s.push(0x456);
    s.push(0x789);
    println!("   String: {}", s);
    println!("   Hex: {}", s.to_hex_string());
    println!("   Bytes: {:?}\n", s.to_bytes());

    // Hex encoding
    println!("8. Hex Utilities:");
    let hex_str = "123456789ABC";
    println!("   Original: {}", hex_str);
    println!("   Spaced: {}", hex::format_spaced(hex_str));
    println!("   Valid: {}", hex::is_valid(hex_str));
    println!("   Dodecet count: {}\n", hex::dodecet_count(hex_str));

    // Geometric example
    println!("9. Geometric Example:");
    use dodecet_encoder::geometric::Point3D;
    let point = Point3D::new(0x100, 0x200, 0x300);
    println!("   Point: ({}, {}, {})", point.x(), point.y(), point.z());
    let (nx, ny, nz) = point.normalized();
    println!("   Normalized: ({:.3}, {:.3}, {:.3})", nx, ny, nz);
    let (sx, sy, sz) = point.signed();
    println!("   Signed: ({}, {}, {})\n", sx, sy, sz);

    // Calculus example
    println!("10. Calculus Example:");
    use dodecet_encoder::calculus;
    let f = |x: f64| x * x;
    let deriv = calculus::derivative(&f, 2.0, 0.01);
    let integral = calculus::integral(&f, 0.0, 2.0, 1000);
    println!("    f(x) = x²");
    println!("    f'(2) = {:.3} (expected: 4.0)", deriv);
    println!("    ∫f(x)dx from 0 to 2 = {:.3} (expected: 2.667)\n", integral);

    println!("=== Examples Complete ===");
}
