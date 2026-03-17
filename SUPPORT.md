# Support & Community Guidelines

## Getting Help

The dodecet-encoder community provides multiple ways to get help and connect with other users.

### Quick Help

**Before asking for help:**
1. Search existing [issues](https://github.com/SuperInstance/dodecet-encoder/issues)
2. Check the [documentation](https://docs.rs/dodecet-encoder)
3. Review the [examples](https://github.com/SuperInstance/dodecet-encoder/tree/main/examples)
4. Read through the [tutorials](https://github.com/SuperInstance/dodecet-encoder/tree/main/tutorials)

### Support Channels

#### 1. GitHub Issues
**For:** Bug reports, feature requests, installation problems

**When to use:**
- You found a bug
- You have a feature request
- Installation or build failures
- Documentation errors or omissions
- Performance issues

**How to use:**
- Open an issue at https://github.com/SuperInstance/dodecet-encoder/issues
- Use appropriate templates (bug report, feature request)
- Provide minimal reproducible examples
- Include environment information

**Response time:** 2-5 business days

#### 2. GitHub Discussions
**For:** Questions, ideas, community discussions

**When to use:**
- General usage questions
- How-to questions
- Design questions
- Best practices
- Integration help
- Show and tell

**How to use:**
- Start a discussion at https://github.com/SuperInstance/dodecet-encoder/discussions
- Search existing discussions first
- Use appropriate categories (q-and-a, ideas, show-and-tell)
- Be respectful and constructive

**Response time:** Community-driven, typically 1-3 days

#### 3. Documentation
**For:** Self-help, reference materials

**Available docs:**
- [API Reference](https://docs.rs/dodecet-encoder) - Complete API documentation
- [README](https://github.com/SuperInstance/dodecet-encoder#readme) - Project overview
- [Examples](https://github.com/SuperInstance/dodecet-encoder/tree/main/examples) - 12+ working examples
- [Tutorials](https://github.com/SuperInstance/dodecet-encoder/tree/main/tutorials) - Step-by-step guides
- [Integration Guide](https://github.com/SuperInstance/dodecet-encoder/blob/main/INTEGRATION_GUIDE.md) - Integration help

#### 4. Gitter/Discord (Coming Soon)
**For:** Real-time chat, community interaction

**When to use:**
- Quick questions
- Real-time collaboration
- Community hangout
- Live debugging help

## Community Guidelines

### Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

**Key principles:**
- Be respectful and considerate
- Use inclusive language
- Focus on constructive feedback
- Assume good intentions
- Help others learn

### Communication Guidelines

#### Asking Questions

**Good questions:**
- Include context and background
- Show what you've tried
- Provide minimal reproducible examples
- Include error messages and stack traces
- Specify your environment (Rust version, OS, etc.)

**Example good question:**
```
Subject: Trouble with dodecet arithmetic overflow

I'm trying to add two dodecets but getting unexpected results.

Environment:
- Rust 1.70.0
- dodecet-encoder 1.0.0
- Windows 11

Code:
```rust
let a = Dodecet::from_hex(0xFFF);
let b = Dodecet::from_hex(0x001);
let sum = a + b;
println!("Sum: {:03X}", sum.value()); // Expected 0x000, got 0x000
```

Expected: The sum should wrap around to 0x000
Actual: It wraps but I'm not sure if this is correct behavior

What I've tried:
- Checked the documentation for arithmetic operations
- Looked at the arithmetic examples
- Searched issues for "overflow"

Is this the expected behavior? Should I use wrapping_add instead?
```

#### Answering Questions

**Good answers:**
- Are respectful and encouraging
- Provide explanations, not just code
- Include examples and documentation links
- Acknowledge when you're unsure
- Help the asker understand the solution

**Example good answer:**
```rust
Great question! Dodecet arithmetic uses wrapping overflow by default.
This is intentional for performance reasons.

If you need checked arithmetic, use the checked operations:

let sum = a.checked_add(b)?;
// Returns Err on overflow

Or saturating arithmetic:

let sum = a.saturating_add(b);
// Returns Dodecet::MAX on overflow

See the Arithmetic Operations section in the docs for more details:
https://docs.rs/dodecet-encoder/latest/dodecet_encoder/struct.Dodecet.html#method.checked_add

Hope this helps!
```

### Contribution Guidelines

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

**Quick start:**
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Triaging Issues

### Issue Labels

We use labels to categorize issues:

- **bug:** Bug reports
- **enhancement:** Feature requests
- **documentation:** Documentation issues
- **performance:** Performance issues
- **good first issue:** Good for newcomers
- **help wanted:** Community help needed
- **priority: high:** High priority issues
- **priority: low:** Low priority issues

### Issue Priorities

**High Priority:**
- Security vulnerabilities
- Breaking changes
- Performance regressions
- Build failures
- Documentation errors that block usage

**Medium Priority:**
- Non-breaking bugs
- Feature requests
- Performance improvements
- Documentation improvements

**Low Priority:**
- Nice-to-have features
- Minor documentation improvements
- Code refactoring

## Release Schedule

### Version Support

- **1.0.x (Stable):** Full support, security updates
- **0.2.x (Legacy):** Critical fixes only
- **0.1.x (EOL):** End of life, no support

### Release Cadence

- **Major releases:** Every 6 months (breaking changes, major features)
- **Minor releases:** Monthly (new features, improvements)
- **Patch releases:** As needed (bug fixes, security updates)

### Upgrade Path

We provide migration guides for all breaking changes:

- [Migration Guide](docs/MIGRATION.md) - How to upgrade between versions
- [Breaking Changes](docs/BREAKING_CHANGES.md) - List of breaking changes
- [Deprecations](docs/DEPRECATIONS.md) - Upcoming deprecations

## Building from Source

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version  # Should be 1.70 or higher
cargo --version
```

### Build Commands

```bash
# Clone repository
git clone https://github.com/SuperInstance/dodecet-encoder.git
cd dodecet-encoder

# Build library
cargo build --release

# Run tests
cargo test --all

# Run examples
cargo run --example basic_usage

# Build documentation
cargo doc --open

# Run benchmarks
cargo bench
```

### Common Build Issues

**Error: `error: linker 'link.exe' not found`**
- **Solution:** Install MSVC build tools on Windows
- Link: https://rust-lang.github.io/rustup/installation/windows-msvc.html

**Error: `error: undefined reference to 'sqrt'`**
- **Solution:** Add `extern crate std;` or use `std::sqrt`
- This is a known issue with some standard library functions

**Error: `error: failed to run custom build command`**
- **Solution:** Ensure you have Rust 1.70 or higher
- Run `rustup update`

## Testing

### Running Tests

```bash
# Run all tests
cargo test --all

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_dodecet_creation

# Run tests in verbose mode
cargo test -- --verbose
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/

# View report
open coverage/index.html
```

## Performance

### Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench dodecet_benchmark dodecet_operations

# Compare benchmarks
cargo bench -- --save-baseline main
# Make changes
cargo bench -- --baseline main
```

### Performance Tips

1. **Use Dodecet Arrays** for batch operations
2. **Pre-allocate** arrays when size is known
3. **Use references** to avoid copies
4. **Leverage integer math** for performance
5. **Consider 12-bit precision** sufficient for discrete geometry

## Integration Help

### In Rust Projects

Add to `Cargo.toml`:
```toml
[dependencies]
dodecet-encoder = "1.0"
```

Basic usage:
```rust
use dodecet_encoder::{Dodecet, Point3D};

let point = Point3D::new(0x100, 0x200, 0x300);
let distance = point.distance_to(&other);
```

### In Web Applications

JavaScript/TypeScript support via WebAssembly:

```javascript
import { Dodecet, Point3D } from './dodecet-bindings.js';

const point = new Point3D(0x100, 0x200, 0x300);
console.log('Hex:', point.toHexString());
```

See [web_integration.html](examples/web_integration.html) for complete examples.

### For Constraint Theory

Integration with constraint theory systems:

```rust
use dodecet_encoder::{Point3D, pythagorean_snapping::PythagoreanSnapper};

let point = Point3D::new(1000, 2000, 3000);
let snapper = PythagoreanSnapper::new();
let snapped = snapper.snap(&point);
```

## Learning Resources

### Tutorials

1. [Getting Started](tutorials/00_GETTING_STARTED.md) - Introduction and setup
2. [Basic Operations](tutorials/01_BASIC_OPERATIONS.md) - Core operations
3. [Geometric Operations](tutorials/02_GEOMETRIC_OPERATIONS.md) - 3D geometry
4. [Calculus Operations](tutorials/03_CALCULUS_OPERATIONS.md) - Numerical methods
5. [Integration](tutorials/04_INTEGRATION.md) - Web and WASM
6. [Advanced Usage](tutorials/05_ADVANCED_USAGE.md) - Performance optimization

### Examples

- [Basic Usage](examples/basic_usage.rs) - Introduction to dodecets
- [Geometric Shapes](examples/geometric_shapes.rs) - 3D geometry
- [Pythagorean Snapping](examples/pythagorean_snapping.rs) - Constraint theory
- [Cellular Agents](examples/cellular_agents.rs) - SuperInstance integration
- [WebGL Integration](examples/webgl_integration.rs) - Browser visualization

### Documentation

- [API Reference](https://docs.rs/dodecet-encoder) - Complete API documentation
- [Architecture Diagram](ARCHITECTURE_DIAGRAM.md) - System design
- [Implementation Summary](IMPLEMENTATION_SUMMARY.md) - Implementation details
- [Integration Guide](INTEGRATION_GUIDE.md) - Integration patterns

## Community Events

### Office Hours (Coming Soon)

Monthly office hours for real-time Q&A:
- **When:** First Thursday of each month, 2:00 PM UTC
- **Where:** Discord voice channel
- **Topics:** Open Q&A, feature discussions, help

### Contributing Sprints (Coming Soon)

Monthly contribution sprints for newcomers:
- **When:** Third Saturday of each month
- **Where:** Discord + GitHub
- **Topics:** Good first issues, help with PRs

## Recognition

### Contributors

All contributors are recognized in [CONTRIBUTORS.md](CONTRIBUTORS.md).

### Contributors Hall of Fame

Notable contributions are recognized in:
- Release notes
- README acknowledgments
- Annual community report

## Contact

### Project Maintainers

- **SuperInstance Team:** team@superinstance.ai
- **GitHub Issues:** https://github.com/SuperInstance/dodecet-encoder/issues
- **GitHub Discussions:** https://github.com/SuperInstance/dodecet-encoder/discussions

### Social Media

- **Twitter:** @SuperInstanceAI
- **Mastodon:** @superinstance@fosstodon.org
- **Blog:** https://blog.superinstance.ai

## Emergency Contact

For urgent security issues:
- **Email:** security@superinstance.ai
- **PGP Key:** https://superinstance.ai/pgp

See [SECURITY.md](SECURITY.md) for details.

## FAQ

See our [FAQ](FAQ.md) for answers to frequently asked questions.

Common questions:
- Why 12 bits instead of 8 or 16?
- Can I use dodecets for floating-point operations?
- How do I migrate from 0.x to 1.0?
- What's the performance impact?

---

**Last Updated:** 2026-03-16
**Version:** 1.0.0
**Support Policy Version:** 1.0
