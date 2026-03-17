# Dodecet Encoder WASM - Quick Reference

## Installation

```bash
npm install @superinstance/dodecet-encoder
```

## Initialization

```javascript
import init from '@superinstance/dodecet-encoder';
await init();
```

## Point3D

### Create
```javascript
import { Point3D } from '@superinstance/dodecet-encoder';

const p = new Point3D(x, y, z);        // 0-4095
const p = Point3D.fromHex("123 456 789");
```

### Properties
```javascript
p.x, p.y, p.z                          // Coordinates
```

### Methods
```javascript
p.toHex()                              // "123 456 789"
p.normalized()                         // [nx, ny, nz] → [0,1]
p.signed()                             // [sx, sy, sz] → [-2048,2047]
p.distanceTo(other)                    // Euclidean distance
```

## Vector3DWasm

### Create
```javascript
import { Vector3DWasm } from '@superinstance/dodecet-encoder';

const v = new Vector3DWasm(x, y, z);   // Signed
```

### Properties
```javascript
v.x, v.y, v.z                          // Components
```

### Methods
```javascript
v.magnitude()                          // Length
v.normalize()                          // [nx, ny, nz] → unit
v.dot(other)                           // Dot product
v.cross(other)                         // Cross product
v.add(other)                           // Addition
v.sub(other)                           // Subtraction
v.scale(scalar)                        // Multiplication
```

## Transform3DWasm

### Create
```javascript
import { Transform3DWasm } from '@superinstance/dodecet-encoder';

const t = new Transform3DWasm();                                    // Identity
const t = Transform3DWasm.translation(dx, dy, dz);                 // Translate
const t = Transform3DWasm.scale(sx, sy, sz);                       // Scale
const t = Transform3DWasm.rotationX(angle);                        // Rotate X
const t = Transform3DWasm.rotationY(angle);                        // Rotate Y
const t = Transform3DWasm.rotationZ(angle);                        // Rotate Z
```

### Methods
```javascript
t.apply(point)                        // Transform point
t.compose(other)                      // Combine transforms
```

## Utility Functions

```javascript
import { maxDodecet, dodecetBits, dodecetCapacity } from '@superinstance/dodecet-encoder';

maxDodecet()                          // 4095
dodecetBits()                          // 12
dodecetCapacity()                      // 4096
```

## Common Patterns

### Distance Between Points
```javascript
const p1 = new Point3D(0, 0, 0);
const p2 = new Point3D(0x100, 0, 0);
const dist = p1.distanceTo(p2);        // 256.0
```

### Vector Operations
```javascript
const v1 = new Vector3DWasm(100, 0, 0);
const v2 = new Vector3DWasm(0, 100, 0);
const dot = v1.dot(v2);                // 0 (perpendicular)
const cross = v1.cross(v2);            // (0, 0, 10000)
```

### Transformations
```javascript
const point = new Point3D(0x100, 0x200, 0x300);
const transform = Transform3DWasm.translation(100, 200, 300);
const result = transform.apply(point);
```

### Chain Transforms
```javascript
const t1 = Transform3DWasm.translation(100, 0, 0);
const t2 = Transform3DWasm.rotationY(90);
const t3 = Transform3DWasm.scale(2.0, 2.0, 2.0);
const combined = t1.compose(t2).compose(t3);
const result = combined.apply(point);
```

## Hex String Operations

### To Hex
```javascript
const point = new Point3D(0x123, 0x456, 0x789);
const hex = point.toHex();              // "123 456 789"
```

### From Hex
```javascript
const point = Point3D.fromHex("ABC DEF 123");
// point.x = 0xABC, point.y = 0xDEF, point.z = 0x123
```

### Case Insensitive
```javascript
Point3D.fromHex("abc def 123");         // Works
Point3D.fromHex("AbC DeF 123");         // Works
Point3D.fromHex("ABC DEF 123");         // Works
```

## Coordinate Systems

### Unsigned (0-4095)
```javascript
const point = new Point3D(0x800, 0x800, 0x800);
point.x, point.y, point.z               // 2048, 2048, 2048
```

### Normalized (0.0-1.0)
```javascript
const [nx, ny, nz] = point.normalized();
// nx, ny, nz ≈ 0.5
```

### Signed (-2048 to 2047)
```javascript
const [sx, sy, sz] = point.signed();
// sx, sy, sz = 0, 0, 0 (center is 0x800)
```

## Performance Tips

### Batch Operations
```javascript
// Bad: Individual operations
points.forEach(p => transform.apply(p));

// Good: Reuse transform
const result = points.map(p => transform.apply(p));
```

### Reuse Objects
```javascript
// Bad: Create new instances
for (let i = 0; i < 1000; i++) {
  const t = new Transform3DWasm();
}

// Good: Reuse instance
const t = new Transform3DWasm();
for (let i = 0; i < 1000; i++) {
  t.apply(point);
}
```

### Initialize Once
```javascript
// Initialize WASM immediately
await init();

// Then use throughout app
const point = new Point3D(0x100, 0x200, 0x300);
```

## Error Handling

### Try-Catch
```javascript
try {
  const point = Point3D.fromHex(invalidHex);
} catch (error) {
  console.error('Invalid hex:', error);
}
```

### Validation
```javascript
function validatePoint(x, y, z) {
  if (x < 0 || x > 0xFFF) throw new Error('Invalid X');
  if (y < 0 || y > 0xFFF) throw new Error('Invalid Y');
  if (z < 0 || z > 0xFFF) throw new Error('Invalid Z');
}

validatePoint(0x123, 0x456, 0x789);
const point = new Point3D(0x123, 0x456, 0x789);
```

## Framework Examples

### React
```jsx
import init, { Point3D } from '@superinstance/dodecet-encoder';

function App() {
  useEffect(() => { init(); }, []);
  const point = new Point3D(0x100, 0x200, 0x300);
  return <div>{point.toHex()}</div>;
}
```

### Vue
```vue
<script setup>
import { onMounted } from 'vue';
import init, { Point3D } from '@superinstance/dodecet-encoder';

let point;
onMounted(async () => {
  await init();
  point = new Point3D(0x100, 0x200, 0x300);
});
</script>
```

### Svelte
```svelte
<script>
import { onMount } from 'svelte';
import init, { Point3D } from '@superinstance/dodecet-encoder';

let point;
onMount(async () => {
  await init();
  point = new Point3D(0x100, 0x200, 0x300);
});
</script>
```

## Constants

```javascript
MAX_DODECET = 4095                      // Maximum value
DODECET_BITS = 12                       // Bits per dodecet
CAPACITY = 4096                         // Total capacity
```

## Hex Format

```javascript
// Format: "XXX YYY ZZZ"
// Each X, Y, Z is 3 hex digits
// Example: "123 456 789"
// Spaces are required
// Case insensitive
```

## Quick Cheatsheet

| Operation | Code |
|-----------|------|
| Create point | `new Point3D(x, y, z)` |
| From hex | `Point3D.fromHex("123 456 789")` |
| To hex | `point.toHex()` |
| Distance | `p1.distanceTo(p2)` |
| Normalize | `point.normalized()` |
| Signed | `point.signed()` |
| Vector dot | `v1.dot(v2)` |
| Vector cross | `v1.cross(v2)` |
| Translate | `Transform3DWasm.translation(dx, dy, dz)` |
| Rotate | `Transform3DWasm.rotationY(90)` |
| Scale | `Transform3DWasm.scale(2, 2, 2)` |
| Apply | `transform.apply(point)` |
| Compose | `t1.compose(t2)` |

## Browser Support

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## TypeScript

```typescript
import init, {
  Point3D,
  Vector3DWasm,
  Transform3DWasm
} from '@superinstance/dodecet-encoder';

await init();

const point: Point3D = new Point3D(0x123, 0x456, 0x789);
const vector: Vector3DWasm = new Vector3DWasm(100, 200, 300);
const transform: Transform3DWasm = Transform3DWasm.translation(100, 200, 300);
```

## Build & Test

```bash
# Build
cd wasm
wasm-pack build --target web --release

# Test
wasm-pack test --node

# Publish
cd pkg
npm publish --access public
```

## Resources

- Full docs: `wasm/README.md`
- Installation: `wasm/INSTALLATION.md`
- Build guide: `wasm/BUILD_GUIDE.md`
- Integration: `INTEGRATION_GUIDE.md`
- Examples: `wasm/examples/`

---

**Status:** ✅ Production Ready
**Version:** 1.0.0
**Package:** @superinstance/dodecet-encoder
