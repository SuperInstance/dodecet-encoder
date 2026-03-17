# Disclaimers and Limitations

**Important: Please read this document before using the dodecet-encoder library.**

---

## General Disclaimer

The dodecet-encoder library is a **specialized tool** for specific computational domains. It is **not** a general-purpose replacement for standard numeric types (f32, f64, i32, etc.).

### Suitability

- **Suitable for**: Memory-constrained applications, embedded systems, 3D geometry with limited precision requirements, function lookup tables
- **Not suitable for**: Scientific simulations, financial calculations, high-precision applications, general-purpose computing

---

## Precision Limitations

### 12-Bit Encoding

The dodecet uses **12 bits** of precision, which means:

| Metric | Value |
|--------|-------|
| Distinct values | 4,096 |
| Decimal precision | ~3-4 significant digits |
| Range (unsigned) | 0 to 4,095 |
| Range (signed) | -2,048 to 2,047 |

### Comparison with Standard Types

| Type | Bits of Precision | Decimal Digits |
|------|-------------------|----------------|
| Dodecet | 12 | ~3-4 |
| f32 | 23 (mantissa) | ~7 |
| f64 | 53 (mantissa) | ~15-16 |

**Implication**: Dodecets cannot represent values with the precision of standard floating-point types. Quantization errors will accumulate in repeated operations.

---

## Range Limitations

### Unsigned Range

```
Minimum: 0x000 = 0
Maximum: 0xFFF = 4,095
```

### Signed Interpretation

```
Minimum: 0x800 = -2,048 (two's complement)
Maximum: 0x7FF = 2,047
Zero:     0x000 = 0
```

### Comparison

| Type | Range |
|------|-------|
| Dodecet | 0 to 4,095 (unsigned) or -2,048 to 2,047 (signed) |
| i16 | -32,768 to 32,767 |
| f32 | approximately +/- 3.4e38 |
| f64 | approximately +/- 1.8e308 |

**Implication**: Many real-world values cannot be represented directly. Scaling and normalization are often required.

---

## Quantization Error

When converting from higher-precision types to dodecets, information is lost:

```rust
// Original value
let precise: f64 = 123.456789;

// Convert to dodecet (must scale and round)
let scale = 10.0;
let dodecet_value = (precise * scale).round() as u16;  // 1235

// Convert back
let restored = dodecet_value as f64 / scale;  // 123.5

// Error
let error = (precise - restored).abs();  // 0.04321...
```

**Error bounds**: For normalized values in [0, 1], the maximum quantization error is approximately `1/8192 = 0.00012`.

---

## Accumulation of Errors

Repeated operations on dodecets can accumulate errors:

```rust
// Example: Sum of many dodecets
let values: Vec<Dodecet> = (0..1000)
    .map(|i| Dodecet::from_hex((i % 4096) as u16))
    .collect();

// Each addition wraps at 4096
// This is different from standard integer addition
```

**Implication**: Long chains of operations may produce different results than equivalent floating-point computations.

---

## Calculus Functions Disclaimer

The calculus module uses **numerical approximation methods**:

### Derivative

- **Method**: Finite differences (central difference)
- **Accuracy**: O(h^2) where h is the step size
- **Limitation**: Approximation error increases with step size; numerical instability with very small step sizes

### Integral

- **Method**: Trapezoidal rule
- **Accuracy**: O(h^2) where h is the step size
- **Limitation**: Approximation error; not suitable for discontinuous functions

### Gradient Descent

- **Method**: Standard gradient descent
- **Limitation**: May converge to local minima; requires appropriate learning rate; not guaranteed to converge

### General Calculus Warning

> **These functions are not suitable for applications requiring:**
> - Exact symbolic computation
> - Certified numerical bounds
> - High-precision scientific calculations
> - Safety-critical computations

For such applications, use dedicated numerical libraries like:
- `rug` (arbitrary precision)
- `arb` (rigorous numerics)
- `symbolica` (symbolic computation)

---

## Memory Savings Disclaimer

The library claims "75% memory savings vs f64" in specific scenarios:

### When This Applies

- Comparing raw storage of 3D points (6 bytes vs 24 bytes)
- Large arrays where memory footprint is the primary concern
- Situations where 12-bit precision is acceptable

### When This Does NOT Apply

- When accounting for Vec/collection overhead
- When intermediate values require conversion to f64
- When the 4 unused bits per u16 are considered waste
- When comparing to compressed formats

**Actual savings depend on your specific use case.**

---

## Performance Claims Disclaimer

Performance benchmarks provided in this project:

1. **Are hardware-dependent**: Results vary by CPU, memory speed, cache size
2. **Are compiler-dependent**: Results vary by Rust version, optimization level
3. **Are context-dependent**: Results vary by workload, data patterns
4. **Were measured on specific hardware**: Your results will differ

### Reproducing Benchmarks

```bash
# Run on your hardware
cargo bench

# Compare with release optimizations
cargo bench --release
```

### Expected Variance

| Factor | Expected Variance |
|--------|-------------------|
| Same hardware, same build | +/- 5% |
| Different CPU of same generation | +/- 20% |
| Different CPU generations | +/- 100% or more |
| Debug vs Release build | 10-50x difference |

---

## Use Case Disclaimers

### 3D Geometry

> **Disclaimer**: 12-bit precision provides approximately 4,096 discrete positions per axis. This may be insufficient for:
> - Large-scale worlds (position precision degrades with distance)
> - Smooth camera movement (may exhibit jitter)
> - Physics simulations (collision detection may be imprecise)

### Function Lookup Tables

> **Disclaimer**: With 4,096 entries, linear interpolation may produce visible artifacts for:
> - High-frequency functions
> - Functions with discontinuities
> - Applications requiring smooth derivatives

### Network Transmission

> **Disclaimer**: While hex encoding is human-readable, it is not the most compact representation. For bandwidth-critical applications, consider:
> - Binary protocols
> - Compression (gzip, zstd)
> - Delta encoding

---

## Not a Cryptographic Library

The dodecet-encoder library is **not designed for cryptographic purposes**:

- No constant-time operations
- No resistance to timing attacks
- Hex encoding reveals all data
- No encryption or authentication

For cryptographic applications, use libraries like:
- `ring` or `rustls` for secure communication
- `sha2` for hashing
- `aes` for encryption

---

## No Warranty

THIS SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

---

## Summary Checklist

Before using dodecet-encoder, verify:

- [ ] Your application can tolerate 12-bit precision
- [ ] Your values fit within the 0-4095 range (or -2048 to 2047 signed)
- [ ] You understand quantization error accumulation
- [ ] Numerical approximations are acceptable for your use case
- [ ] Memory savings justify the precision trade-off
- [ ] Your use case is among the recommended ones (3D geometry, lookup tables, embedded systems)

If any of these do not apply, consider using standard numeric types (f32, f64, i32, etc.) instead.

---

*Last updated: 2026-03-17*
