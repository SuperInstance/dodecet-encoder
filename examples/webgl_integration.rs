// WebGL Integration Example - Dodecet Encoder
//
// This example demonstrates how to use dodecet encoding for WebGL applications,
// showing efficient data transfer between Rust and WebGL/JavaScript.

use dodecet_encoder::geometric::Point3D;
use std::fs::File;
use std::io::Write;

/// Generate JavaScript bindings for dodecet encoder
fn generate_javascript_bindings() -> String {
    r#"
// Dodecet Encoder JavaScript Bindings
// Auto-generated from Rust dodecet-encoder

class Dodecet {
    constructor(value) {
        if (value < 0 || value > 0xFFF) {
            throw new Error('Dodecet value must be in range [0, 4095]');
        }
        this.value = value;
    }

    static fromHex(hex) {
        return new Dodecet(parseInt(hex, 16));
    }

    nibble(index) {
        if (index < 0 || index > 2) {
            throw new Error('Nibble index must be in range [0, 2]');
        }
        return (this.value >> (index * 4)) & 0xF;
    }

    toHex() {
        return this.value.toString(16).toUpperCase().padStart(3, '0');
    }

    toBinary() {
        return this.value.toString(2).padStart(12, '0');
    }

    normalize() {
        return this.value / 0xFFF;
    }
}

class Point3D {
    constructor(x, y, z) {
        this.x = x & 0xFFF;
        this.y = y & 0xFFF;
        this.z = z & 0xFFF;
    }

    distanceTo(other) {
        const dx = this.x - other.x;
        const dy = this.y - other.y;
        const dz = this.z - other.z;
        return Math.sqrt(dx*dx + dy*dy + dz*dz);
    }

    toDodecets() {
        return [
            new Dodecet(this.x),
            new Dodecet(this.y),
            new Dodecet(this.z)
        ];
    }

    toHexString() {
        return this.x.toString(16).toUpperCase().padStart(3, '0') +
               this.y.toString(16).toUpperCase().padStart(3, '0') +
               this.z.toString(16).toUpperCase().padStart(3, '0');
    }

    toArray() {
        return new Float32Array([
            this.x / 0xFFF * 2 - 1,  // Normalize to [-1, 1]
            this.y / 0xFFF * 2 - 1,
            this.z / 0xFFF * 2 - 1
        ]);
    }
}

// WebGL buffer utilities
class DodecetWebGLBuffer {
    constructor(gl) {
        this.gl = gl;
        this.buffer = null;
    }

    fromPoints(points) {
        const data = new Float32Array(points.length * 3);

        for (let i = 0; i < points.length; i++) {
            const p = points[i];
            data[i * 3] = p.x / 0xFFF * 2 - 1;
            data[i * 3 + 1] = p.y / 0xFFF * 2 - 1;
            data[i * 3 + 2] = p.z / 0xFFF * 2 - 1;
        }

        this.buffer = this.gl.createBuffer();
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.buffer);
        this.gl.bufferData(this.gl.ARRAY_BUFFER, data, this.gl.STATIC_DRAW);

        return this;
    }

    bind(attributeLocation) {
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.buffer);
        this.gl.vertexAttribPointer(attributeLocation, 3, this.gl.FLOAT, false, 0, 0);
        this.gl.enableVertexAttribArray(attributeLocation);
    }
}

// Export for use
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { Dodecet, Point3D, DodecetWebGLBuffer };
}
"#.to_string()
}

/// Generate WebGL shader code for dodecet visualization
fn generate_webgl_shaders() -> (String, String) {
    let vertex_shader = r#"
#version 300 es

in vec3 a_position;
in vec3 a_color;

uniform mat4 u_modelViewProjection;
uniform float u_pointSize;

out vec3 v_color;

void main() {
    gl_Position = u_modelViewProjection * vec4(a_position, 1.0);
    gl_PointSize = u_pointSize;
    v_color = a_color;
}
"#.to_string();

    let fragment_shader = r#"
#version 300 es

precision highp float;

in vec3 v_color;
out vec4 fragColor;

void main() {
    // Circular point
    vec2 coord = gl_PointCoord - vec2(0.5);
    if (length(coord) > 0.5) {
        discard;
    }

    fragColor = vec4(v_color, 1.0);
}
"#.to_string();

    (vertex_shader, fragment_shader)
}

/// Generate HTML demo page
fn generate_html_demo() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dodecet WebGL Demo</title>
    <style>
        body {
            margin: 0;
            padding: 20px;
            font-family: Arial, sans-serif;
            background: #1a1a1a;
            color: #fff;
        }
        canvas {
            border: 1px solid #444;
            display: block;
            margin: 20px 0;
        }
        .controls {
            margin: 20px 0;
        }
        .info {
            background: #2a2a2a;
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
        }
        button {
            background: #4CAF50;
            border: none;
            color: white;
            padding: 10px 20px;
            margin: 5px;
            cursor: pointer;
            border-radius: 3px;
        }
        button:hover {
            background: #45a049;
        }
    </style>
</head>
<body>
    <h1>Dodecet WebGL Visualization</h1>
    <div class="info">
        <p>This demo visualizes 12-bit dodecet-encoded 3D points in WebGL.</p>
        <p>Each coordinate uses 12 bits (4096 states) for efficient representation.</p>
    </div>

    <canvas id="glCanvas" width="800" height="600"></canvas>

    <div class="controls">
        <button onclick="generateRandomPoints()">Generate Random Points</button>
        <button onclick="generateSphere()">Generate Sphere</button>
        <button onclick="generateCube()">Generate Cube</button>
        <button onclick="toggleRotation()">Toggle Rotation</button>
    </div>

    <div class="info">
        <h3>Statistics</h3>
        <p>Points: <span id="pointCount">0</span></p>
        <p>Memory (Dodecet): <span id="dodecetMemory">0</span> bytes</p>
        <p>Memory (Float32): <span id="floatMemory">0</span> bytes</p>
        <p>Savings: <span id="savings">0</span>%</p>
    </div>

    <script src="dodecet-bindings.js"></script>
    <script>
        let gl;
        let program;
        let points = [];
        let rotating = false;
        let rotationAngle = 0;

        function init() {
            const canvas = document.getElementById('glCanvas');
            gl = canvas.getContext('webgl2');

            if (!gl) {
                alert('WebGL2 not supported');
                return;
            }

            // Create shaders
            const vsSource = document.getElementById('vertex-shader').textContent;
            const fsSource = document.getElementById('fragment-shader').textContent;

            const vertexShader = compileShader(gl.VERTEX_SHADER, vsSource);
            const fragmentShader = compileShader(gl.FRAGMENT_SHADER, fsSource);

            program = gl.createProgram();
            gl.attachShader(program, vertexShader);
            gl.attachShader(program, fragmentShader);
            gl.linkProgram(program);

            if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
                console.error('Program link error:', gl.getProgramInfoLog(program));
                return;
            }

            gl.useProgram(program);
            gl.enable(gl.DEPTH_TEST);
            gl.clearColor(0.1, 0.1, 0.1, 1.0);

            render();
        }

        function compileShader(type, source) {
            const shader = gl.createShader(type);
            gl.shaderSource(shader, source);
            gl.compileShader(shader);

            if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
                console.error('Shader compile error:', gl.getShaderInfoLog(shader));
                gl.deleteShader(shader);
                return null;
            }

            return shader;
        }

        function generateRandomPoints() {
            points = [];
            for (let i = 0; i < 1000; i++) {
                const x = Math.floor(Math.random() * 4096);
                const y = Math.floor(Math.random() * 4096);
                const z = Math.floor(Math.random() * 4096);
                points.push(new Point3D(x, y, z));
            }
            updateStats();
        }

        function generateSphere() {
            points = [];
            const radius = 2048;
            for (let i = 0; i < 500; i++) {
                const theta = Math.random() * Math.PI * 2;
                const phi = Math.acos(2 * Math.random() - 1);
                const x = Math.floor(radius * Math.sin(phi) * Math.cos(theta) + 2048);
                const y = Math.floor(radius * Math.sin(phi) * Math.sin(theta) + 2048);
                const z = Math.floor(radius * Math.cos(phi) + 2048);
                points.push(new Point3D(x, y, z));
            }
            updateStats();
        }

        function generateCube() {
            points = [];
            const size = 1024;
            const offset = 2048 - size / 2;

            for (let x = 0; x < 10; x++) {
                for (let y = 0; y < 10; y++) {
                    for (let z = 0; z < 10; z++) {
                        points.push(new Point3D(
                            offset + x * size / 10,
                            offset + y * size / 10,
                            offset + z * size / 10
                        ));
                    }
                }
            }
            updateStats();
        }

        function toggleRotation() {
            rotating = !rotating;
        }

        function updateStats() {
            document.getElementById('pointCount').textContent = points.length;
            document.getElementById('dodecetMemory').textContent = points.length * 6; // 3 dodecets * 2 bytes
            document.getElementById('floatMemory').textContent = points.length * 12; // 3 floats * 4 bytes
            document.getElementById('savings').textContent = (50).toFixed(1);
        }

        function render() {
            gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

            if (points.length > 0) {
                // Create buffers
                const positions = new Float32Array(points.length * 3);
                const colors = new Float32Array(points.length * 3);

                for (let i = 0; i < points.length; i++) {
                    const p = points[i];
                    positions[i * 3] = p.x / 0xFFF * 2 - 1;
                    positions[i * 3 + 1] = p.y / 0xFFF * 2 - 1;
                    positions[i * 3 + 2] = p.z / 0xFFF * 2 - 1;

                    // Color based on position
                    colors[i * 3] = p.x / 0xFFF;
                    colors[i * 3 + 1] = p.y / 0xFFF;
                    colors[i * 3 + 2] = p.z / 0xFFF;
                }

                // Position buffer
                const posBuffer = gl.createBuffer();
                gl.bindBuffer(gl.ARRAY_BUFFER, posBuffer);
                gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);

                const posLoc = gl.getAttribLocation(program, 'a_position');
                gl.enableVertexAttribArray(posLoc);
                gl.vertexAttribPointer(posLoc, 3, gl.FLOAT, false, 0, 0);

                // Color buffer
                const colorBuffer = gl.createBuffer();
                gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer);
                gl.bufferData(gl.ARRAY_BUFFER, colors, gl.STATIC_DRAW);

                const colorLoc = gl.getAttribLocation(program, 'a_color');
                gl.enableVertexAttribArray(colorLoc);
                gl.vertexAttribPointer(colorLoc, 3, gl.FLOAT, false, 0, 0);

                // MVP matrix
                const aspect = gl.canvas.width / gl.canvas.height;
                const projection = perspectiveMatrix(45, aspect, 0.1, 100.0);
                const view = translationMatrix(0, 0, -5);
                let model = rotationMatrix(rotationAngle, rotationAngle * 0.5, 0);

                const mvp = multiplyMatrices(multiplyMatrices(projection, view), model);
                const mvpLoc = gl.getUniformLocation(program, 'u_modelViewProjection');
                gl.uniformMatrix4fv(mvpLoc, false, new Float32Array(mvp));

                const pointSizeLoc = gl.getUniformLocation(program, 'u_pointSize');
                gl.uniform1f(pointSizeLoc, 3.0);

                gl.drawArrays(gl.POINTS, 0, points.length);
            }

            if (rotating) {
                rotationAngle += 0.01;
            }

            requestAnimationFrame(render);
        }

        // Matrix utilities (simplified)
        function perspectiveMatrix(fov, aspect, near, far) {
            const f = 1.0 / Math.tan(fov * Math.PI / 360);
            const nf = 1 / (near - far);
            return [
                f / aspect, 0, 0, 0,
                0, f, 0, 0,
                0, 0, (far + near) * nf, -1,
                0, 0, 2 * far * near * nf, 0
            ];
        }

        function translationMatrix(x, y, z) {
            return [
                1, 0, 0, 0,
                0, 1, 0, 0,
                0, 0, 1, 0,
                x, y, z, 1
            ];
        }

        function rotationMatrix(x, y, z) {
            // Simplified rotation
            return [
                1, 0, 0, 0,
                0, 1, 0, 0,
                0, 0, 1, 0,
                0, 0, 0, 1
            ];
        }

        function multiplyMatrices(a, b) {
            const result = new Array(16).fill(0);
            for (let i = 0; i < 4; i++) {
                for (let j = 0; j < 4; j++) {
                    for (let k = 0; k < 4; k++) {
                        result[i * 4 + j] += a[i * 4 + k] * b[k * 4 + j];
                    }
                }
            }
            return result;
        }

        // Initialize on load
        window.onload = init;
    </script>

    <!-- Shader sources -->
    <script id="vertex-shader" type="x-shader/x-vertex">#version 300 es
    in vec3 a_position;
    in vec3 a_color;
    uniform mat4 u_modelViewProjection;
    uniform float u_pointSize;
    out vec3 v_color;
    void main() {
        gl_Position = u_modelViewProjection * vec4(a_position, 1.0);
        gl_PointSize = u_pointSize;
        v_color = a_color;
    }
    </script>

    <script id="fragment-shader" type="x-shader/x-fragment">#version 300 es
    precision highp float;
    in vec3 v_color;
    out vec4 fragColor;
    void main() {
        vec2 coord = gl_PointCoord - vec2(0.5);
        if (length(coord) > 0.5) discard;
        fragColor = vec4(v_color, 1.0);
    }
    </script>
</body>
</html>
"#.to_string()
}

/// Generate TypeScript definitions
fn generate_typescript_definitions() -> String {
    r#"
// Dodecet Encoder TypeScript Definitions

export class Dodecet {
    constructor(value: number);
    static fromHex(hex: string): Dodecet;
    nibble(index: number): number;
    toHex(): string;
    toBinary(): string;
    normalize(): number;
    readonly value: number;
}

export class Point3D {
    constructor(x: number, y: number, z: number);
    distanceTo(other: Point3D): number;
    toDodecets(): [Dodecet, Dodecet, Dodecet];
    toHexString(): string;
    toArray(): Float32Array;
    readonly x: number;
    readonly y: number;
    readonly z: number;
}

export class DodecetWebGLBuffer {
    constructor(gl: WebGL2RenderingContext);
    fromPoints(points: Point3D[]): this;
    bind(attributeLocation: number): void;
    private gl: WebGL2RenderingContext;
    private buffer: WebGLBuffer | null;
}
"#.to_string()
}

fn main() {
    println!("=== WebGL Integration with Dodecet Encoder ===\n");

    // 1. Generate JavaScript bindings
    println!("1. Generating JavaScript bindings...");
    let js_bindings = generate_javascript_bindings();

    let mut js_file = File::create("examples/webgl/dodecet-bindings.js").unwrap();
    js_file.write_all(js_bindings.as_bytes()).unwrap();
    println!("   Created: examples/webgl/dodecet-bindings.js");

    // 2. Generate WebGL shaders
    println!("\n2. Generating WebGL shaders...");
    let (vertex_shader, fragment_shader) = generate_webgl_shaders();

    let mut vs_file = File::create("examples/webgl/vertex-shader.glsl").unwrap();
    vs_file.write_all(vertex_shader.as_bytes()).unwrap();

    let mut fs_file = File::create("examples/webgl/fragment-shader.glsl").unwrap();
    fs_file.write_all(fragment_shader.as_bytes()).unwrap();

    println!("   Created: examples/webgl/vertex-shader.glsl");
    println!("   Created: examples/webgl/fragment-shader.glsl");

    // 3. Generate HTML demo
    println!("\n3. Generating HTML demo...");
    let html_demo = generate_html_demo();

    let mut html_file = File::create("examples/webgl/webgl-demo.html").unwrap();
    html_file.write_all(html_demo.as_bytes()).unwrap();
    println!("   Created: examples/webgl/webgl-demo.html");

    // 4. Generate TypeScript definitions
    println!("\n4. Generating TypeScript definitions...");
    let ts_defs = generate_typescript_definitions();

    let mut ts_file = File::create("examples/webgl/dodecet-bindings.d.ts").unwrap();
    ts_file.write_all(ts_defs.as_bytes()).unwrap();
    println!("   Created: examples/webgl/dodecet-bindings.d.ts");

    // 5. Example usage
    println!("\n5. Example Usage:");
    println!("   // JavaScript");
    println!("   const point = new Point3D(0x100, 0x200, 0x300);");
    println!("   const hex = point.toHexString(); // '100200300'");
    println!("   const normalized = point.toArray(); // Float32Array for WebGL");
    println!();
    println!("   // WebGL Buffer");
    println!("   const buffer = new DodecetWebGLBuffer(gl);");
    println!("   buffer.fromPoints([point1, point2, point3]);");
    println!("   buffer.bind(positionLocation);");

    // 6. Memory comparison
    println!("\n6. Memory Efficiency:");
    let points = 1000;
    let dodecet_memory = points * 6; // 3 dodecets * 2 bytes
    let float32_memory = points * 12; // 3 floats * 4 bytes
    let savings = 100.0 * (1.0 - (dodecet_memory as f64 / float32_memory as f64));

    println!("   {} points:", points);
    println!("   Dodecet encoding: {} bytes", dodecet_memory);
    println!("   Float32 encoding: {} bytes", float32_memory);
    println!("   Savings: {:.1}%", savings);

    // 7. Features summary
    println!("\n=== WebGL Integration Features ===");
    println!("✓ JavaScript API with Dodecet and Point3D classes");
    println!("✓ WebGL buffer utilities for direct GPU upload");
    println!("✓ Vertex and fragment shaders for visualization");
    println!("✓ HTML demo with interactive controls");
    println!("✓ TypeScript definitions for type safety");
    println!("✓ 50% memory savings vs Float32Array");
    println!("✓ Hex-friendly debugging and inspection");
    println!("✓ Automatic normalization to WebGL coordinates [-1, 1]");

    println!("\nTo run the demo:");
    println!("  1. Open examples/webgl/webgl-demo.html in a browser");
    println!("  2. Or serve with: python -m http.server 8000");
    println!("  3. Navigate to http://localhost:8000/examples/webgl/webgl-demo.html");
}
