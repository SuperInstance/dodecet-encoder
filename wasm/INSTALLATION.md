# Installation Guide - @superinstance/dodecet-encoder

This guide will help you install and use the dodecet-encoder WebAssembly package in your project.

## Prerequisites

- Node.js 12.0 or higher
- npm or yarn package manager
- Modern web browser with WebAssembly support

## Installation

### Using npm

```bash
npm install @superinstance/dodecet-encoder
```

### Using yarn

```bash
yarn add @superinstance/dodecet-encoder
```

### Using pnpm

```bash
pnpm add @superinstance/dodecet-encoder
```

## Usage

### Basic Setup

```javascript
import init, { Point3D, Vector3DWasm, Transform3DWasm } from '@superinstance/dodecet-encoder';

// Initialize the WASM module
await init();

// Now you can use the library
const point = new Point3D(0x123, 0x456, 0x789);
console.log(point.toHex()); // "123 456 789"
```

### In a Browser Environment

#### HTML with ES Modules

```html
<!DOCTYPE html>
<html>
<head>
    <title>Dodecet Encoder Example</title>
</head>
<body>
    <script type="module">
        import init, { Point3D } from '@superinstance/dodecet-encoder';

        async function main() {
            await init();

            const point = new Point3D(0x100, 0x200, 0x300);
            console.log(point.toHex());
        }

        main();
    </script>
</body>
</html>
```

#### With a Bundler (Vite, Webpack, Rollup)

```javascript
// main.js
import init, { Point3D } from '@superinstance/dodecet-encoder';

async function main() {
    await init();
    const point = new Point3D(0x100, 0x200, 0x300);
    console.log(point.toHex());
}

main();
```

### In Node.js

```javascript
const { init, Point3D } = require('@superinstance/dodecet-encoder');

async function main() {
    await init();
    const point = new Point3D(0x100, 0x200, 0x300);
    console.log(point.toHex());
}

main();
```

## Framework-Specific Setup

### React

```jsx
import { useEffect, useState } from 'react';
import init, { Point3D } from '@superinstance/dodecet-encoder';

function App() {
    const [wasmReady, setWasmReady] = useState(false);

    useEffect(() => {
        init().then(() => setWasmReady(true));
    }, []);

    if (!wasmReady) {
        return <div>Loading...</div>;
    }

    const point = new Point3D(0x100, 0x200, 0x300);

    return (
        <div>
            <h1>Dodecet Point: {point.toHex()}</h1>
        </div>
    );
}

export default App;
```

### Vue.js

```vue
<template>
    <div v-if="wasmReady">
        <h1>Dodecet Point: {{ pointHex }}</h1>
    </div>
    <div v-else>
        <p>Loading...</p>
    </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import init, { Point3D } from '@superinstance/dodecet-encoder';

export default {
    setup() {
        const wasmReady = ref(false);
        const pointHex = ref('');

        onMounted(async () => {
            await init();
            const point = new Point3D(0x100, 0x200, 0x300);
            pointHex.value = point.toHex();
            wasmReady.value = true;
        });

        return { wasmReady, pointHex };
    }
};
</script>
```

### Svelte

```svelte
<script>
import { onMount } from 'svelte';
import init, { Point3D } from '@superinstance/dodecet-encoder';

let wasmReady = false;
let pointHex = '';

onMount(async () => {
    await init();
    const point = new Point3D(0x100, 0x200, 0x300);
    pointHex = point.toHex();
    wasmReady = true;
});
</script>

{#if wasmReady}
    <h1>Dodecet Point: {pointHex}</h1>
{:else}
    <p>Loading...</p>
{/if}
```

### Angular

```typescript
import { Component, OnInit } from '@angular/core';
import init, { Point3D } from '@superinstance/dodecet-encoder';

@Component({
    selector: 'app-root',
    template: `
        <h1 *ngIf="wasmReady">Dodecet Point: {{ pointHex }}</h1>
        <p *ngIf="!wasmReady">Loading...</p>
    `
})
export class AppComponent implements OnInit {
    wasmReady = false;
    pointHex = '';

    async ngOnInit() {
        await init();
        const point = new Point3D(0x100, 0x200, 0x300);
        this.pointHex = point.toHex();
        this.wasmReady = true;
    }
}
```

### Next.js

```jsx
// pages/index.js
import { useEffect, useState } from 'react';
import init, { Point3D } from '@superinstance/dodecet-encoder';
import Head from 'next/head';

export default function Home() {
    const [wasmReady, setWasmReady] = useState(false);
    const [pointHex, setPointHex] = useState('');

    useEffect(() => {
        (async () => {
            if (typeof window !== 'undefined') {
                await init();
                const point = new Point3D(0x100, 0x200, 0x300);
                setPointHex(point.toHex());
                setWasmReady(true);
            }
        })();
    }, []);

    return (
        <div>
            <Head>
                <title>Dodecet Encoder Demo</title>
            </Head>
            <main>
                {wasmReady ? (
                    <h1>Dodecet Point: {pointHex}</h1>
                ) : (
                    <p>Loading...</p>
                )}
            </main>
        </div>
    );
}
```

## Build Configuration

### Vite

Vite works out of the box with no additional configuration needed.

### Webpack

Webpack 5+ is recommended. Ensure your configuration handles WASM files:

```javascript
// webpack.config.js
module.exports = {
    // ... other config
    experiments: {
        asyncWebAssembly: true,
    },
};
```

### Rollup

Rollup requires `@rollup/plugin-wasm`:

```bash
npm install @rollup/plugin-wasm --save-dev
```

```javascript
// rollup.config.js
import wasm from '@rollup/plugin-wasm';

export default {
    plugins: [wasm()],
    // ... other config
};
```

## TypeScript Support

Full TypeScript definitions are included:

```typescript
import init, {
    Point3D,
    Vector3DWasm,
    Transform3DWasm
} from '@superinstance/dodecet-encoder';

await init();

const point: Point3D = new Point3D(0x123, 0x456, 0x789);
const vector: Vector3DWasm = new Vector3DWasm(100, 200, 300);
const transform: Transform3DWasm = Transform3DWasm.translation(100, 200, 300);
```

## Troubleshooting

### "WebAssembly.instantiate" error

This error occurs when the WASM file cannot be loaded. Ensure:

1. Your bundler is configured to handle `.wasm` files
2. The WASM file is being served correctly
3. You're using a modern browser with WASM support

### "Failed to fetch" error

This can happen due to CORS issues. Ensure:

1. Your server is configured to serve WASM files with the correct MIME type (`application/wasm`)
2. CORS headers are set correctly if loading from a different origin

### Performance issues

If you experience performance issues:

1. Ensure you're calling `await init()` only once
2. Reuse Point3D, Vector3DWasm, and Transform3DWasm instances when possible
3. Use the release build (`wasm-pack build --target web --release`)

## Next Steps

- Check out the [API Reference](./README.md#api-reference)
- Browse [Examples](./examples/)
- Run [Tests](./tests/)

## Support

If you encounter any issues:

1. Check the [GitHub Issues](https://github.com/SuperInstance/dodecet-encoder/issues)
2. Review the [main README](../../README.md)
3. Consult the [API documentation](./README.md)
