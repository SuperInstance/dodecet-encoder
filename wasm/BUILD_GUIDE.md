# Build and Test Guide - Dodecet WASM Package

This guide explains how to build, test, and publish the dodecet-wasm package.

## Prerequisites

### Required Tools

1. **Rust** (1.70+)
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Verify installation
   rustc --version
   ```

2. **wasm-pack**
   ```bash
   # Install wasm-pack
   cargo install wasm-pack

   # Verify installation
   wasm-pack --version
   ```

3. **Node.js** (12.0+)
   ```bash
   # Verify installation
   node --version
   npm --version
   ```

### Optional Tools

- **make** - For using the Makefile
- **Git** - For version control

## Building the Package

### Quick Start

The easiest way to build is using the provided scripts:

#### Linux/macOS

```bash
cd wasm
./build.sh
```

#### Windows

```cmd
cd wasm
build.bat
```

#### Using Make

```bash
cd wasm
make build
```

### Manual Build

```bash
# Navigate to wasm directory
cd wasm

# Build for web target (recommended for browsers)
wasm-pack build --target web --release

# Build for Node.js
wasm-pack build --target nodejs --release

# Build for bundlers (webpack, rollup, etc.)
wasm-pack build --target bundler --release
```

### Build Targets

| Target | Description | Use Case |
|--------|-------------|----------|
| `web` | Standalone WASM for browsers | Direct browser use, no bundler |
| `nodejs` | Node.js compatible | Server-side, CLI tools |
| `bundler` | For bundler integration | Webpack, Rollup, Vite |

### Build Output

After building, you'll find the output in the `pkg/` directory:

```
pkg/
├── dodecet_wasm.js          # JavaScript glue
├── dodecet_wasm_bg.wasm     # WebAssembly binary
├── dodecet_wasm.d.ts        # TypeScript definitions
├── dodecet_wasm_bg.js       # Additional JS helpers
├── package.json             # NPM package manifest
└── LICENSE                  # License file
```

## Testing

### Run Tests

```bash
# Run tests in Node.js
wasm-pack test --node

# Run tests in Firefox
wasm-pack test --firefox

# Run tests in Chrome
wasm-pack test --chrome
```

### Using Make

```bash
make test          # Node.js tests
make test-firefox  # Firefox tests
make test-chrome   # Chrome tests
```

### Manual Testing

```bash
# Navigate to pkg directory
cd pkg

# Link package globally
npm link

# Test in another project
cd /path/to/test-project
npm link @superinstance/dodecet-encoder
```

### Browser Testing

1. Build the package:
   ```bash
   wasm-pack build --target web
   ```

2. Start a local server in the examples directory:
   ```bash
   cd examples
   python -m http.server 8000
   ```

3. Open `http://localhost:8000` in your browser

## Development Workflow

### Development Build

For faster builds during development:

```bash
# Development build (no optimization)
wasm-pack build --target web

# Or use make
make build-dev
```

### Release Build

For production:

```bash
# Release build (full optimization)
wasm-pack build --target web --release

# Or use make
make build
```

### Clean Build Artifacts

```bash
# Using make
make clean

# Or manually
rm -rf pkg
rm -f Cargo.toml~
cargo clean
```

## Publishing

### Dry Run

Test the publishing process without actually publishing:

```bash
# Navigate to pkg directory
cd pkg

# Dry run
npm publish --dry-run
```

### Publish to NPM

```bash
# Navigate to pkg directory
cd pkg

# Publish to npm
npm publish --access public
```

### Using Make

```bash
make publish-dry   # Dry run
make publish       # Actually publish
```

### Pre-Publish Checklist

Before publishing, ensure:

- [ ] All tests pass
- [ ] Documentation is up to date
- [ ] Version number is updated in `Cargo.toml` and `package.json`
- [ ] CHANGELOG.md is updated
- [ ] Build is optimized (`--release` flag)
- [ ] Dry run succeeds

## Performance Optimization

### Build Optimizations

The release build includes several optimizations:

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true          # Link-time optimization
codegen-units = 1    # Better optimization at cost of compile time
```

### WASM Optimizations

Additional WASM-specific optimizations:

```bash
wasm-pack build --target web --release
```

This runs `wasm-opt` with `-O4` optimization level.

### Measuring Performance

```javascript
import init, { Point3D } from '@superinstance/dodecet-encoder';

await init();

const iterations = 10000;
const start = performance.now();

for (let i = 0; i < iterations; i++) {
    const point = new Point3D(i & 0xFFF, (i + 1) & 0xFFF, (i + 2) & 0xFFF);
}

const elapsed = performance.now() - start;
console.log(`Average: ${(elapsed / iterations).toFixed(4)} ms per operation`);
```

## Troubleshooting

### Build Fails

**Problem:** Build fails with compilation errors

**Solution:**
```bash
# Clean build
cargo clean
rm -rf pkg

# Rebuild
wasm-pack build --target web --release
```

### WASM File Too Large

**Problem:** WASM file is larger than expected

**Solution:**
```bash
# Ensure release build
wasm-pack build --target web --release

# Check for unused dependencies
cargo tree --duplicates
```

### Tests Fail in Browser

**Problem:** Tests pass in Node.js but fail in browser

**Solution:**
- Check browser console for errors
- Ensure WASM is being served with correct MIME type
- Verify CORS headers if loading from different origin

### "wasm-pack not found"

**Problem:** Command not found error

**Solution:**
```bash
# Install wasm-pack
cargo install wasm-pack

# Ensure cargo bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

## Continuous Integration

### GitHub Actions Example

```yaml
name: Build and Test

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v2

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable

    - name: Install wasm-pack
      run: cargo install wasm-pack

    - name: Build
      run: wasm-pack build --target web --release

    - name: Run tests
      run: wasm-pack test --node

    - name: Upload artifacts
      uses: actions/upload-artifact@v2
      with:
        name: wasm-package
        path: pkg/
```

## Best Practices

1. **Always use release builds for production**
   ```bash
   wasm-pack build --target web --release
   ```

2. **Run tests before publishing**
   ```bash
   wasm-pack test --node
   ```

3. **Keep dependencies updated**
   ```bash
   cargo update
   ```

4. **Use version control**
   ```bash
   git tag v1.0.0
   git push --tags
   ```

5. **Monitor bundle size**
   ```bash
   ls -lh pkg/dodecet_wasm_bg.wasm
   ```

## Additional Resources

- [wasm-pack documentation](https://rustwasm.github.io/wasm-pack/)
- [Rust WASM book](https://rustwasm.github.io/docs/book/)
- [MDN WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)
