# Integration Guide for Dodecet Encoder

**Integrate dodecet encoding into your applications**

## Table of Contents

1. [Introduction](#introduction)
2. [Rust Integration](#rust-integration)
3. [WebAssembly Integration](#webassembly-integration)
4. [Python Integration](#python-integration)
5. [C/C++ Integration](#cc-integration)
6. [JavaScript Integration](#javascript-integration)
7. [Best Practices](#best-practices)

---

## Introduction

The dodecet-encoder library can be integrated into various environments:

- **Rust**: Native integration, full feature support
- **WebAssembly**: Browser and Node.js support
- **Python**: Via Python bindings (coming soon)
- **C/C++**: Via FFI (Foreign Function Interface)
- **JavaScript**: Direct JavaScript API

---

## Rust Integration

### Cargo.toml

```toml
[dependencies]
dodecet-encoder = "1.0"
```

### Basic Usage

```rust
use dodecet_encoder::{Dodecet, Point3D, DodecetString};

fn main() {
    // Create dodecet
    let d = Dodecet::from_hex(0xABC);
    println!("Dodecet: {}", d.to_hex_string());

    // Create point
    let point = Point3D::new(0x100, 0x200, 0x300);
    println!("Point: {}", point.to_hex_string());

    // Create string
    let mut s = DodecetString::new();
    s.push(0x123);
    s.push(0x456);
    println!("String: {}", s.to_hex_string());
}
```

### Advanced Usage

```rust
use dodecet_encoder::{
    Dodecet, Point3D, Vector3D,
    geometric::Transform3D,
    calculus
};

// Geometric operations
let point = Point3D::new(0x100, 0x200, 0x300);
let transform = Transform3D::rotation_y(45.0);
let rotated = transform.apply(&point);

// Calculus operations
let f = |x: f64| x * x;
let deriv = calculus::derivative(&f, 2.0, 0.01);

// Batch operations
let points: Vec<Point3D> = (0..1000)
    .map(|i| Point3D::new(i, i*2, i*3))
    .collect();
```

### Feature Flags

```toml
[dependencies]
dodecet-encoder = { version = "1.0", features = ["full"] }

# Available features:
# - "default": Core functionality
# - "geometric": Geometric operations
# - "calculus": Calculus operations
# - "full": All features
# - "wasm": WebAssembly support
```

---

## WebAssembly Integration

### Setup

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for WebAssembly
cd dodecet-encoder
wasm-pack build --target web
```

### JavaScript Usage

```javascript
// Import the WASM module
import init, { Dodecet, Point3D } from './dodecet_encoder.js';

async function run() {
    // Initialize WASM
    await init();

    // Create dodecet
    const d = new Dodecet(0xABC);
    console.log('Hex:', d.toHexString());

    // Create point
    const point = new Point3D(0x100, 0x200, 0x300);
    console.log('Point:', point.toHexString());

    // Calculate distance
    const other = new Point3D(0x200, 0x300, 0x400);
    const distance = point.distanceTo(other);
    console.log('Distance:', distance);
}

run();
```

### HTML Integration

```html
<!DOCTYPE html>
<html>
<head>
    <title>Dodecet Demo</title>
</head>
<body>
    <script type="module">
        import init, { Dodecet, Point3D } from './dodecet_encoder.js';

        async function main() {
            await init();

            const d = new Dodecet(0xABC);
            document.getElementById('output').textContent = d.toHexString();
        }

        main();
    </script>
    <div id="output"></div>
</body>
</html>
```

### WebGL Integration

```javascript
// Create WebGL buffer from dodecet points
import { DodecetWebGLBuffer } from './dodecet_encoder.js';

const gl = canvas.getContext('webgl');
const buffer = new DodecetWebGLBuffer(gl);

const points = [
    new Point3D(0x100, 0x200, 0x300),
    new Point3D(0x400, 0x500, 0x600),
    new Point3D(0x700, 0x800, 0x900),
];

buffer.fromPoints(points);
buffer.bind(positionLocation);
gl.drawArrays(gl.TRIANGLES, 0, 3);
```

---

## Python Integration

### Installation (Coming Soon)

```bash
pip install dodecet-encoder
```

### Python Usage

```python
from dodecet_encoder import Dodecet, Point3D

# Create dodecet
d = Dodecet.from_hex(0xABC)
print(f"Hex: {d.to_hex_string()}")

# Create point
point = Point3D(0x100, 0x200, 0x300)
print(f"Point: {point.to_hex_string()}")

# Calculate distance
other = Point3D(0x200, 0x300, 0x400)
distance = point.distance_to(other)
print(f"Distance: {distance}")
```

### NumPy Integration

```python
import numpy as np
from dodecet_encoder import DodecetString

# Convert NumPy array to dodecets
arr = np.array([0x123, 0x456, 0x789], dtype=np.uint16)
ds = DodecetString.from_numpy(arr)

# Convert back
arr2 = ds.to_numpy()
```

---

## C/C++ Integration

### FFI Wrapper

Create a C wrapper (`dodecet.h`):

```c
#ifndef DODECET_H
#define DODECET_H

#ifdef __cplusplus
extern "C" {
#endif

// Opaque pointer to dodecet
typedef struct Dodecet Dodecet;

// Create/destroy
Dodecet* dodecet_from_hex(uint16_t value);
void dodecet_destroy(Dodecet* d);

// Operations
uint16_t dodecet_value(const Dodecet* d);
uint8_t dodecet_nibble(const Dodecet* d, uint8_t index);
char* dodecet_to_hex_string(const Dodecet* d);
void dodecet_string_free(char* s);

#ifdef __cplusplus
}
#endif

#endif // DODECET_H
```

### Implementation (`dodecet.c`)

```c
#include "dodecet.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// Internal structure matches Rust layout
struct Dodecet {
    uint16_t value;
};

Dodecet* dodecet_from_hex(uint16_t value) {
    if (value > 0xFFF) return NULL;
    Dodecet* d = malloc(sizeof(Dodecet));
    d->value = value;
    return d;
}

void dodecet_destroy(Dodecet* d) {
    free(d);
}

uint16_t dodecet_value(const Dodecet* d) {
    return d->value;
}

uint8_t dodecet_nibble(const Dodecet* d, uint8_t index) {
    if (index > 2) return 0;
    return (d->value >> (index * 4)) & 0xF;
}

char* dodecet_to_hex_string(const Dodecet* d) {
    char* s = malloc(4);
    snprintf(s, 4, "%03X", d->value);
    return s;
}

void dodecet_string_free(char* s) {
    free(s);
}
```

### C++ Usage

```cpp
#include "dodecet.h"
#include <iostream>

int main() {
    Dodecet* d = dodecet_from_hex(0xABC);
    if (d) {
        char* hex = dodecet_to_hex_string(d);
        std::cout << "Hex: " << hex << std::endl;
        dodecet_string_free(hex);
        dodecet_destroy(d);
    }
    return 0;
}
```

---

## JavaScript Integration

### Direct JavaScript API

```javascript
// Pure JavaScript implementation
class Dodecet {
    constructor(value) {
        if (value > 0xFFF) throw new Error('Value exceeds 12 bits');
        this.value = value & 0xFFF;
    }

    nibble(index) {
        if (index > 2) return null;
        return (this.value >> (index * 4)) & 0xF;
    }

    toHexString() {
        return this.value.toString(16).toUpperCase().padStart(3, '0');
    }
}

// Usage
const d = new Dodecet(0xABC);
console.log(d.toHexString()); // "ABC"
```

### TypeScript Definitions

```typescript
// dodecet.d.ts
export class Dodecet {
    constructor(value: number);
    nibble(index: number): number | null;
    toHexString(): string;
    toBinaryString(): string;
    normalize(): number;
}

export class Point3D {
    constructor(x: number, y: number, z: number);
    x(): number;
    y(): number;
    z(): number;
    distanceTo(other: Point3D): number;
    toHexString(): string;
}
```

---

## Best Practices

### 1. Error Handling

```rust
// Good: Handle errors gracefully
let d = match Dodecet::new(value) {
    Ok(d) => d,
    Err(e) => {
        eprintln!("Invalid dodecet value: {}", e);
        return Err(e);
    }
};

// Bad: Panic on invalid input
let d = Dodecet::new(value).unwrap();
```

### 2. Performance

```rust
// Good: Use references to avoid copies
fn process_point(point: &Point3D) -> f64 {
    point.x() as f64 + point.y() as f64
}

// Bad: Unnecessary copying
fn process_point(point: Point3D) -> f64 {
    point.x() as f64 + point.y() as f64
}
```

### 3. Memory Efficiency

```rust
// Good: Pre-allocate capacity
let mut points = Vec::with_capacity(1000);
for i in 0..1000 {
    points.push(Point3D::new(i, i*2, i*3));
}

// Bad: Reallocation during growth
let mut points = Vec::new();
for i in 0..1000 {
    points.push(Point3D::new(i, i*2, i*3));
}
```

### 4. Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dodecet_creation() {
        let d = Dodecet::from_hex(0xABC);
        assert_eq!(d.value(), 0xABC);
    }

    #[test]
    fn test_invalid_value() {
        assert!(Dodecet::new(0x1000).is_err());
    }
}
```

---

## Platform-Specific Considerations

### WebAssembly

- **Size**: WASM bundle is ~50KB gzipped
- **Performance**: Near-native speed (2-3x slower than native)
- **Browser Support**: All modern browsers
- **Node.js**: Full support

### Embedded Systems

- **No Std**: Disable std for embedded use
- **Memory**: Minimal memory footprint (~10KB)
- **Performance**: Optimized for speed

### High Performance Computing

- **SIMD**: Ready for SIMD optimization
- **Parallel**: Embarrassingly parallel operations
- **GPU**: CUDA integration coming

---

## Troubleshooting

### Common Issues

**Issue: "Value exceeds 12-bit capacity"**
```rust
// Ensure values are in range [0, 4095]
let d = Dodecet::new(value % 0x1000)?;
```

**Issue: "Linker error on Windows"**
```bash
# Install MSVC build tools
# https://rust-lang.github.io/rustup/installation/windows-msvc.html
```

**Issue: "WASM initialization failed"**
```javascript
// Ensure WASM is loaded before use
await init();
```

---

## Next Steps

- [Tutorial 5: Advanced Usage](05_ADVANCED_USAGE.md) - Performance optimization
- [Examples](../examples/) - Real-world integration examples
- [API Reference](https://docs.rs/dodecet-encoder) - Complete API docs

---

**Found an issue?** [Report it here](https://github.com/SuperInstance/dodecet-encoder/issues)

**Need help?** [Ask in discussions](https://github.com/SuperInstance/dodecet-encoder/discussions)

---

**Last Updated:** 2026-03-16
**Tutorial:** 04 - Integration Guide
