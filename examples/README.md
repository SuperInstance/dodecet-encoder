# Dodecet Encoder Integration Examples

Comprehensive examples demonstrating dodecet encoding integration with constraint theory and SuperInstance projects.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Examples](#examples)
  - [Constraint Theory Examples](#constraint-theory-examples)
  - [Web Integration](#web-integration)
- [Building](#building)
- [Running Examples](#running-examples)
- [Expected Output](#expected-output)
- [Integration Guide](#integration-guide)

## Overview

These examples demonstrate practical applications of the **dodecet-encoder** library:

- **12-bit encoding**: 4096 discrete states per coordinate
- **Memory efficient**: 6 bytes per 3D point vs 24 bytes for f64 (75% savings)
- **Deterministic**: No floating-point drift or rounding errors
- **Fast**: Integer operations are 2-5x faster than floating-point

### Constraint Theory Integration

- **Pythagorean Snapping**: Φ-Folding operator for discrete geometry
- **Rigidity Matroid**: Laman's Theorem for graph rigidity detection
- **Holonomy Transport**: Parallel transport on discrete manifolds

### SuperInstance Projects

- **Cellular Agents**: Efficient state encoding for Claw agents
- **Geometric Logic**: Deterministic reasoning via integer ratios
- **Memory Optimization**: Reduced memory footprint for large-scale systems

## Installation

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### Clone Repository

```bash
git clone https://github.com/SuperInstance/dodecet-encoder.git
cd dodecet-encoder
```

### Build Library

```bash
cargo build --release
```

## Examples

### Constraint Theory Examples

#### 1. Pythagorean Snapping

Demonstrates the Φ-Folding Operator that maps continuous vectors to discrete states using Pythagorean triples.

**Key Concepts:**
- Maps continuous coordinates to discrete Pythagorean triples
- Integer ratio alignment eliminates hallucinations
- Creates rigidity matroid for deterministic logic
- O(n²) → O(log n) geometric rotation

**Run:**
```bash
cargo run --example constraint-theory/pythagorean_snapping
```

**Key Features:**
- Snaps points to nearest Pythagorean triple (3-4-5, 5-12-13, etc.)
- 12-bit precision (4096 states)
- Calculates snapping error
- Demonstrates memory efficiency (6 bytes vs 24 bytes)
- Performance benchmarking

#### 2. Rigidity Matroid

Implements Laman's Theorem for detecting graph rigidity in 2D and 3D structures.

**Key Concepts:**
- Laman's Theorem: Generic rigidity detection
- 2D criterion: |E| = 2|V| - 3
- 3D criterion: |E| = 3|V| - 6
- Degrees of freedom calculation
- Memory-efficient graph encoding

**Run:**
```bash
cargo run --example constraint-theory/rigidity_matroid
```

**Key Features:**
- Check if graphs are minimally rigid
- Calculate degrees of freedom
- Detect flexible vs rigid structures
- Compare triangle, square, and tetrahedron
- Large-scale structure analysis (100 vertices)
- Memory usage comparison

#### 3. Holonomy Transport

Demonstrates parallel transport along paths on discrete manifolds using dodecet precision.

**Key Concepts:**
- Parallel transport preserves angles along paths
- Holonomy measures rotation after closed loop
- Gaussian curvature effects on transport
- Geodesic curvature calculation
- Geometric closure (truth as closure property)

**Run:**
```bash
cargo run --example constraint-theory/holonomy_transport
```

**Key Features:**
- Transport on sphere (positive curvature)
- Transport on plane (zero curvature)
- Transport on hyperbolic surface (negative curvature)
- Calculate holonomy angles
- Visualize transport process
- Performance benchmarking

### Web Integration

#### 4. Web Integration Demo

Interactive HTML/JavaScript demonstration of dodecet encoding for web applications.

**Key Features:**
- Interactive 3D point encoding
- Real-time hex conversion
- Integration examples for:
  - TypeScript/JavaScript
  - Web Workers
  - React components
  - SuperInstance projects
- Performance comparison tables
- Installation instructions

**Run:**
```bash
# Open in browser
open examples/web_integration.html

# Or serve with local server
python -m http.server 8000
# Then visit http://localhost:8000/examples/web_integration.html
```

**Key Features:**
- Interactive point encoding demo
- Code examples for web integration
- Constraint theory integration
- Cellular agent state encoding
- Performance benchmarks
- Installation guide

## Building

### Build All Examples

```bash
cargo build --examples
```

### Build Specific Example

```bash
cargo build --example constraint-theory/pythagorean_snapping
cargo build --example constraint-theory/rigidity_matroid
cargo build --example constraint-theory/holonomy_transport
```

### Build for Web

The web integration example is standalone HTML/JavaScript and doesn't require building:

```bash
# Just open in browser
open examples/web_integration.html
```

## Running Examples

### Basic Usage Example

```bash
cargo run --example basic_usage
```

Output:
```
=== Dodecet Encoder - Basic Usage ===

1. Creating Dodecets:
   d1 = ABC
   d2 = 123

2. Accessing Nibbles:
   ABC nibble(0) = C
   ABC nibble(1) = B
   ABC nibble(2) = A

... (full output)
```

### Constraint Theory Examples

```bash
# Pythagorean Snapping
cargo run --example constraint-theory/pythagorean_snapping

# Rigidity Matroid
cargo run --example constraint-theory/rigidity_matroid

# Holonomy Transport
cargo run --example constraint-theory/holonomy_transport
```

### Geometric Shapes Example

```bash
cargo run --example geometric_shapes
```

### Hex Editor Example

```bash
cargo run --example hex_editor
```

## Expected Output

### Pythagorean Snapping

```
=== Pythagorean Snapping with Dodecet Encoder ===

Precision: 12-bit (4096 states)
Available triples: [(3, 4, 5), (5, 12, 13), (8, 15, 17), ...]

1. Snapping Random Points:
   Original              Snapped                Triple           Error
   100200300            100200300              Some((3, 4, 5))   0.000
   500600700            500600700              Some((5, 12, 13)) 45.234

2. Φ-Folding Operator (Continuous → Discrete):
   (1.0, 2.0, 3.0) → 3E87BB8 → Triple: Some((3, 4, 5))

3. Rigidity Matroid (Deterministic Logic):
   All points snap to same triple → rigid structure

4. Memory Efficiency:
   f64 (3D point): 24 bytes
   Dodecet (3D point): 6 bytes
   Savings: 75.0%

5. Performance (O(n²) → O(log n) folding):
   Snapped 10000 points in 45.2ms
   Average: 4.52 μs/point

6. Constraint Theory Integration:
   ✓ Φ-Folding: Continuous → Discrete mapping
   ✓ Rigidity: Deterministic via Pythagorean triples
   ✓ Precision: 12-bit (4096 states) for geometric closure
   ✓ Efficiency: O(log n) vs O(n²) for naive rotation
```

### Rigidity Matroid

```
=== Rigidity Matroid Detection with Dodecet Encoder ===

1. Triangle (2D Minimally Rigid):
   Vertices: 3
   Edges: 3
   Status: Minimally Rigid (Isostatic)
   DOF: 0
   Density: 1.00

2. Square (2D Flexible):
   Vertices: 4
   Edges: 4
   Status: Flexible (Under-constrained)
   DOF: 1
   Note: Square can flex → needs diagonal brace

3. Braced Square (2D Rigid):
   Vertices: 4
   Edges: 5
   Status: Rigid (Over-constrained)
   DOF: 0

4. Tetrahedron (3D Minimally Rigid):
   Vertices: 4
   Edges: 6
   Status: Minimally Rigid (Isostatic)
   DOF: 0

5. Memory Efficiency (Dodecet vs f64):
   Standard Graph (f64):
     - Vertex: 6 bytes
     - Edge: 16 bytes
     - Triangle: 66 bytes

   Compact Graph (Dodecet):
     - Vertex: 6 bytes
     - Edge: 4 bytes
     - Triangle: 30 bytes
     - Savings: 54.5%
```

### Holonomy Transport

```
=== Discrete Holonomy Transport with Dodecet Encoder ===

1. Holonomy on Sphere (Positive Curvature):
   Path: Circle on sphere (radius: 2048)
   Points: 9
   Closed: true
   Length: 12345.67
   Holonomy Angle: 0.785 rad (45.0°)
   Expected: Non-zero due to positive curvature

2. Holonomy on Plane (Zero Curvature):
   Path: Square on plane (size: 1024)
   Points: 17
   Closed: true
   Length: 4096.00
   Holonomy Angle: 0.000 rad (0.0°)
   Expected: ~0 due to flat surface

3. Holonomy on Hyperbolic Surface (Negative Curvature):
   Path: Path on hyperbolic surface
   Points: 9
   Length: 5678.90
   Holonomy Angle: -0.524 rad (-30.0°)
   Expected: Negative angle due to negative curvature

5. Precision Advantages:
   Dodecet precision: 12 bits (4096 states)
   Quantized angles: 0.088°
   Memory efficient: 6 bytes vs 24 bytes per point
   Deterministic: No floating-point drift
```

## Integration Guide

### In Your Rust Project

**Add to Cargo.toml:**
```toml
[dependencies]
dodecet-encoder = "0.1"
```

**Basic Usage:**
```rust
use dodecet_encoder::{Dodecet, Point3D};

// Create a 3D point
let point = Point3D::new(0x100, 0x200, 0x300);

// Access coordinates
let (x, y, z) = (point.x(), point.y(), point.z());

// Convert to hex
let hex = point.to_hex_string();

// Calculate distance
let other = Point3D::new(0x150, 0x250, 0x350);
let distance = point.distance_to(&other);
```

### In Web Applications

**Option 1: Standalone HTML**
- Use `examples/web_integration.html` as reference
- Copy the `DodecetPoint3D` class implementation
- Integrate into your web application

**Option 2: WASM Bindings (Coming Soon)**
```bash
# Build WASM package
cargo build --target wasm32-unknown-unknown --release
wasm-pack build --target web

# Install npm package (when published)
npm install @superinstance/dodecet-encoder
```

**Option 3: TypeScript Integration**
```typescript
import { DodecetPoint3D } from '@superinstance/dodecet-encoder';

const point = new DodecetPoint3D(1000, 2000, 3000);
console.log('Hex:', point.toHex());
```

### For Constraint Theory

**Pythagorean Snapping:**
```rust
use dodecet_encoder::Point3D;

let point = Point3D::new(1000, 2000, 3000);
let snapper = PythagoreanSnapper::new();
let snapped = snapper.snap(&point);
```

**Rigidity Detection:**
```rust
let mut graph = Graph::new();
let v0 = graph.add_vertex(0x000, 0x000, 0x000);
let v1 = graph.add_vertex(0x100, 0x000, 0x000);
// ... add more vertices and edges

let checker = RigidityChecker::new(2);
let is_rigid = checker.is_rigid(&graph);
```

### For SuperInstance Projects

**Cellular Agent State:**
```rust
use dodecet_encoder::DodecetArray;

// Agent state: position (3) + status (1) = 4 dodecets
let state = DodecetArray::<4>::from_slice(&[0x100, 0x200, 0x300, 0x001]);

// Memory efficient: 6 bytes vs 32 bytes for struct
```

**Geometric Reasoning:**
```rust
// Deterministic geometric logic
let point = Point3D::new(0x100, 0x200, 0x300);
let snapper = PythagoreanSnapper::new();
let snapped = snapper.snap(&point);

// Always snaps to same triple → deterministic reasoning
```

## Performance Tips

1. **Use Dodecet Arrays** for batch operations
2. **Pre-allocate** arrays when size is known
3. **Use references** to avoid copies
4. **Leverage integer math** for performance
5. **Consider 12-bit precision** sufficient for discrete geometry

## Troubleshooting

### Build Errors

**Error:** `error: linker 'link.exe' not found`
- **Solution:** Install MSVC build tools on Windows

**Error:** `error: undefined reference to 'sqrt'`
- **Solution:** Add `extern crate std;` to your crate

### Runtime Errors

**Error:** `Value exceeds 12-bit capacity`
- **Solution:** Ensure values are in range [0, 4095]

**Error:** `Invalid hex string`
- **Solution:** Use valid hex characters (0-9, A-F)

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add your example
4. Submit a pull request

## License

MIT License - See LICENSE file for details

## Resources

- [GitHub Repository](https://github.com/SuperInstance/dodecet-encoder)
- [Documentation](https://docs.rs/dodecet-encoder)
- [Constraint Theory](https://github.com/SuperInstance/constrainttheory)
- [SuperInstance Papers](https://github.com/SuperInstance/SuperInstance-papers)

## Support

For questions or issues:
- Open an issue on GitHub
- Check the documentation
- Review the examples

---

**Last Updated:** 2026-03-16
**Version:** 0.1.0
**Status:** Active Development
