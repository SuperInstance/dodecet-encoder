//! # WebAssembly Bindings for Dodecet Encoder
//!
//! This module provides wasm-bindgen exports for browser and Node.js environments.
//! It enables JavaScript/TypeScript code to use the dodecet encoding system directly.

use wasm_bindgen::prelude::*;
use crate::{Dodecet, DodecetArray, DodecetError, Result};

// Import JavaScript types
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "number")]
    pub type JsNumber;
}

/// Convert Rust error to JavaScript error
fn to_js_error(err: DodecetError) -> JsValue {
    JsValue::from_str(&err.to_string())
}

/// A 12-bit dodecet value for JavaScript
#[wasm_bindgen]
pub struct WasmDodecet {
    inner: Dodecet,
}

#[wasm_bindgen]
impl WasmDodecet {
    /// Create a new dodecet from a value (0-4095)
    ///
    /// # Arguments
    /// * `value` - A number between 0 and 4095
    ///
    /// # Returns
    /// A new WasmDodecet instance
    ///
    /// # Throws
    /// Throws an error if value > 4095
    #[wasm_bindgen(constructor)]
    pub fn new(value: u16) -> Result<WasmDodecet, JsValue> {
        Dodecet::new(value)
            .map(|inner| WasmDodecet { inner })
            .map_err(to_js_error)
    }

    /// Create a dodecet from a hex value (unsafe, no validation)
    #[wasm_bindgen(js_name = fromHexUnchecked)]
    pub fn from_hex_unchecked(value: u16) -> WasmDodecet {
        WasmDodecet {
            inner: Dodecet::from_hex(value),
        }
    }

    /// Get the raw value (0-4095)
    pub fn value(&self) -> u16 {
        self.inner.value()
    }

    /// Get a specific nibble (0, 1, or 2)
    ///
    /// # Arguments
    /// * `index` - Nibble index (0 = LSB, 2 = MSB)
    pub fn nibble(&self, index: u8) -> Result<u8, JsValue> {
        self.inner.nibble(index).map_err(to_js_error)
    }

    /// Set a specific nibble
    ///
    /// # Arguments
    /// * `index` - Nibble index (0 = LSB, 2 = MSB)
    /// * `nibble` - New nibble value (0-15)
    #[wasm_bindgen(js_name = setNibble)]
    pub fn set_nibble(&mut self, index: u8, nibble: u8) -> Result<(), JsValue> {
        self.inner.set_nibble(index, nibble).map_err(to_js_error)
    }

    /// Check if dodecet is zero
    #[wasm_bindgen(js_name = isZero)]
    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    /// Check if dodecet is at maximum value
    #[wasm_bindgen(js_name = isMax)]
    pub fn is_max(&self) -> bool {
        self.inner.is_max()
    }

    /// Count set bits (population count)
    #[wasm_bindgen(js_name = countOnes)]
    pub fn count_ones(&self) -> u32 {
        self.inner.count_ones()
    }

    /// Convert to hex string (3 characters)
    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> String {
        self.inner.to_hex_string()
    }

    /// Parse from hex string
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(s: &str) -> Result<WasmDodecet, JsValue> {
        Dodecet::from_hex_str(s)
            .map(|inner| WasmDodecet { inner })
            .map_err(to_js_error)
    }

    /// Convert to binary string (12 characters)
    #[wasm_bindgen(js_name = toBinary)]
    pub fn to_binary(&self) -> String {
        self.inner.to_binary_string()
    }

    /// Geometric interpretation: Treat as signed value (-2048 to 2047)
    #[wasm_bindgen(js_name = asSigned)]
    pub fn as_signed(&self) -> i16 {
        self.inner.as_signed()
    }

    /// Normalize to floating point [0.0, 1.0]
    pub fn normalize(&self) -> f64 {
        self.inner.normalize()
    }

    /// Bitwise AND with another dodecet
    pub fn and(&self, other: &WasmDodecet) -> WasmDodecet {
        WasmDodecet {
            inner: self.inner.and(other.inner),
        }
    }

    /// Bitwise OR with another dodecet
    pub fn or(&self, other: &WasmDodecet) -> WasmDodecet {
        WasmDodecet {
            inner: self.inner.or(other.inner),
        }
    }

    /// Bitwise XOR with another dodecet
    pub fn xor(&self, other: &WasmDodecet) -> WasmDodecet {
        WasmDodecet {
            inner: self.inner.xor(other.inner),
        }
    }

    /// Bitwise NOT
    pub fn not(&self) -> WasmDodecet {
        WasmDodecet {
            inner: self.inner.not(),
        }
    }

    /// Add another dodecet (wrapping)
    pub fn add(&self, other: &WasmDodecet) -> WasmDodecet {
        WasmDodecet {
            inner: self.inner.wrapping_add(other.inner),
        }
    }

    /// Subtract another dodecet (wrapping)
    pub fn sub(&self, other: &WasmDodecet) -> WasmDodecet {
        WasmDodecet {
            inner: self.inner.wrapping_sub(other.inner),
        }
    }

    /// Multiply by another dodecet (wrapping)
    pub fn mul(&self, other: &WasmDodecet) -> WasmDodecet {
        WasmDodecet {
            inner: self.inner.wrapping_mul(other.inner),
        }
    }

    /// Clone the dodecet
    pub fn clone(&self) -> WasmDodecet {
        WasmDodecet {
            inner: self.inner,
        }
    }

    /// Convert to string representation
    pub fn toString(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// A 3D point encoded with dodecets
#[wasm_bindgen]
pub struct WasmPoint3D {
    inner: crate::geometric::Point3D,
}

#[wasm_bindgen]
impl WasmPoint3D {
    /// Create a new 3D point
    ///
    /// # Arguments
    /// * `x` - X coordinate (0-4095)
    /// * `y` - Y coordinate (0-4095)
    /// * `z` - Z coordinate (0-4095)
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16, z: u16) -> WasmPoint3D {
        WasmPoint3D {
            inner: crate::geometric::Point3D::new(x, y, z),
        }
    }

    /// Get x coordinate
    pub fn x(&self) -> u16 {
        self.inner.x()
    }

    /// Get y coordinate
    pub fn y(&self) -> u16 {
        self.inner.y()
    }

    /// Get z coordinate
    pub fn z(&self) -> u16 {
        self.inner.z()
    }

    /// Convert to normalized floating point coordinates [0.0, 1.0]
    ///
    /// Returns a JavaScript object with x, y, z properties
    #[wasm_bindgen(js_name = normalized)]
    pub fn normalized(&self) -> JsValue {
        let (x, y, z) = self.inner.normalized();
        let result = js_sys::Object::new();
        js_sys::Reflect::set(&result, &"x".into(), &x.into()).unwrap();
        js_sys::Reflect::set(&result, &"y".into(), &y.into()).unwrap();
        js_sys::Reflect::set(&result, &"z".into(), &z.into()).unwrap();
        result.into()
    }

    /// Convert to signed coordinates [-2048, 2047]
    ///
    /// Returns a JavaScript object with x, y, z properties
    #[wasm_bindgen(js_name = signed)]
    pub fn signed(&self) -> JsValue {
        let (x, y, z) = self.inner.signed();
        let result = js_sys::Object::new();
        js_sys::Reflect::set(&result, &"x".into(), &x.into()).unwrap();
        js_sys::Reflect::set(&result, &"y".into(), &y.into()).unwrap();
        js_sys::Reflect::set(&result, &"z".into(), &z.into()).unwrap();
        result.into()
    }

    /// Calculate distance to another point
    #[wasm_bindgen(js_name = distanceTo)]
    pub fn distance_to(&self, other: &WasmPoint3D) -> f64 {
        self.inner.distance_to(&other.inner)
    }

    /// Convert to hex string
    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> String {
        self.inner.to_hex_string()
    }

    /// Parse from hex string
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(s: &str) -> Result<WasmPoint3D, JsValue> {
        crate::geometric::Point3D::from_hex_str(s)
            .map(|inner| WasmPoint3D { inner })
            .map_err(to_js_error)
    }

    /// Clone the point
    pub fn clone(&self) -> WasmPoint3D {
        WasmPoint3D {
            inner: self.inner.clone(),
        }
    }
}

/// A 3D vector encoded with dodecets
#[wasm_bindgen]
pub struct WasmVector3D {
    inner: crate::geometric::Vector3D,
}

#[wasm_bindgen]
impl WasmVector3D {
    /// Create a new 3D vector
    ///
    /// # Arguments
    /// * `x` - X component (signed)
    /// * `y` - Y component (signed)
    /// * `z` - Z component (signed)
    #[wasm_bindgen(constructor)]
    pub fn new(x: i16, y: i16, z: i16) -> WasmVector3D {
        WasmVector3D {
            inner: crate::geometric::Vector3D::new(x, y, z),
        }
    }

    /// Get x component
    pub fn x(&self) -> i16 {
        self.inner.x()
    }

    /// Get y component
    pub fn y(&self) -> i16 {
        self.inner.y()
    }

    /// Get z component
    pub fn z(&self) -> i16 {
        self.inner.z()
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> f64 {
        self.inner.magnitude()
    }

    /// Normalize to unit vector
    ///
    /// Returns a JavaScript object with x, y, z properties
    pub fn normalize(&self) -> JsValue {
        let (x, y, z) = self.inner.normalize();
        let result = js_sys::Object::new();
        js_sys::Reflect::set(&result, &"x".into(), &x.into()).unwrap();
        js_sys::Reflect::set(&result, &"y".into(), &y.into()).unwrap();
        js_sys::Reflect::set(&result, &"z".into(), &z.into()).unwrap();
        result.into()
    }

    /// Dot product with another vector
    pub fn dot(&self, other: &WasmVector3D) -> i32 {
        self.inner.dot(&other.inner)
    }

    /// Cross product with another vector
    pub fn cross(&self, other: &WasmVector3D) -> WasmVector3D {
        WasmVector3D {
            inner: self.inner.cross(&other.inner),
        }
    }

    /// Add two vectors
    pub fn add(&self, other: &WasmVector3D) -> WasmVector3D {
        WasmVector3D {
            inner: self.inner.add(&other.inner),
        }
    }

    /// Subtract two vectors
    pub fn sub(&self, other: &WasmVector3D) -> WasmVector3D {
        WasmVector3D {
            inner: self.inner.sub(&other.inner),
        }
    }

    /// Scale by a scalar
    pub fn scale(&self, scalar: f64) -> WasmVector3D {
        WasmVector3D {
            inner: self.inner.scale(scalar),
        }
    }

    /// Clone the vector
    pub fn clone(&self) -> WasmVector3D {
        WasmVector3D {
            inner: self.inner.clone(),
        }
    }
}

/// Utility functions for dodecet operations
#[wasm_bindgen]
pub struct DodecetUtils;

#[wasm_bindgen]
impl DodecetUtils {
    /// Maximum value of a dodecet (4095)
    #[wasm_bindgen(js_name = MAX_DODECET)]
    pub fn max_dodecet() -> u16 {
        crate::MAX_DODECET
    }

    /// Number of bits in a dodecet (12)
    #[wasm_bindgen(js_name = DODECET_BITS)]
    pub fn dodecet_bits() -> u8 {
        crate::DODECET_BITS
    }

    /// Number of nibbles in a dodecet (3)
    #[wasm_bindgen(js_name = NIBBLES)]
    pub fn nibbles() -> u8 {
        crate::NIBBLES
    }

    /// Number of values a dodecet can represent (4096)
    #[wasm_bindgen(js_name = CAPACITY)]
    pub fn capacity() -> u16 {
        crate::CAPACITY
    }

    /// Encode a floating point value [0.0, 1.0] to dodecet
    #[wasm_bindgen(js_name = encodeFloat)]
    pub fn encode_float(value: f64) -> WasmDodecet {
        let clamped = value.clamp(0.0, 1.0);
        let encoded = (clamped * 4095.0) as u16;
        WasmDodecet {
            inner: Dodecet::from_hex(encoded),
        }
    }

    /// Decode a dodecet to floating point [0.0, 1.0]
    #[wasm_bindgen(js_name = decodeFloat)]
    pub fn decode_float(dodecet: &WasmDodecet) -> f64 {
        dodecet.inner.normalize()
    }

    /// Batch encode an array of floating point values
    #[wasm_bindgen(js_name = encodeFloatArray)]
    pub fn encode_float_array(values: &js_sys::Array) -> JsValue {
        let result = js_sys::Array::new();
        for value in values.iter() {
            if let Some(num) = value.as_f64() {
                let encoded = Self::encode_float(num);
                result.push(&encoded.into());
            }
        }
        result.into()
    }

    /// Batch decode an array of dodecets
    #[wasm_bindgen(js_name = decodeDodecetArray)]
    pub fn decode_dodecet_array(dodecets: &js_sys::Array) -> JsValue {
        let result = js_sys::Array::new();
        for value in dodecets.iter() {
            if let Some(dodecet) = value.dyn_ref::<WasmDodecet>() {
                result.push(&Self::decode_float(dodecet).into());
            }
        }
        result.into()
    }
}

// Export constants
#[wasm_bindgen]
pub const MAX_DODECET: u16 = 4095;

#[wasm_bindgen]
pub const DODECET_BITS: u8 = 12;

#[wasm_bindgen]
pub const NIBBLES: u8 = 3;

#[wasm_bindgen]
pub const CAPACITY: u16 = 4096;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_dodecet_creation() {
        let d = WasmDodecet::new(0xABC).unwrap();
        assert_eq!(d.value(), 0xABC);
    }

    #[test]
    fn test_wasm_point3d() {
        let point = WasmPoint3D::new(0x100, 0x200, 0x300);
        assert_eq!(point.x(), 0x100);
        assert_eq!(point.y(), 0x200);
        assert_eq!(point.z(), 0x300);
    }

    #[test]
    fn test_wasm_vector3d() {
        let v = WasmVector3D::new(100, 200, 300);
        assert_eq!(v.x(), 100);
        assert_eq!(v.y(), 200);
        assert_eq!(v.z(), 300);
    }

    #[test]
    fn test_utils_encode_decode() {
        let original = 0.5;
        let encoded = DodecetUtils::encode_float(original);
        let decoded = DodecetUtils::decode_float(&encoded);
        assert!((decoded - original).abs() < 0.001);
    }
}
