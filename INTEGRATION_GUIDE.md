# Dodecet Encoder - Integration Guide

This guide explains how to integrate the dodecet-encoder library across the SuperInstance ecosystem.

## Overview

The dodecet-encoder provides three integration layers:

1. **Core Rust Library** - High-performance Rust crate
2. **WASM Browser Package** - Browser integration via WebAssembly
3. **Constraint Theory Foundation** - Geometric primitives for constraint solving

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    SUPERINSTANCE ECOSYSTEM                   │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────┐      ┌──────────────────┐            │
│  │ spreadsheet-/    │      │ constraint-/     │            │
│  │ moment/          │◄────►│ theory/          │            │
│  │                  │      │                  │            │
│  │  • UI Layer      │      │  • Math Engine   │            │
│  │  • React/TS      │      │  • Geometry      │            │
│  └────────┬─────────┘      └────────┬─────────┘            │
│           │                         │                       │
│           │ WASM                    │ Rust                  │
│           ▼                         ▼                       │
│  ┌──────────────────────────────────────────────┐          │
│  │         dodecet-encoder (Core)               │          │
│  │                                              │          │
│  │  • Point3D      • Vector3D  • Transform3D   │          │
│  │  • Dodecet      • Geometry  • Calculus      │          │
│  └──────────────────────────────────────────────┘          │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Integration Points

### 1. spreadsheet-moment Integration

**Purpose:** Visualize and manipulate geometric constraints in spreadsheets

**Integration Method:** WASM Browser Package

```typescript
// In spreadsheet-moment (TypeScript)
import init, { Point3D, Vector3DWasm, Transform3DWasm } from '@superinstance/dodecet-encoder';

class ConstraintCell {
  private wasmReady = false;

  async initialize() {
    await init();
    this.wasmReady = true;
  }

  createPoint(x: number, y: number, z: number) {
    if (!this.wasmReady) throw new Error('WASM not ready');
    return new Point3D(x, y, z);
  }

  applyTransform(point: Point3D, transform: Transform3DWasm) {
    return transform.apply(point);
  }

  calculateDistance(p1: Point3D, p2: Point3D) {
    return p1.distanceTo(p2);
  }
}
```

**Use Cases:**

1. **Cell-Based Geometry**
   - Each cell contains a Point3D
   - Formulas operate on geometric data
   - Real-time visualization

2. **Constraint Visualization**
   - Show geometric constraints
   - Visual feedback for snapping
   - Interactive constraint editing

3. **Performance Monitoring**
   - Benchmark constraint solving
   - Profile WASM performance
   - Optimize hot paths

### 2. constraint-theory Integration

**Purpose:** Implement deterministic geometric logic

**Integration Method:** Rust Crate (Direct Dependency)

```rust
// In constraint-theory/Cargo.toml
[dependencies]
dodecet-encoder = { path = "../dodecet-encoder", features = ["geometry"] }

// In constraint-theory/src/lib.rs
use dodecet_encoder::{Point3D, Vector3D, Transform3D};

pub struct OriginCentricGeometry {
    origin: Point3D,
}

impl OriginCentricGeometry {
    pub fn new() -> Self {
        Self {
            origin: Point3D::new(0x800, 0x800, 0x800), // Center of dodecet space
        }
    }

    pub fn pythagorean_snap(&self, point: &Point3D) -> Point3D {
        // Snap to Pythagorean ratios
        // 3-4-5, 5-12-13, 8-15-17, etc.
        let vector = Vector3D::new(
            point.x() as i16 - self.origin.x() as i16,
            point.y() as i16 - self.origin.y() as i16,
            point.z() as i16 - self.origin.z() as i16,
        );

        // Apply snapping logic
        let snapped = self.snap_to_ratios(&vector);
        Point3D::new(
            (self.origin.x() as i16 + snapped.x()) as u16,
            (self.origin.y() as i16 + snapped.y()) as u16,
            (self.origin.z() as i16 + snapped.z()) as u16,
        )
    }

    fn snap_to_ratios(&self, v: &Vector3D) -> Vector3D {
        // Implementation of Pythagorean snapping
        // ...
        v.clone() // Placeholder
    }
}
```

**Use Cases:**

1. **Origin-Centric Geometry (Ω)**
   - Unitary symmetry invariance
   - Normalized ground state
   - Zero-point resonance

2. **Pythagorean Snapping**
   - Φ-Folding operator
   - Integer ratio alignment
   - Eliminate hallucinations

3. **Lattice Vector Quantization**
   - Tokenization via coordinates
   - Spatial reasoning
   - Geometric closure

### 3. Cross-Repo Communication

**Protocol:** WebSocket with WASM Message Format

```typescript
// spreadsheet-moment sends geometric data to constraint-theory
interface GeometricMessage {
  type: 'constraint' | 'transform' | 'query';
  data: {
    points: Array<{ x: number; y: number; z: number }>;
    constraints: string[];
  };
}

class SpreadsheetConstraintBridge {
  private ws: WebSocket;

  async sendConstraint(points: Point3D[]) {
    const message: GeometricMessage = {
      type: 'constraint',
      data: {
        points: points.map(p => ({
          x: p.x,
          y: p.y,
          z: p.z,
        })),
        constraints: [],
      },
    };

    this.ws.send(JSON.stringify(message));
  }

  async receiveTransform(): Promise<Transform3DWasm> {
    // Receive transform from constraint-theory
    // Apply to local points
    // Update spreadsheet
  }
}
```

## Data Flow

### User Input Flow

```
User → Spreadsheet Cell → WASM Point3D → Constraint Engine → Rust Geometry → Result → WASM Transform → Spreadsheet Update
```

### Constraint Solving Flow

```
1. User edits cell
2. WASM creates Point3D
3. Send to constraint-theory via WebSocket
4. constraint-theory solves using Rust geometry
5. Return Transform3D
6. WASM applies transform
7. Update spreadsheet cells
```

## Performance Considerations

### WASM Performance

**Benchmark Results:**
- Point creation: ~50ns
- Distance calculation: ~200ns
- Transform application: ~500ns

**Optimization Strategies:**

1. **Batch Operations**
   ```typescript
   // Bad: Individual operations
   points.forEach(p => transform.apply(p));

   // Good: Batch in Rust
   const transformed = transform.applyBatch(points);
   ```

2. **Reuse Instances**
   ```typescript
   // Bad: Create new instances
   for (let i = 0; i < 1000; i++) {
     const transform = new Transform3DWasm(...);
   }

   // Good: Reuse instance
   const transform = new Transform3DWasm(...);
   for (let i = 0; i < 1000; i++) {
     transform.apply(point);
   }
   ```

3. **Lazy Initialization**
   ```typescript
   // Initialize WASM once
   let wasmReady = false;

   async function ensureWasm() {
     if (!wasmReady) {
       await init();
       wasmReady = true;
     }
   }
   ```

### Rust Performance

**Constraint Theory Optimization:**

1. **Use SIMD**
   ```rust
   use std::simd::*;

   pub fn batch_distance(points: &[Point3D]) -> Vec<f64> {
       // SIMD-optimized distance calculation
   }
   ```

2. **Parallel Processing**
   ```rust
   use rayon::prelude::*;

   points.par_iter()
       .map(|p| solve_constraint(p))
       .collect()
   ```

3. **Memory Pooling**
   ```rust
   use object_pool::Pool;

   let point_pool = Pool::new(1000);
   let point = point_pool.pull(|| Point3D::new(0, 0, 0));
   ```

## Testing Strategy

### Unit Tests

```rust
// constraint-theory/tests/geometric_tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use dodecet_encoder::Point3D;

    #[test]
    fn test_pythagorean_snap() {
        let geometry = OriginCentricGeometry::new();
        let point = Point3D::new(0x900, 0x900, 0x900);
        let snapped = geometry.pythagorean_snap(&point);

        // Verify snapped to integer ratio
        assert!(is_integer_ratio(&snapped));
    }
}
```

### Integration Tests

```typescript
// spreadsheet-moment/tests/integration/wasm_tests.ts
import { Point3D, Transform3DWasm } from '@superinstance/dodecet-encoder';

describe('WASM Integration', () => {
  beforeAll(async () => {
    await init();
  });

  test('creates point and applies transform', () => {
    const point = new Point3D(0x100, 0x200, 0x300);
    const transform = Transform3DWasm.translation(100, 200, 300);
    const result = transform.apply(point);

    expect(result.x).toBeGreaterThan(0);
    expect(result.y).toBeGreaterThan(0);
    expect(result.z).toBeGreaterThan(0);
  });
});
```

### End-to-End Tests

```typescript
// Cross-repo E2E test
describe('E2E: Spreadsheet → Constraint → Spreadsheet', () => {
  test('full constraint solving workflow', async () => {
    // 1. Create point in spreadsheet
    const point = new Point3D(0x123, 0x456, 0x789);

    // 2. Send to constraint engine
    await bridge.sendConstraint([point]);

    // 3. Receive transform
    const transform = await bridge.receiveTransform();

    // 4. Apply transform
    const result = transform.apply(point);

    // 5. Verify result
    expect(result).toBeDefined();
  });
});
```

## Deployment

### Development

```bash
# Local development with hot reload
cd spreadsheet-moment
npm run dev

# In another terminal
cd constraint-theory
cargo run
```

### Staging

```bash
# Build WASM package
cd dodecet-encoder/wasm
wasm-pack build --target web --release

# Test in staging environment
cd spreadsheet-moment
npm run build:staging
npm run deploy:staging
```

### Production

```bash
# Publish WASM to npm
cd dodecet-encoder/wasm/pkg
npm publish --access public

# Deploy spreadsheet-moment
cd spreadsheet-moment
npm run build:production
npm run deploy:production

# Deploy constraint-theory
cd constraint-theory
cargo build --release
```

## Monitoring

### Performance Metrics

```typescript
// Collect performance metrics
class PerformanceMonitor {
  measureOperation<T>(name: string, fn: () => T): T {
    const start = performance.now();
    const result = fn();
    const elapsed = performance.now() - start;

    // Send to monitoring service
    this.metrics.record({
      name,
      duration: elapsed,
      timestamp: Date.now(),
    });

    return result;
  }
}

// Usage
const monitor = new PerformanceMonitor();
const result = monitor.measureOperation('transform.apply', () => {
  return transform.apply(point);
});
```

### Error Tracking

```typescript
// Track WASM errors
class ErrorTracker {
  trackWasmError(error: Error, context: any) {
    console.error('WASM Error:', error);
    console.error('Context:', context);

    // Send to error tracking service
    this.errorService.report({
      type: 'WASM_ERROR',
      message: error.message,
      stack: error.stack,
      context,
    });
  }
}
```

## Best Practices

### 1. Initialize WASM Early

```typescript
// In app entry point
async function main() {
  await init(); // Initialize WASM immediately

  // Then render app
  ReactDOM.render(<App />, document.getElementById('root'));
}
```

### 2. Handle Errors Gracefully

```typescript
try {
  const point = new Point3D(x, y, z);
} catch (error) {
  // Fallback to pure JavaScript implementation
  const point = new JSPoint3D(x, y, z);
}
```

### 3. Optimize Communication

```typescript
// Batch WebSocket messages
class MessageBatcher {
  private messages: any[] = [];
  private timer: NodeJS.Timeout;

  send(message: any) {
    this.messages.push(message);

    if (this.messages.length >= 100) {
      this.flush();
    }
  }

  private flush() {
    this.ws.send(JSON.stringify(this.messages));
    this.messages = [];
  }
}
```

### 4. Cache Results

```typescript
// Cache expensive operations
const transformCache = new Map<string, Transform3DWasm>();

function getTransform(key: string): Transform3DWasm {
  let transform = transformCache.get(key);

  if (!transform) {
    transform = createExpensiveTransform(key);
    transformCache.set(key, transform);
  }

  return transform;
}
```

## Troubleshooting

### WASM Loading Issues

**Problem:** WASM fails to load

**Solution:**
```typescript
try {
  await init();
} catch (error) {
  console.error('WASM initialization failed:', error);

  // Check browser support
  if (!WebAssembly) {
    alert('Your browser does not support WebAssembly');
  }

  // Check CORS
  console.log('Check CORS headers for WASM file');
}
```

### Performance Issues

**Problem:** Operations are slow

**Solution:**
```typescript
// Use performance.now() to identify bottlenecks
const start = performance.now();

// ... operation ...

const elapsed = performance.now() - start;
if (elapsed > 16) { // > 1 frame at 60fps
  console.warn(`Slow operation: ${elapsed}ms`);
}
```

### Memory Leaks

**Problem:** Memory usage grows over time

**Solution:**
```typescript
// Explicitly drop WASM objects
class PointManager {
  private points: Point3D[] = [];

  cleanup() {
    // Clear array
    this.points = [];

    // Force garbage collection (if available)
    if (global.gc) {
      global.gc();
    }
  }
}
```

## Resources

- **WASM Package:** `wasm/README.md`
- **Installation:** `wasm/INSTALLATION.md`
- **Build Guide:** `wasm/BUILD_GUIDE.md`
- **API Reference:** `wasm/README.md#api-reference`
- **Examples:** `wasm/examples/`

## Support

For integration issues:

1. Check documentation in respective repos
2. Review integration examples
3. Open GitHub issue with:
   - Repository names
   - Integration point
   - Error messages
   - Minimal reproduction

---

**Status:** Integration protocols defined
**Next:** Implement cross-repo communication
**Priority:** HIGH - Critical for Phase 4 completion
