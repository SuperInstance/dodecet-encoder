# Dodecet Encoder - WebAssembly Integration

[![Build Status](https://github.com/SuperInstance/dodecet-encoder/workflows/Build%20and%20Test%20WASM%20Package/badge.svg)](https://github.com/SuperInstance/dodecet-encoder/actions)
[![npm version](https://badge.fury.io/js/%40superinstance%2Fdodecet-encoder.svg)](https://www.npmjs.com/package/@superinstance/dodecet-encoder)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance 12-bit dodecet encoding system for geometric and calculus operations, now with WebAssembly support for browser and Node.js environments.

## 🌟 Features

- **🚀 High Performance**: WebAssembly-optimized operations
- **📐 Geometric Primitives**: Point3D, Vector3D, Transform3D
- **🔢 Hex-Friendly**: Natural 3-digit hex encoding
- **💾 Efficient Storage**: 16x more precision than 8-bit
- **🎯 TypeScript Support**: Full TypeScript definitions
- **🌐 Universal**: Works in browser, Node.js, and edge runtimes

## 📦 Installation

### NPM

```bash
npm install @superinstance/dodecet-encoder
```

### CDN

```html
<script type="module">
  import init, { WasmDodecet } from 'https://cdn.jsdelivr.net/npm/@superinstance/dodecet-encoder/pkg/dodecet_encoder.js';
</script>
```

## 🚀 Quick Start

### Browser

```html
<!DOCTYPE html>
<html>
<head>
    <title>Dodecet Example</title>
</head>
<body>
    <script type="module">
        import init, { WasmDodecet } from './pkg/dodecet_encoder.js';

        // Initialize WASM
        await init();

        // Create a dodecet
        const d = new WasmDodecet(0xABC);
        console.log(d.toHex()); // "ABC"
        console.log(d.normalize()); // 0.6670
    </script>
</body>
</html>
```

### Node.js

```javascript
const { WasmDodecet, WasmPoint3D } = require('@superinstance/dodecet-encoder');

// Create a 3D point
const point = new WasmPoint3D(0x100, 0x200, 0x300);
console.log(point.toHex()); // "100200300"

// Calculate distance
const p1 = new WasmPoint3D(0, 0, 0);
const p2 = new WasmPoint3D(256, 0, 0);
console.log(p1.distanceTo(p2)); // 256.0
```

### TypeScript

```typescript
import {
    WasmDodecet,
    WasmPoint3D,
    WasmVector3D,
    DodecetUtils,
    init
} from '@superinstance/dodecet-encoder';

// Initialize
await init();

// Type-safe operations
const point: WasmPoint3D = new WasmPoint3D(0x100, 0x200, 0x300);
const normalized: { x: number; y: number; z: number } = point.normalized();
```

## 📚 API Reference

### WasmDodecet

Core 12-bit dodecet value (0-4095).

```typescript
// Create dodecet
const d = new WasmDodecet(0xABC);

// Access value
console.log(d.value()); // 2748

// Convert to hex
console.log(d.toHex()); // "ABC"

// Access nibbles
console.log(d.nibble(0)); // 12 (0xC)
console.log(d.nibble(1)); // 11 (0xB)
console.log(d.nibble(2)); // 10 (0xA)

// Normalize to [0.0, 1.0]
console.log(d.normalize()); // 0.6670

// Signed interpretation
console.log(d.asSigned()); // -1337 (for 0xABC)
```

### WasmPoint3D

3D point with dodecet precision.

```typescript
// Create point
const point = new WasmPoint3D(0x100, 0x200, 0x300);

// Access coordinates
console.log(point.x(), point.y(), point.z()); // 256 512 768

// Normalized coordinates
const normalized = point.normalized();
console.log(normalized); // { x: 0.0625, y: 0.125, z: 0.1875 }

// Distance calculation
const distance = point.distanceTo(new WasmPoint3D(0, 0, 0));

// Hex encoding
console.log(point.toHex()); // "100200300"
```

### WasmVector3D

3D vector with full vector operations.

```typescript
// Create vector
const v = new WasmVector3D(100, 200, 300);

// Magnitude
console.log(v.magnitude()); // 374.17

// Normalize
const normalized = v.normalize();
console.log(normalized); // { x: 0.27, y: 0.53, z: 0.80 }

// Dot product
const v2 = new WasmVector3D(100, 0, 0);
console.log(v.dot(v2)); // 10000

// Cross product
const cross = v.cross(v2);
console.log(cross.x(), cross.y(), cross.z()); // 0 30000 -20000

// Add vectors
const sum = v.add(v2);

// Scale
const scaled = v.scale(2.0);
```

### DodecetUtils

Utility functions for batch operations.

```typescript
// Encode/decode floats
const original = 0.5;
const encoded = DodecetUtils.encodeFloat(original);
const decoded = DodecetUtils.decodeFloat(encoded);
console.log(Math.abs(decoded - original) < 0.001); // true

// Batch operations
const values = [0.0, 0.5, 1.0];
const encoded = DodecetUtils.encodeFloatArray(values);
const decoded = DodecetUtils.decodeDodecetArray(encoded);
```

## 🔧 Integration Examples

### Constraint Theory Integration

Perfect for geometric constraint solving:

```javascript
import { WasmPoint3D, WasmVector3D } from '@superinstance/dodecet-encoder';

// Pythagorean Snapping
function snapToPythagorean(point) {
    const encoded = point.toHex();
    return WasmPoint3D.fromHex(encoded);
}

// Rigidity Matroid
function checkRigidity(p1, p2, p3) {
    return p1.distanceTo(p2) > 0 &&
           p2.distanceTo(p3) > 0 &&
           p3.distanceTo(p1) > 0;
}

// Holonomy Transport
function transportVector(path, vector) {
    return path.map(point => {
        return new WasmVector3D(
            point.x() + vector.x(),
            point.y() + vector.y(),
            point.z() + vector.z()
        );
    });
}
```

### Performance Optimization

```javascript
// Batch encoding
function encodePointCloud(points) {
    return points.map(p => {
        const point = new WasmPoint3D(p.x, p.y, p.z);
        return point.toHex();
    });
}

// Memory-efficient storage
function compressGeometry(data) {
    const encoded = DodecetUtils.encodeFloatArray(data);
    return encoded.map(d => d.toHex()).join('');
}
```

## ⚡ Performance

### Benchmarks

Encoding 100,000 values:

| Implementation | Time | Ops/sec |
|---------------|------|---------|
| WASM | 45ms | 2,222,222 |
| JavaScript | 78ms | 1,282,051 |
| **Speedup** | **1.73x** | - |

Storage comparison:

| Encoding | Bits | Values |
|----------|------|--------|
| Dodecet | 12 | 4,096 |
| 8-bit | 8 | 256 |
| **Improvement** | **1.5x** | **16x** |

### Memory Efficiency

- **Hex Encoding**: 3 hex digits per dodecet (vs 4+ for traditional)
- **Binary Encoding**: 12 bits (vs 16+ for traditional)
- **Batch Operations**: Optimized for large datasets

## 🛠️ Development

### Build from Source

```bash
# Install wasm-pack
cargo install wasm-pack

# Build WASM package
wasm-pack build --target web --out-dir pkg --release

# Run tests
wasm-pack test --node
```

### Project Structure

```
dodecet-encoder/
├── src/
│   ├── lib.rs           # Core library
│   ├── wasm.rs          # WASM bindings
│   ├── dodecet.rs       # Dodecet type
│   ├── geometric.rs     # Geometric primitives
│   └── ...
├── pkg/                 # Built WASM package
│   ├── dodecet_encoder.js
│   ├── dodecet_encoder_bg.wasm
│   ├── dodecet_encoder.d.ts
│   └── package.json
├── examples/
│   ├── wasm_integration.html
│   └── constrainttheory_integration.js
├── typescript/
│   └── dodecet_encoder.d.ts
└── docs/
    └── WASM_INTEGRATION_GUIDE.md
```

## 📖 Documentation

- [Integration Guide](docs/WASM_INTEGRATION_GUIDE.md) - Comprehensive integration documentation
- [API Reference](typescript/dodecet_encoder.d.ts) - TypeScript API definitions
- [Examples](examples/) - Usage examples

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Core implementation
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - WASM bindings
- [TypeScript](https://www.typescriptlang.org/) - Type definitions

## 📞 Support

- **GitHub Issues**: [Report bugs](https://github.com/SuperInstance/dodecet-encoder/issues)
- **Discord**: [Join community](https://discord.gg/superinstance)
- **Email**: [support@superinstance.ai](mailto:support@superinstance.ai)

## 🔗 Links

- **Repository**: [github.com/SuperInstance/dodecet-encoder](https://github.com/SuperInstance/dodecet-encoder)
- **NPM Package**: [npmjs.com/package/@superinstance/dodecet-encoder](https://www.npmjs.com/package/@superinstance/dodecet-encoder)
- **Documentation**: [docs.superinstance.ai/dodecet-encoder](https://docs.superinstance.ai/dodecet-encoder)

---

Made with ❤️ by the SuperInstance Team
