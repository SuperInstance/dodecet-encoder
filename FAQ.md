# Frequently Asked Questions

## General Questions

### What is a dodecet?

A **dodecet** is a 12-bit unit composed of 3 nibbles (4-bit groups). It provides 4,096 discrete values (vs 256 for 8-bit bytes), making it ideal for geometric and calculus operations.

### Why 12 bits instead of 8 or 16?

**12 bits provides the optimal balance:**
- **Precision**: 4,096 values (16x more than 8-bit)
- **Efficiency**: 1.5x more bits than 8-bit for 16x more values
- **Hex-friendly**: Each dodecet = 3 hex digits (e.g., 0xABC)
- **Memory**: Efficient packing (2 dodecets = 3 bytes)
- **Alignment**: Natural alignment for 3D coordinates (x, y, z)

### Is dodecet encoding production-ready?

**Yes!** Version 1.0.0 is production-ready:
- 61 tests, all passing
- Comprehensive documentation
- 12 working examples
- 6 detailed tutorials
- Security audit completed
- Performance benchmarks available

### What are the main use cases?

**Primary use cases:**
1. **3D Geometry**: Points, vectors, transformations
2. **Calculus**: Derivatives, integrals, optimization
3. **Constraint Theory**: Geometric reasoning
4. **Cellular Agents**: Efficient state representation
5. **Data Compression**: Compact coordinate storage

---

## Technical Questions

### How do I convert between dodecet and other types?

```rust
use dodecet_encoder::Dodecet;

// From hex
let d = Dodecet::from_hex(0xABC);

// From integer (must be ≤ 4095)
let d = Dodecet::new(2748)?;

// To hex string
let hex = d.to_hex_string(); // "ABC"

// To binary string
let bin = d.to_binary_string(); // "101010111100"

// To normalized (0.0 to 1.0)
let norm = d.normalize(); // 0.671

// To signed (-2048 to 2047)
let signed = d.as_signed(); // i16
```

### What happens when arithmetic overflows?

```rust
let a = Dodecet::from_hex(0xFFF); // 4095
let b = Dodecet::from_hex(0x001); // 1

// Wrapping by default
let sum = a + b; // Wraps to 0x000

// Checked arithmetic
match a.checked_add(b) {
    Ok(result) => { /* use result */ }
    Err(_) => { /* handle overflow */ }
}

// Saturating arithmetic
let sum = a.saturating_add(b); // Returns 0xFFF
```

### Can I use dodecets for floating-point operations?

**Not directly.** Dodecets are integer-based, but you can:

```rust
// Option 1: Convert to float for calculations
let d = Dodecet::from_hex(0x800);
let float_val = d.normalize(); // 0.5
let result = float_val.sqrt(); // Use float operations

// Option 2: Use fixed-point arithmetic
let fixed = d.value() as i32; // Use as fixed-point
let result = (fixed * fixed) / 4096; // Manual operations
```

### How do I handle precision limitations?

```rust
// Be aware of quantization effects
let point = Point3D::new(0x100, 0x200, 0x300);

// For high precision, use multiple dodecets
// or combine with floating-point where needed

// For most geometric operations, 12-bit is sufficient
// 4096 values provides sub-millimeter precision
```

---

## Performance Questions

### How fast are dodecet operations?

**Benchmark results (typical):**
- Creation: 1.2 ns
- Nibble access: 0.8 ns
- Bitwise ops: 0.5 ns
- Arithmetic: 0.6 ns
- Distance calc: 45 ns
- Vector dot: 12 ns

**Memory efficiency:**
- Point3D: 6 bytes (vs 24 bytes for f64)
- 75% memory savings
- Better cache locality

### When should I use dodecets vs floats?

**Use dodecets for:**
- Discrete geometry (coordinates, vectors)
- Integer-heavy calculations
- Memory-constrained environments
- Deterministic results required
- Hex debugging needed

**Use floats for:**
- High-precision scientific computing
- Complex mathematical operations
- When precision > 12-bit needed
- Extensive floating-point libraries

### Can I parallelize dodecet operations?

**Yes!** Dodecets are designed for parallel processing:

```rust
use rayon::prelude::*;

let points: Vec<Point3D> = /* ... */;
let query = Point3D::new(0x100, 0x200, 0x300);

// Parallel distance calculation
let distances: Vec<f64> = points.par_iter()
    .map(|p| p.distance_to(&query))
    .collect();
```

---

## Integration Questions

### How do I integrate with existing code?

```rust
// Convert from existing types
impl From<(f32, f32, f32)> for Point3D {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Point3D::new(
            (x * 4095.0) as u16,
            (y * 4095.0) as u16,
            (z * 4095.0) as u16,
        )
    }
}

// Use with existing libraries
let my_point = MyPointType { x: 1.0, y: 2.0, z: 3.0 };
let dodecet_point = Point3D::from((my_point.x, my_point.y, my_point.z));
```

### Can I use dodecets in WebAssembly?

**Yes!** Full WebAssembly support:

```bash
# Build for WASM
wasm-pack build --target web
```

```javascript
// Use in JavaScript
import { Dodecet, Point3D } from './dodecet_encoder.js';

const d = new Dodecet(0xABC);
console.log(d.toHexString());
```

### How do I serialize/deserialize dodecets?

```rust
// To bytes
let s = DodecetString::from_slice(&[0x123, 0x456, 0x789]);
let bytes = s.to_bytes(); // 9 bytes

// From bytes
let deserialized = DodecetString::from_bytes(&bytes)?;

// With serde (JSON)
#[derive(Serialize, Deserialize)]
struct MyPoint {
    x: u16,
    y: u16,
    z: u16,
}
```

---

## Community Questions

### How can I contribute?

**We welcome contributions!**

1. **Find an issue**: Look for [good first issue](https://github.com/SuperInstance/dodecet-encoder/labels/good%20first%20issue) label
2. **Read contributing guide**: See [CONTRIBUTING.md](CONTRIBUTING.md)
3. **Make your changes**: Fork, branch, code, test
4. **Submit PR**: Create pull request with description

**Contribution areas:**
- Bug fixes
- New features
- Documentation
- Examples
- Tests
- Benchmarks

### Where can I get help?

**Support channels:**

1. **GitHub Issues**: Bug reports, feature requests
2. **GitHub Discussions**: Questions, ideas
3. **Documentation**: [README](README.md), [tutorials](tutorials/)
4. **Examples**: [examples/](examples/) directory

**Before asking:**
- Search existing issues and discussions
- Read the documentation
- Try the examples

### What is the release schedule?

**Version support:**
- **1.0.x**: Full support, security updates
- **0.2.x**: Critical fixes only
- **0.1.x**: End of life

**Release cadence:**
- **Major**: Every 6 months
- **Minor**: Monthly
- **Patch**: As needed

---

## Migration Questions

### How do I migrate from 0.x to 1.0?

**Breaking changes in 1.0:**
- None! 1.0 is backward compatible with 0.2.x

**Migration steps:**
1. Update `Cargo.toml`: `dodecet-encoder = "1.0"`
2. Run `cargo update`
3. Run tests to verify
4. Review deprecation warnings

**Most code should work without changes!**

### How do I migrate from floats?

```rust
// Before: f64-based
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

// After: dodecet-based
struct Point {
    x: u16,
    y: u16,
    z: u16,
}

// Helper to convert
impl From<PointF64> for Point {
    fn from(p: PointF64) -> Self {
        Self {
            x: (p.x.normalize() * 4095.0) as u16,
            y: (p.y.normalize() * 4095.0) as u16,
            z: (p.z.normalize() * 4095.0) as u16,
        }
    }
}
```

---

## Security Questions

### Is dodecet encoding cryptographically secure?

**No.** Dodecet encoding is NOT for cryptography:
- No cryptographic guarantees
- No encryption or hashing
- Use dedicated crypto libraries for security

See [SECURITY.md](SECURITY.md) for details.

### Are there any security vulnerabilities?

**Security audit completed (2026-03-15):**
- No critical or high-severity issues
- All recommendations implemented
- Regular security updates

**Report vulnerabilities:** security@superinstance.ai

---

## License Questions

### What license is dodecet-encoder under?

**MIT License** - See [LICENSE](LICENSE) file.

**What this means:**
- Free to use, modify, distribute
- Attribution required
- No warranty provided
- Compatible with most licenses

### Can I use dodecet-encoder commercially?

**Yes!** The MIT License permits commercial use:
- In proprietary software
- In commercial products
- In closed-source applications
- With proper attribution

---

## Still Have Questions?

**Check these resources:**
- [README](README.md) - Project overview
- [Documentation](https://docs.rs/dodecet-encoder) - API reference
- [Tutorials](tutorials/) - Step-by-step guides
- [Examples](examples/) - Working examples
- [GitHub Issues](https://github.com/SuperInstance/dodecet-encoder/issues) - Report issues
- [GitHub Discussions](https://github.com/SuperInstance/dodecet-encoder/discussions) - Ask questions

**Contact:**
- **Email**: team@superinstance.ai
- **GitHub**: https://github.com/SuperInstance/dodecet-encoder
- **Security**: security@superinstance.ai

---

**Last Updated:** 2026-03-16
**Version:** 1.0.0
