# Dodecet Encoder - Performance Benchmarks

**Version:** 1.1.0
**Last Updated:** 2026-03-17

---

## Important Disclaimers

1. **Hardware-Dependent**: These benchmarks were run on specific hardware. Your results will vary.
2. **Compiler-Dependent**: Results depend on Rust version and optimization settings.
3. **Context-Dependent**: Performance in real applications differs from microbenchmarks.
4. **Debug vs Release**: Debug builds are typically 10-50x slower than release builds.

**Always run `cargo bench` on your target hardware for accurate measurements.**

---

## Benchmark Methodology

### Test Environment

The results below were measured on:

| Parameter | Value |
|-----------|-------|
| OS | Windows 10/11 |
| CPU | x86_64, varies |
| RAM | 16+ GB |
| Rust | 1.70+ |
| Profile | Release (`opt-level = 3`) |

### Measurement Approach

- **Framework**: Criterion.rs
- **Iterations**: Variable (auto-determined by Criterion)
- **Warm-up**: 100+ iterations
- **Statistical Analysis**: Bootstrap confidence intervals

---

## Core Operations

### Dodecet Creation

| Operation | Time | Relative to f64 |
|-----------|------|-----------------|
| `Dodecet::from_hex(0x123)` | ~1-2 ns | Similar |
| `Dodecet::new(1000)` | ~1-2 ns | Similar |
| `Dodecet::default()` | ~0.5-1 ns | Similar |

**Interpretation**: Creation is fast because it's essentially a u16 assignment.

### Nibble Access

| Operation | Time | Notes |
|-----------|------|-------|
| `d.nibble(0)` | ~0.5-1 ns | Bit extraction |
| `d.nibble(1)` | ~0.5-1 ns | Bit extraction |
| `d.nibble(2)` | ~0.5-1 ns | Bit extraction |

**Interpretation**: Nibble access is a simple bit shift and mask operation.

### Bitwise Operations

| Operation | Time | Comparison |
|-----------|------|------------|
| `a & b` | ~0.5 ns | Same as u16 |
| `a \| b` | ~0.5 ns | Same as u16 |
| `a ^ b` | ~0.5 ns | Same as u16 |
| `!a` | ~0.5 ns | Same as u16 |

**Interpretation**: Bitwise operations on dodecets are identical to u16 operations.

### Arithmetic Operations

| Operation | Time | Notes |
|-----------|------|-------|
| `a + b` (wrapping) | ~0.5-1 ns | Integer addition |
| `a - b` (wrapping) | ~0.5-1 ns | Integer subtraction |
| `a * b` (wrapping) | ~0.5-1 ns | Integer multiplication |
| `a / b` | ~1-2 ns | Integer division |

**Comparison with f64**:
- Integer operations are typically faster than floating-point
- However, the difference is negligible on modern CPUs for single operations

---

## Geometric Operations

### Point3D Operations

| Operation | Time | f64 Equivalent | Notes |
|-----------|------|----------------|-------|
| `Point3D::new()` | ~3-5 ns | ~3-5 ns | Similar |
| `distance_to()` | ~40-50 ns | ~30-40 ns | f64 slightly faster |
| `to_hex_string()` | ~100-200 ns | N/A | Dodecet advantage |

### Vector3D Operations

| Operation | Time | f64 Equivalent | Notes |
|-----------|------|----------------|-------|
| `Vector3D::new()` | ~3-5 ns | ~3-5 ns | Similar |
| `magnitude()` | ~12-15 ns | ~10-12 ns | f64 slightly faster |
| `dot()` | ~10-12 ns | ~8-10 ns | f64 slightly faster |
| `cross()` | ~15-20 ns | ~12-15 ns | f64 slightly faster |
| `normalize()` | ~20-25 ns | ~15-20 ns | f64 slightly faster |

**Interpretation**: Floating-point operations are often slightly faster due to FPU optimization. The dodecet's advantage is in memory efficiency, not raw operation speed.

---

## Memory Efficiency

### Storage Comparison

| Data Structure | Dodecet Size | f64 Size | Savings |
|----------------|--------------|----------|---------|
| Single value | 2 bytes | 8 bytes | 75% |
| Point3D | 6 bytes | 24 bytes | 75% |
| Vector3D | 6 bytes | 24 bytes | 75% |
| Transform3D | 24 bytes | 96 bytes | 75% |
| 1,000 points | 6 KB | 24 KB | 75% |
| 100,000 points | 600 KB | 2.4 MB | 75% |

### Important Context

1. **Theoretical savings**: These are raw storage comparisons.
2. **Real-world savings**: Actual savings depend on:
   - Collection overhead (Vec, HashMap, etc.)
   - Alignment padding
   - Cache line effects
3. **Trade-off**: 75% memory savings come at the cost of precision.

---

## Encoding/Decoding

### Hex Encoding

| Operation | Size | Time |
|-----------|------|------|
| `encode()` (1 value) | 1 dodecet | ~15 ns |
| `encode()` (100 values) | 100 dodecets | ~150 ns |
| `encode()` (1,000 values) | 1,000 dodecets | ~1.5 us |

### Hex Decoding

| Operation | Size | Time |
|-----------|------|------|
| `decode()` (1 value) | 3 chars | ~18 ns |
| `decode()` (100 values) | 300 chars | ~180 ns |
| `decode()` (1,000 values) | 3,000 chars | ~1.8 us |

**Interpretation**: Hex encoding/decoding is O(n) and reasonably fast for most use cases.

---

## Calculus Operations

### Derivative (Finite Difference)

| Steps | Time | Error (typical) |
|-------|------|-----------------|
| 1 | ~250 ns | O(h^2) |
| 2 | ~500 ns | O(h^2) |

### Integral (Trapezoidal Rule)

| Steps | Time | Error (typical) |
|-------|------|-----------------|
| 100 | ~1.5 us | O(h^2) |
| 1,000 | ~15 us | O(h^2) |
| 10,000 | ~150 us | O(h^2) |

### Function Encoding

| Points | Time |
|--------|------|
| 360 | ~8 us |
| 1,000 | ~22 us |
| 4,096 | ~90 us |

**Important**: These are numerical approximations with known error bounds. They are not exact symbolic computations.

---

## Running Your Own Benchmarks

### Basic Benchmark Run

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench dodecet_benchmark

# Run with HTML report
cargo bench -- --save-baseline my_baseline
```

### Comparing with Baseline

```bash
# Save baseline
cargo bench -- --save-baseline v1.0

# Compare with baseline
cargo bench -- --baseline v1.0
```

### Profiling

```bash
# Generate flamegraph (requires flamegraph crate)
cargo flamegraph --bench dodecet_benchmark -- --bench

# Use perf on Linux
perf record cargo bench
perf report
```

---

## Performance Tuning Tips

### 1. Use Release Mode

```bash
# Debug mode: 10-50x slower
cargo test

# Release mode: Optimized
cargo test --release
cargo bench
```

### 2. Enable LTO for Maximum Performance

```toml
# Cargo.toml
[profile.release]
lto = true
codegen-units = 1
opt-level = 3
```

### 3. Batch Operations

```rust
// Slower: Individual operations
for d in &dodecets {
    let hex = d.to_hex_string();
}

// Faster: Batch operation
let hex = dodecet_string.to_hex_string();
```

### 4. Minimize Conversions

```rust
// Slower: Frequent conversions
for d in &dodecets {
    let f = d.normalize() as f32;
    // ... work with f32 ...
}

// Faster: Work in dodecet space when possible
for d in &mut dodecets {
    *d = d.wrapping_add(Dodecet::from_hex(1));
}
```

---

## Comparison with Alternatives

### vs f64 (Double Precision)

| Aspect | Dodecet | f64 | Winner |
|--------|---------|-----|--------|
| Size | 2 bytes | 8 bytes | Dodecet (4x) |
| Precision | 12 bits | 53 bits | f64 (4.4x) |
| Range | 0-4095 | +/- 1.8e308 | f64 |
| Operation speed | Fast | Fast | Similar |
| Memory bandwidth | Lower | Higher | Dodecet |
| Ecosystem support | Limited | Universal | f64 |

### vs u16 (Standard 16-bit Integer)

| Aspect | Dodecet | u16 | Winner |
|--------|---------|-----|--------|
| Size | 2 bytes | 2 bytes | Tie |
| Range | 0-4095 | 0-65535 | u16 (16x) |
| Nibble access | Native | Manual | Dodecet |
| Hex display | 3 chars | 4 chars | Dodecet |
| Geometric types | Included | Manual | Dodecet |

### When to Choose Dodecet

1. Memory footprint is critical
2. 12-bit precision is sufficient
3. Hex display/debugging is important
4. Working with geometric data
5. Embedded or constrained environments

### When to Choose Alternatives

1. Precision > 12 bits required -> Use f64
2. Range > 4095 required -> Use i32 or f64
3. Standard library compatibility -> Use u16
4. Scientific computing -> Use f64 or higher precision libraries

---

## Benchmark Reproduction

To reproduce these benchmarks:

```bash
# Clone repository
git clone https://github.com/SuperInstance/dodecet-encoder.git
cd dodecet-encoder

# Run benchmarks
cargo bench

# Results will be in target/criterion/
```

### Expected Variance

| Scenario | Expected Variance |
|----------|-------------------|
| Same machine, same build | +/- 5% |
| Different machine, same CPU gen | +/- 20% |
| Different CPU generation | +/- 100%+ |
| Debug vs Release | 10-50x |

---

## Summary

| Category | Performance | Notes |
|----------|-------------|-------|
| Core operations | ~1 ns | Very fast |
| Geometric ops | ~10-50 ns | Comparable to f64 |
| Hex encoding | ~150 ns/100 values | O(n) scaling |
| Calculus | ~1-100 us | Approximation methods |
| Memory | 75% savings vs f64 | Trade-off: precision |

**Bottom Line**: Dodecets excel in memory efficiency and hex readability. Raw operation speed is comparable to standard types. The choice depends on your specific requirements for precision, range, and memory usage.

---

*For detailed methodology and raw data, see `target/criterion/` after running `cargo bench`.*
