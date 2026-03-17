# Geometric Operations with Dodecet Encoder

**Master 3D geometry using 12-bit dodecet encoding**

## Table of Contents

1. [Introduction](#introduction)
2. [Point3D](#point3d)
3. [Vector3D](#vector3d)
4. [Transformations](#transformations)
5. [Shapes](#shapes)
6. [Spatial Queries](#spatial-queries)
7. [Performance Tips](#performance-tips)
8. [Exercises](#exercises)

---

## Introduction

Dodecet encoding shines for geometric operations because:

- **Discrete Precision**: 4,096 values per axis provides sub-millimeter precision
- **Memory Efficiency**: 6 bytes per point vs 24 bytes for f64 (75% savings)
- **Integer Operations**: No floating-point rounding or drift
- **Hex-Friendly**: Easy to debug in hex editors

### Common Use Cases

- 3D graphics and game engines
- CAD systems
- Robotics and automation
- Spatial databases
- Constraint theory systems

---

## Point3D

### Creating Points

```rust
use dodecet_encoder::geometric::Point3D;

// From dodecets (hex values)
let p1 = Point3D::new(0x100, 0x200, 0x300);
let p2 = Point3D::new(0x400, 0x500, 0x600);

// From integers (will be truncated to 12-bit)
let p3 = Point3D::new(100, 200, 300);

// From normalized values (0.0 to 1.0)
let p4 = Point3D::from_normalized(0.25, 0.5, 0.75);
```

### Accessing Coordinates

```rust
let point = Point3D::new(0x100, 0x200, 0x300);

// Access individual coordinates
let x = point.x(); // Returns u16
let y = point.y();
let z = point.z();

// Get all coordinates as tuple
let (x, y, z) = point.to_tuple();

// Convert to array
let coords = point.to_array(); // [u16; 3]

// Hex representation
let hex = point.to_hex_string(); // "100200300"
```

### Distance Calculations

```rust
let p1 = Point3D::new(0x000, 0x000, 0x000);
let p2 = Point3D::new(0x300, 0x400, 0x500); // 3-4-5 triangle

// Euclidean distance
let distance = p1.distance_to(&p2);
// sqrt(3² + 4² + 5²) = sqrt(50) ≈ 7.071

// Squared distance (faster, no sqrt)
let dist_sq = p1.distance_squared_to(&p2);
// 50

// Manhattan distance (L1 norm)
let manhattan = p1.manhattan_distance_to(&p2);
// 3 + 4 + 5 = 12
```

### Point Operations

```rust
let p1 = Point3D::new(0x100, 0x200, 0x300);
let p2 = Point3D::new(0x050, 0x050, 0x050);

// Vector between points
let vector = p1.vector_to(&p2);

// Midpoint
let midpoint = p1.midpoint(&p2);

// Translate point
let translated = p1.translate(10, 20, 30);
```

---

## Vector3D

### Creating Vectors

```rust
use dodecet_encoder::geometric::Vector3D;

// From components
let v1 = Vector3D::new(100, 200, 300);

// From two points
let p1 = Point3D::new(0x100, 0x200, 0x300);
let p2 = Point3D::new(0x200, 0x300, 0x400);
let v2 = Vector3D::from_points(&p1, &p2);

// Zero vector
let zero = Vector3D::zero();

// Unit vector along X axis
let unit_x = Vector3D::unit_x();
```

### Vector Arithmetic

```rust
let v1 = Vector3D::new(100, 200, 300);
let v2 = Vector3D::new(400, 500, 600);

// Addition
let sum = v1 + v2;

// Subtraction
let diff = v2 - v1;

// Scalar multiplication
let scaled = v1 * 2;

// Scalar division
let halved = v1 / 2;

// Negation
let negated = -v1;
```

### Vector Properties

```rust
let v = Vector3D::new(300, 400, 500);

// Magnitude (length)
let magnitude = v.magnitude();
// sqrt(300² + 400² + 500²) = sqrt(500000) ≈ 707.1

// Squared magnitude (faster)
let mag_sq = v.magnitude_squared();
// 500000

// Normalization (unit vector)
let unit = v.normalize();
// Length = 1.0
```

### Dot Product

```rust
let v1 = Vector3D::new(100, 0, 0);
let v2 = Vector3D::new(100, 0, 0);

// Dot product
let dot = v1.dot(&v2);
// 100*100 + 0*0 + 0*0 = 10000

// Angle between vectors
let angle = v1.angle_to(&v2);
// 0 radians (parallel)

// Orthogonal check
let v3 = Vector3D::new(0, 100, 0);
assert!(v1.is_orthogonal_to(&v3));
```

### Cross Product

```rust
let v1 = Vector3D::new(1, 0, 0);
let v2 = Vector3D::new(0, 1, 0);

// Cross product (perpendicular vector)
let cross = v1.cross(&v2);
// Vector pointing in Z direction: (0, 0, 1)

// Cross product magnitude = |a|*|b|*sin(θ)
// For perpendicular unit vectors: 1*1*1 = 1
```

---

## Transformations

### Translation

```rust
use dodecet_encoder::geometric::Transform3D;

let point = Point3D::new(0x100, 0x200, 0x300);

// Create translation
let translation = Transform3D::translation(50, 100, 150);

// Apply translation
let translated = translation.apply(&point);
// Point at (0x100+50, 0x200+100, 0x300+150)
```

### Scaling

```rust
let point = Point3D::new(0x100, 0x200, 0x300);

// Uniform scaling
let scale = Transform3D::scale(2.0, 2.0, 2.0);
let doubled = scale.apply(&point);

// Non-uniform scaling
let stretch = Transform3D::scale(2.0, 1.0, 0.5);
let stretched = stretch.apply(&point);
```

### Rotation

```rust
let point = Point3D::new(0x100, 0x000, 0x000);

// Rotation around X axis
let rot_x = Transform3D::rotation_x(90.0);

// Rotation around Y axis
let rot_y = Transform3D::rotation_y(90.0);

// Rotation around Z axis
let rot_z = Transform3D::rotation_z(90.0);

// Apply rotation
let rotated = rot_y.apply(&point);
```

### Combining Transformations

```rust
let point = Point3D::new(0x100, 0x200, 0x300);

// Create multiple transforms
let t1 = Transform3D::translation(50, 0, 0);
let t2 = Transform3D::rotation_y(45.0);
let t3 = Transform3D::scale(2.0, 2.0, 2.0);

// Chain transformations
let chained = t1.then(&t2).then(&t3);

// Apply all at once
let result = chained.apply(&point);

// Order matters! Translation → Rotation → Scale
```

---

## Shapes

### Triangle

```rust
use dodecet_encoder::geometric::{Triangle, Point3D};

let p1 = Point3D::new(0x000, 0x000, 0x000);
let p2 = Point3D::new(0x300, 0x000, 0x000);
let p3 = Point3D::new(0x000, 0x400, 0x000);

let triangle = Triangle::new(p1, p2, p3);

// Area
let area = triangle.area();

// Perimeter
let perimeter = triangle.perimeter();

// Centroid
let centroid = triangle.centroid();

// Normal vector
let normal = triangle.normal();
```

### Box3D (Bounding Box)

```rust
use dodecet_encoder::geometric::{Box3D, Point3D};

let min = Point3D::new(0x000, 0x000, 0x000);
let max = Point3D::new(0xFFF, 0xFFF, 0xFFF);

let box3d = Box3D::new(min, max);

// Dimensions
let width = box3d.width();
let height = box3d.height();
let depth = box3d.depth();
let volume = box3d.volume();

// Center
let center = box3d.center();

// Contains point?
let point = Point3D::new(0x100, 0x200, 0x300);
assert!(box3d.contains(&point));

// Intersection
let other = Box3D::new(
    Point3D::new(0x500, 0x500, 0x500),
    Point3D::new(0xFFF, 0xFFF, 0xFFF)
);
if let Some(intersection) = box3d.intersection(&other) {
    // Handle intersection
}
```

---

## Spatial Queries

### Nearest Neighbor

```rust
use dodecet_encoder::geometric::Point3D;

let points = vec![
    Point3D::new(0x100, 0x200, 0x300),
    Point3D::new(0x400, 0x500, 0x600),
    Point3D::new(0x700, 0x800, 0x900),
];

let query = Point3D::new(0x150, 0x250, 0x350);

// Find nearest point
let nearest = points.iter()
    .min_by(|a, b| {
        a.distance_to(&query).partial_cmp(&b.distance_to(&query)).unwrap()
    });
```

### Points Within Radius

```rust
let points = vec![
    Point3D::new(0x100, 0x200, 0x300),
    Point3D::new(0x400, 0x500, 0x600),
    Point3D::new(0x700, 0x800, 0x900),
];

let center = Point3D::new(0x200, 0x300, 0x400);
let radius = 500.0;

// Find points within radius
let within_radius: Vec<_> = points.iter()
    .filter(|p| p.distance_to(&center) <= radius)
    .collect();
```

---

## Performance Tips

### 1. Use Squared Distance When Possible

```rust
// Slower (uses sqrt)
let distance = p1.distance_to(&p2);
if distance < 100.0 { /* ... */ }

// Faster (no sqrt)
let dist_sq = p1.distance_squared_to(&p2);
if dist_sq < 10000.0 { /* ... */ }
```

### 2. Pre-allocate Arrays

```rust
// Slower (reallocation)
let mut points = Vec::new();
for i in 0..1000 {
    points.push(Point3D::new(i, i, i));
}

// Faster (pre-allocated)
let mut points = Vec::with_capacity(1000);
for i in 0..1000 {
    points.push(Point3D::new(i, i, i));
}
```

### 3. Use References

```rust
// Avoids copying
fn process_point(point: &Point3D) {
    // ...
}

let p = Point3D::new(0x100, 0x200, 0x300);
process_point(&p); // Pass by reference
```

### 4. Batch Operations

```rust
// Process multiple points at once
let points = vec![/* ... */];

// Calculate all distances
let distances: Vec<f64> = points.iter()
    .map(|p| p.distance_to(&query))
    .collect();
```

---

## Exercises

### Exercise 1: Create a Cube

Create a function that generates the 8 vertices of a cube:

```rust
use dodecet_encoder::geometric::Point3D;

fn create_cube(center: Point3D, size: u16) -> Vec<Point3D> {
    // Your code here
    // Should return 8 vertices of a cube
}
```

**Solution:**
```rust
fn create_cube(center: Point3D, size: u16) -> Vec<Point3D> {
    let half = size / 2;
    let (cx, cy, cz) = center.to_tuple();

    vec![
        Point3D::new(cx - half, cy - half, cz - half),
        Point3D::new(cx + half, cy - half, cz - half),
        Point3D::new(cx - half, cy + half, cz - half),
        Point3D::new(cx + half, cy + half, cz - half),
        Point3D::new(cx - half, cy - half, cz + half),
        Point3D::new(cx + half, cy - half, cz + half),
        Point3D::new(cx - half, cy + half, cz + half),
        Point3D::new(cx + half, cy + half, cz + half),
    ]
}
```

### Exercise 2: Calculate Sphere Volume

Calculate the volume of a sphere from dodecet-encoded points:

```rust
use dodecet_encoder::geometric::Point3D;

fn sphere_volume(center: Point3D, radius: f64) -> f64 {
    // Your code here
    // Volume = (4/3) * π * r³
}
```

**Solution:**
```rust
fn sphere_volume(_center: Point3D, radius: f64) -> f64 {
    (4.0 / 3.0) * std::f64::consts::PI * radius.powi(3)
}
```

### Exercise 3: Rotate Point Around Axis

Rotate a point around an arbitrary axis:

```rust
use dodecet_encoder::geometric::{Point3D, Vector3D};

fn rotate_around_axis(
    point: Point3D,
    axis: Vector3D,
    angle_degrees: f64
) -> Point3D {
    // Your code here
    // Use Rodrigues' rotation formula
}
```

---

## Next Steps

- [Tutorial 3: Calculus Operations](03_CALCULUS_OPERATIONS.md) - Numerical methods
- [Tutorial 4: Integration](04_INTEGRATION.md) - WebAssembly and web
- [Tutorial 5: Advanced Usage](05_ADVANCED_USAGE.md) - Performance optimization

---

**Found an issue?** [Report it here](https://github.com/SuperInstance/dodecet-encoder/issues)

**Need help?** [Ask in discussions](https://github.com/SuperInstance/dodecet-encoder/discussions)

---

**Last Updated:** 2026-03-16
**Tutorial:** 02 - Geometric Operations
