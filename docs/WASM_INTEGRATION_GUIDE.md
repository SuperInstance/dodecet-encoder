# Dodecet Encoder - WebAssembly Integration Guide

## Overview

The dodecet encoder now includes complete WebAssembly bindings for browser and Node.js environments. This enables JavaScript/TypeScript applications to leverage the high-performance 12-bit encoding system directly.

## Features

- **🚀 High Performance**: WASM-based operations for geometric calculations
- **📐 Geometric Primitives**: Point3D, Vector3D, Transform3D with dodecet precision
- **🔢 Hex-Friendly**: Natural hex encoding (3 hex digits per dodecet)
- **💾 Efficient Storage**: 16x more precision than 8-bit in compact format
- **🎯 TypeScript Support**: Full TypeScript definitions included
- **🌐 Browser & Node.js**: Works in any JavaScript environment

## Installation

### NPM Package

```bash
npm install @superinstance/dodecet-encoder
```

### Build from Source

```bash
# Install wasm-pack
cargo install wasm-pack

# Build WASM package
cd dodecet-encoder
wasm-pack build --target web --out-dir pkg
```

## Quick Start

### Browser Usage

```html
<!DOCTYPE html>
<html>
<head>
    <title>Dodecet Example</title>
</head>
<body>
    <script type="module">
        import init, { WasmDodecet, WasmPoint3D } from './pkg/dodecet_encoder.js';

        // Initialize WASM
        await init();

        // Create a dodecet
        const d = new WasmDodecet(0xABC);
        console.log(d.toHex()); // "ABC"

        // Create a 3D point
        const point = new WasmPoint3D(0x100, 0x200, 0x300);
        console.log(point.toHex()); // "100200300"
    </script>
</body>
</html>
```

### Node.js Usage

```javascript
const { WasmDodecet, WasmPoint3D, DodecetUtils } = require('@superinstance/dodecet-encoder');

// Create a dodecet
const d = new WasmDodecet(0xABC);
console.log(d.toHex()); // "ABC"

// Encode/decode floats
const original = 0.5;
const encoded = DodecetUtils.encodeFloat(original);
const decoded = DodecetUtils.decodeFloat(encoded);
console.log(Math.abs(decoded - original) < 0.001); // true
```

### TypeScript Usage

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

// Create dodecet
const d: WasmDodecet = new WasmDodecet(0xABC);
console.log(d.toHex()); // "ABC"

// Create point
const point: WasmPoint3D = new WasmPoint3D(0x100, 0x200, 0x300);
const normalized = point.normalized();
console.log(normalized); // { x: 0.0625, y: 0.125, z: 0.1875 }
```

## API Reference

### WasmDodecet

Core 12-bit dodecet value.

#### Constructor

```typescript
constructor(value: number)
```

Creates a new dodecet from a value (0-4095).

**Parameters:**
- `value`: Number between 0 and 4095

**Throws:** Error if value > 4095

#### Methods

##### `value(): number`

Get the raw value (0-4095).

```typescript
const d = new WasmDodecet(0xABC);
console.log(d.value()); // 2748
```

##### `nibble(index: number): number`

Get a specific nibble (0, 1, or 2).

**Parameters:**
- `index`: Nibble index (0 = LSB, 2 = MSB)

```typescript
const d = new WasmDodecet(0xABC);
console.log(d.nibble(0)); // 12 (0xC)
console.log(d.nibble(1)); // 11 (0xB)
console.log(d.nibble(2)); // 10 (0xA)
```

##### `setNibble(index: number, nibble: number): void`

Set a specific nibble.

**Parameters:**
- `index`: Nibble index (0 = LSB, 2 = MSB)
- `nibble`: New nibble value (0-15)

```typescript
const d = new WasmDodecet(0xABC);
d.setNibble(0, 0xD);
console.log(d.toHex()); // "ABD"
```

##### `toHex(): string`

Convert to hex string (3 characters).

```typescript
const d = new WasmDodecet(0xABC);
console.log(d.toHex()); // "ABC"
```

##### `static fromHex(s: string): WasmDodecet`

Parse from hex string.

```typescript
const d = WasmDodecet.fromHex("ABC");
console.log(d.value()); // 2748
```

##### `normalize(): number`

Normalize to floating point [0.0, 1.0].

```typescript
const d = new WasmDodecet(0x800);
console.log(d.normalize()); // 0.5
```

##### `asSigned(): number`

Geometric interpretation as signed value (-2048 to 2047).

```typescript
const d = new WasmDodecet(0x800);
console.log(d.asSigned()); // -2048
```

##### Bitwise Operations

```typescript
const a = new WasmDodecet(0xF0F);
const b = new WasmDodecet(0x0F0);

console.log(a.and(b).toHex());  // "000"
console.log(a.or(b).toHex());   // "FFF"
console.log(a.xor(b).toHex());  // "FFF"
console.log(a.not().toHex());   // "0F0"
```

##### Arithmetic Operations

```typescript
const a = new WasmDodecet(0x100);
const b = new WasmDodecet(0x200);

console.log(a.add(b).toHex());  // "300"
console.log(a.sub(b).toHex());  // "F00" (wrapping)
console.log(a.mul(b).value());  // 131072 & 0xFFF = 256
```

### WasmPoint3D

A 3D point encoded with dodecets.

#### Constructor

```typescript
constructor(x: number, y: number, z: number)
```

**Parameters:**
- `x`: X coordinate (0-4095)
- `y`: Y coordinate (0-4095)
- `z`: Z coordinate (0-4095)

#### Methods

##### `x(): number`, `y(): number`, `z(): number`

Get coordinates.

```typescript
const point = new WasmPoint3D(0x100, 0x200, 0x300);
console.log(point.x()); // 256
console.log(point.y()); // 512
console.log(point.z()); // 768
```

##### `normalized(): { x: number; y: number; z: number }`

Convert to normalized floating point coordinates [0.0, 1.0].

```typescript
const point = new WasmPoint3D(0x800, 0x800, 0x800);
const normalized = point.normalized();
console.log(normalized); // { x: 0.5, y: 0.5, z: 0.5 }
```

##### `signed(): { x: number; y: number; z: number }`

Convert to signed coordinates [-2048, 2047].

```typescript
const point = new WasmPoint3D(0x800, 0x000, 0x7FF);
const signed = point.signed();
console.log(signed); // { x: -2048, y: 0, z: 2047 }
```

##### `distanceTo(other: WasmPoint3D): number`

Calculate distance to another point.

```typescript
const p1 = new WasmPoint3D(0x000, 0x000, 0x000);
const p2 = new WasmPoint3D(0x100, 0x000, 0x000);
console.log(p1.distanceTo(p2)); // 256.0
```

##### `toHex(): string`

Convert to hex string.

```typescript
const point = new WasmPoint3D(0x123, 0x456, 0x789);
console.log(point.toHex()); // "123456789"
```

##### `static fromHex(s: string): WasmPoint3D`

Parse from hex string.

```typescript
const point = WasmPoint3D.fromHex("123456789");
console.log(point.x()); // 291
```

### WasmVector3D

A 3D vector encoded with dodecets.

#### Constructor

```typescript
constructor(x: number, y: number, z: number)
```

**Parameters:**
- `x`: X component (signed)
- `y`: Y component (signed)
- `z`: Z component (signed)

#### Methods

##### `magnitude(): number`

Calculate magnitude.

```typescript
const v = new WasmVector3D(100, 0, 0);
console.log(v.magnitude()); // 100.0
```

##### `normalize(): { x: number; y: number; z: number }`

Normalize to unit vector.

```typescript
const v = new WasmVector3D(100, 0, 0);
const normalized = v.normalize();
console.log(normalized); // { x: 1.0, y: 0.0, z: 0.0 }
```

##### `dot(other: WasmVector3D): number`

Dot product with another vector.

```typescript
const v1 = new WasmVector3D(100, 0, 0);
const v2 = new WasmVector3D(0, 100, 0);
console.log(v1.dot(v2)); // 0
```

##### `cross(other: WasmVector3D): WasmVector3D`

Cross product with another vector.

```typescript
const v1 = new WasmVector3D(100, 0, 0);
const v2 = new WasmVector3D(0, 100, 0);
const cross = v1.cross(v2);
console.log(cross.z()); // 10000
```

##### `add(other: WasmVector3D): WasmVector3D`

Add two vectors.

```typescript
const v1 = new WasmVector3D(100, 0, 0);
const v2 = new WasmVector3D(0, 100, 0);
const sum = v1.add(v2);
console.log(sum.x(), sum.y()); // 100, 100
```

##### `scale(scalar: number): WasmVector3D`

Scale by a scalar.

```typescript
const v = new WasmVector3D(100, 100, 100);
const scaled = v.scale(2.0);
console.log(scaled.x()); // 200
```

### DodecetUtils

Utility functions for dodecet operations.

#### Static Methods

##### `encodeFloat(value: number): WasmDodecet`

Encode a floating point value [0.0, 1.0] to dodecet.

```typescript
const encoded = DodecetUtils.encodeFloat(0.5);
console.log(encoded.value()); // 2048
```

##### `decodeFloat(dodecet: WasmDodecet): number`

Decode a dodecet to floating point [0.0, 1.0].

```typescript
const d = new WasmDodecet(0x800);
const decoded = DodecetUtils.decodeFloat(d);
console.log(decoded); // 0.5
```

##### `encodeFloatArray(values: number[]): WasmDodecet[]`

Batch encode an array of floating point values.

```typescript
const values = [0.0, 0.5, 1.0];
const encoded = DodecetUtils.encodeFloatArray(values);
console.log(encoded.length); // 3
```

##### `decodeDodecetArray(dodecets: WasmDodecet[]): number[]`

Batch decode an array of dodecets.

```typescript
const dodecets = [
    new WasmDodecet(0x000),
    new WasmDodecet(0x800),
    new WasmDodecet(0xFFF)
];
const decoded = DodecetUtils.decodeDodecetArray(dodecets);
console.log(decoded); // [0.0, 0.5, 1.0]
```

## Integration Examples

### Constraint Theory Integration

```javascript
import { WasmPoint3D, WasmVector3D } from '@superinstance/dodecet-encoder';

// Pythagorean Snapping
function snapToPythagorean(point) {
    const encoded = point.toHex();
    const snapped = WasmPoint3D.fromHex(encoded);
    return snapped;
}

// Rigidity Matroid
function checkRigidity(p1, p2, p3) {
    const d12 = p1.distanceTo(p2);
    const d23 = p2.distanceTo(p3);
    const d31 = p3.distanceTo(p1);
    return d12 > 0 && d23 > 0 && d31 > 0;
}

// Holonomy Transport
function transportVector(path, vector) {
    return path.map(point => {
        const v = new WasmVector3D(
            point.x() + vector.x(),
            point.y() + vector.y(),
            point.z() + vector.z()
        );
        return v;
    });
}
```

### Performance Optimization

```javascript
// Batch operations
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

## Performance Considerations

### WASM vs JavaScript

The WASM implementation provides significant performance benefits for:

- **Geometric calculations**: Vector operations, transformations
- **Batch operations**: Encoding/decoding large datasets
- **Numerical computations**: Distance calculations, normalization

### Memory Efficiency

- **12-bit precision**: 16x more precise than 8-bit
- **Hex encoding**: Natural 3-digit hex representation
- **Compact storage**: Efficient for large geometric datasets

### Browser Optimization

```javascript
// Use SharedArrayBuffer for parallel processing
const sharedBuffer = new SharedArrayBuffer(4096);
// ... integrate with Web Workers
```

## Troubleshooting

### Common Issues

#### WASM Module Loading

**Problem**: WASM module fails to load

**Solution**: Ensure proper MIME type configuration for `.wasm` files:

```nginx
# nginx.conf
location ~ \.wasm$ {
    default_type application/wasm;
}
```

#### Memory Issues

**Problem**: Out of memory errors with large datasets

**Solution**: Process data in batches:

```javascript
async function processLargeDataset(data, batchSize = 1000) {
    for (let i = 0; i < data.length; i += batchSize) {
        const batch = data.slice(i, i + batchSize);
        await processBatch(batch);
    }
}
```

#### Precision Loss

**Problem**: Unexpected precision loss

**Solution**: Remember dodecet has 12-bit precision (4096 values):

```javascript
const original = 0.123456789;
const encoded = DodecetUtils.encodeFloat(original);
const decoded = DodecetUtils.decodeFloat(encoded);
const error = Math.abs(decoded - original);
console.log(`Error: ${error}`); // Expected: < 0.001
```

## Best Practices

1. **Initialize Once**: Call `init()` once at application startup
2. **Reuse Objects**: Reuse dodecet objects instead of creating new ones
3. **Batch Operations**: Use batch methods for large datasets
4. **Error Handling**: Always handle potential overflow errors
5. **Type Safety**: Use TypeScript for better type checking

## Resources

- **GitHub Repository**: https://github.com/SuperInstance/dodecet-encoder
- **Documentation**: https://docs.superinstance.ai/dodecet-encoder
- **Examples**: `/examples` directory
- **TypeScript Definitions**: `typescript/dodecet_encoder.d.ts`

## Support

For issues, questions, or contributions:
- GitHub Issues: https://github.com/SuperInstance/dodecet-encoder/issues
- Discord: https://discord.gg/superinstance
- Email: support@superinstance.ai
