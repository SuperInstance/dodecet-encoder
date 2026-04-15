# Dodecet Encoder

**A 12-bit encoding system for geometric and calculus operations.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

## What is a Dodecet?

A **dodecet** is a 12-bit unit (4,096 possible values) designed as an alternative building block for specific computational domains. The name comes from "dozen" (12) + "octet" (8 bits).

### 12-Bit Encoding Structure

```mermaid
graph LR
    subgraph "Dodecet Bit Layout (12 bits)"
        B11[Bit 11] --> B8[Bit 8]
        B7[Bit 7] --> B4[Bit 4]
        B3[Bit 3] --> B0[Bit 0]
    end

    subgraph "Nibble Organization"
        N2["Nibble 2<br/>(Bits 11-8)<br/>0-15"]
        N1["Nibble 1<br/>(Bits 7-4)<br/>0-15"]
        N0["Nibble 0<br/>(Bits 3-0)<br/>0-15"]
    end

    subgraph "Storage"
        U16["u16 Storage<br/>(16 bits)<br/>4 bits unused"]
    end

    B11 --> N2
    B7 --> N1
    B3 --> N0
    N2 --> U16
    N1 --> U16
    N0 --> U16

    style N2 fill:#e1f5fe
    style N1 fill:#fff3e0
    style N0 fill:#e8f5e9
```

### Example: Dodecet 0xABC

```
+---------------------------------------------------+
|                    DODECET (12 bits)              |
+---------------------------------------------------+
|  Bits 11-8  |  Bits 7-4   |  Bits 3-0   |  Storage |
|  Nibble 2   |  Nibble 1   |  Nibble 0   |  (u16)   |
+-------------+-------------+-------------+----------+
|    0xA      |    0xB      |    0xC      |   0x0ABC |
+-------------+-------------+-------------+----------+
|   1010      |   1011      |   1100      |          |
+-------------+-------------+-------------+----------+
```

### Key Property: Hex-Editor Friendly

Each dodecet maps to exactly **3 hexadecimal digits**:
- `0x000` to `0xFFF` (0 to 4095 in decimal)
- Easy visual inspection in hex editors
- Simple string encoding/decoding

```rust
use dodecet_encoder::Dodecet;

let d = Dodecet::from_hex(0xABC);
assert_eq!(d.to_hex_string(), "ABC");  // Exactly 3 hex chars
```

## When to Use Dodecets

### Good Use Cases

| Use Case | Why Dodecets Help |
|----------|-------------------|
| **3D Geometry** | One dodecet per axis (x,y,z) = compact coordinate storage |
| **Function Lookup Tables** | 4,096 values often sufficient for smooth interpolation |
| **Memory-Constrained Systems** | 2 bytes vs 8 bytes for f64 (75% savings) |
| **Network Transmission** | Compact hex representation, easy debugging |
| **Embedded/Edge Computing** | Predictable memory footprint |

### When to Avoid Dodecets

| Situation | Why Not Suitable |
|-----------|------------------|
| **High-Precision Calculations** | Only 12 bits vs f64's 53-bit mantissa |
| **Large Dynamic Range** | Range is 0-4095 vs f64's ~10^308 |
| **General-Purpose Computing** | Standard types (f32, f64, i32) have better tooling |
| **Scientific Simulations** | Accumulated quantization errors |
| **Financial Applications** | Precision requirements exceed 12 bits |

**Bottom Line**: Dodecets are a **domain-specific optimization**, not a general-purpose replacement for standard numeric types.

---

## Architecture Overview

```mermaid
graph TB
    subgraph "Core Types"
        D[Dodecet<br/>12-bit value]
        DA[DodecetArray&lt;N&gt;<br/>Fixed-size stack array]
        DS[DodecetString<br/>Heap-allocated vector]
    end

    subgraph "Geometric Types"
        P3[Point3D<br/>3 Dodecets]
        V3[Vector3D<br/>3 Dodecets]
        T3[Transform3D<br/>12 Dodecets]
    end

    subgraph "Operations"
        HEX[Hex Encoding<br/>3 chars per dodecet]
        BYTE[Byte Packing<br/>2 dodecets = 3 bytes]
        CALC[Calculus<br/>Derivatives, Integrals]
    end

    D --> DA
    D --> DS
    D --> P3
    D --> V3
    P3 --> T3
    V3 --> T3
    D --> HEX
    DS --> BYTE
    V3 --> CALC
```

### Coordinate Transformation Flow

```mermaid
graph LR
    A[u16 Value] --> B[Dodecet]
    B --> C[Normalize 0.0-1.0]
    C --> D[Signed -2048 to 2047]
    D --> E[Encode to Hex String]

    E --> F[Decode back to Dodecet]
    F --> G[Apply Operations]
```
    style B fill:#e1f5fe
    style C fill:#e8f5e9
    style D fill:#90EE90
    style E fill:#fff
    style F fill:#e1f5fe
```

### Byte Packing Efficiency
```mermaid
graph LR
    subgraph "Standard Storage"
        S1[2 Dodecets<br/>4 bytes]
    end

    subgraph "Packed Storage"
        S2[2 Dodecets<br/>3 bytes]
    end

    subgraph "Compression Ratio"
        R[25% smaller]
    end

    S1 --> |"2:3 ratio"| S2
    S2 --> R

    style S2 fill:#90EE90
    style R fill:#228B22
```
    style S1 fill:#fbb
```

### API Usage Flow
```mermaid
sequenceDiagram
    participant User
    participant DE as DodecetEncoder
    participant P3D as Point3D
    participant V3D as Vector3D
    participant T3D as Transform3D
    participant Calc as calculus

    User->>DE: Create Dodecet
    DE->>P3D: Create Point3D
    P3D->>V3D: Get distance
    P3D->V3D: Get vector
    V3D->>V3D: dot product
    V3D->>V3D: cross product
    V3D->>T3D: Apply transformation
    T3D->>P3D: Return transformed point
    User->>Calc: derivative
    Calc->>Calc: integral
    Calc->>Calc: gradient_descent
    Calc-->User: Return result
```

---

## Quick Start

### Installation

```toml
# Cargo.toml
[dependencies]
dodecet-encoder = "0.1"
```

### Basic Usage

```rust
use dodecet_encoder::{Dodecet, DodecetArray, DodecetString};
use dodecet_encoder::geometric::{Point3D, Vector3D};

// Create a dodecet
let d = Dodecet::from_hex(0xABC);
println!("Hex: {}", d.to_hex_string());  // "ABC"
println!("Decimal: {}", d.value());       // 2748

// Access nibbles (4-bit groups)
assert_eq!(d.nibble(0).unwrap(), 0xC);  // Bits 3-0
assert_eq!(d.nibble(1).unwrap(), 0xB);  // Bits 7-4
assert_eq!(d.nibble(2).unwrap(), 0xA);  // Bits 11-8

// 3D Point (3 dodecets = 6 bytes)
let point = Point3D::new(0x100, 0x200, 0x300);
let distance = point.distance_to(&Point3D::new(0, 0, 0));
```

---

## Core Concepts

### 1. The Dodecet Type

```rust
use dodecet_encoder::Dodecet;

// Creation methods
let d1 = Dodecet::from_hex(0x123);           // From hex value
let d2 = Dodecet::new(1000).unwrap();        // Checked (returns error if >4095)
let d3 = Dodecet::from_hex_str("ABC").unwrap(); // From string

// Conversions
let normalized: f64 = d1.normalize();  // 0.0 to 1.0
let signed: i16 = d1.as_signed();      // -2048 to 2047

// Operations
let sum = d1.wrapping_add(d2);         // Wraps at 4096
let diff = d1.wrapping_sub(d2);
let product = d1.wrapping_mul(d2);
```

### 2. Arrays and Strings

```rust
use dodecet_encoder::{DodecetArray, DodecetString};

// Fixed-size array (stack allocated)
let arr = DodecetArray::<3>::from_slice(&[0x100, 0x200, 0x300]);
assert_eq!(arr.sum().value(), 0x600);

// Growable string (heap allocated)
let mut s = DodecetString::new();
s.push(0x123);
s.push(0x456);
assert_eq!(s.to_hex_string(), "123456");

// Byte packing: 2 dodecets = 3 bytes
let bytes = s.to_bytes();
let restored = DodecetString::from_bytes(&bytes).unwrap();
```

### 3. Geometric Operations

```rust
use dodecet_encoder::geometric::{Point3D, Vector3D, Transform3D};

// 3D points
let p1 = Point3D::new(100, 200, 300);
let p2 = Point3D::new(400, 500, 600);
let dist = p1.distance_to(&p2);  // Euclidean distance

// Vector math
let v1 = Vector3D::new(100, 200, 300);
let v2 = Vector3D::new(400, 500, 600);
let dot = v1.dot(&v2);           // Dot product
let cross = v1.cross(&v2);       // Cross product
let mag = v1.magnitude();        // Magnitude

// Transformations
let translate = Transform3D::translation(100, 200, 300);
let scale = Transform3D::scale(2.0, 2.0, 2.0);
let rotate = Transform3D::rotation_z(90.0);
let transformed = translate.apply(&p1);
```

---

## Memory Layout

```mermaid
graph LR
    subgraph "Storage Efficiency"
        DODECET[Dodecet<br/>2 bytes<br/>12-bit data + 4 unused]
        F64[f64<br/>8 bytes<br/>53-bit mantissa]
    end

    subgraph "Point3D Comparison"
        P3_DODECET[3 Dodecets<br/>6 bytes total]
        P3_F64[3 x f64<br/>24 bytes total]
    end

    DODECET --> |"75% smaller"| P3_DODECET
    F64 --> P3_F64

    style DODECET fill:#90EE90
    style P3_DODECET fill:#90EE90
```

### Trade-off Analysis

| Metric | Dodecet | f64 | Winner |
|--------|---------|-----|--------|
| Storage | 2 bytes | 8 bytes | Dodecet (4x) |
| Range | 0-4095 | +/-1.8e308 | f64 |
| Precision | 12 bits | 53 bits | f64 |
| Operations | Integer ops | FPU ops | Context-dependent |
| Memory bandwidth | Lower | Higher | Dodecet |

---

## Calculus Support

The library includes numerical methods for calculus operations:

```rust
use dodecet_encoder::calculus;

// Numerical derivative (finite differences)
let f = |x: f64| x * x;
let deriv = calculus::derivative(&f, 2.0, 0.01);
// f'(2) = 4.0, computed ~= 4.0

// Numerical integral (trapezoidal rule)
let integral = calculus::integral(&f, 0.0, 2.0, 1000);
// Integral of x^2 from 0 to 2 = 8/3 ~= 2.667

// Function encoding for fast lookup
let table = calculus::encode_function(
    &|x| x.sin(),
    0.0,
    std::f64::consts::TAU,
    360
);
let y = calculus::decode_function(&table, 0.0, std::f64::consts::TAU, 1.57);
// sin(pi/2) ~= 1.0
```

**Note**: These are **numerical approximations** using standard methods. They are not suitable for applications requiring exact symbolic computation or high-precision results.

---

## Performance Characteristics

### Measured Performance (Release Build)

| Operation | Time | Notes |
|-----------|------|-------|
| Dodecet creation | ~1-2 ns | Inline, const-constructible |
| Nibble access | ~1 ns | Bit extraction |
| Bitwise ops | ~0.5 ns | Native CPU operations |
| Distance calc | ~45 ns | Includes sqrt |
| Function decode | ~180 ns | With interpolation |

**Important**: Performance varies significantly by:
- Build configuration (debug vs release)
- CPU architecture
- Compiler version
- Data locality

Run `cargo bench` on your target hardware for accurate measurements.

---

## Hex Editor Integration

Dodecets are designed to be human-readable in hex editors:

```
Offset  +0    +1    +2    +3    +4    +5    +6    +7
--------+-----+-----+-----+-----+-----+-----+-----+----
00000000+123  456  789  ABC  DEF  012  345  678
         ^^^  ^^^  ^^^  ^^^
         |    |    |    +-- 4th dodecet: 0xABC
         |    |    +------- 3rd dodecet: 0x789
         |    +------------ 2nd dodecet: 0x456
         +----------------- 1st dodecet: 0x123
```

```rust
use dodecet_encoder::hex;

// Formatting utilities
let spaced = hex::format_spaced("123456789");  // "123 456 789"
let valid = hex::is_valid("123456");           // true
let invalid = hex::is_valid("12345");          // false (not multiple of 3)
```

---

## API Summary

### Core Types

| Type | Description | Size |
|------|-------------|------|
| `Dodecet` | 12-bit value | 2 bytes (u16 storage) |
| `DodecetArray<N>` | Fixed-size array | 2N bytes (stack) |
| `DodecetString` | Growable vector | 2N + overhead (heap) |

**Component Relationships:**

```mermaid
graph TB
    subgraph "Foundation"
        DODECET[Dodecet]
    end

    subgraph "Collections"
        ARRAY[DodecetArray&lt;N&gt;]
        STRING[DodecetString]
    end

    subgraph "Geometry"
        POINT[Point3D]
        VECTOR[Vector3D]
        TRANSFORM[Transform3D]
        BOX[Box3D]
        TRIANGLE[Triangle]
    end

    subgraph "Operations"
        HEX[hex module]
        CALC[calculus module]
        SIMD[simd module]
    end

    DODECET --> ARRAY
    DODECET --> STRING
    DODECET --> POINT
    DODECET --> VECTOR

    ARRAY --> HEX
    STRING --> HEX

    POINT --> VECTOR
    POINT --> BOX
    POINT --> TRIANGLE

    VECTOR --> VECTOR
    VECTOR --> TRANSFORM

    POINT --> CALC
    VECTOR --> SIMD

    style DODECET fill:#4CAF50
    style POINT fill:#2196F3
    style VECTOR fill:#FF9800
    style TRANSFORM fill:#9C27B0
```

### Geometric Types

| Type | Description | Dodecets |
|------|-------------|----------|
| `Point3D` | 3D coordinate | 3 |
| `Vector3D` | 3D vector | 3 |
| `Transform3D` | 3x4 transform matrix | 12 |
| `Triangle` | 3 vertices | 9 |
| `Box3D` | Axis-aligned bounding box | 6 |

### Calculus Functions

| Function | Description |
|----------|-------------|
| `derivative` | Finite difference approximation |
| `integral` | Trapezoidal rule integration |
| `gradient` | Multivariate gradient |
| `laplacian` | Sum of second derivatives |
| `gradient_descent` | Optimization routine |
| `encode_function` | Create lookup table |
| `decode_function` | Interpolate from table |

---

## Examples

Run the included examples:

```bash
# Basic usage
cargo run --example basic_usage

# 3D geometry
cargo run --example geometric_shapes

# Hex visualization
cargo run --example hex_editor

# Pythagorean snapping (constraint theory)
cargo run --example pythagorean_snapping

# Comprehensive integration
cargo run --example comprehensive_integration
```

---

## Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run benchmarks (release mode)
cargo bench
```

---

## Design Decisions

### Why 12 Bits?

```mermaid
graph TB
    subgraph "Design Requirements"
        HEX[Hex-friendly<br/>3 digits]
        GEO[3D Geometry<br/>good resolution]
        STORE[Efficient Storage<br/>fits in u16]
        PREC[Historical Precedent<br/>nibbles well-understood]
    end

    subgraph "Solution: 12 Bits"
        ALIGN[4 x 3 = 12<br/>hex alignment]
        RES[4096 states<br/>per axis]
        FIT[u16 + 4 bits<br/>minimal waste]
        NIB[3 nibbles<br/>familiar concept]
    end

    HEX --> ALIGN
    GEO --> RES
    STORE --> FIT
    PREC --> NIB

    ALIGN --> TWELVE[12-Bit Dodecet]
    RES --> TWELVE
    FIT --> TWELVE
    NIB --> TWELVE

    style TWELVE fill:#4CAF50
    style ALIGN fill:#e1f5fe
    style RES fill:#fff3e0
    style FIT fill:#f3e5f5
    style NIB fill:#e8f5e9
```

1. **Hex alignment**: 12 = 4 x 3, maps to 3 hex digits cleanly
2. **3D geometry**: Good resolution for spatial coordinates
3. **Storage**: Fits in u16 with 4 unused bits (acceptable overhead)
4. **Historical precedent**: Nibbles (4 bits) are well-understood

### Why Not 8 or 16 Bits?

- 8 bits: Only 256 values, insufficient for 3D coordinates
- 16 bits: 65,536 values but requires 4 hex digits, less elegant

### Trade-offs Acknowledged

1. **Precision loss**: 12 bits < f32 (23-bit mantissa) < f64 (53-bit mantissa)
2. **Storage overhead**: 4 bits wasted per u16 (25% of storage)
3. **Ecosystem fit**: Standard libraries expect u8/u16/u32/f32/f64
4. **Operation cost**: Some conversions require computation

---

## Comparison with Standard Types

```rust
// Standard approach
let point_f64: (f64, f64, f64) = (1.234, 5.678, 9.012);
// Size: 24 bytes
// Precision: ~15 decimal digits
// Range: +/- 1.8e308

// Dodecet approach
let point_dodecet = Point3D::new(0x4D2, 0x162E, 0x2346);
// Size: 6 bytes (75% smaller)
// Precision: ~3 decimal digits
// Range: 0 to 4095 per axis
```

Choose based on your requirements:
- **Use dodecets** when memory/dominates and 12-bit precision is sufficient
- **Use f64** when precision or range matters more than memory

---

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass (`cargo test`)
5. Submit a pull request

See [CONTRIBUTING.md](.github/CONTRIBUTING.md) for details.

---

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

## Use Cases

Dodecet encoding is particularly useful for:

- **3D Graphics & Games**: Memory-efficient coordinate storage, especially for voxel engines
- **Embedded Systems & IoT**: Compact data representation in resource-constrained environments
- **Scientific Computing**: Discrete grid-based simulations with 12-bit precision
- **Network Protocols**: Compact spatial data transmission with easy debugging (hex-friendly)
- **Data Compression**: Efficient encoding of bounded numeric data
- **Lookup Tables**: Function approximation with 4096-entry resolution

---

## Used By SuperInstance Projects

This library is used by several SuperInstance projects as a reference implementation:

- **constrainttheory** - Uses dodecet encoding for compact geometric state representation
- **claw** - Uses for memory-efficient agent positioning in cellular environments
- **spreadsheet-moment** - Uses for compact cell coordinate encoding

However, **dodecet-encoder is a general-purpose library** suitable for any application requiring memory-efficient 12-bit encoding. The SuperInstance projects demonstrate real-world usage but are not required to use this library.

See [https://github.com/SuperInstance](https://github.com/SuperInstance) for more information about these projects.

---

## References

- [API Documentation](https://docs.rs/dodecet-encoder)
- [GitHub Repository](https://github.com/SuperInstance/dodecet-encoder)
- [Examples Directory](./examples/)

---

## Disclaimer

This library is provided as-is for specialized use cases. It is **not** intended as a general-purpose replacement for standard numeric types. Users should carefully evaluate whether the precision and range limitations of 12-bit encoding are appropriate for their specific application.

The calculus functions use standard numerical approximation techniques and may not be suitable for applications requiring exact symbolic computation or certified numerical bounds.

---

*Built with Rust's performance and safety guarantees.*

---

## 🔌 Integration Guide

### Feature Flags

The library uses Cargo feature flags for optional functionality:

```toml
[dependencies]
dodecet-encoder = { version = "1.1", features = ["serde", "geometry", "wasm"] }
```

| Feature | Description | Dependencies Added |
|---------|-------------|--------------------|
| `default` | Core dodecet type, hex encoding, arrays, strings | None |
| `serde` | Serialize/deserialize support | `serde`, `serde_json` |
| `geometry` | 3D geometry with `nalgebra` integration | `nalgebra` |
| `wasm` | WebAssembly bindings for browser use | `wasm-bindgen`, `js-sys`, `web-sys` |

### Ecosystem Integration

Dodecet-encoder is used across the SuperInstance ecosystem:

```mermaid
graph LR
    subgraph "SuperInstance Ecosystem"
        DE[dodecet-encoder] --> CT[constrainttheory]
        DE --> CL[claw]
        DE --> SM[spreadsheet-moment]
    end

    subgraph "Use Cases"
        CT[constrainttheory<br/>Geometric state] --> |"12-bit coords"| CG[Constraint graphs]
        CL[claw<br/>Agent positioning] --> |"12-bit cells"| CA[Cellular automata]
        SM[spreadsheet-moment<br/>Cell coords] --> |"12-bit refs"| SC[Spreadsheet cells]
    end

    style DE fill:#4CAF50
    style CT fill:#2196F3
    style CL fill:#FF9800
    style SM fill:#9C27B0
```

#### constrainttheory — Geometric Constraint Graphs

```rust
use dodecet_encoder::{Dodecet, DodecetArray};
use dodecet_encoder::geometric::{Point3D, Vector3D};

// Compact node positions in constraint graphs
// 3 dodecets per point = 6 bytes vs 24 bytes for f64
let nodes: Vec<Point3D> = graph.nodes()
    .map(|n| Point3D::new(n.x, n.y, n.z))
    .collect();

// Distance constraints between nodes
for (i, j) in graph.edges() {
    let dist = nodes[i].distance_to(&nodes[j]);
    assert!(dist.value() < Dodecet::new(500).unwrap().value());
}
```

#### claw — Cellular Agent Positioning

```rust
use dodecet_encoder::Dodecet;

// Each cell in a 64x64 grid fits in one dodecet (0-4095)
let grid_size = 64; // 64x64 = 4096 cells
let agent_x = Dodecet::new(agent.position.x)?;
let agent_y = Dodecet::new(agent.position.y)?;

// Movement within grid
agent_x = agent_x.wrapping_add(Dodecet::new(1)?); // Move right
```

#### spreadsheet-moment — Cell Reference Encoding

```rust
use dodecet_encoder::{Dodecet, DodecetString};

// Row (0-4095) and Column (0-4095) each fit in one dodecet
let row = Dodecet::new(42)?;   // Row 42
let col = Dodecet::new(1337)?; // Column 1337

// Serialize as compact hex
let reference = format!("{}{}", row.to_hex_string(), col.to_hex_string());
// "02A539" — 3 bytes instead of 8 bytes for two u16
```

### WASM Integration

The library ships WebAssembly bindings for browser use:

```html
<script type="module">
import init, { Dodecet, Point3D } from './pkg/dodecet_encoder.js';

await init();

const d = new Dodecet(0xABC);
console.log(d.to_hex_string()); // "ABC"
console.log(d.nibble(0));        // 12
console.log(d.nibble(1));        // 11
console.log(d.nibble(2));        // 10

const p1 = new Point3D(0x100, 0x200, 0x300);
const p2 = new Point3D(0x400, 0x500, 0x600);
console.log(p1.distanceTo(p2));
</script>
```

Build the WASM package:

```bash
cd wasm
wasm-pack build --target web --out-dir pkg
```

### SIMD Acceleration

The `simd` module provides batch operations using platform SIMD intrinsics:

```rust
use dodecet_encoder::simd;

// Process 16 dodecets in parallel using SIMD
let inputs: [u16; 16] = [/* ... */];
let results = simd::add_batch(&inputs, &offsets);
```

### Advanced: Constraint Theory Integration

The `pythagorean_snapping` example demonstrates using dodecet coordinates with Pythagorean triple snapping for geometric constraint systems:

```bash
cargo run --example pythagorean_snapping
```

This creates constraint graphs where edge lengths snap to Pythagorean triples (3-4-5, 5-12-13, etc.) for integer-coordinate geometric reasoning.

---

<img src="callsign1.jpg" width="128" alt="callsign">
