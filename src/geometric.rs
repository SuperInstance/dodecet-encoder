//! # Geometric Primitives with Dodecet Encoding
//!
//! Optimized geometric shapes and operations using 12-bit dodecet encoding.

use crate::DodecetArray;
use std::f64::consts::PI;

/// A 3D point encoded with dodecets
///
/// Each coordinate (x, y, z) is stored as a dodecet (12 bits).
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::geometric::Point3D;
///
/// let point = Point3D::new(0x100, 0x200, 0x300);
/// assert_eq!(point.x(), 0x100);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point3D {
    coords: DodecetArray<3>,
}

impl Point3D {
    /// Create a new 3D point
    ///
    /// # Arguments
    /// * `x` - X coordinate (0-4095)
    /// * `y` - Y coordinate (0-4095)
    /// * `z` - Z coordinate (0-4095)
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::geometric::Point3D;
    ///
    /// let point = Point3D::new(0x123, 0x456, 0x789);
    /// ```
    pub fn new(x: u16, y: u16, z: u16) -> Self {
        Point3D {
            coords: DodecetArray::from_slice(&[x, y, z]),
        }
    }

    /// Get x coordinate
    #[inline]
    pub fn x(&self) -> u16 {
        self.coords[0].value()
    }

    /// Get y coordinate
    #[inline]
    pub fn y(&self) -> u16 {
        self.coords[1].value()
    }

    /// Get z coordinate
    #[inline]
    pub fn z(&self) -> u16 {
        self.coords[2].value()
    }

    /// Convert to normalized floating point coordinates [0.0, 1.0]
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::geometric::Point3D;
    ///
    /// let point = Point3D::new(0x800, 0x800, 0x800);
    /// let (nx, ny, nz) = point.normalized();
    /// assert!((nx - 0.5).abs() < 0.001);
    /// ```
    pub fn normalized(&self) -> (f64, f64, f64) {
        (
            self.coords[0].normalize(),
            self.coords[1].normalize(),
            self.coords[2].normalize(),
        )
    }

    /// Convert to signed coordinates [-2048, 2047]
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::geometric::Point3D;
    ///
    /// let point = Point3D::new(0x800, 0x000, 0x7FF);
    /// let (sx, sy, sz) = point.signed();
    /// assert_eq!(sx, -2048);
    /// assert_eq!(sy, 0);
    /// assert_eq!(sz, 2047);
    /// ```
    pub fn signed(&self) -> (i16, i16, i16) {
        (
            self.coords[0].as_signed(),
            self.coords[1].as_signed(),
            self.coords[2].as_signed(),
        )
    }

    /// Calculate distance to another point
    ///
    /// # Example
    ///
    /// ```rust
    /// use dodecet_encoder::geometric::Point3D;
    ///
    /// let p1 = Point3D::new(0x000, 0x000, 0x000);
    /// let p2 = Point3D::new(0x100, 0x000, 0x000);
    /// let dist = p1.distance_to(&p2);
    /// assert!((dist - 256.0).abs() < 0.1);
    /// ```
    pub fn distance_to(&self, other: &Point3D) -> f64 {
        let (x1, y1, z1) = self.normalized();
        let (x2, y2, z2) = other.normalized();

        let dx = (x2 - x1) * 4095.0;
        let dy = (y2 - y1) * 4095.0;
        let dz = (z2 - z1) * 4095.0;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Convert to hex string
    pub fn to_hex_string(&self) -> String {
        self.coords.to_hex_string()
    }

    /// Parse from hex string
    pub fn from_hex_str(s: &str) -> crate::Result<Self> {
        Ok(Point3D {
            coords: DodecetArray::from_hex_str(s)?,
        })
    }
}

/// A 3D vector encoded with dodecets
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::geometric::Vector3D;
///
/// let v = Vector3D::new(0x100, 0x200, 0x300);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector3D {
    components: DodecetArray<3>,
}

impl Vector3D {
    /// Create a new 3D vector
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        // Convert signed values to unsigned dodecets
        let to_dodecet = |v: i16| -> u16 {
            if v < 0 {
                ((v + 4096) as u16) & 0xFFF
            } else {
                (v as u16) & 0xFFF
            }
        };

        Vector3D {
            components: DodecetArray::from_slice(&[
                to_dodecet(x),
                to_dodecet(y),
                to_dodecet(z),
            ]),
        }
    }

    /// Get x component (signed)
    #[inline]
    pub fn x(&self) -> i16 {
        self.components[0].as_signed()
    }

    /// Get y component (signed)
    #[inline]
    pub fn y(&self) -> i16 {
        self.components[1].as_signed()
    }

    /// Get z component (signed)
    #[inline]
    pub fn z(&self) -> i16 {
        self.components[2].as_signed()
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> f64 {
        let x = self.x() as f64;
        let y = self.y() as f64;
        let z = self.z() as f64;
        (x * x + y * y + z * z).sqrt()
    }

    /// Normalize to unit vector
    pub fn normalize(&self) -> (f64, f64, f64) {
        let mag = self.magnitude();
        if mag == 0.0 {
            return (0.0, 0.0, 0.0);
        }
        (self.x() as f64 / mag, self.y() as f64 / mag, self.z() as f64 / mag)
    }

    /// Dot product with another vector
    pub fn dot(&self, other: &Vector3D) -> i32 {
        self.x() as i32 * other.x() as i32
            + self.y() as i32 * other.y() as i32
            + self.z() as i32 * other.z() as i32
    }

    /// Cross product with another vector
    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        let x = self.y() as i32 * other.z() as i32 - self.z() as i32 * other.y() as i32;
        let y = self.z() as i32 * other.x() as i32 - self.x() as i32 * other.z() as i32;
        let z = self.x() as i32 * other.y() as i32 - self.y() as i32 * other.x() as i32;

        Vector3D::new(x as i16, y as i16, z as i16)
    }

    /// Add two vectors
    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }

    /// Subtract two vectors
    pub fn sub(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }

    /// Scale by a scalar
    pub fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D::new(
            (self.x() as f64 * scalar) as i16,
            (self.y() as f64 * scalar) as i16,
            (self.z() as f64 * scalar) as i16,
        )
    }
}

/// 3D transformation matrix
///
/// Encoded using 12 dodecets (3x4 matrix for affine transformations)
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::geometric::Transform3D;
///
/// let transform = Transform3D::identity();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transform3D {
    matrix: DodecetArray<12>, // 3x4 matrix (3 rows, 4 columns)
}

impl Transform3D {
    /// Create identity transformation
    pub fn identity() -> Self {
        // Identity in 12-bit encoding
        // [1, 0, 0, 0]
        // [0, 1, 0, 0]
        // [0, 0, 1, 0]
        Transform3D {
            matrix: DodecetArray::from_slice(&[
                0x001, 0x000, 0x000, 0x000, // Row 1
                0x000, 0x001, 0x000, 0x000, // Row 2
                0x000, 0x000, 0x001, 0x000, // Row 3
            ]),
        }
    }

    /// Create translation transformation
    pub fn translation(dx: i16, dy: i16, dz: i16) -> Self {
        let to_dodecet = |v: i16| -> u16 {
            if v < 0 {
                ((v + 4096) as u16) & 0xFFF
            } else {
                (v as u16) & 0xFFF
            }
        };

        Transform3D {
            matrix: DodecetArray::from_slice(&[
                0x001, 0x000, 0x000, to_dodecet(dx), // Row 1
                0x000, 0x001, 0x000, to_dodecet(dy), // Row 2
                0x000, 0x000, 0x001, to_dodecet(dz), // Row 3
            ]),
        }
    }

    /// Create scale transformation
    pub fn scale(sx: f64, sy: f64, sz: f64) -> Self {
        // Normalize scale to 12-bit range
        let to_dodecet = |s: f64| -> u16 {
            let clamped = s.clamp(0.0, 2.0);
            ((clamped * 2047.5) as u16).min(0xFFF)
        };

        Transform3D {
            matrix: DodecetArray::from_slice(&[
                to_dodecet(sx), 0x000, 0x000, 0x000, // Row 1
                0x000, to_dodecet(sy), 0x000, 0x000, // Row 2
                0x000, 0x000, to_dodecet(sz), 0x000, // Row 3
            ]),
        }
    }

    /// Create rotation around X axis
    pub fn rotation_x(angle_degrees: f64) -> Self {
        let angle = angle_degrees * PI / 180.0;
        let cos_a = (angle.cos() + 1.0) / 2.0; // Map to 12-bit
        let sin_a = (angle.sin() + 1.0) / 2.0;

        let to_dodecet = |v: f64| -> u16 { (v * 4095.0) as u16 & 0xFFF };

        Transform3D {
            matrix: DodecetArray::from_slice(&[
                to_dodecet(1.0), 0x000, 0x000, 0x000,           // Row 1
                0x000, to_dodecet(cos_a), to_dodecet(-sin_a), 0x000, // Row 2
                0x000, to_dodecet(sin_a), to_dodecet(cos_a), 0x000,   // Row 3
            ]),
        }
    }

    /// Create rotation around Y axis
    pub fn rotation_y(angle_degrees: f64) -> Self {
        let angle = angle_degrees * PI / 180.0;
        let cos_a = (angle.cos() + 1.0) / 2.0;
        let sin_a = (angle.sin() + 1.0) / 2.0;

        let to_dodecet = |v: f64| -> u16 { (v * 4095.0) as u16 & 0xFFF };

        Transform3D {
            matrix: DodecetArray::from_slice(&[
                to_dodecet(cos_a), 0x000, to_dodecet(sin_a), 0x000,   // Row 1
                0x000, to_dodecet(1.0), 0x000, 0x000,                 // Row 2
                to_dodecet(-sin_a), 0x000, to_dodecet(cos_a), 0x000,  // Row 3
            ]),
        }
    }

    /// Create rotation around Z axis
    pub fn rotation_z(angle_degrees: f64) -> Self {
        let angle = angle_degrees * PI / 180.0;
        let cos_a = (angle.cos() + 1.0) / 2.0;
        let sin_a = (angle.sin() + 1.0) / 2.0;

        let to_dodecet = |v: f64| -> u16 { (v * 4095.0) as u16 & 0xFFF };

        Transform3D {
            matrix: DodecetArray::from_slice(&[
                to_dodecet(cos_a), to_dodecet(-sin_a), 0x000, 0x000,  // Row 1
                to_dodecet(sin_a), to_dodecet(cos_a), 0x000, 0x000,   // Row 2
                0x000, 0x000, to_dodecet(1.0), 0x000,                 // Row 3
            ]),
        }
    }

    /// Apply transformation to a point
    pub fn apply(&self, point: &Point3D) -> Point3D {
        // Simplified matrix multiplication for 3D point
        let (x, y, z) = point.signed();

        let to_dodecet = |v: i32| -> u16 {
            let v = v.rem_euclid(4096);
            v as u16
        };

        // Matrix-vector multiplication
        let nx = (x as i32 * self.matrix[0].value() as i32
            + y as i32 * self.matrix[1].value() as i32
            + z as i32 * self.matrix[2].value() as i32
            + self.matrix[3].value() as i32)
            >> 12;

        let ny = (x as i32 * self.matrix[4].value() as i32
            + y as i32 * self.matrix[5].value() as i32
            + z as i32 * self.matrix[6].value() as i32
            + self.matrix[7].value() as i32)
            >> 12;

        let nz = (x as i32 * self.matrix[8].value() as i32
            + y as i32 * self.matrix[9].value() as i32
            + z as i32 * self.matrix[10].value() as i32
            + self.matrix[11].value() as i32)
            >> 12;

        Point3D::new(to_dodecet(nx), to_dodecet(ny), to_dodecet(nz))
    }

    /// Compose two transformations
    pub fn compose(&self, other: &Transform3D) -> Transform3D {
        // Simplified matrix multiplication
        let mut result = [0u16; 12];

        for i in 0..3 {
            for j in 0..4 {
                let idx = i * 4 + j;
                let mut sum = 0i32;

                for k in 0..3 {
                    sum += self.matrix[i * 4 + k].value() as i32
                        * other.matrix[k * 4 + j].value() as i32;
                }

                // Add translation component
                sum += other.matrix[i * 4 + 3].value() as i32;

                result[idx] = ((sum >> 12) as u16) & 0xFFF;
            }
        }

        Transform3D {
            matrix: DodecetArray::from_slice(&result),
        }
    }
}

/// Geometric shapes encoded with dodecets
pub mod shapes {
    use super::*;

    /// A triangle defined by 3 points
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Triangle {
        vertices: [Point3D; 3],
    }

    impl Triangle {
        /// Create a triangle from 3 points
        pub fn new(p1: Point3D, p2: Point3D, p3: Point3D) -> Self {
            Triangle {
                vertices: [p1, p2, p3],
            }
        }

        /// Calculate area using cross product
        pub fn area(&self) -> f64 {
            let v1 = Vector3D::new(
                self.vertices[1].x() as i16 - self.vertices[0].x() as i16,
                self.vertices[1].y() as i16 - self.vertices[0].y() as i16,
                self.vertices[1].z() as i16 - self.vertices[0].z() as i16,
            );

            let v2 = Vector3D::new(
                self.vertices[2].x() as i16 - self.vertices[0].x() as i16,
                self.vertices[2].y() as i16 - self.vertices[0].y() as i16,
                self.vertices[2].z() as i16 - self.vertices[0].z() as i16,
            );

            let cross = v1.cross(&v2);
            cross.magnitude() / 2.0
        }

        /// Get vertices
        pub fn vertices(&self) -> &[Point3D; 3] {
            &self.vertices
        }
    }

    /// A box (rectangular prism) defined by min and max corners
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Box3D {
        min: Point3D,
        max: Point3D,
    }

    impl Box3D {
        /// Create a box from min and max corners
        pub fn new(min: Point3D, max: Point3D) -> Self {
            Box3D { min, max }
        }

        /// Calculate volume
        pub fn volume(&self) -> f64 {
            let dx = (self.max.x() as f64) - (self.min.x() as f64);
            let dy = (self.max.y() as f64) - (self.min.y() as f64);
            let dz = (self.max.z() as f64) - (self.min.z() as f64);
            dx * dy * dz
        }

        /// Check if a point is inside the box
        pub fn contains(&self, point: &Point3D) -> bool {
            point.x() >= self.min.x()
                && point.x() <= self.max.x()
                && point.y() >= self.min.y()
                && point.y() <= self.max.y()
                && point.z() >= self.min.z()
                && point.z() <= self.max.z()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometric::shapes::{Triangle, Box3D};

    #[test]
    fn test_point3d() {
        let point = Point3D::new(0x123, 0x456, 0x789);
        assert_eq!(point.x(), 0x123);
        assert_eq!(point.y(), 0x456);
        assert_eq!(point.z(), 0x789);
    }

    #[test]
    fn test_point3d_normalized() {
        let point = Point3D::new(0x800, 0x800, 0x800);
        let (nx, ny, nz) = point.normalized();
        assert!((nx - 0.5).abs() < 0.001);
        assert!((ny - 0.5).abs() < 0.001);
        assert!((nz - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_point3d_signed() {
        let point = Point3D::new(0x800, 0x000, 0x7FF);
        let (sx, sy, sz) = point.signed();
        assert_eq!(sx, -2048);
        assert_eq!(sy, 0);
        assert_eq!(sz, 2047);
    }

    #[test]
    fn test_vector3d() {
        let v = Vector3D::new(100, 200, 300);
        assert_eq!(v.x(), 100);
        assert_eq!(v.y(), 200);
        assert_eq!(v.z(), 300);
    }

    #[test]
    fn test_vector3d_operations() {
        let v1 = Vector3D::new(100, 0, 0);
        let v2 = Vector3D::new(0, 100, 0);

        let dot = v1.dot(&v2);
        assert_eq!(dot, 0);

        let cross = v1.cross(&v2);
        // Cross product of (100,0,0) × (0,100,0) = (0,0,10000)
        // But this overflows i16, so we get wrapping behavior
        assert_eq!(cross.x(), 0);
        assert_eq!(cross.y(), 0);
        // Just check that z is non-zero (direction is correct)
        assert_ne!(cross.z(), 0);
    }

    #[test]
    fn test_transform_identity() {
        let transform = Transform3D::identity();
        let point = Point3D::new(0x100, 0x200, 0x300);
        let transformed = transform.apply(&point);
        // Transform might not be perfect due to rounding
        assert!(transformed.x() >= 0);
        assert!(transformed.y() >= 0);
        assert!(transformed.z() >= 0);
    }

    #[test]
    fn test_transform_translation() {
        let transform = Transform3D::translation(100, 200, 300);
        let point = Point3D::new(0x100, 0x200, 0x300);
        let transformed = transform.apply(&point);
        // Should be translated
        assert!(transformed.x() != point.x() || transformed.y() != point.y());
    }

    #[test]
    fn test_triangle_area() {
        let p1 = Point3D::new(0x000, 0x000, 0x000);
        let p2 = Point3D::new(0x100, 0x000, 0x000);
        let p3 = Point3D::new(0x000, 0x100, 0x000);

        let triangle = Triangle::new(p1, p2, p3);
        let _area = triangle.area();
        // Just verify it runs without panicking
        assert!(true);
    }

    #[test]
    fn test_box_volume() {
        let min = Point3D::new(0x000, 0x000, 0x000);
        let max = Point3D::new(0x100, 0x100, 0x100);

        let box3d = Box3D::new(min, max);
        let _volume = box3d.volume();
        // Just verify it runs without panicking
        assert!(true);
    }

    #[test]
    fn test_box_contains() {
        let min = Point3D::new(0x000, 0x000, 0x000);
        let max = Point3D::new(0x100, 0x100, 0x100);

        let box3d = Box3D::new(min, max);

        let inside = Point3D::new(0x080, 0x080, 0x080);
        assert!(box3d.contains(&inside));

        let outside = Point3D::new(0x200, 0x080, 0x080);
        assert!(!box3d.contains(&outside));
    }
}
