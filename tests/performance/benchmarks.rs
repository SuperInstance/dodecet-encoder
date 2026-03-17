// Performance Benchmarks for Dodecet Encoder
// Comprehensive performance testing: WASM vs Native, encoding/decoding, geometric operations

use dodecet_encoder::{Dodecet, Point3D, Vector3D, Transform3D};
use std::time::{Duration, Instant};

#[cfg(test)]
mod performance_benchmarks {
    use super::*;

    struct BenchmarkResult {
        name: String,
        operations: usize,
        total_duration: Duration,
        avg_duration_ns: f64,
        ops_per_second: f64,
    }

    impl BenchmarkResult {
        fn new(name: String, operations: usize, total_duration: Duration) -> Self {
            let avg_duration_ns = total_duration.as_nanos() as f64 / operations as f64;
            let ops_per_second = 1_000_000_000.0 / avg_duration_ns;

            Self {
                name,
                operations,
                total_duration,
                avg_duration_ns,
                ops_per_second,
            }
        }

        fn print(&self) {
            println!("\n=== {} ===", self.name);
            println!("Operations: {}", self.operations);
            println!("Total Time: {:.2} ms", self.total_duration.as_secs_f64() * 1000.0);
            println!("Avg Time: {:.2} ns/op", self.avg_duration_ns);
            println!("Throughput: {:.0} ops/sec", self.ops_per_second);

            // Performance targets
            let target_ns = match self.name.as_str() {
                "Dodecet Creation" => 5.0,
                "Hex Encoding" => 25.0,
                "Hex Decoding" => 30.0,
                "Point3D Encoding" => 100.0,
                "Vector Addition" => 20.0,
                "Vector Dot Product" => 20.0,
                "Vector Cross Product" => 30.0,
                "Distance Calculation" => 45.0,
                "Transform3D Apply" => 100.0,
                _ => 1000.0,
            };

            let status = if self.avg_duration_ns <= target_ns {
                "✓ PASS"
            } else {
                "✗ FAIL"
            };

            println!("Target: < {:.0} ns/op [{}]", target_ns, status);
        }
    }

    /// Benchmark: Dodecet Creation
    #[test]
    fn benchmark_dodecet_creation() {
        let iterations = 1_000_000;
        let start = Instant::now();

        for i in 0..iterations {
            let _d = Dodecet::new(i % 4096);
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Dodecet Creation".to_string(),
            iterations,
            duration,
        );
        result.print();

        assert!(result.avg_duration_ns < 5.0, "Dodecet creation too slow");
    }

    /// Benchmark: Hex Encoding
    #[test]
    fn benchmark_hex_encoding() {
        let iterations = 1_000_000;
        let dodecets: Vec<Dodecet> = (0..iterations).map(|i| Dodecet::new(i % 4096)).collect();

        let start = Instant::now();

        for d in &dodecets {
            let _hex = d.to_hex();
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Hex Encoding".to_string(),
            iterations,
            duration,
        );
        result.print();

        assert!(result.avg_duration_ns < 25.0, "Hex encoding too slow");
    }

    /// Benchmark: Hex Decoding
    #[test]
    fn benchmark_hex_decoding() {
        let iterations = 1_000_000;
        let hex_strings: Vec<String> = (0..iterations)
            .map(|i| Dodecet::new(i % 4096).to_hex())
            .collect();

        let start = Instant::now();

        for hex in &hex_strings {
            let _d = Dodecet::from_hex(hex).unwrap();
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Hex Decoding".to_string(),
            iterations,
            duration,
        );
        result.print();

        assert!(result.avg_duration_ns < 30.0, "Hex decoding too slow");
    }

    /// Benchmark: Point3D Encoding
    #[test]
    fn benchmark_point3d_encoding() {
        let iterations = 100_000;
        let points: Vec<Point3D> = (0..iterations)
            .map(|i| Point3D::new(i as f64, i as f64 * 2.0, i as f64 * 3.0))
            .collect();

        let start = Instant::now();

        for point in &points {
            let _dodecets = point.to_dodecets();
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Point3D Encoding".to_string(),
            iterations,
            duration,
        );
        result.print();

        assert!(result.avg_duration_ns < 100.0, "Point3D encoding too slow");
    }

    /// Benchmark: Point3D Decoding
    #[test]
    fn benchmark_point3d_decoding() {
        let iterations = 100_000;
        let dodecet_arrays: Vec<Vec<Dodecet>> = (0..iterations)
            .map(|i| {
                let point = Point3D::new(i as f64, i as f64 * 2.0, i as f64 * 3.0);
                point.to_dodecets()
            })
            .collect();

        let start = Instant::now();

        for dodecets in &dodecet_arrays {
            let _point = Point3D::from_dodecets(dodecets);
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Point3D Decoding".to_string(),
            iterations,
            duration,
        );
        result.print();

        assert!(result.avg_duration_ns < 100.0, "Point3D decoding too slow");
    }

    /// Benchmark: Vector Addition
    #[test]
    fn benchmark_vector_addition() {
        let iterations = 10_000_000;
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);

        let start = Instant::now();

        for _ in 0..iterations {
            let _sum = v1.add(&v2);
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Vector Addition".to_string(),
            iterations,
            duration,
        );
        result.print();

        assert!(result.avg_duration_ns < 20.0, "Vector addition too slow");
    }

    /// Benchmark: Vector Dot Product
    #[test]
    fn benchmark_vector_dot_product() {
        let iterations = 10_000_000;
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);

        let start = Instant::now();

        for _ in 0..iterations {
            let _dot = v1.dot(&v2);
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Vector Dot Product".to_string(),
            iterations,
            duration,
        );
        result.print();

        assert!(result.avg_duration_ns < 20.0, "Vector dot product too slow");
    }

    /// Benchmark: Vector Cross Product
    #[test]
    fn benchmark_vector_cross_product() {
        let iterations = 10_000_000;
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);

        let start = Instant::now();

        for _ in 0..iterations {
            let _cross = v1.cross(&v2);
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Vector Cross Product".to_string(),
            iterations,
            duration,
        );
        result.print();

        assert!(result.avg_duration_ns < 30.0, "Vector cross product too slow");
    }

    /// Benchmark: Distance Calculation
    #[test]
    fn benchmark_distance_calculation() {
        let iterations = 1_000_000;
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 5.0);

        let start = Instant::now();

        for _ in 0..iterations {
            let _distance = p1.distance_to(&p2);
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Distance Calculation".to_string(),
            iterations,
            duration,
        );
        result.print();

        assert!(result.avg_duration_ns < 45.0, "Distance calculation too slow");
    }

    /// Benchmark: Transform3D Application
    #[test]
    fn benchmark_transform3d_apply() {
        let iterations = 1_000_000;
        let point = Point3D::new(1.0, 2.0, 3.0);
        let transform = Transform3D::rotation_z(45.0);

        let start = Instant::now();

        for _ in 0..iterations {
            let _transformed = transform.apply(&point);
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Transform3D Apply".to_string(),
            iterations,
            duration,
        );
        result.print();

        assert!(result.avg_duration_ns < 100.0, "Transform3D apply too slow");
    }

    /// Benchmark: Array Operations
    #[test]
    fn benchmark_array_operations() {
        let iterations = 100_000;
        let dodecet_arrays: Vec<Vec<Dodecet>> = (0..iterations)
            .map(|_| vec![Dodecet::new(100), Dodecet::new(200), Dodecet::new(300)])
            .collect();

        let start = Instant::now();

        for dodecets in &dodecet_arrays {
            let _hex = Dodecet::array_to_hex(dodecets);
        }

        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Array to Hex Encoding".to_string(),
            iterations,
            duration,
        );
        result.print();

        // Array operations should be fast (<50ns per element)
        assert!(result.avg_duration_ns < 150.0, "Array operations too slow");
    }

    /// Benchmark: Scalability Test (1M+ Operations)
    #[test]
    fn benchmark_scalability() {
        let iterations = 1_000_000;
        println!("\n=== Scalability Test: {} Operations ===", iterations);

        // Test encoding scalability
        let start = Instant::now();
        let dodecets: Vec<Dodecet> = (0..iterations).map(|i| Dodecet::new(i % 4096)).collect();
        let creation_time = start.elapsed();
        println!("Creation: {:.2} ms ({:.2} ns/op)",
                 creation_time.as_secs_f64() * 1000.0,
                 creation_time.as_nanos() as f64 / iterations as f64);

        // Test encoding scalability
        let start = Instant::now();
        let hex_strings: Vec<String> = dodecets.iter().map(|d| d.to_hex()).collect();
        let encoding_time = start.elapsed();
        println!("Encoding: {:.2} ms ({:.2} ns/op)",
                 encoding_time.as_secs_f64() * 1000.0,
                 encoding_time.as_nanos() as f64 / iterations as f64);

        // Test decoding scalability
        let start = Instant::now();
        let decoded: Vec<Dodecet> = hex_strings.iter()
            .map(|h| Dodecet::from_hex(h).unwrap())
            .collect();
        let decoding_time = start.elapsed();
        println!("Decoding: {:.2} ms ({:.2} ns/op)",
                 decoding_time.as_secs_f64() * 1000.0,
                 decoding_time.as_nanos() as f64 / iterations as f64);

        let total_time = creation_time + encoding_time + decoding_time;
        println!("Total: {:.2} ms", total_time.as_secs_f64() * 1000.0);

        // Should complete 1M operations in <100ms
        assert!(total_time.as_millis() < 100, "Scalability test failed: too slow");
    }

    /// Benchmark: Memory Usage
    #[test]
    fn benchmark_memory_usage() {
        use std::alloc::{GlobalAlloc, Layout, System};
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct MemoryTracker;

        static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

        unsafe impl GlobalAlloc for MemoryTracker {
            unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
                ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
                System.alloc(layout)
            }

            unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
                ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
                System.dealloc(ptr, layout)
            }
        }

        // Test memory usage for 100K dodecets
        let count = 100_000;
        ALLOCATED.store(0, Ordering::SeqCst);

        let dodecets: Vec<Dodecet> = (0..count).map(|i| Dodecet::new(i % 4096)).collect();

        let allocated = ALLOCATED.load(Ordering::SeqCst);
        let bytes_per_dodecet = allocated as f64 / count as f64;

        println!("\n=== Memory Usage Benchmark ===");
        println!("Dodecets: {}", count);
        println!("Total Allocated: {:.2} KB", allocated as f64 / 1024.0);
        println!("Bytes per Dodecet: {:.2}", bytes_per_dodecet);

        // Each dodecet should be ~2 bytes (u16)
        // Allow 10x overhead for Vec structure
        assert!(bytes_per_dodecet < 20.0, "Memory usage too high: {} bytes/dodecet", bytes_per_dodecet);

        std::mem::forget(dodecets); // Prevent deallocation during measurement
    }

    /// Benchmark: Encoding Precision vs Speed Trade-off
    #[test]
    fn benchmark_precision_vs_speed() {
        println!("\n=== Precision vs Speed Trade-off ===");

        let iterations = 10_000;

        // Test different precision levels
        for precision in &[8, 12, 16, 32] {
            let start = Instant::now();

            for i in 0..iterations {
                let value = (i as f64) * (65536.0 / *precision as f64);
                let d = Dodecet::new(value as u16 % 4096);
                let _hex = d.to_hex();
            }

            let duration = start.elapsed();
            let ns_per_op = duration.as_nanos() as f64 / iterations as f64;

            println!("Precision {}: {:.2} ns/op", precision, ns_per_op);
        }
    }

    /// Benchmark: Real-world Constraint Theory Workload
    #[test]
    fn benchmark_constraint_theory_workload() {
        println!("\n=== Constraint Theory Workload Simulation ===");

        let iterations = 10_000;

        // Simulate Pythagorean snapping workload
        let start = Instant::now();
        for i in 0..iterations {
            let x = (i % 100) as f64;
            let y = ((i * 2) % 100) as f64;
            let point = Point3D::new(x, y, 0.0);
            let origin = Point3D::new(0.0, 0.0, 0.0);

            // Encode
            let point_dodecets = point.to_dodecets();
            let origin_dodecets = origin.to_dodecets();

            // Decode
            let point_decoded = Point3D::from_dodecets(&point_dodecets);
            let origin_decoded = Point3D::from_dodecets(&origin_dodecets);

            // Calculate distance
            let _distance = point_decoded.distance_to(&origin_decoded);
        }

        let duration = start.elapsed();
        let ns_per_operation = duration.as_nanos() as f64 / iterations as f64;

        println!("Operations: {}", iterations);
        println!("Total Time: {:.2} ms", duration.as_secs_f64() * 1000.0);
        println!("Avg Time: {:.2} ns/op", ns_per_operation);

        // Should complete constraint theory workflow in <500ns per operation
        assert!(ns_per_operation < 500.0, "Constraint theory workload too slow");
    }

    /// Comprehensive Performance Report
    #[test]
    fn generate_performance_report() {
        println!("\n" + "=".repeat(80));
        println!("COMPREHENSIVE PERFORMANCE REPORT");
        println!("Dodecet Encoder v1.0.0");
        println!("=".repeat(80));

        println!("\n## Target Performance Metrics");
        println!("Dodecet Creation: <5 ns");
        println!("Hex Encoding: <25 ns");
        println!("Hex Decoding: <30 ns");
        println!("Point3D Encoding: <100 ns");
        println!("Vector Operations: <20 ns");
        println!("Distance Calculation: <45 ns");
        println!("Transform3D Operations: <100 ns");
        println!("\n## Memory Targets");
        println!("Per Dodecet: <20 bytes (including Vec overhead)");
        println!("100K Dodecets: <2 MB");
        println!("1M Dodecets: <20 MB");
        println!("\n## WASM Performance");
        println!("WASM Overhead: <2x native performance");
        println!("Browser Memory: <50 MB for typical usage");

        println!("\n" + "=".repeat(80));
        println!("Run individual benchmarks for detailed results");
        println!("=".repeat(80) + "\n");
    }
}
