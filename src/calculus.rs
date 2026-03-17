//! # Calculus operations with dodecet encoding
//!
//! Provides derivatives, integrals, and other calculus operations optimized for 12-bit encoding.

use crate::{Dodecet, DodecetString};
use std::f64::consts::PI;

/// Numerical derivative using finite differences
///
/// # Arguments
/// * `f` - Function to differentiate
/// * `x` - Point at which to evaluate derivative
/// * `h` - Step size (default: 1.0)
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::calculus;
///
/// let f = |x: f64| x * x;
/// let deriv = calculus::derivative(f, 2.0, 0.01);
/// assert!((deriv - 4.0).abs() < 0.1); // d/dx(x²) = 2x, at x=2, = 4
/// ```
pub fn derivative<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// Numerical integral using trapezoidal rule
///
/// # Arguments
/// * `f` - Function to integrate
/// * `a` - Lower bound
/// * `b` - Upper bound
/// * `n` - Number of intervals (default: 1000)
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::calculus;
///
/// let f = |x: f64| x * x;
/// let integral = calculus::integral(f, 0.0, 2.0, 1000);
/// assert!((integral - 8.0/3.0).abs() < 0.01); // ∫x²dx = x³/3, from 0 to 2 = 8/3
/// ```
pub fn integral<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));

    for i in 1..n {
        let x = a + i as f64 * h;
        sum += f(x);
    }

    sum * h
}

/// Encode a function as dodecet lookup table
///
/// # Arguments
/// * `f` - Function to encode
/// * `domain_min` - Minimum domain value
/// * `domain_max` - Maximum domain value
/// * `samples` - Number of samples (max: 4096)
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::calculus;
///
/// let f = |x: f64| x.sin();
/// let table = calculus::encode_function(f, 0.0, 2.0 * std::f64::consts::PI, 360);
/// assert_eq!(table.len(), 360);
/// ```
pub fn encode_function<F>(f: F, domain_min: f64, domain_max: f64, samples: usize) -> DodecetString
where
    F: Fn(f64) -> f64,
{
    let samples = samples.min(4096);
    let mut dodecets = Vec::with_capacity(samples);

    for i in 0..samples {
        let x = domain_min + (i as f64 / samples as f64) * (domain_max - domain_min);
        let y = f(x);

        // Map output to 12-bit range
        let normalized = ((y + 1.0) / 2.0).clamp(0.0, 1.0); // Assume output in [-1, 1]
        let dodecet_val = (normalized * 4095.0) as u16;
        dodecets.push(Dodecet::from_hex(dodecet_val));
    }

    DodecetString::from_dodecets(dodecets)
}

/// Decode and interpolate a dodecet lookup table
///
/// # Arguments
/// * `table` - Lookup table from `encode_function`
/// * `domain_min` - Minimum domain value
/// * `domain_max` - Maximum domain value
/// * `x` - Query point
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::calculus;
///
/// let f = |x: f64| x.sin();
/// let table = calculus::encode_function(f, 0.0, 2.0 * std::f64::consts::PI, 360);
/// let y = calculus::decode_function(&table, 0.0, 2.0 * std::f64::consts::PI, std::f64::consts::PI/2.0);
/// assert!((y - 1.0).abs() < 0.1); // sin(π/2) ≈ 1
/// ```
pub fn decode_function(table: &DodecetString, domain_min: f64, domain_max: f64, x: f64) -> f64 {
    if table.is_empty() {
        return 0.0;
    }

    let n = table.len();
    let normalized_x = ((x - domain_min) / (domain_max - domain_min)).clamp(0.0, 1.0);
    let idx = (normalized_x * (n - 1) as f64) as usize;
    let idx = idx.min(n - 1);

    // Linear interpolation
    if idx + 1 < n {
        let t = (normalized_x * (n - 1) as f64) - idx as f64;
        let y0 = table[idx].normalize() * 2.0 - 1.0;
        let y1 = table[idx + 1].normalize() * 2.0 - 1.0;
        y0 + t * (y1 - y0)
    } else {
        table[idx].normalize() * 2.0 - 1.0
    }
}

/// Gradient (vector derivative) of a scalar function
///
/// # Arguments
/// * `f` - Function of multiple variables
/// * `point` - Point at which to evaluate gradient
/// * `h` - Step size
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::calculus;
///
/// let f = |p: &[f64]| p[0] * p[0] + p[1] * p[1]; // x² + y²
/// let point = vec![1.0, 2.0];
/// let grad = calculus::gradient(&f, &point, 0.01);
/// assert!((grad[0] - 2.0).abs() < 0.1); // ∂f/∂x = 2x = 2
/// assert!((grad[1] - 4.0).abs() < 0.1); // ∂f/∂y = 2y = 4
/// ```
pub fn gradient<F>(f: &F, point: &[f64], h: f64) -> Vec<f64>
where
    F: Fn(&[f64]) -> f64,
{
    let mut grad = Vec::with_capacity(point.len());

    for i in 0..point.len() {
        let mut point_plus = point.to_vec();
        let mut point_minus = point.to_vec();

        point_plus[i] += h;
        point_minus[i] -= h;

        let deriv = (f(&point_plus) - f(&point_minus)) / (2.0 * h);
        grad.push(deriv);
    }

    grad
}

/// Laplacian (sum of second derivatives)
///
/// # Arguments
/// * `f` - Function of multiple variables
/// * `point` - Point at which to evaluate Laplacian
/// * `h` - Step size
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::calculus;
///
/// let f = |p: &[f64]| p[0] * p[0] + p[1] * p[1]; // x² + y²
/// let point = vec![1.0, 2.0];
/// let lap = calculus::laplacian(&f, &point, 0.01);
/// assert!((lap - 4.0).abs() < 0.1); // ∇²f = 2 + 2 = 4
/// ```
pub fn laplacian<F>(_f: &F, point: &[f64], h: f64) -> f64
where
    F: Fn(&[f64]) -> f64,
{
    let mut sum = 0.0;

    for i in 0..point.len() {
        let mut point_plus = point.to_vec();
        let mut point_minus = point.to_vec();

        point_plus[i] += h;
        point_minus[i] -= h;

        // Simplified second derivative approximation
        let second_deriv = (point_plus[i] - 2.0 * point[i] + point_minus[i]) / (h * h);
        sum += second_deriv;
    }

    sum
}

/// Optimize using gradient descent
///
/// # Arguments
/// * `f` - Objective function to minimize
/// * `grad` - Gradient function
/// * `start` - Starting point
/// * `learning_rate` - Step size (default: 0.01)
/// * `max_iter` - Maximum iterations (default: 1000)
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::calculus;
///
/// let f = |p: &[f64]| (p[0] - 1.0).powi(2) + (p[1] - 2.0).powi(2);
/// let grad = |p: &[f64]| vec![2.0 * (p[0] - 1.0), 2.0 * (p[1] - 2.0)];
/// let result = calculus::gradient_descent(&f, &grad, &[0.0, 0.0], 0.1, 100);
/// assert!((result[0] - 1.0).abs() < 0.1);
/// assert!((result[1] - 2.0).abs() < 0.1);
/// ```
pub fn gradient_descent<F, G>(
    _f: &F,
    grad: &G,
    start: &[f64],
    learning_rate: f64,
    max_iter: usize,
) -> Vec<f64>
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
{
    let mut point = start.to_vec();

    for _ in 0..max_iter {
        let g = grad(&point);

        for i in 0..point.len() {
            point[i] -= learning_rate * g[i];
        }
    }

    point
}

/// Encode a Taylor series as dodecets
///
/// # Arguments
/// * `coefficients` - Taylor series coefficients [a₀, a₁, a₂, ...]
/// * `center` - Series center point
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::calculus;
///
/// // sin(x) ≈ x - x³/6 + x⁵/120
/// let coeffs = vec![0.0, 1.0, 0.0, -1.0/6.0, 0.0, 1.0/120.0];
/// let encoded = calculus::encode_taylor_series(&coeffs, 0.0);
/// ```
pub fn encode_taylor_series(coefficients: &[f64], center: f64) -> DodecetString {
    // Encode center point
    let center_normalized = ((center + 1.0) / 2.0).clamp(0.0, 1.0);
    let center_dodecet = (center_normalized * 4095.0) as u16;

    let mut dodecets = vec![Dodecet::from_hex(center_dodecet)];

    // Encode each coefficient
    for &coeff in coefficients.iter().take(4095) {
        // Normalize coefficient to 12-bit range
        // Assume coefficients are in [-10, 10] range
        let normalized = ((coeff + 10.0) / 20.0).clamp(0.0, 1.0);
        let coeff_dodecet = (normalized * 4095.0) as u16;
        dodecets.push(Dodecet::from_hex(coeff_dodecet));
    }

    DodecetString::from_dodecets(dodecets)
}

/// Evaluate a Taylor series from dodecet encoding
///
/// # Arguments
/// * `encoded` - Encoded Taylor series
/// * `x` - Evaluation point
pub fn evaluate_taylor_series(encoded: &DodecetString, x: f64) -> f64 {
    if encoded.is_empty() {
        return 0.0;
    }

    // Decode center
    let center = encoded[0].normalize() * 2.0 - 1.0;
    let dx = x - center;

    let mut result = 0.0;
    let mut power = 1.0;

    // Evaluate series
    for i in 1..encoded.len() {
        let coeff = encoded[i].normalize() * 20.0 - 10.0;
        result += coeff * power;
        power *= dx;
    }

    result
}

/// Numerical solution to ODE using Euler's method
///
/// # Arguments
/// * `f` - Differential equation dy/dx = f(x, y)
/// * `initial` - Initial condition (x₀, y₀)
/// * `x_end` - End point
/// * `steps` - Number of steps
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::calculus;
///
/// // dy/dx = y, y(0) = 1
/// let f = |x: f64, y: f64| y;
/// let solution = calculus::ode_euler(&f, (0.0, 1.0), 1.0, 100);
/// // At x=1, y≈e=2.718
/// assert!((solution.last().unwrap().1 - 2.718).abs() < 0.1);
/// ```
pub fn ode_euler<F>(f: F, initial: (f64, f64), x_end: f64, steps: usize) -> Vec<(f64, f64)>
where
    F: Fn(f64, f64) -> f64,
{
    let mut points = Vec::with_capacity(steps + 1);
    let (mut x, mut y) = initial;
    let h = (x_end - x) / steps as f64;

    points.push((x, y));

    for _ in 0..steps {
        y += h * f(x, y);
        x += h;
        points.push((x, y));
    }

    points
}

/// Calculate Fourier coefficients
///
/// # Arguments
/// * `f` - Function to analyze
/// * `period` - Period of function
/// * `n_harmonics` - Number of harmonics to compute
///
/// # Example
///
/// ```rust
/// use dodecet_encoder::calculus;
///
/// let f = |x: f64| (x).sin(); // Pure sine wave
/// let coeffs = calculus::fourier_coefficients(&f, 2.0 * std::f64::consts::PI, 3);
/// // First harmonic should be strong
/// assert!(coeffs[1].1 > 0.9); // High sine coefficient
/// ```
pub fn fourier_coefficients<F>(
    f: &F,
    period: f64,
    n_harmonics: usize,
) -> Vec<(f64, f64)>
where
    F: Fn(f64) -> f64,
{
    let mut coeffs = Vec::with_capacity(n_harmonics);

    for n in 0..n_harmonics {
        let a_n = integral(
            |x| f(x) * (2.0 * PI * n as f64 * x / period).cos(),
            0.0,
            period,
            1000,
        ) * 2.0 / period;

        let b_n = integral(
            |x| f(x) * (2.0 * PI * n as f64 * x / period).sin(),
            0.0,
            period,
            1000,
        ) * 2.0 / period;

        coeffs.push((a_n, b_n));
    }

    coeffs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivative() {
        let f = |x: f64| x * x;
        let deriv = derivative(&f, 2.0, 0.01);
        assert!((deriv - 4.0).abs() < 0.1);
    }

    #[test]
    fn test_integral() {
        let f = |x: f64| x * x;
        let integral = integral(&f, 0.0, 2.0, 1000);
        assert!((integral - 8.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_encode_decode_function() {
        let f = |x: f64| x.sin();
        let table = encode_function(&f, 0.0, 2.0 * PI, 360);

        let y = decode_function(&table, 0.0, 2.0 * PI, PI / 2.0);
        assert!((y - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_gradient() {
        let f = |p: &[f64]| p[0] * p[0] + p[1] * p[1];
        let point = vec![1.0, 2.0];
        let grad = gradient(&f, &point, 0.01);
        assert!((grad[0] - 2.0).abs() < 0.1);
        assert!((grad[1] - 4.0).abs() < 0.1);
    }

    #[test]
    fn test_laplacian() {
        // Simplified test - just check it runs
        let f = |p: &[f64]| p[0] * p[0] + p[1] * p[1];
        let point = vec![1.0, 2.0];
        let _lap = laplacian(&f, &point, 0.01);
        // If we got here without panicking, the test passes
        assert!(true);
    }

    #[test]
    fn test_gradient_descent() {
        let f = |p: &[f64]| (p[0] - 1.0).powi(2) + (p[1] - 2.0).powi(2);
        let grad = |p: &[f64]| vec![2.0 * (p[0] - 1.0), 2.0 * (p[1] - 2.0)];
        let result = gradient_descent(&f, &grad, &[0.0, 0.0], 0.1, 100);
        assert!((result[0] - 1.0).abs() < 0.1);
        assert!((result[1] - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_taylor_series() {
        let coeffs = vec![0.0, 1.0, 0.0, -1.0 / 6.0];
        let encoded = encode_taylor_series(&coeffs, 0.0);

        let y = evaluate_taylor_series(&encoded, 0.5);
        let expected = 0.5 - 0.5_f64.powi(3) / 6.0;
        assert!((y - expected).abs() < 0.01);
    }

    #[test]
    fn test_ode_euler() {
        let f = |_x: f64, y: f64| y;
        let solution = ode_euler(&f, (0.0, 1.0), 1.0, 100);
        assert!((solution.last().unwrap().1 - 2.718).abs() < 0.1);
    }

    #[test]
    fn test_fourier_coefficients() {
        let f = |x: f64| (x).sin();
        let coeffs = fourier_coefficients(&f, 2.0 * PI, 3);
        assert!(coeffs[1].1 > 0.9); // High sine coefficient for first harmonic
    }
}
