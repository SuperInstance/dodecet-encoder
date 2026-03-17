# Contributing to Dodecet Encoder

Thank you for your interest in contributing to the dodecet-encoder project! This document provides guidelines and instructions for contributing.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Coding Standards](#coding-standards)
5. [Testing Guidelines](#testing-guidelines)
6. [Documentation Standards](#documentation-standards)
7. [Pull Request Process](#pull-request-process)

---

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please:

- Be respectful and considerate
- Use inclusive language
- Focus on constructive feedback
- Assume good intentions

For detailed guidelines, see [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).

---

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- A GitHub account

### Setup Development Environment

```bash
# Fork the repository
# Click "Fork" on https://github.com/SuperInstance/dodecet-encoder

# Clone your fork
git clone https://github.com/YOUR_USERNAME/dodecet-encoder.git
cd dodecet-encoder

# Add upstream remote
git remote add upstream https://github.com/SuperInstance/dodecet-encoder.git

# Install development dependencies
cargo install cargo-watch
cargo install cargo-edit

# Run tests to verify setup
cargo test --all
```

### Development Tools

We recommend installing:

```bash
# Watch for file changes and run tests
cargo install cargo-watch

# Automatically add dependencies
cargo install cargo-edit

# Format checking
cargo install cargo-temp

# Linting
cargo install clippy
```

---

## Development Workflow

### 1. Create a Branch

```bash
# Update from upstream
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

```bash
# Watch for changes and run tests
cargo watch -x test

# Watch for changes and check
cargo watch -x 'check --all'
```

### 3. Commit Your Changes

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "feat: add new geometric operation"

# Use conventional commit messages
# feat: new feature
# fix: bug fix
# docs: documentation changes
# test: adding tests
# refactor: code refactoring
# perf: performance improvements
# chore: maintenance tasks
```

### 4. Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create pull request on GitHub
# Visit: https://github.com/SuperInstance/dodecet-encoder/compare
```

---

## Coding Standards

### Rust Style

Follow standard Rust conventions:

```rust
// Use rustfmt
cargo fmt

// Use clippy for lints
cargo clippy -- -D warnings

// Run both
cargo fmt --check
cargo clippy -- -D warnings
```

### Naming Conventions

```rust
// Types: PascalCase
struct DodecetArray;
enum State;

// Functions: snake_case
fn calculate_distance() {}

// Constants: SCREAMING_SNAKE_CASE
const MAX_VALUE: u16 = 4095;

// Modules: snake_case
mod geometric_operations;
```

### Documentation

All public items must be documented:

```rust
/// Calculate the Euclidean distance between two points.
///
/// # Arguments
///
/// * `other` - The other point to measure distance to
///
/// # Returns
///
/// The distance as a f64 value
///
/// # Examples
///
/// ```
/// use dodecet_encoder::Point3D;
///
/// let p1 = Point3D::new(100, 200, 300);
/// let p2 = Point3D::new(400, 500, 600);
/// let distance = p1.distance_to(&p2);
/// ```
///
/// # Performance
///
/// This operation is O(1) and runs in ~45ns.
fn distance_to(&self, other: &Point3D) -> f64 {
    // implementation
}
```

### Error Handling

```rust
// Use Result for fallible operations
pub fn from_hex(value: u16) -> Result<Dodecet, Error> {
    if value > 0xFFF {
        return Err(Error::InvalidValue(value));
    }
    Ok(Dodecet { value })
}

// Use Option for nullable values
pub fn nibble(&self, index: u8) -> Option<u8> {
    if index > 2 {
        return None;
    }
    Some((self.value >> (index * 4)) & 0xF)
}
```

---

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dodecet_creation() {
        let d = Dodecet::from_hex(0xABC);
        assert_eq!(d.value(), 0xABC);
    }

    #[test]
    fn test_dodecet_arithmetic() {
        let a = Dodecet::from_hex(0x100);
        let b = Dodecet::from_hex(0x200);
        let sum = a + b;
        assert_eq!(sum.value(), 0x300);
    }

    #[test]
    #[should_panic]
    fn test_invalid_value() {
        Dodecet::new(5000).unwrap();
    }
}
```

### Integration Tests

Create tests in `tests/` directory:

```rust
// tests/integration_test.rs
use dodecet_encoder::{Dodecet, Point3D};

#[test]
fn test_complete_workflow() {
    // Create dodecet
    let d = Dodecet::from_hex(0xABC);

    // Create point
    let point = Point3D::new(0x100, 0x200, 0x300);

    // Calculate distance
    let other = Point3D::new(0x400, 0x500, 0x600);
    let distance = point.distance_to(&other);

    assert!(distance > 0.0);
}
```

### Performance Tests

Use Criterion for benchmarks:

```rust
// benches/dodecet_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use dodecet_encoder::Dodecet;

fn bench_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("dodecet_creation");

    for value in [0x000, 0xABC, 0xFFF].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(value), value, |b, &value| {
            b.iter(|| Dodecet::from_hex(black_box(value)));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_creation);
criterion_main!(benches);
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

---

## Documentation Standards

### API Documentation

- Every public item must have a doc comment
- Include examples for complex operations
- Document panics and errors
- Note performance characteristics

### README Documentation

Keep README.md up to date with:
- Project description
- Installation instructions
- Quick start guide
- Feature overview
- Links to detailed docs

### Tutorial Documentation

Create tutorials in `tutorials/` directory:
- Number sequentially: `00_`, `01_`, etc.
- Use clear, step-by-step instructions
- Include code examples
- Add expected output

### Example Documentation

Examples in `examples/` directory:
- Must compile and run
- Include comments
- Demonstrate single concept
- Show expected output in comments

---

## Pull Request Process

### Before Submitting

1. **Run all tests**
   ```bash
   cargo test --all
   ```

2. **Check formatting**
   ```bash
   cargo fmt --check
   ```

3. **Run clippy**
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Update documentation**
   - Update relevant documentation
   - Add doc comments to new items
   - Update CHANGELOG.md

5. **Add tests**
   - Unit tests for new functionality
   - Integration tests for workflows
   - Performance benchmarks if relevant

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] All tests passing

## Documentation
- [ ] API documentation updated
- [ ] README updated (if needed)
- [ ] Tutorials updated (if needed)

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added to complex code
- [ ] Documentation updated
- [ ] No new warnings generated
- [ ] Tests added/updated
- [ ] All tests passing
```

### Review Process

1. Automated checks must pass
2. At least one maintainer approval required
3. Address all review comments
4. Update PR based on feedback

### Merge Policy

- Maintainers will review and merge PRs
- Squash commits for clean history
- Update version number if needed
- Tag release for significant changes

---

## Community Guidelines

### Communication

- Use GitHub issues for bug reports and feature requests
- Use GitHub discussions for questions and ideas
- Be patient with responses (maintainers volunteer time)

### Issue Reporting

When reporting issues, include:

- Rust version (`rustc --version`)
- Operating system
- Minimal reproduction code
- Expected vs actual behavior
- Backtrace if applicable

### Feature Requests

When requesting features:

- Describe the use case
- Explain why it's important
- Suggest API design if possible
- Consider contributing implementation

---

## Recognition

Contributors will be:

- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Credited in significant features

---

## Questions?

- Open an issue for bugs or feature requests
- Start a discussion for questions
- Check existing issues and discussions first

---

**Thank you for contributing to dodecet-encoder!**

Every contribution, no matter how small, helps make the project better for everyone.
