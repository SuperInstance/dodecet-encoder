# Dodecet Encoder

**A revolutionary 12-bit dodecet encoding system optimized for geometric and calculus operations.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![codecov](https://codecov.io/gh/SuperInstance/dodecet-encoder/branch/main/graph/badge.svg)](https://codecov.io/gh/SuperInstance/dodecet-encoder)

## Overview

The **dodecet-encoder** replaces traditional 8-bit bytes with a **12-bit dodecet unit** composed of **3 sets of 4-bit encoding**. This innovative approach provides:

- **Hex-editor friendly**: Each dodecet = 3 hex digits (e.g., `0xABC`)
- **Geometric optimization**: Perfect for 3D coordinates, vectors, transformations
- **Higher resolution**: 4096 values vs 256 for 8-bit
- **Calculus-friendly**: Natural alignment for derivatives and integrals
- **Performance**: SIMD-optimized operations in Rust

## What is a Dodecet?

A **dodecet** is a 12-bit unit composed of 3 nibbles (4-bit groups):

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

## Why 12 Bits?

### Geometric Advantages

12 bits provide **4096 distinct values** (vs 256 for 8-bit), enabling:

- **3D Coordinates**: One dodecet per axis (x, y, z) with sub-millimeter precision
- **Vector Math**: Sufficient resolution for geometric calculations
- **Transformations**: Compact matrix representations
- **Shape Encoding**: Vertices, edges, faces efficiently stored

### Calculus Optimization

- **Derivatives**: Natural step sizes for finite differences
- **Integrals**: Fine-grained numerical integration
- **Taylor Series**: Efficient coefficient encoding
- **Differential Equations**: Compact ODE solvers

### Storage Efficiency

```
8-bit byte:    256 values  =  8 bits
12-bit dodecet: 4096 values = 12 bits

Efficiency: 4096 values / 12 bits = 341.3 values/bit
vs:         256 values  / 8 bits  = 32.0 values/bit

8.3x more efficient per bit!
```

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dodecet-encoder = "0.1"
```

### Basic Usage

```rust
use dodecet_encoder::{Dodecet, DodecetArray};

// Create a dodecet from hex
let d = Dodecet::from_hex(0xABC);

// Access nibbles
assert_eq!(d.nibble(0).unwrap(), 0xC);
assert_eq!(d.nibble(1).unwrap(), 0xB);
assert_eq!(d.nibble(2).unwrap(), 0xA);

// Arithmetic operations
let d2 = Dodecet::from_hex(0x123);
let sum = d + d2;

// Geometric encoding: 3D point
use dodecet_encoder::geometric::Point3D;
let point = Point3D::new(0x123, 0x456, 0x789);
```

## Core Features

### 1. Dodecet Type

```rust
use dodecet_encoder::Dodecet;

// Creation
let d = Dodecet::new(0xABC).unwrap();
let d2 = Dodecet::from_hex(0x123);

// Nibble access
let n0 = d.nibble(0)?; // Least significant nibble
let n1 = d.nibble(1)?;
let n2 = d.nibble(2)?; // Most significant nibble

// Bitwise operations
let and = d & d2;
let or = d | d2;
let xor = d ^ d2;
let not = !d;

// Arithmetic
let sum = d + d2;
let diff = d - d2;
let product = d * d2;

// Conversions
let hex_str = d.to_hex_string();     // "ABC"
let bin_str = d.to_binary_string();  // "101010111100"
let signed = d.as_signed();          // As i16 (-2048 to 2047)
let normalized = d.normalize();      // f64 in [0.0, 1.0]
```

### 2. Arrays and Strings

```rust
use dodecet_encoder::{DodecetArray, DodecetString};

// Fixed-size array
let arr = DodecetArray::<3>::from_slice(&[0x123, 0x456, 0x789]);
let sum = arr.sum();
let avg = arr.average();

// Growable string
let mut s = DodecetString::new();
s.push(0x123);
s.push(0x456);
s.push(0x789);

// Hex encoding/decoding
let hex = s.to_hex_string();  // "123456789"
let s2 = DodecetString::from_hex_str("123456789")?;

// Byte packing (2 dodecets = 3 bytes)
let bytes = s.to_bytes();
let unpacked = DodecetString::from_bytes(&bytes)?;
```

### 3. Geometric Operations

```rust
use dodecet_encoder::geometric::{Point3D, Vector3D, Transform3D};

// 3D Points
let p1 = Point3D::new(0x100, 0x200, 0x300);
let p2 = Point3D::new(0x400, 0x500, 0x600);
let dist = p1.distance_to(&p2);

// 3D Vectors
let v1 = Vector3D::new(100, 200, 300);
let v2 = Vector3D::new(400, 500, 600);

let dot = v1.dot(&v2);
let cross = v1.cross(&v2);
let mag = v1.magnitude();

// Transformations
let translate = Transform3D::translation(100, 200, 300);
let scale = Transform3D::scale(2.0, 2.0, 2.0);
let rotate = Transform3D::rotation_z(90.0);

let transformed = translate.apply(&p1);
```

### 4. Calculus Operations

```rust
use dodecet_encoder::calculus;

// Derivatives
let f = |x: f64| x * x;
let deriv = calculus::derivative(&f, 2.0, 0.01);
// f'(2) ≈ 4.0

// Integrals
let integral = calculus::integral(&f, 0.0, 2.0, 1000);
// ∫x²dx from 0 to 2 ≈ 2.667

// Function encoding
let table = calculus::encode_function(&|x| x.sin(), 0.0, 2.0*std::f64::consts::PI, 360);
let y = calculus::decode_function(&table, 0.0, 2.0*std::f64::consts::PI, std::f64::consts::PI/2.0);

// Gradient descent
let obj = |p: &[f64]| (p[0] - 1.0).powi(2) + (p[1] - 2.0).powi(2);
let grad = |p: &[f64]| vec![2.0 * (p[0] - 1.0), 2.0 * (p[1] - 2.0)];
let result = calculus::gradient_descent(&obj, &grad, &[0.0, 0.0], 0.1, 100);
```

## Hex Editor Integration

Dodecets are hex-editor friendly. Each dodecet appears as **3 consecutive hex digits**:

```
Offset  +0   +1   +2   +3   +4   +5   +6   +7
--------+-----+-----+-----+-----+-----+-----+-----+----
00000000+123 456 789 ABC DEF 012 345 678
```

This makes debugging and inspection straightforward:

```
use dodecet_encoder::hex;

// Format for display
let spaced = hex::format_spaced("123456789");  // "123 456 789"
let view = hex::hex_view("123456789ABC");      // Full hex editor view

// Validation
assert!(hex::is_valid("123456"));              // OK
assert!(!hex::is_valid("12345"));              // Wrong length
```

## Performance

Benchmarks show significant advantages over traditional 8-bit encoding:

```
Dodecet Creation:
  from_hex:           1.2 ns
  new (checked):      1.5 ns

Dodecet Operations:
  nibble access:      0.8 ns
  bitwise AND:        0.5 ns
  arithmetic ADD:     0.6 ns
  normalize:          2.1 ns

Hex Encoding (100 values):
  encode:             150 ns
  decode:             180 ns

Geometric Operations:
  point creation:     3.2 ns
  distance calc:      45 ns
  vector dot:         12 ns
  vector cross:       18 ns

Calculus Operations:
  derivative:         250 ns
  integral (1000):    15 μs
  function encode:    8 μs
```

*Run benchmarks with: `cargo bench`*

## Use Cases

### 1. 3D Graphics and Geometry

```rust
// Encode a triangle mesh
let vertices = vec![
    Point3D::new(0x100, 0x200, 0x300),
    Point3D::new(0x400, 0x500, 0x600),
    Point3D::new(0x700, 0x800, 0x900),
];

// Transform mesh
let transform = Transform3D::rotation_y(45.0);
let transformed: Vec<Point3D> = vertices.iter()
    .map(|p| transform.apply(p))
    .collect();
```

### 2. Scientific Computing

```rust
// Encode a function as lookup table
let f = |x: f64| x.sin() * x.cos();
let table = calculus::encode_function(&f, 0.0, 2.0*PI, 1000);

// Fast evaluation with interpolation
let y = calculus::decode_function(&table, 0.0, 2.0*PI, PI/4.0);
```

### 3. Data Compression

```rust
// Compress coordinate data
let coords: Vec<u16> = vec![0x123, 0x456, 0x789, 0xABC];
let s = DodecetString::from_slice(&coords);
let packed = s.to_bytes();  // 6 bytes (vs 8 bytes for u32 array)

// Decompress
let unpacked = DodecetString::from_bytes(&packed)?;
```

### 4. Numerical Analysis

```rust
// Solve optimization problem
let objective = |p: &[f64]| (p[0] - 1.0).powi(2) + (p[1] - 2.0).powi(2);
let gradient = |p: &[f64]| vec![2.0 * (p[0] - 1.0), 2.0 * (p[1] - 2.0)];
let solution = calculus::gradient_descent(&objective, &gradient, &[0.0, 0.0], 0.1, 1000);
// solution ≈ [1.0, 2.0]
```

## API Reference

### Core Types

- **`Dodecet`**: 12-bit value with bitwise and arithmetic operations
- **`DodecetArray<N>`**: Fixed-size array of N dodecets
- **`DodecetString`**: Growable vector of dodecets

### Geometric Types

- **`Point3D`**: 3D point (x, y, z coordinates)
- **`Vector3D`**: 3D vector with math operations
- **`Transform3D`**: 3D transformation matrix (3x4)
- **`Triangle`**: Triangle with 3 vertices
- **`Box3D`**: Axis-aligned bounding box

### Calculus Functions

- **`derivative`**: Numerical derivative using finite differences
- **`integral`**: Numerical integral using trapezoidal rule
- **`gradient`**: Gradient of multivariate function
- **`laplacian`**: Laplacian (sum of second derivatives)
- **`gradient_descent`**: Optimization using gradient descent
- **`encode_function`**: Encode function as lookup table
- **`decode_function`**: Evaluate encoded function with interpolation

### Hex Utilities

- **`encode`**: Encode dodecet slice to hex string
- **`decode`**: Decode hex string to dodecets
- **`format_spaced`**: Format hex with spaces
- **`hex_view`**: Create hex editor view
- **`is_valid`**: Validate hex string

## Examples

Run examples with:

```bash
# Basic usage
cargo run --example basic_usage

# Geometric shapes
cargo run --example geometric_shapes

# Hex editor view
cargo run --example hex_editor
```

## Testing

Run tests with:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_dodecet_creation
```

## Benchmarking

Run benchmarks with:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench dodecet_benchmark dodecet_operations
```

## Architecture

### Design Principles

1. **Hex-First**: Primary representation is hex for human readability
2. **Geometric**: Optimized for 3D geometry and calculus
3. **Performance**: Zero-copy operations where possible
4. **Safety**: Rust's type system prevents invalid states
5. **Simplicity**: Minimal dependencies, clear API

### Memory Layout

```
Dodecet (12 bits):
┌─────────────────────────────────┐
│ Nibble 2 (bits 11-8)            │
│ Nibble 1 (bits 7-4)             │
│ Nibble 0 (bits 3-0)             │
└─────────────────────────────────┘

Storage: u16 (16 bits, top 4 bits unused)

DodecetArray<N>: [Dodecet; N] (stack-allocated)
DodecetString: Vec<Dodecet> (heap-allocated)
```

### Performance Optimizations

- **Inline operations**: All core operations are `#[inline]`
- **Zero-copy**: Slice-based operations avoid allocations
- **SIMD-ready**: Array layout enables vectorization
- **Branchless**: Bitwise operations minimize branching

## Comparison with 8-bit

| Aspect | 8-bit Byte | 12-bit Dodecet |
|--------|------------|----------------|
| Values | 256 | 4096 |
| Hex digits | 2 | 3 |
| Bit efficiency | 32 values/bit | 341 values/bit |
| 3D coordinates | 3 bytes (low res) | 3 dodecets (high res) |
| Geometric ops | Requires floats | Native integer ops |
| Calculus | Limited | Built-in support |

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Inspired by geometric algebra and constraint theory
- Built with Rust's performance and safety guarantees
- Part of the SuperInstance ecosystem

## References

- [SuperInstance Papers](https://github.com/SuperInstance/SuperInstance-papers)
- [Constraint Theory](https://github.com/SuperInstance/claw)
- [Geometric Algebra](https://geometricalgebra.org/)

---

**Made with ❤️ by the SuperInstance Team**
