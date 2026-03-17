# Basic Operations Tutorial

**Mastering fundamental dodecet operations**

## Table of Contents

1. [Dodecet Operations](#dodecet-operations)
2. [Array Operations](#array-operations)
3. [String Operations](#string-operations)
4. [Hex Utilities](#hex-utilities)
5. [Performance Tips](#performance-tips)
6. [Common Patterns](#common-patterns)

---

## Dodecet Operations

### Creation Methods

```rust
use dodecet_encoder::Dodecet;

// From hex value (fastest)
let d1 = Dodecet::from_hex(0xABC);

// From decimal (checked)
let d2 = Dodecet::new(2748).unwrap();

// From hex string (parsed)
let d3 = Dodecet::from_hex_str("ABC").unwrap();

// Default (zero)
let d4 = Dodecet::default();
assert_eq!(d4.value(), 0);
```

### Nibble Manipulation

```rust
let d = Dodecet::from_hex(0xABC);

// Get nibbles
let n0 = d.nibble(0).unwrap(); // 0xC
let n1 = d.nibble(1).unwrap(); // 0xB
let n2 = d.nibble(2).unwrap(); // 0xA

// Set nibble (create new dodecet)
let new_d = Dodecet::from_hex(
    ((d.nibble(2).unwrap() as u16) << 8) |
    ((d.nibble(1).unwrap() as u16) << 4) |
    (d.nibble(0).unwrap() as u16)
);
```

### Arithmetic Best Practices

```rust
let a = Dodecet::from_hex(0x100);
let b = Dodecet::from_hex(0x200);

// Basic arithmetic (all wrap around)
let sum = a + b;      // 0x300
let diff = b - a;     // 0x100
let product = a * b;  // 0x200 (with wrapping)

// Checked arithmetic (returns None on overflow)
let checked_sum = a.checked_add(b);
let checked_diff = a.checked_sub(b);

// Saturating arithmetic (clamps at bounds)
let sat_sum = a.saturating_add(b);
let sat_diff = a.saturating_sub(b);
```

### Bitwise Tricks

```rust
let d = Dodecet::from_hex(0xF0F); // 1111 0000 1111

// Check if bit is set
let is_set = (d.value() & 0x800) != 0; // Check bit 11

// Set bit
let with_bit = Dodecet::from_hex(d.value() | 0x800);

// Clear bit
let without_bit = Dodecet::from_hex(d.value() & !0x800);

// Toggle bit
let toggled = Dodecet::from_hex(d.value() ^ 0x800);

// Extract specific bits
let bits_3_6 = (d.value() >> 3) & 0xF; // Bits 3-6
```

### Comparison and Ordering

```rust
let a = Dodecet::from_hex(0x100);
let b = Dodecet::from_hex(0x200);

// Equality
assert_eq!(a, a);
assert_ne!(a, b);

// Ordering
assert!(a < b);
assert!(b > a);

// Min/Max
let min_val = a.min(b);
let max_val = a.max(b);

// Clamping
let clamped = Dodecet::new(5000).unwrap().min(Dodecet::from_hex(0xFFF));
```

---

## Array Operations

### Fixed-Size Arrays

```rust
use dodecet_encoder::DodecetArray;

// Create from slice
let arr = DodecetArray::<4>::from_slice(&[0x123, 0x456, 0x789, 0xABC]);

// Access elements
let first = arr.get(0).unwrap();
let last = arr.get(3).unwrap();

// Mutable access
let mut arr = DodecetArray::<4>::default();
arr.set(0, Dodecet::from_hex(0x100));

// Iteration
for d in arr.iter() {
    println!("{}", d.to_hex_string());
}

// Mutable iteration
let mut arr = DodecetArray::<4>::default();
for d in arr.iter_mut() {
    *d = Dodecet::from_hex(0x123);
}
```

### Aggregate Operations

```rust
let arr = DodecetArray::<4>::from_slice(&[0x100, 0x200, 0x300, 0x400]);

// Sum
let sum = arr.sum(); // 0xA00

// Average
let avg = arr.average(); // 0x280

// Min/Max
let min = arr.min(); // 0x100
let max = arr.max(); // 0x400

// Product
let product = arr.product();

// All/Any
let all_positive = arr.iter().all(|d| d.value() > 0);
let any_zero = arr.iter().any(|d| d.value() == 0);
```

### Array Transformations

```rust
let arr = DodecetArray::<4>::from_slice(&[0x100, 0x200, 0x300, 0x400]);

// Map
let doubled: DodecetArray<4> = arr.map(|d| {
    Dodecet::from_hex((d.value() * 2) & 0xFFF)
});

// Filter (returns Vec)
let filtered: Vec<Dodecet> = arr.iter()
    .filter(|d| d.value() > 0x200)
    .copied()
    .collect();

// Fold
let concatenated = arr.fold(0u32, |acc, d| {
    acc * 0x1000 + d.value() as u32
});
```

---

## String Operations

### Creating DodecetStrings

```rust
use dodecet_encoder::DodecetString;

// Empty
let s = DodecetString::new();

// With capacity
let s = DodecetString::with_capacity(100);

// From slice
let s = DodecetString::from_slice(&[0x123, 0x456, 0x789]);

// From hex string
let s = DodecetString::from_hex_str("123456789").unwrap();
```

### Modification

```rust
let mut s = DodecetString::new();

// Push
s.push(0x123);
s.push(0x456);

// Pop
if let Some(d) = s.pop() {
    println!("Popped: {}", d.to_hex_string());
}

// Insert
s.insert(0, Dodecet::from_hex(0x000));

// Remove
if let Some(d) = s.remove(0) {
    println!("Removed: {}", d.to_hex_string());
}

// Clear
s.clear();
```

### Conversion

```rust
let s = DodecetString::from_slice(&[0x123, 0x456, 0x789]);

// To hex string
let hex = s.to_hex_string(); // "123456789"

// To bytes (2 dodecets = 3 bytes)
let bytes = s.to_bytes();

// From bytes
let s2 = DodecetString::from_bytes(&bytes).unwrap();

// To Vec
let vec: Vec<Dodecet> = s.to_vec();
```

### Searching

```rust
let s = DodecetString::from_slice(&[0x123, 0x456, 0x789, 0x123]);

// Find
if let Some(pos) = s.find(&Dodecet::from_hex(0x123)) {
    println!("Found at position: {}", pos);
}

// Contains
let contains = s.contains(&Dodecet::from_hex(0x456));

// Count
let count = s.count(&Dodecet::from_hex(0x123));
```

---

## Hex Utilities

### Validation

```rust
use dodecet_encoder::hex;

// Check if valid
assert!(hex::is_valid("123"));      // Valid
assert!(hex::is_valid("ABC"));      // Valid
assert!(!hex::is_valid("1234"));    // Wrong length
assert!(!hex::is_valid("XYZ"));     // Invalid chars

// Validate with length
assert!(hex::is_valid_length("123", 3));  // OK
assert!(!hex::is_valid_length("12", 3));  // Too short
```

### Formatting

```rust
use dodecet_encoder::hex;

// Space-separated
let spaced = hex::format_spaced("123456789");
// "123 456 789"

// Grouped
let grouped = hex::format_grouped("123456789ABC", 6);
// "123456 789ABC"

// Hex view (like hex editor)
let view = hex::hex_view("123456789ABCDEF");
// 00000000: 123 456 789 ABC DEF
```

### Encoding/Decoding

```rust
use dodecet_encoder::hex;

// Encode dodecets to hex
let dodecets = vec![0x123u16, 0x456, 0x789];
let encoded = hex::encode(&dodecets); // "123456789"

// Decode hex to dodecets
let decoded = hex::decode("123456789").unwrap();
assert_eq!(decoded, vec![0x123, 0x456, 0x789]);
```

---

## Performance Tips

### 1. Use Fixed-Size Arrays When Possible

```rust
// Faster (stack-allocated)
let arr = DodecetArray::<100>::from_slice(&values);

// Slower (heap-allocated)
let vec: Vec<Dodecet> = values.to_vec();
```

### 2. Pre-allocate Strings

```rust
// Faster
let mut s = DodecetString::with_capacity(1000);
for i in 0..1000 {
    s.push(i as u16);
}

// Slower (reallocations)
let mut s = DodecetString::new();
for i in 0..1000 {
    s.push(i as u16);
}
```

### 3. Batch Operations

```rust
// Slower (individual operations)
for i in 0..1000 {
    let d = Dodecet::from_hex(i);
    process(d);
}

// Faster (batch)
let arr = DodecetArray::<1000>::from_slice(&values);
for d in arr.iter() {
    process(*d);
}
```

### 4. Use Iterators

```rust
let arr = DodecetArray::<100>::from_slice(&values);

// Efficient iterator usage
let sum: u32 = arr.iter()
    .map(|d| d.value() as u32)
    .sum();

// Instead of manual loop
let mut sum = 0u32;
for i in 0..100 {
    sum += arr.get(i).unwrap().value() as u32;
}
```

---

## Common Patterns

### Pattern 1: Color Encoding

```rust
// RGB color (12 bits per channel = 36 bits total)
struct Color {
    r: Dodecet, // 0-4095
    g: Dodecet,
    b: Dodecet,
}

impl Color {
    fn from_rgb(r: u16, g: u16, b: u16) -> Self {
        Color {
            r: Dodecet::new(r & 0xFFF).unwrap(),
            g: Dodecet::new(g & 0xFFF).unwrap(),
            b: Dodecet::new(b & 0xFFF).unwrap(),
        }
    }

    fn to_hex(&self) -> String {
        format!("{}{}{}",
            self.r.to_hex_string(),
            self.g.to_hex_string(),
            self.b.to_hex_string()
        )
    }
}
```

### Pattern 2: Coordinate System

```rust
// 2D coordinate
struct Coord2D {
    x: Dodecet,
    y: Dodecet,
}

impl Coord2D {
    fn distance(&self, other: &Coord2D) -> f64 {
        let dx = self.x.value() as f64 - other.x.value() as f64;
        let dy = self.y.value() as f64 - other.y.value() as f64;
        (dx * dx + dy * dy).sqrt()
    }

    fn midpoint(&self, other: &Coord2D) -> Self {
        Coord2D {
            x: Dodecet::from_hex((self.x.value() + other.x.value()) / 2),
            y: Dodecet::from_hex((self.y.value() + other.y.value()) / 2),
        }
    }
}
```

### Pattern 3: State Encoding

```rust
// State machine
#[derive(Debug, Clone, Copy)]
enum State {
    Idle,
    Running,
    Paused,
    Error,
}

impl State {
    fn encode(&self) -> Dodecet {
        match self {
            State::Idle => Dodecet::from_hex(0x000),
            State::Running => Dodecet::from_hex(0x001),
            State::Paused => Dodecet::from_hex(0x002),
            State::Error => Dodecet::from_hex(0x003),
        }
    }

    fn decode(d: Dodecet) -> Option<Self> {
        match d.value() {
            0x000 => Some(State::Idle),
            0x001 => Some(State::Running),
            0x002 => Some(State::Paused),
            0x003 => Some(State::Error),
            _ => None,
        }
    }
}
```

---

## Complete Example

```rust
use dodecet_encoder::{Dodecet, DodecetArray, DodecetString, hex};

fn main() {
    println!("=== Basic Operations Demo ===\n");

    // 1. Dodecet operations
    let a = Dodecet::from_hex(0xABC);
    let b = Dodecet::from_hex(0x123);

    println!("Arithmetic:");
    println!("  {} + {} = {}", a.to_hex_string(), b.to_hex_string(), (a + b).to_hex_string());
    println!("  {} - {} = {}", a.to_hex_string(), b.to_hex_string(), (a - b).to_hex_string());
    println!("  {} * {} = {}", a.to_hex_string(), b.to_hex_string(), (a * b).to_hex_string());

    // 2. Array operations
    let arr = DodecetArray::<4>::from_slice(&[0x100, 0x200, 0x300, 0x400]);
    println!("\nArray operations:");
    println!("  Sum: {}", arr.sum());
    println!("  Average: {}", arr.average());
    println!("  Min: {}", arr.min());
    println!("  Max: {}", arr.max());

    // 3. String operations
    let mut s = DodecetString::new();
    s.push(0x123);
    s.push(0x456);
    s.push(0x789);

    println!("\nString operations:");
    println!("  Hex: {}", s.to_hex_string());
    println!("  Length: {}", s.len());

    // 4. Hex utilities
    println!("\nHex utilities:");
    println!("  Spaced: {}", hex::format_spaced("123456789"));

    println!("\n=== Demo Complete ===");
}
```

---

**Next: [Geometric Operations](./02_GEOMETRIC_OPERATIONS.md)**
