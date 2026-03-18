# Dodecet Encoder Source Modules

This document provides an overview of all source modules in the dodecet-encoder library, their purposes, and relationships.

## Module Architecture

```mermaid
graph TB
    subgraph "Core Library (lib.rs)"
        LIB[lib.rs<br/>Public API exports]
    end

    subgraph "Foundation Modules"
        DODECET[dodecet.rs<br/>12-bit type]
        ARRAY[array.rs<br/>Fixed arrays]
        STRING[string.rs<br/>Dynamic strings]
        STRING_OPT[string_optimized.rs<br/>Optimized strings]
    end

    subgraph "Operations Modules"
        HEX[hex.rs<br/>Hex encoding]
        CALC[calculus.rs<br/>Numerical methods]
        SIMD[simd.rs<br/>Vector operations]
    end

    subgraph "Domain Modules"
        GEO[geometric.rs<br/>3D geometry]
        WASM[wasm.rs<br/>WebAssembly bindings]
    end

    LIB --> DODECET
    LIB --> ARRAY
    LIB --> STRING
    LIB --> HEX
    LIB --> CALC
    LIB --> SIMD
    LIB --> GEO
    LIB --> WASM

    DODECET --> ARRAY
    DODECET --> STRING
    STRING --> STRING_OPT

    GEO --> DODECET
    GEO --> ARRAY
    GEO --> SIMD

    WASM --> DODECET
    WASM --> GEO
    WASM --> HEX

    style LIB fill:#4CAF50
    style DODECET fill:#2196F3
    style GEO fill:#FF9800
    style WASM fill:#9C27B0
```

## Module Overview

### Core Modules

#### dodecet.rs
**Purpose**: Core 12-bit dodecet type implementation

**Key Types**:
- `Dodecet` - 12-bit value (0-4095)
- Stored as `u16` with 4 unused bits
- Hex-friendly (3 hex digits)

**Key Operations**:
- Creation: `from_hex()`, `new()`, `from_hex_str()`
- Conversion: `normalize()`, `as_signed()`, `to_hex_string()`
- Arithmetic: `wrapping_add()`, `wrapping_sub()`, `wrapping_mul()`
- Bitwise: AND, OR, XOR operations
- Nibble access: `nibble(0)`, `nibble(1)`, `nibble(2)`

**Dependencies**: None (foundational type)

#### array.rs
**Purpose**: Fixed-size stack-allocated dodecet arrays

**Key Types**:
- `DodecetArray<N>` - Compile-time sized array

**Key Operations**:
- Creation: `from_slice()`, `new()`
- Access: Index-based, iteration
- Aggregation: `sum()`, `product()`, `min()`, `max()`
- Conversion: `to_hex_string()`, `to_bytes()`

**Dependencies**: `dodecet.rs`

#### string.rs
**Purpose**: Dynamic heap-allocated dodecet sequences

**Key Types**:
- `DodecetString` - Growable vector of dodecets

**Key Operations**:
- Creation: `new()`, `with_capacity()`
- Mutation: `push()`, `pop()`, `extend()`
- Conversion: `to_hex_string()`, `to_bytes()`, `from_bytes()`
- Query: `len()`, `is_empty()`, `capacity()`

**Dependencies**: `dodecet.rs`

#### string_optimized.rs
**Purpose**: Performance-optimized string operations

**Key Features**:
- Batch operations
- Memory-efficient packing
- SIMD-friendly layouts

**Dependencies**: `dodecet.rs`, `string.rs`

### Operations Modules

#### hex.rs
**Purpose**: Hex encoding/decoding utilities

**Key Functions**:
- `format_spaced()` - Add spaces between dodecets
- `is_valid()` - Validate hex strings
- `to_bytes()` - Convert hex to bytes
- `from_bytes()` - Convert bytes to hex

**Key Features**:
- 3-char-per-dodecet validation
- Spaced formatting for readability
- Error handling for invalid input

**Dependencies**: None

#### calculus.rs
**Purpose**: Numerical methods and calculus operations

**Key Functions**:
- `derivative()` - Finite difference approximation
- `integral()` - Trapezoidal rule integration
- `gradient()` - Multivariate gradient
- `laplacian()` - Sum of second derivatives
- `gradient_descent()` - Optimization routine
- `encode_function()` - Create lookup table
- `decode_function()` - Interpolate from table

**Key Features**:
- Numerical approximations (not symbolic)
- Function encoding for fast lookup
- Optimization algorithms

**Dependencies**: None (uses f64 for calculations)

#### simd.rs
**Purpose**: SIMD-optimized vector operations

**Key Features**:
- Batch dodecet operations
- Platform-specific optimizations
- Cache-friendly memory layouts

**Dependencies**: `dodecet.rs`, `array.rs`

### Domain Modules

#### geometric.rs
**Purpose**: 3D geometry and vector math

**Key Types**:
- `Point3D` - 3D point (x, y, z)
- `Vector3D` - 3D vector (dx, dy, dz)
- `Transform3D` - 3x4 transformation matrix
- `Box3D` - Axis-aligned bounding box
- `Triangle` - 3-vertex triangle

**Key Operations**:
- Distance: `distance_to()`, `magnitude()`
- Vector math: `dot()`, `cross()`, `normalize()`
- Transformations: translation, rotation, scale
- Queries: contains, intersects, bounds

**Dependencies**: `dodecet.rs`, `array.rs`, `simd.rs`

#### wasm.rs
**Purpose**: WebAssembly bindings for browser use

**Key Types Exported**:
- `Point3D` - WASM-compatible 3D point
- `Vector3DWasm` - WASM-compatible vector
- `Transform3DWasm` - WASM-compatible transform

**Key Features**:
- `wasm-bindgen` integration
- JavaScript/TypeScript API
- Browser-compatible memory management

**Dependencies**: `dodecet.rs`, `geometric.rs`, `hex.rs`

## Module Size & Complexity

| Module | Lines | Purpose | Complexity |
|--------|-------|---------|------------|
| `dodecet.rs` | ~400 | Core type | Low |
| `array.rs` | ~300 | Fixed arrays | Low |
| `string.rs` | ~400 | Dynamic strings | Medium |
| `string_optimized.rs` | ~350 | Optimizations | Medium |
| `hex.rs` | ~250 | Hex utilities | Low |
| `calculus.rs` | ~400 | Numerical methods | High |
| `simd.rs` | ~250 | Vector ops | High |
| `geometric.rs` | ~600 | 3D geometry | Medium |
| `wasm.rs` | ~400 | WASM bindings | Medium |

## Data Flow Diagrams

### Dodecet Creation Flow

```mermaid
graph LR
    INPUT[Input Value] --> VALIDATE{Validate}
    VALIDATE -->|u16| FROM_U16[from_hex]
    VALIDATE -->|str| FROM_STR[from_hex_str]
    VALIDATE -->|number| CHECK{Check Range}

    CHECK -->|0-4095| NEW[new]
    CHECK -->|>4095| ERROR[Error]

    FROM_U16 --> DODECET[Dodecet]
    FROM_STR --> PARSE[Parse Hex]
    PARSE --> DODECET
    NEW --> DODECET

    style DODECET fill:#4CAF50
    style ERROR fill:#f44336
```

### Geometric Operations Flow

```mermaid
graph TB
    POINTS[Two Point3D] --> SUB[Subtract coordinates]
    SUB --> VECTOR[Vector3D]

    VECTOR --> DOT[dot product]
    VECTOR --> CROSS[cross product]
    VECTOR --> MAG[magnitude]

    DOT --> SCALAR[Scalar result]
    CROSS --> VECTOR2[New Vector3D]
    MAG --> DIST[Distance]

    style POINTS fill:#e1f5fe
    style VECTOR fill:#fff3e0
    style DIST fill:#f3e5f5
```

### WASM Binding Flow

```mermaid
sequenceDiagram
    participant JS as JavaScript
    participant WASM as wasm.rs
    participant GEO as geometric.rs
    participant DODECET as dodecet.rs

    JS->>WASM: new Point3D(x, y, z)
    WASM->>GEO: Point3D::new()
    GEO->>DODECET: Dodecet::new()
    DODECET-->>GEO: Dodecet instances
    GEO-->>WASM: Point3D
    WASM-->>JS: Point3D instance

    JS->>WASM: point.distanceTo(other)
    WASM->>GEO: distance_to()
    GEO->>GEO: Calculate sqrt(dx² + dy² + dz²)
    GEO-->>WASM: f64 distance
    WASM-->>JS: number
```

## Testing Strategy

### Unit Tests per Module

```mermaid
graph TB
    subgraph "Test Coverage"
        DODECET_TESTS[dodecet.rs<br/>40+ tests]
        ARRAY_TESTS[array.rs<br/>30+ tests]
        STRING_TESTS[string.rs<br/>35+ tests]
        HEX_TESTS[hex.rs<br/>25+ tests]
        CALC_TESTS[calculus.rs<br/>20+ tests]
        SIMD_TESTS[simd.rs<br/>15+ tests]
        GEO_TESTS[geometric.rs<br/>50+ tests]
        WASM_TESTS[wasm.rs<br/>30+ tests]
    end

    subgraph "Test Categories"
        UNIT[Unit Tests]
        INTG[Integration Tests]
        PROP[Property Tests]
        PERF[Performance Tests]
    end

    DODECET_TESTS --> UNIT
    ARRAY_TESTS --> UNIT
    STRING_TESTS --> UNIT
    HEX_TESTS --> UNIT

    CALC_TESTS --> INTG
    GEO_TESTS --> INTG
    WASM_TESTS --> INTG

    SIMD_TESTS --> PERF

    style UNIT fill:#e1f5fe
    style INTG fill:#fff3e0
    style PERF fill:#f3e5f5
```

## Build Configuration

### Feature Flags

```toml
[features]
default = ["geometric", "calculus"]
geometric = ["dep:geometric"]
calculus = ["dep:calculus"]
wasm = ["dep:wasm-bindgen"]
simd = ["dep:simd"]
```

### Conditional Compilation

```mermaid
graph TB
    subgraph "Platform Targets"
        NATIVE[Native Rust]
        WASM32[WASM32]
        SIMD[SIMD-enabled]
    end

    subgraph "Enabled Features"
        GEO[geometric.rs]
        CALC[calculus.rs]
        WASM[wasm.rs]
        SIMD_OPS[simd.rs]
    end

    NATIVE --> GEO
    NATIVE --> CALC
    NATIVE --> SIMD_OPS

    WASM32 --> GEO
    WASM32 --> WASM

    SIMD --> SIMD_OPS

    style NATIVE fill:#4CAF50
    style WASM32 fill:#2196F3
    style SIMD fill:#FF9800
```

## Performance Considerations

### Memory Layout

```mermaid
graph LR
    subgraph "Dodecet"
        U16[u16: 2 bytes]
        BITS[12 bits data]
        UNUSED[4 bits unused]
    end

    subgraph "DodecetArray<3>"
        D1[Dodecet 1]
        D2[Dodecet 2]
        D3[Dodecet 3]
        TOTAL[6 bytes total]
    end

    subgraph "Point3D"
        X[x: Dodecet]
        Y[y: Dodecet]
        Z[z: Dodecet]
        SIZE[6 bytes total]
    end

    U16 --> BITS
    U16 --> UNUSED

    D1 --> TOTAL
    D2 --> TOTAL
    D3 --> TOTAL

    X --> SIZE
    Y --> SIZE
    Z --> SIZE

    style BITS fill:#4CAF50
    style TOTAL fill:#2196F3
    style SIZE fill:#FF9800
```

### Optimization Strategies

1. **Inline Functions**: Small operations are inlined
2. **Const Generics**: Compile-time array sizing
3. **SIMD**: Vector operations where available
4. **Cache Locality**: Compact memory layout
5. **Zero-copy**: References over clones

## Contributing Guidelines

### Adding New Features

1. Choose appropriate module
2. Add comprehensive tests
3. Document with examples
4. Update this README
5. Run `cargo test` and `cargo clippy`

### Code Style

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Add doc comments to public items
- Include usage examples in docs
- Keep functions focused and small

## Module Dependencies Summary

```
dodecet.rs (foundation)
├── array.rs
├── string.rs
│   └── string_optimized.rs
├── geometric.rs
│   └── simd.rs
└── wasm.rs
    ├── hex.rs
    └── geometric.rs
```

---

**Last Updated**: 2026-03-17
**Version**: 1.0.0
**Status**: Production Ready
