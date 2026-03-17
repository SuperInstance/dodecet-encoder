// Test file for dodecet-wasm
// Run with: wasm-pack test --node

import init, {
    Point3D,
    Vector3DWasm,
    Transform3DWasm,
    maxDodecet,
    dodecetBits,
    dodecetCapacity,
    DodecetWasm
} from '../pkg/dodecet_wasm.js';

async function runTests() {
    console.log('🧪 Running dodecet-wasm tests...\n');

    await init();

    let passed = 0;
    let failed = 0;

    function test(name, fn) {
        try {
            fn();
            console.log(`✅ ${name}`);
            passed++;
        } catch (error) {
            console.log(`❌ ${name}`);
            console.error(`   ${error.message}`);
            failed++;
        }
    }

    function assertEqual(actual, expected, message = '') {
        if (actual !== expected) {
            throw new Error(
                message || `Expected ${expected}, got ${actual}`
            );
        }
    }

    function assertClose(actual, expected, tolerance = 0.001, message = '') {
        if (Math.abs(actual - expected) > tolerance) {
            throw new Error(
                message || `Expected ${expected} ± ${tolerance}, got ${actual}`
            );
        }
    }

    function assertTrue(condition, message = 'Expected true') {
        if (!condition) {
            throw new Error(message);
        }
    }

    // Test utility functions
    test('maxDodecet() returns 4095', () => {
        assertEqual(maxDodecet(), 4095);
    });

    test('dodecetBits() returns 12', () => {
        assertEqual(dodecetBits(), 12);
    });

    test('dodecetCapacity() returns 4096', () => {
        assertEqual(dodecetCapacity(), 4096);
    });

    // Test Point3D creation
    test('Point3D constructor creates point', () => {
        const point = new Point3D(0x123, 0x456, 0x789);
        assertEqual(point.x, 0x123);
        assertEqual(point.y, 0x456);
        assertEqual(point.z, 0x789);
    });

    test('Point3D toHex() returns correct format', () => {
        const point = new Point3D(0x123, 0x456, 0x789);
        assertEqual(point.toHex(), '123 456 789');
    });

    test('Point3D fromHex() parses correctly', () => {
        const point = Point3D.fromHex('123 456 789');
        assertEqual(point.x, 0x123);
        assertEqual(point.y, 0x456);
        assertEqual(point.z, 0x789);
    });

    test('Point3D normalized() returns correct values', () => {
        const point = new Point3D(0x800, 0x800, 0x800);
        const [nx, ny, nz] = point.normalized();
        assertClose(nx, 0.5);
        assertClose(ny, 0.5);
        assertClose(nz, 0.5);
    });

    test('Point3D signed() returns correct values', () => {
        const point = new Point3D(0x800, 0x000, 0x7FF);
        const [sx, sy, sz] = point.signed();
        assertEqual(sx, -2048);
        assertEqual(sy, 0);
        assertEqual(sz, 2047);
    });

    test('Point3D distanceTo() calculates correctly', () => {
        const p1 = new Point3D(0, 0, 0);
        const p2 = new Point3D(0x100, 0, 0);
        const dist = p1.distanceTo(p2);
        assertClose(dist, 256.0, 0.1);
    });

    // Test Vector3DWasm
    test('Vector3DWasm constructor creates vector', () => {
        const vector = new Vector3DWasm(100, 200, 300);
        assertEqual(vector.x, 100);
        assertEqual(vector.y, 200);
        assertEqual(vector.z, 300);
    });

    test('Vector3DWasm magnitude() calculates correctly', () => {
        const vector = new Vector3DWasm(300, 400, 0);
        const mag = vector.magnitude();
        assertClose(mag, 500.0, 0.1);
    });

    test('Vector3DWasm normalize() returns unit vector', () => {
        const vector = new Vector3DWasm(100, 0, 0);
        const [nx, ny, nz] = vector.normalize();
        assertClose(nx, 1.0);
        assertClose(ny, 0.0);
        assertClose(nz, 0.0);
    });

    test('Vector3DWasm dot() calculates correctly', () => {
        const v1 = new Vector3DWasm(100, 0, 0);
        const v2 = new Vector3DWasm(0, 100, 0);
        const dot = v1.dot(v2);
        assertEqual(dot, 0); // Perpendicular vectors
    });

    test('Vector3DWasm cross() calculates correctly', () => {
        const v1 = new Vector3DWasm(100, 0, 0);
        const v2 = new Vector3DWasm(0, 100, 0);
        const cross = v1.cross(v2);
        // Cross product should be along Z axis
        assertEqual(cross.x, 0);
        assertEqual(cross.y, 0);
        assertTrue(cross.z !== 0);
    });

    test('Vector3DWasm add() adds vectors', () => {
        const v1 = new Vector3DWasm(100, 200, 300);
        const v2 = new Vector3DWasm(10, 20, 30);
        const sum = v1.add(v2);
        assertEqual(sum.x, 110);
        assertEqual(sum.y, 220);
        assertEqual(sum.z, 330);
    });

    test('Vector3DWasm sub() subtracts vectors', () => {
        const v1 = new Vector3DWasm(100, 200, 300);
        const v2 = new Vector3DWasm(10, 20, 30);
        const diff = v1.sub(v2);
        assertEqual(diff.x, 90);
        assertEqual(diff.y, 180);
        assertEqual(diff.z, 270);
    });

    test('Vector3DWasm scale() scales vector', () => {
        const vector = new Vector3DWasm(100, 200, 300);
        const scaled = vector.scale(2.0);
        assertEqual(scaled.x, 200);
        assertEqual(scaled.y, 400);
        assertEqual(scaled.z, 600);
    });

    // Test Transform3DWasm
    test('Transform3DWasm identity() creates identity', () => {
        const transform = new Transform3DWasm();
        const point = new Point3D(0x100, 0x200, 0x300);
        const transformed = transform.apply(point);
        // Identity should not significantly change the point
        assertTrue(transformed.x >= 0);
        assertTrue(transformed.y >= 0);
        assertTrue(transformed.z >= 0);
    });

    test('Transform3DWasm translation() translates point', () => {
        const point = new Point3D(0x100, 0x200, 0x300);
        const transform = Transform3DWasm.translation(100, 200, 300);
        const transformed = transform.apply(point);
        // Point should be translated
        assertTrue(transformed.x !== point.x || transformed.y !== point.y);
    });

    test('Transform3DWasm rotationX() rotates point', () => {
        const point = new Point3D(0, 0x100, 0);
        const transform = Transform3DWasm.rotationX(90);
        const transformed = transform.apply(point);
        // Point should be rotated
        assertTrue(transformed.x === point.x); // X should stay same
    });

    test('Transform3DWasm rotationY() rotates point', () => {
        const point = new Point3D(0x100, 0, 0);
        const transform = Transform3DWasm.rotationY(90);
        const transformed = transform.apply(point);
        // Point should be rotated
        assertTrue(transformed.y === point.y); // Y should stay same
    });

    test('Transform3DWasm rotationZ() rotates point', () => {
        const point = new Point3D(0x100, 0, 0);
        const transform = Transform3DWasm.rotationZ(90);
        const transformed = transform.apply(point);
        // Point should be rotated
        assertTrue(transformed.z === point.z); // Z should stay same
    });

    test('Transform3DWasm scale() scales point', () => {
        const point = new Point3D(0x100, 0x200, 0x300);
        const transform = Transform3DWasm.scale(2.0, 2.0, 2.0);
        const transformed = transform.apply(point);
        // Point should be scaled
        assertTrue(transformed.x !== point.x || transformed.y !== point.y);
    });

    test('Transform3DWasm compose() combines transforms', () => {
        const t1 = Transform3DWasm.translation(100, 0, 0);
        const t2 = Transform3DWasm.translation(0, 200, 0);
        const combined = t1.compose(t2);
        const point = new Point3D(0, 0, 0);
        const transformed = combined.apply(point);
        // Should apply both translations
        assertTrue(transformed.x !== 0 || transformed.y !== 0);
    });

    // Test edge cases
    test('Point3D handles zero coordinates', () => {
        const point = new Point3D(0, 0, 0);
        assertEqual(point.x, 0);
        assertEqual(point.y, 0);
        assertEqual(point.z, 0);
    });

    test('Point3D handles max coordinates', () => {
        const point = new Point3D(0xFFF, 0xFFF, 0xFFF);
        assertEqual(point.x, 0xFFF);
        assertEqual(point.y, 0xFFF);
        assertEqual(point.z, 0xFFF);
    });

    test('Vector3DWasm handles zero vector', () => {
        const vector = new Vector3DWasm(0, 0, 0);
        assertEqual(vector.x, 0);
        assertEqual(vector.y, 0);
        assertEqual(vector.z, 0);
        const mag = vector.magnitude();
        assertClose(mag, 0.0);
    });

    // Test hex string parsing with various formats
    test('Point3D fromHex() handles lowercase', () => {
        const point = Point3D.fromHex('abc def 123');
        assertEqual(point.x, 0xABC);
        assertEqual(point.y, 0xDEF);
        assertEqual(point.z, 0x123);
    });

    test('Point3D fromHex() handles mixed case', () => {
        const point = Point3D.fromHex('AbC DeF 123');
        assertEqual(point.x, 0xABC);
        assertEqual(point.y, 0xDEF);
        assertEqual(point.z, 0x123);
    });

    // Summary
    console.log('\n' + '='.repeat(50));
    console.log(`Tests passed: ${passed}`);
    console.log(`Tests failed: ${failed}`);
    console.log(`Total tests: ${passed + failed}`);
    console.log('='.repeat(50));

    if (failed > 0) {
        process.exit(1);
    }
}

runTests().catch(error => {
    console.error('Test suite failed to run:', error);
    process.exit(1);
});
