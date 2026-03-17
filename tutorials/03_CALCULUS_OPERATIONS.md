# Calculus Operations with Dodecet Encoder

**Numerical methods and calculus operations using 12-bit dodecet encoding**

## Table of Contents

1. [Introduction](#introduction)
2. [Derivatives](#derivatives)
3. [Integrals](#integrals)
4. [Gradient and Optimization](#gradient-and-optimization)
5. [Function Encoding](#function-encoding)
6. [Practical Applications](#practical-applications)
7. [Exercises](#exercises)

---

## Introduction

Dodecet encoding is well-suited for numerical calculus operations:

- **Discrete Precision**: 4,096 values provide sufficient resolution
- **Fast Operations**: Integer arithmetic is faster than floating-point
- **Deterministic**: No rounding errors or drift
- **Memory Efficient**: Compact representation of functions

### Why Calculus with Dodecets?

```
Traditional floating-point:
- Slow operations (especially on embedded systems)
- Non-deterministic across platforms
- Rounding errors accumulate

Dodecet encoding:
- Fast integer operations
- Deterministic results
- No rounding errors
- Memory efficient
```

---

## Derivatives

### Numerical Derivative

```rust
use dodecet_encoder::calculus;

// Define a function: f(x) = x²
let f = |x: f64| x * x;

// Calculate derivative at x = 2.0
let x = 2.0;
let h = 0.01; // Step size
let deriv = calculus::derivative(&f, x, h);

// f'(x) = 2x, so f'(2) = 4
assert!((deriv - 4.0).abs() < 0.01);
```

### Higher Order Derivatives

```rust
// f(x) = x³
let f = |x: f64| x.powi(3);

// First derivative: f'(x) = 3x²
let deriv1 = calculus::derivative(&f, 2.0, 0.01);
// Expected: 3 * 4 = 12

// Second derivative: f''(x) = 6x
let deriv2 = calculus::derivative(&|x| {
    calculus::derivative(&f, x, 0.01)
}, 2.0, 0.01);
// Expected: 6 * 2 = 12
```

### Partial Derivatives

```rust
// Multivariate function: f(x, y) = x² + y²
let f = |vars: &[f64]| vars[0].powi(2) + vars[1].powi(2);

// Partial derivative with respect to x at (2, 3)
let point = [2.0, 3.0];
let h = 0.01;

// ∂f/∂x = 2x = 4
let partial_x = calculus::partial_derivative(&f, &point, 0, h);

// ∂f/∂y = 2y = 6
let partial_y = calculus::partial_derivative(&f, &point, 1, h);
```

### Gradient Vector

```rust
// f(x, y) = x² + y²
let f = |vars: &[f64]| vars[0].powi(2) + vars[1].powi(2);

// Calculate gradient at (2, 3)
let point = [2.0, 3.0];
let grad = calculus::gradient(&f, &point, 0.01);

// Gradient = [2x, 2y] = [4, 6]
assert!((grad[0] - 4.0).abs() < 0.01);
assert!((grad[1] - 6.0).abs() < 0.01);
```

---

## Integrals

### Definite Integral

```rust
use dodecet_encoder::calculus;

// f(x) = x²
let f = |x: f64| x * x;

// Integrate from 0 to 2: ∫x²dx = [x³/3]₀² = 8/3 ≈ 2.667
let integral = calculus::integral(&f, 0.0, 2.0, 1000);
assert!((integral - 2.667).abs() < 0.01);
```

### Numerical Integration Methods

```rust
// Trapezoidal rule (default)
let trapezoidal = calculus::integral(&f, 0.0, 2.0, 1000);

// Simpson's rule (more accurate)
let simpson = calculus::integral_simpson(&f, 0.0, 2.0, 1000);

// Simpson's rule is typically more accurate
assert!((simpson - 2.667).abs() < (trapezoidal - 2.667).abs());
```

### Multiple Integrals

```rust
// f(x, y) = x + y
let f = |vars: &[f64]| vars[0] + vars[1];

// Integrate over rectangle [0,1] × [0,2]
// ∫₀¹∫₀² (x + y) dy dx
let x_range = (0.0, 1.0);
let y_range = (0.0, 2.0);
let result = calculus::double_integral(&f, x_range, y_range, 100);

// Analytical solution: ∫₀¹ [xy + y²/2]₀² dx = ∫₀¹ (2x + 2) dx = [x² + 2x]₀¹ = 3
assert!((result - 3.0).abs() < 0.1);
```

---

## Gradient and Optimization

### Gradient Descent

```rust
use dodecet_encoder::calculus;

// Objective function: f(x, y) = (x-1)² + (y-2)²
let objective = |vars: &[f64]| {
    (vars[0] - 1.0).powi(2) + (vars[1] - 2.0).powi(2)
};

// Gradient: ∇f = [2(x-1), 2(y-2)]
let gradient = |vars: &[f64]| {
    vec![2.0 * (vars[0] - 1.0), 2.0 * (vars[1] - 2.0)]
};

// Initial guess
let initial = vec![0.0, 0.0];

// Run gradient descent
let result = calculus::gradient_descent(
    &objective,
    &gradient,
    &initial,
    0.1,  // Learning rate
    1000  // Max iterations
);

// Should converge to [1, 2] (the minimum)
assert!((result[0] - 1.0).abs() < 0.01);
assert!((result[1] - 2.0).abs() < 0.01);
```

### Finding Minima

```rust
// f(x) = x² - 4x + 3
let f = |x: f64| x * x - 4.0 * x + 3.0;

// Find minimum using gradient descent
let deriv = |x: f64| calculus::derivative(&f, x, 0.01);
let result = calculus::minimize_1d(&f, 0.0, 0.1, 1000);

// Minimum at x = 2 (f'(x) = 2x - 4 = 0)
assert!((result - 2.0).abs() < 0.01);
```

### Finding Maxima

```rust
// f(x) = -x² + 4x - 3
let f = |x: f64| -x * x + 4.0 * x - 3.0;

// Find maximum
let result = calculus::maximize_1d(&f, 0.0, 0.1, 1000);

// Maximum at x = 2
assert!((result - 2.0).abs() < 0.01);
```

---

## Function Encoding

### Encoding Functions

```rust
use dodecet_encoder::calculus;

// f(x) = sin(x)
let f = |x: f64| x.sin();

// Encode over [0, 2π] with 360 samples
let table = calculus::encode_function(
    &f,
    0.0,
    2.0 * std::f64::consts::PI,
    360
);

// Table is now a compact representation
// Each value is a dodecet (12 bits)
```

### Decoding Functions

```rust
// Evaluate encoded function at x = π/2
let y = calculus::decode_function(
    &table,
    0.0,
    2.0 * std::f64::consts::PI,
    std::f64::consts::PI / 2.0
);

// sin(π/2) = 1.0
assert!((y - 1.0).abs() < 0.01);
```

### Interpolation

```rust
// Decode function uses linear interpolation
// for values between sample points

let y1 = calculus::decode_function(&table, start, end, 1.57);
let y2 = calculus::decode_function(&table, start, end, 1.58);

// Smooth interpolation between values
assert!((y2 - y1).abs() < 0.1);
```

---

## Practical Applications

### 1. Path Optimization

```rust
// Minimize path length through points
let points = vec![
    (0.0, 0.0),
    (1.0, 2.0),
    (3.0, 1.0),
    (4.0, 3.0),
];

// Total distance function
let total_distance = |vars: &[f64]| {
    // vars represents visiting order
    // Calculate total distance
    // ...
    0.0 // Simplified
};

// Optimize visiting order using gradient descent
// let optimal_order = calculus::gradient_descent(...);
```

### 2. Curve Fitting

```rust
// Fit quadratic to data points
let data = vec![
    (0.0, 1.0),
    (1.0, 2.0),
    (2.0, 5.0),
];

// Error function: sum of squared residuals
let error = |coeffs: &[f64]| {
    // coeffs = [a, b, c] for ax² + bx + c
    coeffs.iter().zip(data.iter())
        .map(|(&c, &(x, y)| {
            let predicted = coeffs[0] * x * x + coeffs[1] * x + coeffs[2];
            (predicted - y).powi(2)
        })
        .sum()
};

// Minimize error to find best fit
// let best_fit = calculus::minimize(&error, ...);
```

### 3. Numerical Root Finding

```rust
// Find root of f(x) = x² - 4
let f = |x: f64| x * x - 4.0;

// Use Newton's method
let root = calculus::newton_raphson(
    &f,
    &|x| calculus::derivative(&f, x, 0.001),
    3.0,  // Initial guess
    0.0001,  // Tolerance
    100   // Max iterations
);

// Root at x = 2
assert!((root - 2.0).abs() < 0.01);
```

### 4. Area Under Curve

```rust
// Calculate area under normal distribution
let normal = |x: f64| {
    (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt()
};

// Integrate from -1 to 1 (68% of data)
let area = calculus::integral(&normal, -1.0, 1.0, 10000);

// Should be ~0.68
assert!((area - 0.68).abs() < 0.01);
```

---

## Performance Considerations

### Step Size Selection

```rust
// Too large: inaccurate
let h_large = 0.1;
let deriv_large = calculus::derivative(&f, x, h_large);

// Too small: numerical errors
let h_small = 0.0001;
let deriv_small = calculus::derivative(&f, x, h_small);

// Just right: balanced
let h_optimal = 0.01;
let deriv_optimal = calculus::derivative(&f, x, h_optimal);
```

### Number of Samples

```rust
// More samples = more accurate but slower
let integral_100 = calculus::integral(&f, 0.0, 1.0, 100);
let integral_1000 = calculus::integral(&f, 0.0, 1.0, 1000);
let integral_10000 = calculus::integral(&f, 0.0, 1.0, 10000);

// Diminishing returns after certain point
```

---

## Exercises

### Exercise 1: Calculate Arc Length

Calculate the length of a curve using integrals:

```rust
// Arc length of y = x² from x=0 to x=1
// L = ∫√(1 + (dy/dx)²) dx

fn arc_length(f: &dyn Fn(f64) -> f64, start: f64, end: f64) -> f64 {
    // Your code here
    // 1. Calculate derivative
    // 2. Integrate sqrt(1 + (f'(x))²)
}
```

**Solution:**
```rust
fn arc_length(f: &dyn Fn(f64) -> f64, start: f64, end: f64) -> f64 {
    let integrand = |x: f64| {
        let deriv = calculus::derivative(f, x, 0.001);
        (1.0 + deriv * deriv).sqrt()
    };
    calculus::integral(&integrand, start, end, 1000)
}
```

### Exercise 2: Optimize Function

Find the minimum of a complex function:

```rust
// f(x, y) = x² + 2xy + y² - 4x - 6y + 10
let f = |vars: &[f64]| {
    let x = vars[0];
    let y = vars[1];
    x * x + 2.0 * x * y + y * y - 4.0 * x - 6.0 * y + 10.0
};

// Find minimum using gradient descent
// Your code here
```

### Exercise 3: Encode and Decode

Create a fast lookup table for trigonometric functions:

```rust
// Create lookup table for sine
let sine_table = calculus::encode_function(
    &|x| x.sin(),
    0.0,
    2.0 * std::f64::consts::PI,
    360
);

// Compare speed of lookup vs calculation
// Your code here
```

---

## Next Steps

- [Tutorial 4: Integration](04_INTEGRATION.md) - WebAssembly and web
- [Tutorial 5: Advanced Usage](05_ADVANCED_USAGE.md) - Performance optimization
- [Examples](../examples/) - Real-world applications

---

**Found an issue?** [Report it here](https://github.com/SuperInstance/dodecet-encoder/issues)

**Need help?** [Ask in discussions](https://github.com/SuperInstance/dodecet-encoder/discussions)

---

**Last Updated:** 2026-03-16
**Tutorial:** 03 - Calculus Operations
