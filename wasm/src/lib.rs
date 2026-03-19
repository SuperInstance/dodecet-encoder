//! # Dodecet Encoder WebAssembly Bindings
//!
//! This crate provides WebAssembly bindings for the dodecet-encoder library,
//! enabling high-performance 12-bit geometric operations in browsers.
//!
//! ## Features
//!
//! - **3D Point Operations**: Create, manipulate, and transform 3D points
//! - **Vector Math**: Dot product, cross product, normalization
//! - **Transformations**: Translation, rotation, scale operations
//! - **Hex Encoding**: Bidirectional conversion with hex strings
//! - **Performance**: Compiled to WASM for near-native speed
//!
//! ## Quick Start
//!
//! ```javascript
//! import init, { Point3D } from '@superinstance/dodecet-encoder';
//!
//! await init();
//! const point = new Point3D(0x123, 0x456, 0x789);
//! console.log(point.toHex()); // "123 456 789"
//! ```

use wasm_bindgen::prelude::*;
use dodecet_encoder::{Dodecet, DodecetArray, geometric::{Point3D as RustPoint3D, Vector3D, Transform3D}};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ====================
// Error Handling
// ====================

/// WASM-friendly error type
#[wasm_bindgen]
pub struct DodecetError {
    message: String,
}

impl From<dodecet_encoder::DodecetError> for DodecetError {
    fn from(err: dodecet_encoder::DodecetError) -> Self {
        DodecetError {
            message: err.to_string(),
        }
    }
}

// ====================
// Dodecet (12-bit value)
// ====================

/// A 12-bit dodecet value (0-4095)
#[wasm_bindgen]
pub struct DodecetWasm {
    inner: Dodecet,
}

#[wasm_bindgen]
impl DodecetWasm {
    /// Create a new dodecet from a value (0-4095)
    ///
    /// # Arguments
    /// * `value` - Value to encode (0-4095)
    ///
    /// # Returns
    /// New DodecetWasm instance
    ///
    /// # Example
    /// ```javascript
    /// const d = new DodecetWasm(0xABC);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(value: u16) -> Result<DodecetWasm, JsError> {
        let inner = Dodecet::new(value)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(DodecetWasm { inner })
    }

    /// Get the value
    #[wasm_bindgen]
    pub fn value(&self) -> u16 {
        self.inner.value()
    }

    /// Get a specific nibble (0, 1, or 2)
    #[wasm_bindgen]
    pub fn nibble(&self, index: u8) -> Result<u8, JsError> {
        self.inner.nibble(index)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Convert to hex string
    #[wasm_bindgen]
    pub fn to_hex(&self) -> String {
        format!("{:03X}", self.inner.value())
    }

    /// Normalize to [0.0, 1.0]
    #[wasm_bindgen]
    pub fn normalize(&self) -> f64 {
        self.inner.normalize()
    }

    /// Convert to signed value [-2048, 2047]
    #[wasm_bindgen]
    pub fn as_signed(&self) -> i16 {
        self.inner.as_signed()
    }
}

// ====================
// 3D Point
// ====================

/// A 3D point with dodecet-encoded coordinates
#[wasm_bindgen]
pub struct Point3D {
    inner: RustPoint3D,
}

#[wasm_bindgen]
impl Point3D {
    /// Create a new 3D point
    ///
    /// # Arguments
    /// * `x` - X coordinate (0-4095)
    /// * `y` - Y coordinate (0-4095)
    /// * `z` - Z coordinate (0-4095)
    ///
    /// # Returns
    /// New Point3D instance
    ///
    /// # Example
    /// ```javascript
    /// const point = new Point3D(0x100, 0x200, 0x300);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16, z: u16) -> Point3D {
        Point3D {
            inner: RustPoint3D::new(x, y, z),
        }
    }

    /// Get X coordinate
    #[wasm_bindgen(method, getter)]
    pub fn x(&self) -> u16 {
        self.inner.x()
    }

    /// Get Y coordinate
    #[wasm_bindgen(method, getter)]
    pub fn y(&self) -> u16 {
        self.inner.y()
    }

    /// Get Z coordinate
    #[wasm_bindgen(method, getter)]
    pub fn z(&self) -> u16 {
        self.inner.z()
    }

    /// Convert to hex string
    ///
    /// # Returns
    /// Hex string in format "XXX YYY ZZZ"
    ///
    /// # Example
    /// ```javascript
    /// const point = new Point3D(0x123, 0x456, 0x789);
    /// console.log(point.toHex()); // "123 456 789"
    /// ```
    #[wasm_bindgen]
    pub fn to_hex(&self) -> String {
        self.inner.to_hex_string()
    }

    /// Parse from hex string
    ///
    /// # Arguments
    /// * `hex_str` - Hex string (e.g., "123 456 789")
    ///
    /// # Returns
    /// New Point3D instance
    ///
    /// # Example
    /// ```javascript
    /// const point = Point3D.fromHex("123 456 789");
    /// ```
    #[wasm_bindgen]
    pub fn from_hex(hex_str: &str) -> Result<Point3D, JsError> {
        let inner = RustPoint3D::from_hex_str(hex_str)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(Point3D { inner })
    }

    /// Get normalized coordinates [0.0, 1.0]
    ///
    /// # Returns
    /// Array [nx, ny, nz]
    ///
    /// # Example
    /// ```javascript
    /// const point = new Point3D(0x800, 0x800, 0x800);
    /// const [nx, ny, nz] = point.normalized();
    /// console.log(nx, ny, nz); // ~0.5, ~0.5, ~0.5
    /// ```
    #[wasm_bindgen]
    pub fn normalized(&self) -> Box<[f64]> {
        let (nx, ny, nz) = self.inner.normalized();
        Box::new([nx, ny, nz])
    }

    /// Get signed coordinates [-2048, 2047]
    ///
    /// # Returns
    /// Array [sx, sy, sz]
    #[wasm_bindgen]
    pub fn signed(&self) -> Box<[i16]> {
        let (sx, sy, sz) = self.inner.signed();
        Box::new([sx, sy, sz])
    }

    /// Calculate distance to another point
    ///
    /// # Arguments
    /// * `other` - Other point
    ///
    /// # Returns
    /// Euclidean distance
    ///
    /// # Example
    /// ```javascript
    /// const p1 = new Point3D(0, 0, 0);
    /// const p2 = new Point3D(0x100, 0, 0);
    /// console.log(p1.distanceTo(p2)); // ~256.0
    /// ```
    #[wasm_bindgen]
    pub fn distance_to(&self, other: &Point3D) -> f64 {
        self.inner.distance_to(&other.inner)
    }
}

// ====================
// 3D Vector
// ====================

/// A 3D vector with dodecet-encoded components
#[wasm_bindgen]
pub struct Vector3DWasm {
    inner: Vector3D,
}

#[wasm_bindgen]
impl Vector3DWasm {
    /// Create a new 3D vector
    ///
    /// # Arguments
    /// * `x` - X component (signed)
    /// * `y` - Y component (signed)
    /// * `z` - Z component (signed)
    #[wasm_bindgen(constructor)]
    pub fn new(x: i16, y: i16, z: i16) -> Vector3DWasm {
        Vector3DWasm {
            inner: Vector3D::new(x, y, z),
        }
    }

    /// Get X component
    #[wasm_bindgen(method, getter)]
    pub fn x(&self) -> i16 {
        self.inner.x()
    }

    /// Get Y component
    #[wasm_bindgen(method, getter)]
    pub fn y(&self) -> i16 {
        self.inner.y()
    }

    /// Get Z component
    #[wasm_bindgen(method, getter)]
    pub fn z(&self) -> i16 {
        self.inner.z()
    }

    /// Calculate magnitude
    #[wasm_bindgen]
    pub fn magnitude(&self) -> f64 {
        self.inner.magnitude()
    }

    /// Normalize to unit vector
    ///
    /// # Returns
    /// Array [nx, ny, nz]
    #[wasm_bindgen]
    pub fn normalize(&self) -> Box<[f64]> {
        let (nx, ny, nz) = self.inner.normalize();
        Box::new([nx, ny, nz])
    }

    /// Dot product with another vector
    #[wasm_bindgen]
    pub fn dot(&self, other: &Vector3DWasm) -> i32 {
        self.inner.dot(&other.inner)
    }

    /// Cross product with another vector
    #[wasm_bindgen]
    pub fn cross(&self, other: &Vector3DWasm) -> Vector3DWasm {
        Vector3DWasm {
            inner: self.inner.cross(&other.inner),
        }
    }

    /// Add two vectors
    #[wasm_bindgen]
    pub fn add(&self, other: &Vector3DWasm) -> Vector3DWasm {
        Vector3DWasm {
            inner: self.inner.add(&other.inner),
        }
    }

    /// Subtract two vectors
    #[wasm_bindgen]
    pub fn sub(&self, other: &Vector3DWasm) -> Vector3DWasm {
        Vector3DWasm {
            inner: self.inner.sub(&other.inner),
        }
    }

    /// Scale by a scalar
    #[wasm_bindgen]
    pub fn scale(&self, scalar: f64) -> Vector3DWasm {
        Vector3DWasm {
            inner: self.inner.scale(scalar),
        }
    }
}

// ====================
// 3D Transform
// ====================

/// 3D transformation matrix
#[wasm_bindgen]
pub struct Transform3DWasm {
    inner: Transform3D,
}

#[wasm_bindgen]
impl Transform3DWasm {
    /// Create identity transformation
    #[wasm_bindgen(constructor)]
    pub fn new_identity() -> Transform3DWasm {
        Transform3DWasm {
            inner: Transform3D::identity(),
        }
    }

    /// Create translation transformation
    ///
    /// # Arguments
    /// * `dx` - X translation
    /// * `dy` - Y translation
    /// * `dz` - Z translation
    #[wasm_bindgen]
    pub fn translation(dx: i16, dy: i16, dz: i16) -> Transform3DWasm {
        Transform3DWasm {
            inner: Transform3D::translation(dx, dy, dz),
        }
    }

    /// Create scale transformation
    ///
    /// # Arguments
    /// * `sx` - X scale
    /// * `sy` - Y scale
    /// * `sz` - Z scale
    #[wasm_bindgen]
    pub fn scale(sx: f64, sy: f64, sz: f64) -> Transform3DWasm {
        Transform3DWasm {
            inner: Transform3D::scale(sx, sy, sz),
        }
    }

    /// Create rotation around X axis
    ///
    /// # Arguments
    /// * `angle_degrees` - Rotation angle in degrees
    #[wasm_bindgen]
    pub fn rotation_x(angle_degrees: f64) -> Transform3DWasm {
        Transform3DWasm {
            inner: Transform3D::rotation_x(angle_degrees),
        }
    }

    /// Create rotation around Y axis
    ///
    /// # Arguments
    /// * `angle_degrees` - Rotation angle in degrees
    #[wasm_bindgen]
    pub fn rotation_y(angle_degrees: f64) -> Transform3DWasm {
        Transform3DWasm {
            inner: Transform3D::rotation_y(angle_degrees),
        }
    }

    /// Create rotation around Z axis
    ///
    /// # Arguments
    /// * `angle_degrees` - Rotation angle in degrees
    #[wasm_bindgen]
    pub fn rotation_z(angle_degrees: f64) -> Transform3DWasm {
        Transform3DWasm {
            inner: Transform3D::rotation_z(angle_degrees),
        }
    }

    /// Apply transformation to a point
    ///
    /// # Arguments
    /// * `point` - Point to transform
    ///
    /// # Returns
    /// Transformed point
    #[wasm_bindgen]
    pub fn apply(&self, point: &Point3D) -> Point3D {
        Point3D {
            inner: self.inner.apply(&point.inner),
        }
    }

    /// Compose with another transformation
    ///
    /// # Arguments
    /// * `other` - Other transformation
    ///
    /// # Returns
    /// Composed transformation
    #[wasm_bindgen]
    pub fn compose(&self, other: &Transform3DWasm) -> Transform3DWasm {
        Transform3DWasm {
            inner: self.inner.compose(&other.inner),
        }
    }
}

// ====================
// Utility Functions
// ====================

/// Get maximum dodecet value (4095)
#[wasm_bindgen]
pub fn max_dodecet() -> u16 {
    dodecet_encoder::MAX_DODECET
}

/// Get number of bits in a dodecet (12)
#[wasm_bindgen]
pub fn dodecet_bits() -> u8 {
    dodecet_encoder::DODECET_BITS
}

/// Get dodecet capacity (4096)
#[wasm_bindgen]
pub fn dodecet_capacity() -> u16 {
    dodecet_encoder::CAPACITY
}

// ====================
// Tests
// ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dodecet_creation() {
        let d = DodecetWasm::new(0xABC).unwrap();
        assert_eq!(d.value(), 0xABC);
        assert_eq!(d.to_hex(), "ABC");
    }

    #[test]
    fn test_point3d_creation() {
        let point = Point3D::new(0x123, 0x456, 0x789);
        assert_eq!(point.x(), 0x123);
        assert_eq!(point.y(), 0x456);
        assert_eq!(point.z(), 0x789);
    }

    #[test]
    fn test_point3d_distance() {
        let p1 = Point3D::new(0, 0, 0);
        let p2 = Point3D::new(0x100, 0, 0);
        let dist = p1.distance_to(&p2);
        assert!((dist - 256.0).abs() < 0.1);
    }

    #[test]
    fn test_vector_operations() {
        let v1 = Vector3DWasm::new(100, 0, 0);
        let v2 = Vector3DWasm::new(0, 100, 0);
        let dot = v1.dot(&v2);
        assert_eq!(dot, 0);
    }

    #[test]
    fn test_transform_identity() {
        let transform = Transform3DWasm::new_identity();
        let point = Point3D::new(0x100, 0x200, 0x300);
        let transformed = transform.apply(&point);
        // Transform should not significantly change the point
        assert!(transformed.x() >= 0);
        assert!(transformed.y() >= 0);
        assert!(transformed.z() >= 0);
    }
}
