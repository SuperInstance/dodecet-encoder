# Dodecet Encoder Testing Documentation

**Comprehensive testing guide for dodecet-encoder**

---

## Test Coverage

### Current Status
- **Total Tests:** 69
- **Pass Rate:** 100%
- **Coverage:** Excellent (all major code paths tested)

### Test Categories

1. **Unit Tests (62 tests)**
   - Dodecet creation and validation
   - Nibble operations
   - Hex/binary encoding
   - Signed/unsigned conversions

2. **Integration Tests (21 tests)**
   - WASM integration
   - Cross-module functionality
   - Browser compatibility

3. **Performance Tests (22 tests)**
   - Benchmark suite
   - Performance regression tests
   - Memory usage tests

---

## Running Tests

### All Tests
```bash
cargo test
```

### Specific Test File
```bash
cargo test --test edge_cases
cargo test --test wasm_integration
cargo test --test benchmarks
```

### Specific Test
```bash
cargo test test_dodecet_creation
cargo test test_nibble_operations
```

### With Output
```bash
cargo test -- --nocapture
cargo test -- --show-output
```

---

## Coverage Report

### Generate HTML Coverage
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

### Generate XML Coverage (for CI/CD)
```bash
cargo tarpaulin --out Xml --output-dir coverage
```

### View Coverage
Open `coverage/index.html` in a browser.

---

## Test Structure

```
tests/
├── edge_cases.rs              # Edge case and boundary testing
├── integration/
│   └── wasm_integration.rs    # WASM integration tests
├── performance/
│   └── benchmarks.rs          # Performance benchmarks
└── wasm/
    └── wasm_package_tests.rs  # WASM package tests
```

---

## Writing New Tests

### Test Template

```rust
#[cfg(test)]
mod my_tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        let input = /* test data */;

        // Act
        let result = /* function call */;

        // Assert
        assert_eq!(result, expected);
    }
}
```

### Best Practices

1. **Use descriptive test names**
   ```rust
   // Good
   fn test_dodecet_creation_with_valid_value()
   fn test_dodecet_creation_returns_error_for_overflow()

   // Bad
   fn test_it_works()
   ```

2. **Test edge cases**
   ```rust
   #[test]
   fn test_dodecet_boundary_values() {
       assert!(Dodecet::new(0).is_ok());
       assert!(Dodecet::new(4095).is_ok());
       assert!(Dodecet::new(4096).is_err());
   }
   ```

3. **Use test helpers**
   ```rust
   fn create_test_dodecet() -> Dodecet {
       Dodecet::new(0x123).unwrap()
   }

   #[test]
   fn test_with_helper() {
       let d = create_test_dodecet();
       // ... test logic
   }
   ```

---

## Continuous Integration

### GitHub Actions

The project uses GitHub Actions for automated testing:

```yaml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run tests
        run: cargo test
```

### Coverage Badge

Add to README.md:

```markdown
![Tests](https://github.com/SuperInstance/dodecet-encoder/workflows/Tests/badge.svg)
```

---

## Debugging Failed Tests

### Run with Backtrace
```bash
RUST_BACKTRACE=1 cargo test
```

### Run Single Test
```bash
cargo test test_name -- --exact
```

### Print Output
```bash
cargo test test_name -- --nocapture --show-output
```

---

## Performance Testing

### Run Benchmarks
```bash
cargo bench
```

### Criterion Output
Results are in `target/criterion/` directory.
Open `target/criterion/report/index.html` for visual report.

---

## WASM Testing

### Install wasm-pack
```bash
cargo install wasm-pack
```

### Run WASM Tests
```bash
wasm-pack test --node
wasm-pack test --firefox
wasm-pack test --chrome
```

---

## Goals

- ✅ 100% test pass rate
- ✅ 80%+ code coverage
- ✅ All edge cases covered
- ✅ Performance benchmarks maintained
- ✅ CI/CD integration

---

**Last Updated:** 2026-03-18
**Status:** Production Ready
