/**
 * @superinstance/dodecet-encoder
 *
 * A 12-bit dodecet encoding system optimized for geometric and calculus operations.
 */

/**
 * Maximum value of a dodecet (4095)
 */
export const MAX_DODECET: number;

/**
 * Number of bits in a dodecet (12)
 */
export const DODECET_BITS: number;

/**
 * Number of nibbles in a dodecet (3)
 */
export const NIBBLES: number;

/**
 * Number of values a dodecet can represent (4096)
 */
export const CAPACITY: number;

/**
 * A 12-bit dodecet value (0-4095)
 */
export class WasmDodecet {
  /**
   * Create a new dodecet from a value (0-4095)
   * @param value - A number between 0 and 4095
   * @throws Error if value > 4095
   */
  constructor(value: number);

  /**
   * Get the raw value (0-4095)
   */
  value(): number;

  /**
   * Get a specific nibble (0, 1, or 2)
   * @param index - Nibble index (0 = LSB, 2 = MSB)
   * @throws Error if index > 2
   */
  nibble(index: number): number;

  /**
   * Set a specific nibble
   * @param index - Nibble index (0 = LSB, 2 = MSB)
   * @param nibble - New nibble value (0-15)
   * @throws Error if index > 2 or nibble > 15
   */
  setNibble(index: number, nibble: number): void;

  /**
   * Check if dodecet is zero
   */
  isZero(): boolean;

  /**
   * Check if dodecet is at maximum value
   */
  isMax(): boolean;

  /**
   * Count set bits (population count)
   */
  countOnes(): number;

  /**
   * Convert to hex string (3 characters)
   */
  toHex(): string;

  /**
   * Parse from hex string
   * @param s - Hex string (e.g., "ABC")
   * @throws Error if invalid hex
   */
  static fromHex(s: string): WasmDodecet;

  /**
   * Convert to binary string (12 characters)
   */
  toBinary(): string;

  /**
   * Geometric interpretation: Treat as signed value (-2048 to 2047)
   */
  asSigned(): number;

  /**
   * Normalize to floating point [0.0, 1.0]
   */
  normalize(): number;

  /**
   * Bitwise AND with another dodecet
   */
  and(other: WasmDodecet): WasmDodecet;

  /**
   * Bitwise OR with another dodecet
   */
  or(other: WasmDodecet): WasmDodecet;

  /**
   * Bitwise XOR with another dodecet
   */
  xor(other: WasmDodecet): WasmDodecet;

  /**
   * Bitwise NOT
   */
  not(): WasmDodecet;

  /**
   * Add another dodecet (wrapping)
   */
  add(other: WasmDodecet): WasmDodecet;

  /**
   * Subtract another dodecet (wrapping)
   */
  sub(other: WasmDodecet): WasmDodecet;

  /**
   * Multiply by another dodecet (wrapping)
   */
  mul(other: WasmDodecet): WasmDodecet;

  /**
   * Clone the dodecet
   */
  clone(): WasmDodecet;

  /**
   * Convert to string representation
   */
  toString(): string;
}

/**
 * A 3D point encoded with dodecets
 */
export class WasmPoint3D {
  /**
   * Create a new 3D point
   * @param x - X coordinate (0-4095)
   * @param y - Y coordinate (0-4095)
   * @param z - Z coordinate (0-4095)
   */
  constructor(x: number, y: number, z: number);

  /**
   * Get x coordinate
   */
  x(): number;

  /**
   * Get y coordinate
   */
  y(): number;

  /**
   * Get z coordinate
   */
  z(): number;

  /**
   * Convert to normalized floating point coordinates [0.0, 1.0]
   */
  normalized(): { x: number; y: number; z: number };

  /**
   * Convert to signed coordinates [-2048, 2047]
   */
  signed(): { x: number; y: number; z: number };

  /**
   * Calculate distance to another point
   */
  distanceTo(other: WasmPoint3D): number;

  /**
   * Convert to hex string
   */
  toHex(): string;

  /**
   * Parse from hex string
   */
  static fromHex(s: string): WasmPoint3D;

  /**
   * Clone the point
   */
  clone(): WasmPoint3D;
}

/**
 * A 3D vector encoded with dodecets
 */
export class WasmVector3D {
  /**
   * Create a new 3D vector
   * @param x - X component (signed)
   * @param y - Y component (signed)
   * @param z - Z component (signed)
   */
  constructor(x: number, y: number, z: number);

  /**
   * Get x component
   */
  x(): number;

  /**
   * Get y component
   */
  y(): number;

  /**
   * Get z component
   */
  z(): number;

  /**
   * Calculate magnitude
   */
  magnitude(): number;

  /**
   * Normalize to unit vector
   */
  normalize(): { x: number; y: number; z: number };

  /**
   * Dot product with another vector
   */
  dot(other: WasmVector3D): number;

  /**
   * Cross product with another vector
   */
  cross(other: WasmVector3D): WasmVector3D;

  /**
   * Add two vectors
   */
  add(other: WasmVector3D): WasmVector3D;

  /**
   * Subtract two vectors
   */
  sub(other: WasmVector3D): WasmVector3D;

  /**
   * Scale by a scalar
   */
  scale(scalar: number): WasmVector3D;

  /**
   * Clone the vector
   */
  clone(): WasmVector3D;
}

/**
 * Utility functions for dodecet operations
 */
export class DodecetUtils {
  /**
   * Maximum value of a dodecet (4095)
   */
  static MAX_DODECET(): number;

  /**
   * Number of bits in a dodecet (12)
   */
  static DODECET_BITS(): number;

  /**
   * Number of nibbles in a dodecet (3)
   */
  static NIBBLES(): number;

  /**
   * Number of values a dodecet can represent (4096)
   */
  static CAPACITY(): number;

  /**
   * Encode a floating point value [0.0, 1.0] to dodecet
   */
  static encodeFloat(value: number): WasmDodecet;

  /**
   * Decode a dodecet to floating point [0.0, 1.0]
   */
  static decodeFloat(dodecet: WasmDodecet): number;

  /**
   * Batch encode an array of floating point values
   */
  static encodeFloatArray(values: number[]): WasmDodecet[];

  /**
   * Batch decode an array of dodecets
   */
  static decodeDodecetArray(dodecets: WasmDodecet[]): number[];
}

/**
 * Initialize the WASM module
 * This must be called before using any other functions
 *
 * @param moduleUrl - URL to the WASM module (optional, defaults to bundled WASM)
 * @returns Promise that resolves when the module is loaded
 */
export function init(moduleUrl?: string): Promise<void>;

/**
 * Load the WASM module from a URL
 *
 * @param url - URL to the WASM module
 * @returns Promise that resolves when the module is loaded
 */
export function load(url: string): Promise<void>;
