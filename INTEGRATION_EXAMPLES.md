# Integration Examples with SuperInstance Projects

This document shows how to integrate dodecet-encoder with other SuperInstance projects.

## Integration 1: ConstraintTheory (Geometric Visualization)

Use dodecet-encoded points for geometric visualizations:

```rust
use dodecet_encoder::geometric::{Point3D, Vector3D};

// Create geometric points for visualization
let point_a = Point3D::new(0x100, 0x200, 0x300);
let point_b = Point3D::new(0x400, 0x500, 0x600);

// Calculate distance for visualization
let distance = point_a.distance_to(&point_b);
println!("Distance: {}", distance);

// Create vectors for direction visualization
let vector = point_a.vector_to(&point_b);
let magnitude = vector.magnitude();
println!("Vector magnitude: {}", magnitude);
```

## Integration 2: Claw (Cellular Agents)

Encode claw agent states using dodecets for compact storage:

```rust
use dodecet_encoder::{Dodecet, DodecetString};

// Encode claw state (5 values: x, y, z, energy, task_id)
let mut state = DodecetString::new();
state.push(0x100);  // position_x
state.push(0x200);  // position_y
state.push(0x300);  // position_z
state.push(0xFFF);  // energy (max)
state.push(0x042);  // task_id

// Serialize to bytes for network transmission
let encoded = state.to_bytes();
println!("State size: {} bytes (10 bytes vs 40 bytes with f64)", encoded.len());
```

## Integration 3: Spreadsheet-Moment (Cell Data)

Store cell coordinates using dodecets for memory efficiency:

```rust
use dodecet_encoder::Dodecet;

// Cell coordinate (row, col, sheet)
struct CellCoordinate {
    row: Dodecet,    // 0 to 4095
    col: Dodecet,    // 0 to 4095
    sheet: Dodecet,  // 0 to 4095
}

impl CellCoordinate {
    fn new(row: u16, col: u16, sheet: u16) -> Self {
        Self {
            row: Dodecet::from_hex(row),
            col: Dodecet::from_hex(col),
            sheet: Dodecet::from_hex(sheet),
        }
    }
    
    fn to_hex_string(&self) -> String {
        format!(
            "{}:{}:{}",
            self.sheet.to_hex_string(),
            self.row.to_hex_string(),
            self.col.to_hex_string()
        )
    }
}

// Usage
let coord = CellCoordinate::new(0x10, 0x20, 0x01);
println!("Cell: {}", coord.to_hex_string());  // "01:10:20"
```

## Memory Savings Comparison

| Data Structure | Traditional (f64) | Dodecet | Savings |
|----------------|-------------------|---------|---------|
| 3D Point | 24 bytes | 6 bytes | 75% |
| Cell Coordinate | 24 bytes | 6 bytes | 75% |
| Agent State (5 values) | 40 bytes | 10 bytes | 75% |

## Next Steps

- See [examples/](./examples/) for complete examples
- Read [GETTING_STARTED_GUIDE.md](./GETTING_STARTED_GUIDE.md) for beginners
- Check [RELEASE_CHECKLIST.md](./RELEASE_CHECKLIST.md) for release info
