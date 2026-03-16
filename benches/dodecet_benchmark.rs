//! Performance benchmarks for dodecet operations

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use dodecet_encoder::{Dodecet, DodecetArray, DodecetString};

fn bench_dodecet_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("dodecet_creation");

    group.bench_function("from_hex", |b| {
        b.iter(|| Dodecet::from_hex(black_box(0xABC)));
    });

    group.bench_function("new_checked", |b| {
        b.iter(|| Dodecet::new(black_box(0xABC)));
    });

    group.finish();
}

fn bench_dodecet_operations(c: &mut Criterion) {
    let d = Dodecet::from_hex(0xABC);
    let d2 = Dodecet::from_hex(0x123);

    let mut group = c.benchmark_group("dodecet_operations");

    group.bench_function("nibble_access", |b| {
        b.iter(|| {
            black_box(d).nibble(black_box(1)).unwrap();
        });
    });

    group.bench_function("bitwise_and", |b| {
        b.iter(|| black_box(d).and(black_box(d2)));
    });

    group.bench_function("bitwise_or", |b| {
        b.iter(|| black_box(d).or(black_box(d2)));
    });

    group.bench_function("bitwise_xor", |b| {
        b.iter(|| black_box(d).xor(black_box(d2)));
    });

    group.bench_function("arithmetic_add", |b| {
        b.iter(|| black_box(d) + black_box(d2));
    });

    group.bench_function("arithmetic_mul", |b| {
        b.iter(|| black_box(d) * black_box(d2));
    });

    group.bench_function("normalize", |b| {
        b.iter(|| black_box(d).normalize());
    });

    group.finish();
}

fn bench_dodecet_conversions(c: &mut Criterion) {
    let d = Dodecet::from_hex(0xABC);

    let mut group = c.benchmark_group("dodecet_conversions");

    group.bench_function("to_hex_string", |b| {
        b.iter(|| black_box(d).to_hex_string());
    });

    group.bench_function("to_binary_string", |b| {
        b.iter(|| black_box(d).to_binary_string());
    });

    group.bench_function("from_hex_str", |b| {
        b.iter(|| Dodecet::from_hex_str(black_box("ABC")));
    });

    group.finish();
}

fn bench_array_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_operations");

    for size in [2, 3, 4, 8, 16].iter() {
        group.bench_with_input(BenchmarkId::new("from_slice", size), size, |b, &size| {
            let values: Vec<u16> = (0..size).map(|i| i as u16).collect();
            b.iter(|| DodecetArray::<3>::from_slice(black_box(&values)));
        });
    }

    group.finish();
}

fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    let sizes = vec![10, 100, 1000];

    for size in sizes {
        group.bench_with_input(BenchmarkId::new("push", size), &size, |b, &size| {
            b.iter(|| {
                let mut s = DodecetString::with_capacity(size);
                for i in 0..size {
                    s.push(black_box(i as u16 % 4096));
                }
                black_box(s);
            });
        });

        group.bench_with_input(BenchmarkId::new("to_hex_string", size), &size, |b, &size| {
            let s: DodecetString = (0..size).map(|i| Dodecet::from_hex(i as u16 % 4096)).collect();
            b.iter(|| black_box(&s).to_hex_string());
        });

        group.bench_with_input(BenchmarkId::new("from_hex_str", size), &size, |b, &size| {
            let hex: String = (0..size).map(|i| format!("{:03X}", i % 4096)).collect();
            b.iter(|| DodecetString::from_hex_str(black_box(&hex)));
        });
    }

    group.finish();
}

fn bench_hex_encoding(c: &mut Criterion) {
    use dodecet_encoder::hex;

    let mut group = c.benchmark_group("hex_encoding");

    let dodecets: Vec<Dodecet> = (0..100).map(|i| Dodecet::from_hex(i)).collect();

    group.bench_function("encode_100", |b| {
        b.iter(|| hex::encode(black_box(&dodecets)));
    });

    let hex_string = dodecets.iter().map(|d| d.to_hex_string()).collect::<String>();

    group.bench_function("decode_100", |b| {
        b.iter(|| hex::decode(black_box(&hex_string)));
    });

    group.finish();
}

fn bench_geometric_operations(c: &mut Criterion) {
    use dodecet_encoder::geometric::{Point3D, Vector3D};

    let mut group = c.benchmark_group("geometric_operations");

    let p1 = Point3D::new(0x100, 0x200, 0x300);
    let p2 = Point3D::new(0x400, 0x500, 0x600);

    group.bench_function("point_creation", |b| {
        b.iter(|| Point3D::new(black_box(0x100), black_box(0x200), black_box(0x300)));
    });

    group.bench_function("point_distance", |b| {
        b.iter(|| black_box(p1).distance_to(black_box(&p2)));
    });

    let v1 = Vector3D::new(100, 200, 300);
    let v2 = Vector3D::new(400, 500, 600);

    group.bench_function("vector_dot", |b| {
        b.iter(|| black_box(v1).dot(black_box(&v2)));
    });

    group.bench_function("vector_cross", |b| {
        b.iter(|| black_box(v1).cross(black_box(&v2)));
    });

    group.finish();
}

fn bench_calculus_operations(c: &mut Criterion) {
    use dodecet_encoder::calculus;

    let mut group = c.benchmark_group("calculus_operations");

    let f = |x: f64| x.sin();

    group.bench_function("derivative", |b| {
        b.iter(|| calculus::derivative(&f, black_box(1.0), black_box(0.01)));
    });

    group.bench_function("integral", |b| {
        b.iter(|| calculus::integral(&f, black_box(0.0), black_box(std::f64::consts::PI), black_box(1000)));
    });

    group.bench_function("encode_function", |b| {
        b.iter(|| calculus::encode_function(&f, 0.0, 2.0 * std::f64::consts::PI, 360));
    });

    group.finish();
}

fn bench_comparison_with_u8(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison_with_u8");

    // Compare storage density
    group.bench_function("u8_array_256_bytes", |b| {
        b.iter(|| {
            let data: [u8; 256] = [0; 256];
            black_box(data);
        });
    });

    group.bench_function("dodecet_string_171", |b| {
        b.iter(|| {
            // 171 dodecets = 2052 bytes = ~256 bytes packed
            let data: DodecetString = (0..171).map(|_| Dodecet::from_hex(0)).collect();
            black_box(data);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_dodecet_creation,
    bench_dodecet_operations,
    bench_dodecet_conversions,
    bench_array_operations,
    bench_string_operations,
    bench_hex_encoding,
    bench_geometric_operations,
    bench_calculus_operations,
    bench_comparison_with_u8
);

criterion_main!(benches);
