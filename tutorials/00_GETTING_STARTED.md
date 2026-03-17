# Getting Started with Dodecet Encoder

**A comprehensive introduction to the 12-bit dodecet encoding system**

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Basic Concepts](#basic-concepts)
4. [Your First Dodecet](#your-first-dodecet)
5. [Common Operations](#common-operations)
6. [Next Steps](#next-steps)

---

## Introduction

The **dodecet-encoder** is a revolutionary encoding system that replaces traditional 8-bit bytes with 12-bit "dodecets". Each dodecet provides 4,096 discrete states (vs 256 for bytes), making it ideal for:

- **3D Geometry**: Perfect precision for coordinates
- **Calculus**: Natural step sizes for numerical methods
- **Memory Efficiency**: 50% smaller than floating-point
- **Deterministic Computing**: No floating-point drift

### Why Dodecet?

```
Traditional (8-bit):  256 states    = 8 bits
Dodecet (12-bit):     4096 states   = 12 bits

Efficiency gain: 16x more values with only 1.5x more bits!
```

---

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Add to Your Project

```bash
# New project
cargo new my_dodecet_project
cd my_dodecet_project

# Add dependency
cargo add dodecet-encoder
```

Or manually add to `Cargo.toml`:

```toml
[dependencies]
dodecet-encoder = "0.1"
```

### Verify Installation

Create `src/main.rs`:

```rust
use dodecet_encoder::Dodecet;

fn main() {
    let d = Dodecet::from_hex(0xABC);
    println!("My first dodecet: {}", d.to_hex_string());
}
```

Run it:

```bash
cargo run
```

Expected output:

```
My first dodecet: ABC
```

---

## Basic Concepts

### What is a Dodecet?

A **dodecet** is a 12-bit value composed of 3 nibbles (4-bit groups):

```
┌─────────────────────────────────────────┐
│           DODECET (12 bits)             │
├─────────────────────────────────────────┤
│  Nibble 2  │  Nibble 1  │  Nibble 0    │
│  (4 bits)  │  (4 bits)  │  (4 bits)    │
│  [11:8]    │  [7:4]     │  [3:0]       │
├─────────────────────────────────────────┤
│  Example:   0xA        0xB        0xC  │
│  Hex: 0xABC = 1010 1011 1100 (binary)  │
│  Decimal: 2748                             │
└─────────────────────────────────────────┘
```

### Value Range

- **Minimum**: `0x000` = 0
- **Maximum**: `0xFFF` = 4,095
- **Total States**: 4,096

### Hex Representation

Each dodecet = 3 hex digits:

```
0x123 → "123"
0xABC → "ABC"
0x000 → "000"
```

This makes dodecets **hex-editor friendly**!

---

## Your First Dodecet

### Creating a Dodecet

```rust
use dodecet_encoder::Dodecet;

fn main() {
    // Method 1: From hex value
    let d1 = Dodecet::from_hex(0xABC);

    // Method 2: From decimal value (checked)
    let d2 = Dodecet::new(2748); // Returns Result

    // Method 3: From hex string
    let d3 = Dodecet::from_hex_str("ABC");
}
```

### Accessing Nibbles

```rust
let d = Dodecet::from_hex(0xABC);

// Access individual nibbles
let n0 = d.nibble(0).unwrap(); // 0xC (least significant)
let n1 = d.nibble(1).unwrap(); // 0xB
let n2 = d.nibble(2).unwrap(); // 0xA (most significant)

println!("Nibbles: {} {} {}", n2, n1, n0);
// Output: Nibbles: 10 11 12
```

### Arithmetic Operations

```rust
let d1 = Dodecet::from_hex(0x100);
let d2 = Dodecet::from_hex(0x001);

// Addition
let sum = d1 + d2;
println!("Sum: {}", sum.to_hex_string()); // "101"

// Subtraction
let diff = d1 - d2;
println!("Difference: {}", diff.to_hex_string()); // "0FF"

// Multiplication
let product = d1 * d2;
println!("Product: {}", product.to_hex_string()); // "100"

// All operations wrap around at 4096
```

### Bitwise Operations

```rust
let d1 = Dodecet::from_hex(0xFF0); // 1111 1111 0000
let d2 = Dodecet::from_hex(0x0F0); // 0000 1111 0000

// AND
let and = d1 & d2;
println!("AND: {}", and.to_hex_string()); // "0F0"

// OR
let or = d1 | d2;
println!("OR: {}", or.to_hex_string()); // "FF0"

// XOR
let xor = d1 ^ d2;
println!("XOR: {}", xor.to_hex_string()); // "F00"

// NOT (only 12 bits)
let not = !d1;
println!("NOT: {}", not.to_hex_string()); // "00F"
```

---

## Common Operations

### Conversion

```rust
let d = Dodecet::from_hex(0xABC);

// To hex string
let hex = d.to_hex_string(); // "ABC"

// To binary string
let binary = d.to_binary_string(); // "101010111100"

// To decimal
let decimal = d.value(); // 2748

// To signed integer (-2048 to 2047)
let signed = d.as_signed(); // 2748 as i16

// To normalized float (0.0 to 1.0)
let normalized = d.normalize(); // ≈ 0.671
```

### Arrays of Dodecets

```rust
use dodecet_encoder::{Dodecet, DodecetArray};

// Fixed-size array
let arr = DodecetArray::<4>::from_slice(&[0x123, 0x456, 0x789, 0xABC]);

// Access elements
let first = arr.get(0).unwrap();
println!("First: {}", first.to_hex_string()); // "123"

// Iterator
for d in arr.iter() {
    println!("Dodecet: {}", d.to_hex_string());
}

// Aggregate operations
let sum = arr.sum();
let avg = arr.average();
let min = arr.min();
let max = arr.max();

println!("Sum: {}, Avg: {}, Min: {}, Max: {}", sum, avg, min, max);
```

### Strings of Dodecets

```rust
use dodecet_encoder::DodecetString;

// Create growable string
let mut s = DodecetString::new();
s.push(0x123);
s.push(0x456);
s.push(0x789);

// Access
println!("Length: {}", s.len()); // 3
println!("First: {}", s.get(0).unwrap().to_hex_string()); // "123"

// Convert to hex
let hex = s.to_hex_string(); // "123456789"

// Parse from hex
let s2 = DodecetString::from_hex_str("123456789").unwrap();

// Byte packing (2 dodecets = 3 bytes)
let bytes = s.to_bytes(); // [0x12, 0x34, 0x56, 0x78, 0x90]
let unpacked = DodecetString::from_bytes(&bytes).unwrap();
```

---

## Complete Example

Here's a complete working example:

```rust
use dodecet_encoder::{Dodecet, DodecetArray, Point3D};

fn main() {
    println!("=== My First Dodecet Program ===\n");

    // 1. Create some dodecets
    let d1 = Dodecet::from_hex(0xABC);
    let d2 = Dodecet::from_hex(0x123);

    println!("Dodecet 1: {}", d1.to_hex_string());
    println!("Dodecet 2: {}", d2.to_hex_string());

    // 2. Do some arithmetic
    let sum = d1 + d2;
    println!("Sum: {}", sum.to_hex_string());

    // 3. Access nibbles
    println!("Nibbles of {}: {}, {}, {}",
        d1.to_hex_string(),
        d1.nibble(2).unwrap(),
        d1.nibble(1).unwrap(),
        d1.nibble(0).unwrap()
    );

    // 4. Create a 3D point
    let point = Point3D::new(0x100, 0x200, 0x300);
    println!("\n3D Point: {:?}", point);

    // 5. Calculate distance
    let other = Point3D::new(0x400, 0x500, 0x600);
    let distance = point.distance_to(&other);
    println!("Distance: {:.2}", distance);

    // 6. Use arrays
    let arr = DodecetArray::<4>::from_slice(&[0x100, 0x200, 0x300, 0x400]);
    println!("\nArray sum: {}", arr.sum());
    println!("Array average: {}", arr.average());

    println!("\n=== Program Complete ===");
}
```

---

## Next Steps

Now that you understand the basics:

1. **[Basic Operations Tutorial](./01_BASIC_OPERATIONS.md)** - Learn more operations
2. **[Geometric Operations Tutorial](./02_GEOMETRIC_OPERATIONS.md)** - 3D geometry
3. **[Calculus Operations Tutorial](./03_CALCULUS_OPERATIONS.md)** - Numerical methods
4. **[Integration Tutorial](./04_INTEGRATION.md)** - Web and WASM
5. **[Advanced Usage](./05_ADVANCED_USAGE.md)** - Performance optimization

---

## Common Issues

### Issue: Value exceeds 12-bit capacity

```rust
// This will fail
let d = Dodecet::new(5000); // > 4095

// Solution: Wrap or mask
let d = Dodecet::new(5000 & 0xFFF).unwrap();
```

### Issue: Invalid hex string

```rust
// This will fail
let d = Dodecet::from_hex_str("XYZ"); // Invalid hex

// Solution: Validate first
use dodecet_encoder::hex;
if hex::is_valid("123") {
    let d = Dodecet::from_hex_str("123").unwrap();
}
```

---

## Resources

- [API Documentation](https://docs.rs/dodecet-encoder)
- [GitHub Repository](https://github.com/SuperInstance/dodecet-encoder)
- [Examples](../examples/)
- [Performance Benchmarks](../benches/)

---

**Ready to dive deeper? Continue to [Basic Operations](./01_BASIC_OPERATIONS.md)!**
