// Performance Comparison Example - Dodecet vs Traditional Encoding
//
// This example provides comprehensive performance benchmarks comparing
// dodecet encoding with traditional 8-bit and floating-point approaches.

use dodecet_encoder::{Dodecet, DodecetArray, Point3D, Vector3D};
use std::time::{Duration, Instant};

/// Benchmark result structure
#[derive(Debug)]
struct BenchmarkResult {
    name: String,
    iterations: usize,
    total_time: Duration,
    avg_time_ns: f64,
    ops_per_second: f64,
}

impl BenchmarkResult {
    fn new(name: String, iterations: usize, total_time: Duration) -> Self {
        let avg_time_ns = total_time.as_nanos() as f64 / iterations as f64;
        let ops_per_second = 1_000_000_000.0 / avg_time_ns;

        BenchmarkResult {
            name,
            iterations,
            total_time,
            avg_time_ns,
            ops_per_second,
        }
    }

    fn print(&self) {
        println!("   {:<30} | {:>10} iter | {:>10.2} ns/op | {:>12.0} ops/s",
            self.name, self.iterations, self.avg_time_ns, self.ops_per_second);
    }
}

/// Memory comparison structure
#[derive(Debug)]
struct MemoryComparison {
    name: String,
    bytes_per_item: usize,
    total_bytes: usize,
    items: usize,
}

impl MemoryComparison {
    fn print(&self) {
        println!("   {:<30} | {:>8} bytes/item | {:>12} bytes total | {:>10} items",
            self.name, self.bytes_per_item, self.total_bytes, self.items);
    }
}

fn benchmark_dodecet_creation(iterations: usize) -> BenchmarkResult {
    let start = Instant::now();
    for i in 0..iterations {
        let _ = Dodecet::new((i % 4096) as u16);
    }
    let elapsed = start.elapsed();
    BenchmarkResult::new("Dodecet Creation".to_string(), iterations, elapsed)
}

fn benchmark_dodecet_arithmetic(iterations: usize) -> BenchmarkResult {
    let d1 = Dodecet::from_hex(0xABC);
    let d2 = Dodecet::from_hex(0x123);
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = d1 + d2;
        let _ = d1 - d2;
        let _ = d1 * d2;
    }
    let elapsed = start.elapsed();
    BenchmarkResult::new("Dodecet Arithmetic".to_string(), iterations * 3, elapsed)
}

fn benchmark_point3d_creation(iterations: usize) -> BenchmarkResult {
    let start = Instant::now();
    for i in 0..iterations {
        let x = (i % 4096) as u16;
        let y = ((i * 2) % 4096) as u16;
        let z = ((i * 3) % 4096) as u16;
        let _ = Point3D::new(x, y, z);
    }
    let elapsed = start.elapsed();
    BenchmarkResult::new("Point3D Creation".to_string(), iterations, elapsed)
}

fn benchmark_distance_calculation(iterations: usize) -> BenchmarkResult {
    let p1 = Point3D::new(0x100, 0x200, 0x300);
    let p2 = Point3D::new(0x400, 0x500, 0x600);
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = p1.distance_to(&p2);
    }
    let elapsed = start.elapsed();
    BenchmarkResult::new("Distance Calculation".to_string(), iterations, elapsed)
}

fn benchmark_vector_operations(iterations: usize) -> BenchmarkResult {
    let v1 = Vector3D::new(100, 200, 300);
    let v2 = Vector3D::new(400, 500, 600);
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = v1.dot(&v2);
        let _ = v1.cross(&v2);
        let _ = v1.magnitude();
    }
    let elapsed = start.elapsed();
    BenchmarkResult::new("Vector Operations".to_string(), iterations * 3, elapsed)
}

fn benchmark_hex_encoding(iterations: usize) -> BenchmarkResult {
    let points: Vec<Point3D> = (0..iterations)
        .map(|i| Point3D::new(
            (i % 4096) as u16,
            ((i * 2) % 4096) as u16,
            ((i * 3) % 4096) as u16
        ))
        .collect();

    let start = Instant::now();
    for p in &points {
        let _ = p.to_hex_string();
    }
    let elapsed = start.elapsed();
    BenchmarkResult::new("Hex Encoding".to_string(), iterations, elapsed)
}

fn benchmark_batch_operations(iterations: usize) -> BenchmarkResult {
    let arr = DodecetArray::<100>::from_slice(&[0x123; 100]);
    let start = Instant::now();
    for _ in 0..iterations {
        let arr_clone = arr.clone();
        let _ = arr_clone.sum();
        let _ = arr.average();
        let _ = arr.iter().min();
        let _ = arr.iter().max();
    }
    let elapsed = start.elapsed();
    BenchmarkResult::new("Batch Operations".to_string(), iterations * 4, elapsed)
}

fn benchmark_serialization(iterations: usize) -> BenchmarkResult {
    let points: Vec<Point3D> = (0..iterations)
        .map(|i| Point3D::new(
            (i % 4096) as u16,
            ((i * 2) % 4096) as u16,
            ((i * 3) % 4096) as u16
        ))
        .collect();

    let start = Instant::now();
    for p in &points {
        let x = Dodecet::new(p.x()).unwrap();
        let y = Dodecet::new(p.y()).unwrap();
        let z = Dodecet::new(p.z()).unwrap();
        let _ = [x, y, z];
    }
    let elapsed = start.elapsed();
    BenchmarkResult::new("Serialization".to_string(), iterations, elapsed)
}

fn compare_dodecet_vs_u8(iterations: usize) -> (MemoryComparison, MemoryComparison) {
    let dodecet_size = std::mem::size_of::<Dodecet>();
    let u8_size = std::mem::size_of::<u8>();

    let dodecet_bytes = iterations * dodecet_size;
    let u8_bytes = iterations * u8_size;

    (
        MemoryComparison {
            name: "Dodecet (12-bit)".to_string(),
            bytes_per_item: dodecet_size,
            total_bytes: dodecet_bytes,
            items: iterations,
        },
        MemoryComparison {
            name: "u8 (8-bit)".to_string(),
            bytes_per_item: u8_size,
            total_bytes: u8_bytes,
            items: iterations,
        }
    )
}

fn compare_point3d_vs_f64(iterations: usize) -> (MemoryComparison, MemoryComparison) {
    let point3d_size = std::mem::size_of::<Point3D>();
    let f64_triplet_size = std::mem::size_of::<f64>() * 3;

    let point3d_bytes = iterations * point3d_size;
    let f64_bytes = iterations * f64_triplet_size;

    (
        MemoryComparison {
            name: "Point3D (Dodecet)".to_string(),
            bytes_per_item: point3d_size,
            total_bytes: point3d_bytes,
            items: iterations,
        },
        MemoryComparison {
            name: "f64 triplet".to_string(),
            bytes_per_item: f64_triplet_size,
            total_bytes: f64_bytes,
            items: iterations,
        }
    )
}

fn main() {
    println!("=== Performance Comparison: Dodecet vs Traditional Encoding ===\n");

    // 1. Microbenchmarks
    println!("1. Microbenchmarks (1,000,000 iterations):");
    println!("   {:<30} | {:>10}      | {:>14} | {:>14}", "Operation", "Iterations", "Time/Operation", "Operations/sec");
    println!("   {:<30} | {:>10}      | {:>14} | {:>14}", "-----------", "-----------", "---------------", "--------------");

    let iterations = 1_000_000;

    let results = vec![
        benchmark_dodecet_creation(iterations),
        benchmark_dodecet_arithmetic(iterations),
        benchmark_point3d_creation(iterations),
        benchmark_distance_calculation(iterations),
        benchmark_vector_operations(iterations),
        benchmark_hex_encoding(iterations),
        benchmark_batch_operations(iterations / 100), // Fewer iterations for batch ops
        benchmark_serialization(iterations),
    ];

    for result in &results {
        result.print();
    }

    // 2. Memory comparison
    println!("\n2. Memory Comparison (10,000 items):");
    println!("   {:<30} | {:>16} | {:>16} | {:>12}", "Type", "Bytes per Item", "Total Bytes", "Items");
    println!("   {:<30} | {:>16} | {:>16} | {:>12}", "----", "---------------", "-----------", "-----");

    let items = 10_000;

    let (dodecet_mem, u8_mem) = compare_dodecet_vs_u8(items);
    dodecet_mem.print();
    u8_mem.print();

    let (point3d_mem, f64_mem) = compare_point3d_vs_f64(items);
    point3d_mem.print();
    f64_mem.print();

    // 3. Memory savings calculation
    println!("\n3. Memory Savings Analysis:");

    let dodecet_point_size = std::mem::size_of::<Point3D>();
    let f64_point_size = std::mem::size_of::<f64>() * 3;
    let point_savings = 100.0 * (1.0 - (dodecet_point_size as f64 / f64_point_size as f64));

    println!("   Point3D vs f64 triplet:");
    println!("     Dodecet Point3D: {} bytes", dodecet_point_size);
    println!("     f64 triplet: {} bytes", f64_point_size);
    println!("     Savings: {:.1}%", point_savings);

    // 4. Precision comparison
    println!("\n4. Precision Comparison:");
    println!("   8-bit (u8):      256 states        (8 states per bit)");
    println!("   12-bit (Dodecet): 4096 states       (341 states per bit)");
    println!("   16-bit (u16):    65536 states      (4096 states per bit)");
    println!("   32-bit (f32):    ~10^7 values      (floating point)");
    println!("   64-bit (f64):    ~10^16 values     (floating point)");

    let efficiency_8bit = 256.0 / 8.0;
    let efficiency_12bit = 4096.0 / 12.0;
    let efficiency_gain = efficiency_12bit / efficiency_8bit;

    println!("\n   Bit Efficiency:");
    println!("     8-bit: {:.2} states/bit", efficiency_8bit);
    println!("     12-bit: {:.2} states/bit", efficiency_12bit);
    println!("     Gain: {:.2}x more efficient", efficiency_gain);

    // 5. Cache efficiency
    println!("\n5. Cache Efficiency Analysis:");

    let cache_line = 64; // Typical cache line size
    let dodecets_per_line = cache_line / std::mem::size_of::<Dodecet>();
    let points_per_line = cache_line / std::mem::size_of::<Point3D>();
    let f64_points_per_line = cache_line / (std::mem::size_of::<f64>() * 3);

    println!("   Cache line size: {} bytes", cache_line);
    println!("   Dodecets per cache line: {}", dodecets_per_line);
    println!("   Point3Ds per cache line: {}", points_per_line);
    println!("   f64 triplets per cache line: {}", f64_points_per_line);

    // 6. SIMD potential
    println!("\n6. SIMD Optimization Potential:");
    println!("   Dodecet arrays enable efficient SIMD operations:");
    println!("   - Aligned 16-bit values for vectorization");
    println!("   - Natural alignment for 128/256-bit SIMD registers");
    println!("   - Batch operations process multiple values simultaneously");

    // 7. Real-world use case performance
    println!("\n7. Real-World Use Case: Point Cloud Processing (10,000 points)");

    let points: Vec<Point3D> = (0..10_000)
        .map(|i| Point3D::new(
            (i % 4096) as u16,
            ((i * 2) % 4096) as u16,
            ((i * 3) % 4096) as u16
        ))
        .collect();

    // Serialization benchmark
    let start = Instant::now();
    let serialized: Vec<String> = points.iter()
        .map(|p| p.to_hex_string())
        .collect();
    let serial_time = start.elapsed();

    // Deserialization benchmark
    let start = Instant::now();
    let _deserialized: Vec<Point3D> = serialized.iter()
        .filter_map(|s| Point3D::from_hex_str(s).ok())
        .collect();
    let deserial_time = start.elapsed();

    println!("   Serialization:");
    println!("     Time: {:?}", serial_time);
    println!("     Rate: {:.2} points/ms", 10_000.0 / serial_time.as_millis() as f64);

    println!("   Deserialization:");
    println!("     Time: {:?}", deserial_time);
    println!("     Rate: {:.2} points/ms", 10_000.0 / deserial_time.as_millis() as f64);

    // 8. Comparison summary
    println!("\n=== Performance Summary ===");

    let avg_creation_ns = results.iter()
        .find(|r| r.name.contains("Creation"))
        .map(|r| r.avg_time_ns)
        .unwrap_or(0.0);

    let avg_arithmetic_ns = results.iter()
        .find(|r| r.name.contains("Arithmetic"))
        .map(|r| r.avg_time_ns)
        .unwrap_or(0.0);

    println!("✓ Dodecet Creation: {:.2} ns/op", avg_creation_ns);
    println!("✓ Dodecet Arithmetic: {:.2} ns/op", avg_arithmetic_ns);
    println!("✓ Distance Calculation: {:.2} ns/op",
        results.iter().find(|r| r.name.contains("Distance")).map(|r| r.avg_time_ns).unwrap_or(0.0));
    println!("✓ Memory Efficiency: {:.1}% savings vs f64", point_savings);
    println!("✓ Bit Efficiency: {:.2}x better than 8-bit", efficiency_gain);

    // 9. Recommendations
    println!("\n=== Recommendations ===");
    println!("✓ Use Dodecet encoding for:");
    println!("  - 3D geometry requiring sub-millimeter precision");
    println!("  - Memory-constrained applications");
    println!("  - Deterministic calculations (no floating drift)");
    println!("  - Hex-friendly debugging and inspection");
    println!("  - Cache-efficient data structures");
    println!("  - SIMD-optimizable batch operations");

    println!("\n✓ Consider traditional encoding for:");
    println!("  - Very high precision requirements (>12 bits)");
    println!("  - Scientific computing with wide dynamic range");
    println!("  - Legacy code compatibility");
}
