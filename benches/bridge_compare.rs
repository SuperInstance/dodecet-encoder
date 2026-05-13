//! Benchmark comparing Rust vs C bridge Eisenstein snap performance.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use dodecet_encoder::eisenstein::EisensteinConstraint;

#[cfg(feature = "c-bridge")]
use dodecet_encoder::c_bridge::CBridgeEisensteinConstraint;

/// Generate deterministic test points using a simple PRNG.
fn generate_points(n: usize) -> Vec<(f64, f64)> {
    let mut seed: u64 = 42;
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let x = ((seed >> 33) as f64 / (1u64 << 31) as f64) * 20.0 - 10.0;
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let y = ((seed >> 33) as f64 / (1u64 << 31) as f64) * 20.0 - 10.0;
        points.push((x, y));
    }
    points
}

fn bench_rust_snap(c: &mut Criterion) {
    let ec = EisensteinConstraint::new();
    let points = generate_points(100_000);

    c.bench_function("rust_eisenstein_snap_100k", |b| {
        b.iter(|| {
            for &(x, y) in black_box(&points) {
                black_box(ec.snap(x, y));
            }
        })
    });
}

#[cfg(feature = "c-bridge")]
fn bench_c_bridge_snap(c: &mut Criterion) {
    let cb = CBridgeEisensteinConstraint::new();
    let points = generate_points(100_000);

    c.bench_function("c_bridge_eisenstein_snap_100k", |b| {
        b.iter(|| {
            for &(x, y) in black_box(&points) {
                black_box(cb.snap(x, y));
            }
        })
    });
}

#[cfg(feature = "c-bridge")]
fn bench_c_bridge_raw_snap(c: &mut Criterion) {
    let cb = CBridgeEisensteinConstraint::new();
    let points = generate_points(100_000);

    c.bench_function("c_bridge_raw_snap_100k", |b| {
        b.iter(|| {
            for &(x, y) in black_box(&points) {
                black_box(cb.snap_raw(x, y));
            }
        })
    });
}

#[cfg(feature = "c-bridge")]
fn bench_comparison(c: &mut Criterion) {
    let ec = EisensteinConstraint::new();
    let cb = CBridgeEisensteinConstraint::new();
    let points = generate_points(100_000);

    let mut group = c.benchmark_group("snap_comparison");
    group.throughput(Throughput::Elements(100_000));

    group.bench_function("rust", |b| {
        b.iter(|| {
            for &(x, y) in black_box(&points) {
                black_box(ec.snap(x, y));
            }
        })
    });

    group.bench_function("c_bridge", |b| {
        b.iter(|| {
            for &(x, y) in black_box(&points) {
                black_box(cb.snap(x, y));
            }
        })
    });

    group.bench_function("c_bridge_raw", |b| {
        b.iter(|| {
            for &(x, y) in black_box(&points) {
                black_box(cb.snap_raw(x, y));
            }
        })
    });

    group.finish();
}

#[cfg(feature = "c-bridge")]
fn verify_correctness(c: &mut Criterion) {
    let ec = EisensteinConstraint::new();
    let cb = CBridgeEisensteinConstraint::new();
    let points = generate_points(100_000);

    c.bench_function("correctness_verify_100k", |b| {
        b.iter(|| {
            let mut mismatches = 0u64;
            for &(x, y) in black_box(&points) {
                let r = ec.snap(x, y);
                let c = cb.snap(x, y);
                if r.snap_a != c.snap_a || r.snap_b != c.snap_b {
                    mismatches += 1;
                }
            }
            black_box(mismatches);
        })
    });
}

#[cfg(feature = "c-bridge")]
criterion_group!(
    benches,
    bench_rust_snap,
    bench_c_bridge_snap,
    bench_c_bridge_raw_snap,
    bench_comparison,
    verify_correctness,
);

#[cfg(not(feature = "c-bridge"))]
criterion_group!(benches, bench_rust_snap);

criterion_main!(benches);
