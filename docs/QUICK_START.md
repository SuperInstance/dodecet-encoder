# Dodecet Encoder - Quick Start Guide

**Version:** 1.0.0
**Status:** Production Ready

---

## Installation

### Rust (Crates.io)

```toml
[dependencies]
dodecet-encoder = "1.0.0"
```

### JavaScript/TypeScript (NPM)

```bash
npm install dodecet-encoder
```

---

## Quick Start

### Rust

```rust
use dodecet_encoder::Dodecet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a dodecet from hex
    let d = Dodecet::from_hex(0xABC)?;

    // Access nibbles
    println!("High nibble: 0x{:X}", d.nibble(2)?);
    println!("Mid nibble:  0x{:X}", d.nibble(1)?);
    println!("Low nibble:  0x{:X}", d.nibble(0)?);

    // Arithmetic operations
    let sum = d.add(Dodecet::from_hex(0x123)?)?;
    println!("Sum: 0x{:X}", sum.value());

    Ok(())
}
```

### JavaScript/TypeScript

```typescript
import { WasmDodecet, DodecetUtils } from 'dodecet-encoder';

// Create from hex string
const d = new WasmDodecet(0xABC);

// Access nibbles
console.log(`High nibble: 0x${d.nibble(2).toString(16)}`);
console.log(`Mid nibble:  0x${d.nibble(1).toString(16)}`);
console.log(`Low nibble:  0x${d.nibble(0).toString(16)}`);

// Encode/decode
const hex = DodecetUtils.encodeToHex(0xABC);
console.log(`Hex: ${hex}`); // "ABC"
```

---

## Common Operations

### Creating Dodecets

```rust
// From hex
let d1 = Dodecet::from_hex(0x123)?;

// From nibbles
let d2 = Dodecet::from_nibbles(0x1, 0x2, 0x3)?;

// From binary
let bits = [false, false, false, true, false, false, true, false,
            false, false, true, true];
let d3 = Dodecet::from_binary(&bits);
```

### Arithmetic

```rust
let a = Dodecet::from_hex(0x100)?;
let b = Dodecet::from_hex(0x0FF)?;

// Addition
let sum = a.add(b)?;

// Subtraction
let diff = a.sub(b)?;

// Multiplication
let prod = a.mul(b)?;

// Division
let quot = a.div(b)?;
```

### Bit Operations

```rust
let d = Dodecet::from_hex(0x123)?;

// Left shift
let shifted = d.shl(4)?;

// Right shift
let shifted = d.shr(4);

// Get bits
let bits = d.bits();
```

### Arrays

```rust
use dodecet_encoder::DodecetArray;

// Create from slice
let arr = DodecetArray::<3>::from_slice(&[0x123, 0x456, 0x789])?;

// Access elements
let first = arr.get(0);

// Sum all
let total = arr.sum();

// SIMD operations (if available)
let result = simd::simd_add(&arr, &arr)?;
```

### Geometric Operations

```rust
use dodecet_encoder::{Point3D, Vector3D};

// Create point
let p1 = Point3D::new(100, 200, 300)?;
let p2 = Point3D::new(150, 250, 350)?;

// Calculate distance
let dist = p1.distance_to(&p2);

// Create vector
let v1 = Vector3D::new(1, 2, 3)?;
let v2 = Vector3D::new(4, 5, 6)?;

// Dot product
let dot = v1.dot(&v2);

// Cross product
let cross = v1.cross(&v2)?;
```

### Hex Encoding/Decoding

```rust
use dodecet_encoder::hex;

// Encode single value
let hex = hex::encode_to_hex(0xABC)?;
assert_eq!(hex, "ABC");

// Decode single value
let val = hex::decode_from_hex("ABC")?;
assert_eq!(val, 0xABC);

// Encode array
let arr = vec![0x123, 0x456, 0x789];
let hex = hex::encode_array_to_hex(&arr)?;
assert_eq!(hex, "123456789");

// Decode array
let decoded = hex::decode_array_from_hex("123456789")?;
assert_eq!(decoded, arr);
```

---

## Examples

### Example 1: Color Encoding

```rust
use dodecet_encoder::Dodecet;

fn encode_rgb(r: u8, g: u8, b: u8) -> Result<Dodecet, Box<dyn std::error::Error>> {
    // Each channel gets 4 bits (0-15)
    let r4 = (r >> 4) & 0xF;
    let g4 = (g >> 4) & 0xF;
    let b4 = (b >> 4) & 0xF;

    Dodecet::from_nibbles(r4, g4, b4)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let color = encode_rgb(255, 128, 64)?;
    println!("Color encoded: 0x{:X}", color.value());

    Ok(())
}
```

### Example 2: 3D Point Cloud

```rust
use dodecet_encoder::{DodecetArray, Point3D};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create point cloud
    let points = vec![
        Point3D::new(100, 200, 300)?,
        Point3D::new(150, 250, 350)?,
        Point3D::new(200, 300, 400)?,
    ];

    // Calculate distances from origin
    let origin = Point3D::new(0, 0, 0)?;
    for (i, p) in points.iter().enumerate() {
        let dist = p.distance_to(&origin);
        println!("Point {}: Distance = {}", i, dist);
    }

    Ok(())
}
```

### Example 3: Geometric Transformations

```rust
use dodecet_encoder::{Point3D, Transform3D};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let point = Point3D::new(100, 200, 300)?;

    // Translate
    let translation = Transform3D::translate(50.0, 50.0, 50.0);
    let translated = translation.apply(&point)?;

    // Rotate
    let rotation = Transform3D::rotate_y(std::f64::consts::PI / 4.0);
    let rotated = rotation.apply(&point)?;

    // Scale
    let scale = Transform3D::scale(2.0, 2.0, 2.0);
    let scaled = scale.apply(&point)?;

    Ok(())
}
```

---

## Performance Tips

1. **Use SIMD for large arrays**: When working with arrays of 16+ elements, use SIMD operations for better performance.

2. **Reuse dodecets**: Dodecets are small (2 bytes), so reuse them when possible.

3. **Batch operations**: Process multiple dodecets in batches to take advantage of CPU caching.

4. **WASM for web**: Use the WASM package for browser applications to get near-native performance.

---

## Next Steps

- Read the [API Reference](./API_REFERENCE.md) for detailed API documentation
- Check out [Examples](./EXAMPLES.md) for more code examples
- Visit [GitHub](https://github.com/SuperInstance/dodecet-encoder) for source code

---

## Support

For issues or questions:
- **GitHub:** https://github.com/SuperInstance/dodecet-encoder
- **Issues:** https://github.com/SuperInstance/dodecet-encoder/issues
