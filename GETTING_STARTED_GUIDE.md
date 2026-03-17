# Getting Started with Dodecet Encoder

**A beginner-friendly guide to using the dodecet-encoder library**

---

## What is a Dodecet?

A **dodecet** is a 12-bit number (4,096 possible values from 0 to 4095). Think of it as a compact way to store numbers when you do not need full precision.

**Why 12 bits?**
- 12 bits = 3 hexadecimal digits (0x000 to 0xFFF)
- Perfect for 3D coordinates (x, y, z each fit in one dodecet)
- Saves memory: 6 bytes vs 24 bytes for 3D points (75% savings)
- Easy to read in hex editors

---

## Quick Start (5 Minutes)

### Step 1: Add to Your Project

Add to Cargo.toml:

```toml
[dependencies]
dodecet-encoder = "1.1.0"
```

Then build:

```bash
cargo build
```

### Step 2: Create Your First Dodecet

```rust
use dodecet_encoder::Dodecet;

fn main() {
    // Create a dodecet from a hex value
    let d = Dodecet::from_hex(0xABC);
    
    println!("Hex: {}", d.to_hex_string());  // "ABC"
    println!("Decimal: {}", d.value());       // 2748
    
    // Convert to normalized value (0.0 to 1.0)
    let normalized = d.normalize();
    println!("Normalized: {}", normalized);   // 0.6708984...
}
```

**Congratulations!** You have created your first dodecet.

---

## Understanding Dodecets

### The 12-Bit Structure

```
+-------------------------------------------+
|           DODECET (12 bits)               |
+-------------------------------------------+
|  Nibble 2  |  Nibble 1  |  Nibble 0      |
|  (4 bits)  |  (4 bits)  |  (4 bits)      |
+------------+------------+----------------+
|     0xA    |     0xB    |      0xC       |
+------------+------------+----------------+
|           Stored in u16 (2 bytes)        |
+-------------------------------------------+
```

Each dodecet has 3 nibbles (4-bit groups):
- **Nibble 0**: Bits 3-0 (rightmost)
- **Nibble 1**: Bits 7-4 (middle)
- **Nibble 2**: Bits 11-8 (leftmost)

---

## Common Operations

### Creating Dodecets

```rust
use dodecet_encoder::Dodecet;

// From hex value
let d1 = Dodecet::from_hex(0xABC);

// From decimal (checked, returns Result)
let d2 = Dodecet::new(1000).unwrap();

// From hex string
let d3 = Dodecet::from_hex_str("123").unwrap();

// Maximum and minimum
let max = Dodecet::max_value();  // 0xFFF = 4095
let min = Dodecet::min_value();  // 0x000 = 0
```

### Conversions

```rust
let d = Dodecet::from_hex(0x800);

// To different formats
let hex_str = d.to_hex_string();    // "800"
let binary_str = d.to_binary_string(); // "100000000000"
let decimal = d.value();             // 2048

// To normalized (0.0 to 1.0)
let normalized = d.normalize();      // 0.5

// To signed (-2048 to 2047)
let signed = d.as_signed();          // -2048 (two complement)
```

---

## Working with 3D Points

### Creating Points

```rust
use dodecet_encoder::geometric::Point3D;

// Create a 3D point (x, y, z)
let point = Point3D::new(100, 200, 300);

// Calculate distance
let other = Point3D::new(300, 400, 0);
let distance = point.distance_to(&other);
println!("Distance: {}", distance);  // 500.0
```

### Memory Savings

```rust
// Traditional approach (3 x f64)
// Size: 24 bytes

// Dodecet approach (3 x Dodecet in u16)
let point_dodecet = Point3D::new(100, 200, 300);
// Size: 6 bytes (75% smaller!)
```

---

## Next Steps

1. **Read the examples**: cargo run --example basic_usage
2. **Explore tutorials**: Check the tutorials/ directory
3. **Review limitations**: Read DISCLAIMERS.md
4. **Try the comprehensive example**: cargo run --example comprehensive_integration

---

## Resources

- **API Documentation**: https://docs.rs/dodecet-encoder
- **Examples**: examples/ directory
- **Tutorials**: tutorials/ directory
- **GitHub**: https://github.com/SuperInstance/dodecet-encoder

---

**Happy coding!**
