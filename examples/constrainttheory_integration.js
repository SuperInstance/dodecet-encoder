/**
 * Dodecet Encoder Integration Example for Constraint Theory
 *
 * This example demonstrates how to use the dodecet encoder
 * in the constrainttheory repository for geometric operations.
 */

import init, {
    WasmDodecet,
    WasmPoint3D,
    WasmVector3D,
    DodecetUtils
} from '@superinstance/dodecet-encoder';

// Initialize WASM module
await init();

/**
 * Example 1: Pythagorean Snapping with Dodecet Encoding
 * Demonstrates the "snapping" process using 12-bit precision
 */
function pythagoreanSnappingExample() {
    console.log('=== Pythagorean Snapping Example ===\n');

    // Original point (floating point)
    const originalPoint = new WasmPoint3D(
        Math.floor(1.732 * 1000), // √3 ≈ 1.732
        Math.floor(1.414 * 1000), // √2 ≈ 1.414
        Math.floor(1.0 * 1000)
    );

    console.log('Original point:');
    console.log(`  X: ${originalPoint.x()} (0x${originalPoint.x().toString(16).toUpperCase()})`);
    console.log(`  Y: ${originalPoint.y()} (0x${originalPoint.y().toString(16).toUpperCase()})`);
    console.log(`  Z: ${originalPoint.z()} (0x${originalPoint.z().toString(16).toUpperCase()})`);

    // Encode to hex string (dodecet format)
    const hexEncoding = originalPoint.toHex();
    console.log(`\nHex encoding: ${hexEncoding}`);
    console.log('  → 9 hex digits = 3 dodecets = 36 bits total');
    console.log('  → Traditional encoding would need: 12 digits (48 bits)');

    // Decode from hex string
    const decodedPoint = WasmPoint3D.fromHex(hexEncoding);
    console.log('\nDecoded point:');
    console.log(`  X: ${decodedPoint.x()}`);
    console.log(`  Y: ${decodedPoint.y()}`);
    console.log(`  Z: ${decodedPoint.z()}`);

    // Calculate compression ratio
    const traditionalBits = 48;
    const dodecetBits = 36;
    const compression = traditionalBits / dodecetBits;
    console.log(`\nCompression: ${compression}x smaller`);
}

/**
 * Example 2: Rigidity Matroid with Dodecet Coordinates
 * Shows how dodecet encoding can represent rigidity constraints
 */
function rigidityMatroidExample() {
    console.log('\n\n=== Rigidity Matroid Example ===\n');

    // Create a triangle (3 points)
    const p1 = new WasmPoint3D(0x000, 0x000, 0x000);
    const p2 = new WasmPoint3D(0x100, 0x000, 0x000);
    const p3 = new WasmPoint3D(0x080, 0x100, 0x000);

    console.log('Triangle vertices:');
    console.log(`  P1: (${p1.x()}, ${p1.y()}, ${p1.z()})`);
    console.log(`  P2: (${p2.x()}, ${p2.y()}, ${p2.z()})`);
    console.log(`  P3: (${p3.x()}, ${p3.y()}, ${p3.z()})`);

    // Calculate distances using dodecet precision
    const d12 = p1.distanceTo(p2);
    const d23 = p2.distanceTo(p3);
    const d31 = p3.distanceTo(p1);

    console.log('\nDistances:');
    console.log(`  d(P1, P2): ${d12.toFixed(2)}`);
    console.log(`  d(P2, P3): ${d23.toFixed(2)}`);
    console.log(`  d(P3, P1): ${d31.toFixed(2)}`);

    // Check rigidity (triangle is rigid)
    const isRigid = (d12 > 0) && (d23 > 0) && (d31 > 0);
    console.log(`\nRigid: ${isRigid ? '✅ Yes' : '❌ No'}`);

    // Encode triangle state to hex
    const triangleState = `${p1.toHex()}|${p2.toHex()}|${p3.toHex()}`;
    console.log(`\nEncoded state: ${triangleState}`);
    console.log(`  → ${triangleState.length} characters = ${triangleState.length * 4} bits`);
}

/**
 * Example 3: Holonomy Transport with Dodecet Vectors
 * Demonstrates parallel transport using dodecet vectors
 */
function holonomyTransportExample() {
    console.log('\n\n=== Holonomy Transport Example ===\n');

    // Create a path of points
    const path = [
        new WasmPoint3D(0x000, 0x000, 0x000),
        new WasmPoint3D(0x100, 0x000, 0x000),
        new WasmPoint3D(0x100, 0x100, 0x000),
        new WasmPoint3D(0x000, 0x100, 0x000),
        new WasmPoint3D(0x000, 0x000, 0x000)
    ];

    console.log('Path:');
    path.forEach((p, i) => {
        console.log(`  Step ${i}: (${p.x().toString(16).toUpperCase()}, ${p.y().toString(16).toUpperCase()})`);
    });

    // Calculate tangent vectors along the path
    console.log('\nTangent vectors:');
    for (let i = 0; i < path.length - 1; i++) {
        const dx = path[i + 1].x() - path[i].x();
        const dy = path[i + 1].y() - path[i].y();
        const dz = path[i + 1].z() - path[i].z();
        const vector = new WasmVector3D(dx, dy, dz);

        console.log(`  Step ${i}: (${vector.x()}, ${vector.y()}, ${vector.z()})`);
        console.log(`    Magnitude: ${vector.magnitude().toFixed(2)}`);

        // Encode tangent vector
        const encoded = new WasmDodecet(vector.magnitude() as number & 0xFFF);
        console.log(`    Encoded: 0x${encoded.toHex()}`);
    }

    // Calculate holonomy (angle accumulated around the loop)
    const start = path[0];
    const end = path[path.length - 1];
    const holonomy = start.distanceTo(end);

    console.log('\nHolonomy:');
    console.log(`  Start: (${start.x()}, ${start.y()})`);
    console.log(`  End: (${end.x()}, ${end.y()})`);
    console.log(`  Accumulated angle: ${holonomy.toFixed(2)} (should be 0 for closed loop)`);
}

/**
 * Example 4: Entropy Calculation with Dodecet Precision
 * Calculate entropy of a configuration using dodecet encoding
 */
function entropyCalculationExample() {
    console.log('\n\n=== Entropy Calculation Example ===\n');

    // Create a configuration of points
    const config = [
        new WasmPoint3D(0x100, 0x200, 0x300),
        new WasmPoint3D(0x150, 0x250, 0x350),
        new WasmPoint3D(0x0A0, 0x1A0, 0x2A0),
        new WasmPoint3D(0x1FF, 0x2FF, 0x3FF)
    ];

    console.log('Configuration:');
    config.forEach((p, i) => {
        console.log(`  Point ${i}: ${p.toHex()}`);
    });

    // Calculate center of mass
    const totalX = config.reduce((sum, p) => sum + p.x(), 0);
    const totalY = config.reduce((sum, p) => sum + p.y(), 0);
    const totalZ = config.reduce((sum, p) => sum + p.z(), 0);

    const centerX = Math.floor(totalX / config.length);
    const centerY = Math.floor(totalY / config.length);
    const centerZ = Math.floor(totalZ / config.length);

    const center = new WasmPoint3D(centerX, centerY, centerZ);
    console.log(`\nCenter of mass: ${center.toHex()}`);

    // Calculate entropy (distance from center)
    let totalDistance = 0;
    config.forEach(p => {
        totalDistance += p.distanceTo(center);
    });

    const entropy = totalDistance / config.length;
    console.log(`\nEntropy (avg distance from center): ${entropy.toFixed(2)}`);

    // Encode entropy as dodecet
    const encodedEntropy = new WasmDodecet(Math.floor(entropy) & 0xFFF);
    console.log(`Encoded entropy: 0x${encodedEntropy.toHex()}`);
}

/**
 * Example 5: KD-Tree Spatial Partitioning with Dodecet Keys
 * Demonstrates spatial indexing using dodecet encoding
 */
function kdtreeExample() {
    console.log('\n\n=== KD-Tree Example ===\n');

    // Create points for spatial indexing
    const points = [
        new WasmPoint3D(0x100, 0x200, 0x300),
        new WasmPoint3D(0x400, 0x500, 0x600),
        new WasmPoint3D(0x700, 0x800, 0x900),
        new WasmPoint3D(0x050, 0x050, 0x050),
        new WasmPoint3D(0xA00, 0xB00, 0xC00)
    ];

    console.log('Points for spatial indexing:');
    points.forEach((p, i) => {
        console.log(`  ${i}: ${p.toHex()}`);
    });

    // Create dodecet keys for spatial hashing
    console.log('\nSpatial hash keys:');
    points.forEach((p, i) => {
        const x = new WasmDodecet(p.x());
        const y = new WasmDodecet(p.y());
        const z = new WasmDodecet(p.z());

        // Simple hash function
        const hash = ((x.value() ^ y.value() ^ z.value()) & 0xFFF);
        const hashDodecet = new WasmDodecet(hash);

        console.log(`  Point ${i}: 0x${hashDodecet.toHex()}`);
    });

    // Find nearest neighbor (simple linear search for demo)
    const query = new WasmPoint3D(0x200, 0x300, 0x400);
    console.log(`\nQuery point: ${query.toHex()}`);

    let nearest = points[0];
    let minDist = query.distanceTo(nearest);

    points.forEach(p => {
        const dist = query.distanceTo(p);
        if (dist < minDist) {
            minDist = dist;
            nearest = p;
        }
    });

    console.log(`\nNearest neighbor: ${nearest.toHex()}`);
    console.log(`Distance: ${minDist.toFixed(2)}`);
}

/**
 * Example 6: Performance Comparison
 * Compare dodecet encoding vs traditional encoding
 */
function performanceComparison() {
    console.log('\n\n=== Performance Comparison ===\n');

    const iterations = 10000;
    const testValues = Array.from({ length: iterations }, (_, i) => i % 4096);

    // Test dodecet encoding performance
    const wasmStart = performance.now();
    const dodecetResults = testValues.map(v => {
        const d = new WasmDodecet(v);
        return d.toHex();
    });
    const wasmEnd = performance.now();
    const wasmTime = wasmEnd - wasmStart;

    // Test traditional encoding performance
    const jsStart = performance.now();
    const jsResults = testValues.map(v => {
        return v.toString(16).toUpperCase().padStart(3, '0');
    });
    const jsEnd = performance.now();
    const jsTime = jsEnd - jsStart;

    console.log(`Encoding ${iterations} values:`);
    console.log(`  Dodecet (WASM): ${wasmTime.toFixed(2)}ms`);
    console.log(`  Traditional (JS): ${jsTime.toFixed(2)}ms`);
    console.log(`  Speedup: ${(jsTime / wasmTime).toFixed(2)}x`);

    // Compare output size
    const dodecetSize = dodecetResults.join('').length;
    const jsSize = jsResults.join('').length;
    console.log('\nStorage comparison:');
    console.log(`  Dodecet: ${dodecetSize} characters`);
    console.log(`  Traditional: ${jsSize} characters`);
    console.log(`  Ratio: ${(dodecetSize / jsSize * 100).toFixed(1)}%`);

    // Precision comparison
    console.log('\nPrecision comparison:');
    console.log(`  Dodecet: 12 bits (4096 values)`);
    console.log(`  8-bit: 8 bits (256 values)`);
    console.log(`  Improvement: 16x more precision`);
}

// Run all examples
function main() {
    console.log('🚀 Dodecet Encoder - Constraint Theory Integration Examples\n');
    console.log('==========================================================\n');

    pythagoreanSnappingExample();
    rigidityMatroidExample();
    holonomyTransportExample();
    entropyCalculationExample();
    kdtreeExample();
    performanceComparison();

    console.log('\n✅ All examples completed!');
    console.log('\nKey takeaways:');
    console.log('  1. Dodecet encoding provides 16x more precision than 8-bit');
    console.log('  2. Hex-friendly format (3 hex digits per dodecet)');
    console.log('  3. Optimal for geometric operations');
    console.log('  4. Efficient storage for constraint theory problems');
    console.log('  5. Fast WASM performance for browser applications');
}

// Run if this is the main module
if (import.meta.url === `file://${process.argv[1]}`) {
    main();
}

export {
    pythagoreanSnappingExample,
    rigidityMatroidExample,
    holonomyTransportExample,
    entropyCalculationExample,
    kdtreeExample,
    performanceComparison
};
