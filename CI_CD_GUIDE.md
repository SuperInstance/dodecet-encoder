# CI/CD Guide - Dodecet Encoder

## Overview

This guide covers the CI/CD pipelines for the dodecet-encoder repository, including cross-platform testing, security scanning, and automated releases.

## Workflows

### CI Workflow (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `main` branch
- Pull requests
- Manual workflow dispatch

**Jobs:**

1. **test**: Rust tests across multiple toolchains and OSes
2. **wasm**: WASM build and testing
3. **cross-compile**: Cross-compilation for multiple targets
4. **security**: Security audit and dependency checks
5. **coverage**: Code coverage reporting
6. **docs**: Documentation checks
7. **perf-check**: Performance regression detection (PRs only)

### Release Workflow (`.github/workflows/release.yml`)

**Triggers:**
- Git tags matching `v*`
- Manual workflow dispatch with version input

**Jobs:**

1. **create-release**: Create GitHub release
2. **publish-cargo**: Publish to crates.io
3. **publish-npm**: Publish WASM to npm
4. **build-binaries**: Build release binaries for multiple platforms
5. **publish-docker**: Build and push Docker image
6. **publish-docs**: Deploy documentation to GitHub Pages
7. **notify**: Send release notification

## Local Development

### Running Tests Locally

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test test_name --all-features

# Run with output
cargo test --all-features -- --nocapture

# Run doc tests
cargo test --doc
```

### Building Locally

```bash
# Build for native platform
cargo build --release --all-features

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu

# Build WASM
cd wasm
wasm-pack build --target web --scope superinstance
```

### Linting and Formatting

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

### Cross-Compilation

```bash
# Install cross
cargo install cross

# Build for different targets
cross build --target aarch64-unknown-linux-gnu
cross build --target armv7-unknown-linux-gnueabihf
cross build --target powerpc64le-unknown-linux-gnu
```

## CI/CD Features

### 1. Cross-Platform Testing

Tests run across:
- **OS**: Linux, macOS, Windows
- **Rust**: stable, beta, nightly
- **Targets**: x86_64, aarch64, ARMv7, PowerPC, RISC-V

### 2. WASM Testing

```bash
# Build WASM
cd wasm
wasm-pack build --target web --scope superinstance

# Test WASM
wasm-pack test --node
```

### 3. Code Coverage

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Generate coverage
cargo llvm-cov --all-features --lcov --output-path lcov.info

# View summary
cargo llvm-cov --all-features --summary
```

### 4. Performance Benchmarks

```bash
# Run benchmarks
cargo bench --all-features

# Compare with baseline
cargo bench --all-features -- --baseline main
```

### 5. Security Scanning

```bash
# Audit dependencies
cargo install cargo-audit
cargo audit

# Check for outdated deps
cargo install cargo-outdated
cargo outdated --exit-code 1
```

## Release Process

### Automated Release

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Commit changes
4. Create and push tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

5. CI/CD automatically:
   - Creates GitHub release
   - Publishes to crates.io
   - Publishes WASM to npm
   - Builds release binaries
   - Publishes Docker image
   - Deploys documentation

### Manual Release

1. Go to Actions tab
2. Select "Release" workflow
3. Click "Run workflow"
4. Enter version number
5. Workflow executes all release steps

### Release Verification

After release, verify:

```bash
# Check crates.io
cargo search dodecet-encoder

# Check npm
npm view @superinstance/dodecet-encoder

# Check Docker
docker pull ghcr.io/superinstance/dodecet-encoder:latest
```

## Performance Monitoring

### Benchmark Storage

Benchmarks are stored and compared across runs:

- PRs: Alert if performance regresses > 150%
- Main: Store as baseline for future comparisons

### Benchmark Commands

```bash
# Run benchmarks
cargo bench --all-features

# Generate flamegraph
cargo install flamegraph
cargo flamegraph --bench dodecet_benchmark
```

## Troubleshooting

### Common Issues

**Issue**: Cross-compilation fails
```bash
# Install cross
cargo install cross

# Use cross instead of cargo
cross build --target aarch64-unknown-linux-gnu
```

**Issue**: WASM build fails
```bash
# Ensure wasm-pack is installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build WASM
cd wasm
wasm-pack build --target web --scope superinstance
```

**Issue**: Coverage generation fails
```bash
# Install llvm-tools
rustup component add llvm-tools-preview

# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Generate coverage
cargo llvm-cov --all-features --lcov
```

### CI Failures

1. Check specific job logs
2. Reproduce locally using same commands
3. Verify Rust toolchain version
4. Check for platform-specific issues

### Release Failures

1. Verify version number format
2. Check CHANGELOG.md is updated
3. Verify crates.io credentials
4. Check npm authentication
5. Verify tag format: `v*`

## Platform Support

### Supported Targets

**Tier 1 (Guaranteed to work):**
- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `x86_64-pc-windows-msvc`

**Tier 2 (Best effort):**
- `aarch64-unknown-linux-gnu`
- `aarch64-apple-darwin`
- `armv7-unknown-linux-gnueabihf`

**Tier 3 (Experimental):**
- `powerpc64le-unknown-linux-gnu`
- `riscv64gc-unknown-linux-gnu`

### WASM Targets

- `web`: Browser environments
- `bundler`: Bundler environments
- `nodejs`: Node.js environments

## Performance Targets

- CI runtime: < 20 minutes
- Build time: < 10 minutes
- Test coverage: > 90%
- Release time: < 30 minutes
- WASM size: < 50KB gzipped

## Best Practices

### 1. Pre-Commit Checks

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features

# Run tests
cargo test --all-features

# Check docs
cargo doc --all-features --no-deps
```

### 2. Pre-Push Checks

```bash
# Run full test suite
cargo test --all-features

# Run benchmarks
cargo bench --all-features

# Check for security issues
cargo audit
```

### 3. Pre-Release Checks

```bash
# Update version
# Update CHANGELOG

# Run all checks
cargo fmt --all
cargo clippy --all-targets --all-features
cargo test --all-features
cargo doc --all-features --no-deps

# Build release
cargo build --release --all-features

# Run benchmarks
cargo bench --all-features
```

## Further Reading

- [Rust CI/CD Best Practices](https://rust-lang.github.io/rust-cli/book/index.html)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)
- [cargo-llvm-cov Documentation](https://github.com/taiki-e/cargo-llvm-cov)
- [Cross Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
