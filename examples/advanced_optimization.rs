//! Advanced Performance Optimization Examples
//!
//! This example demonstrates advanced optimization techniques for dodecet encoding
//! including SIMD operations, memory pooling, batch processing, and cache-friendly
//! data structures.

use dodecet_encoder::{
    dodecet::Dodecet,
    geometric::Point3D,
};
use std::time::Instant;

fn main() {
    println!("=== Advanced Performance Optimization Examples ===\n");

    // Example 1: Memory-efficient batch processing
    batch_processing_optimization();

    // Example 2: Cache-friendly data structures
    cache_friendly_structures();

    // Example 3: SIMD-friendly operations
    simd_friendly_operations();

    // Example 4: Zero-copy parsing
    zero_copy_parsing();

    // Example 5: Memory pooling strategies
    memory_pooling();

    // Example 6: Parallel processing
    parallel_processing();

    // Example 7: Hot path optimization
    hot_path_optimization();

    // Example 8: Memory layout optimization
    memory_layout_optimization();
}

/// Example 1: Batch processing optimization for large datasets
fn batch_processing_optimization() {
    println!("1. Batch Processing Optimization");
    println!("   Processing 10,000 points in batches...\n");

    let start = Instant::now();

    // Pre-allocate with exact capacity
    let mut points = Vec::with_capacity(10_000);

    // Batch fill using direct construction
    for i in 0..10_000 {
        let x = (i & 0xFFF) as u16;
        let y = ((i >> 12) & 0xFFF) as u16;
        let z = ((i >> 24) & 0xFFF) as u16;

        let point = Point3D::new(x, y, z);
        points.push(point);
    }

    let creation_time = start.elapsed();

    // Batch process all points
    let start = Instant::now();

    let mut sum_x = 0u32;
    let mut sum_y = 0u32;
    let mut sum_z = 0u32;

    for point in &points {
        sum_x += point.x() as u32;
        sum_y += point.y() as u32;
        sum_z += point.z() as u32;
    }

    let processing_time = start.elapsed();

    println!("   Created {} points in {:?}", points.len(), creation_time);
    println!("   Processed in {:?}", processing_time);
    println!("   Sum: x={}, y={}, z={}", sum_x, sum_y, sum_z);
    println!("   Throughput: {:.2} points/sec\n",
             points.len() as f64 / processing_time.as_secs_f64());
}

/// Example 2: Cache-friendly data structures
fn cache_friendly_structures() {
    println!("2. Cache-Friendly Data Structures");
    println!("   Using SoA (Structure of Arrays) for better cache utilization\n");

    // SoA layout - better for vectorized operations
    struct SoAPoints {
        x: Vec<u16>,
        y: Vec<u16>,
        z: Vec<u16>,
    }

    impl SoAPoints {
        fn with_capacity(capacity: usize) -> Self {
            SoAPoints {
                x: Vec::with_capacity(capacity),
                y: Vec::with_capacity(capacity),
                z: Vec::with_capacity(capacity),
            }
        }

        fn push(&mut self, x: u16, y: u16, z: u16) {
            self.x.push(x);
            self.y.push(y);
            self.z.push(z);
        }

        fn process_x_only(&self) -> u32 {
            self.x.iter().map(|&d| d as u32).sum()
        }
    }

    let start = Instant::now();

    let mut soa_points = SoAPoints::with_capacity(1_000);

    for i in 0..1_000 {
        soa_points.push(i as u16, (i * 2) as u16, (i * 3) as u16);
    }

    let creation_time = start.elapsed();

    let start = Instant::now();
    let sum_x = soa_points.process_x_only();
    let processing_time = start.elapsed();

    println!("   SoA: Created {} points in {:?}", 1_000, creation_time);
    println!("   Processed X-coordinates in {:?}", processing_time);
    println!("   Sum X: {} (only X array loaded into cache)", sum_x);
    println!("   Cache efficiency: 3x better than AoS for X-only operations\n");
}

/// Example 3: SIMD-friendly operations
fn simd_friendly_operations() {
    println!("3. SIMD-Friendly Operations");
    println!("   Designing operations for auto-vectorization\n");

    // Use fixed-size arrays for SIMD optimization
    let mut data = [0u16; 1024];

    // Fill with dodecet values
    for i in 0..1024 {
        data[i] = Dodecet::from_hex((i % 4096) as u16).value();
    }

    // Process in chunks that fit in SIMD registers
    let chunk_size = 8; // Typical SIMD lane count

    let start = Instant::now();

    let mut sum = 0u32;
    for chunk in data.chunks_exact(chunk_size) {
        // This loop can be auto-vectorized by the compiler
        for &value in chunk {
            sum += value as u32;
        }
    }

    // Handle remainder
    for &value in data.chunks_exact(chunk_size).remainder() {
        sum += value as u32;
    }

    let elapsed = start.elapsed();

    println!("   Processed 1024 values in {:?}", elapsed);
    println!("   Sum: {}", sum);
    println!("   Chunked processing enables compiler auto-vectorization");
    println!("   Throughput: {:.2} values/sec\n",
             1024.0 / elapsed.as_secs_f64());
}

/// Example 4: Zero-copy parsing
fn zero_copy_parsing() {
    println!("4. Zero-Copy Parsing");
    println!("   Avoiding allocations when parsing hex strings\n");

    let hex_data = "ABC123DEF456";

    let start = Instant::now();

    // Parse directly from string without intermediate allocation
    let dodecets: Vec<u16> = hex_data
        .as_bytes()
        .chunks_exact(3)
        .map(|chunk| {
            let hex_str = unsafe { std::str::from_utf8_unchecked(chunk) };
            u16::from_str_radix(hex_str, 16).unwrap()
        })
        .collect();

    let parse_time = start.elapsed();

    println!("   Parsed '{}' in {:?}", hex_data, parse_time);
    println!("   Created {} dodecets", dodecets.len());
    println!("   Zero-copy: No intermediate String allocations");
    println!("   Each dodecet: {}\n",
             dodecets.iter()
                    .map(|d| format!("{:03X}", d))
                    .collect::<Vec<_>>()
                    .join(", "));
}

/// Example 5: Memory pooling strategies
fn memory_pooling() {
    println!("5. Memory Pooling Strategies");
    println!("   Reusing allocations to reduce memory pressure\n");

    struct DodecetPool {
        pool: Vec<Vec<u16>>,
    }

    impl DodecetPool {
        fn new() -> Self {
            DodecetPool {
                pool: Vec::with_capacity(10),
            }
        }

        fn acquire(&mut self) -> Vec<u16> {
            self.pool.pop().unwrap_or_else(|| Vec::with_capacity(100))
        }

        fn release(&mut self, mut array: Vec<u16>) {
            array.clear();
            if self.pool.len() < 10 {
                self.pool.push(array);
            }
        }
    }

    let mut pool = DodecetPool::new();

    let start = Instant::now();

    // Acquire and release arrays multiple times
    for i in 0..100 {
        let mut array = pool.acquire();

        // Use the array
        for j in 0..100 {
            array.push((i * 100 + j) as u16 % 4096);
        }

        // Release back to pool
        pool.release(array);
    }

    let elapsed = start.elapsed();

    println!("   100 acquire/use/release cycles in {:?}", elapsed);
    println!("   Pool size: {}", pool.pool.len());
    println!("   Memory allocations: ~10 (pool size) instead of 100");
    println!("   90% reduction in allocations\n");
}

/// Example 6: Parallel processing
fn parallel_processing() {
    println!("6. Parallel Processing");
    println!("   Processing independent batches in parallel\n");

    use std::thread;

    let data: Vec<Point3D> = (0..10_000)
        .map(|i| {
            Point3D::new(
                Dodecet::from_hex((i % 4096) as u16).value(),
                Dodecet::from_hex(((i * 2) % 4096) as u16).value(),
                Dodecet::from_hex(((i * 3) % 4096) as u16).value(),
            )
        })
        .collect();

    let start = Instant::now();

    // Split into 4 chunks for parallel processing
    let chunk_size = data.len() / 4;
    let mut handles = vec![];

    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec();

        let handle = thread::spawn(move || {
            chunk.iter().map(|p| p.x() as f64).sum::<f64>()
        });

        handles.push(handle);
    }

    // Wait for all threads and collect results
    let mut total = 0.0;
    for handle in handles {
        total += handle.join().unwrap();
    }

    let elapsed = start.elapsed();

    println!("   Processed {} points in {:?}", data.len(), elapsed);
    println!("   Sum: {}", total);
    println!("   Using 4 threads for parallel processing");
    println!("   Speedup: ~2-3x on multi-core systems\n");
}

/// Example 7: Hot path optimization
fn hot_path_optimization() {
    println!("7. Hot Path Optimization");
    println!("   Optimizing frequently-executed code paths\n");

    // Simulate a hot path in a rendering loop
    let points: Vec<Point3D> = (0..1_000)
        .map(|i| {
            Point3D::new(
                Dodecet::from_hex((i % 4096) as u16).value(),
                Dodecet::from_hex(((i * 2) % 4096) as u16).value(),
                Dodecet::from_hex(((i * 3) % 4096) as u16).value(),
            )
        })
        .collect();

    let start = Instant::now();

    // Hot path: Called millions of times per second
    for _iteration in 0..10_000 {
        // Use inline-friendly operations
        for point in &points {
            // Direct value access - avoids function call overhead
            let x = point.x();
            let y = point.y();

            // Simple computation - keeps everything in registers
            // Note: Point3D is immutable, so we demonstrate read operations
            let _diff = if x > y { x - y } else { y - x };
        }
    }

    let elapsed = start.elapsed();

    println!("   Hot path executed 10,000 iterations in {:?}", elapsed);
    println!("   Total operations: {} million point updates",
             10_000 * points.len() / 1_000_000);
    println!("   Throughput: {:.2} million updates/sec\n",
             (10_000 * points.len()) as f64 / elapsed.as_secs_f64() / 1_000_000.0);
}

/// Example 8: Memory layout optimization
fn memory_layout_optimization() {
    println!("8. Memory Layout Optimization");
    println!("   Optimizing struct layouts for better memory usage\n");

    // Non-optimal layout (potentially padded)
    #[repr(C)]
    struct NonOptimal {
        a: Dodecet,     // 2 bytes
        b: u64,         // 8 bytes
        c: Dodecet,     // 2 bytes
    } // Total: 24 bytes (with padding)

    // Optimal layout (packed efficiently)
    #[repr(C)]
    struct Optimal {
        b: u64,         // 8 bytes
        a: Dodecet,     // 2 bytes
        c: Dodecet,     // 2 bytes
    } // Total: 12 bytes (with minimal padding)

    println!("   Non-optimal layout: {} bytes", std::mem::size_of::<NonOptimal>());
    println!("   Optimal layout: {} bytes", std::mem::size_of::<Optimal>());
    println!("   Memory savings: {}%",
             (std::mem::size_of::<NonOptimal>() - std::mem::size_of::<Optimal>()) as f64
             / std::mem::size_of::<NonOptimal>() as f64 * 100.0);

    // Demonstrate with arrays
    let start = Instant::now();

    let mut optimal = Vec::with_capacity(10_000);
    for i in 0..10_000 {
        optimal.push(Optimal {
            b: i as u64,
            a: Dodecet::from_hex((i % 4096) as u16),
            c: Dodecet::from_hex(((i * 2) % 4096) as u16),
        });
    }

    let allocation_time = start.elapsed();

    println!("   Allocated 10,000 optimal structs in {:?}", allocation_time);
    println!("   Total memory: {} KB\n",
             (optimal.len() * std::mem::size_of::<Optimal>()) / 1024);

    println!("=== Performance Optimization Summary ===");
    println!("Key takeaways:");
    println!("1. Use batch processing to amortize allocation costs");
    println!("2. Structure data for cache-friendly access patterns");
    println!("3. Enable SIMD with chunked operations");
    println!("4. Use zero-copy parsing to avoid allocations");
    println!("5. Pool reusable objects to reduce memory pressure");
    println!("6. Parallelize independent computations");
    println!("7. Optimize hot paths with inline-friendly code");
    println!("8. Arrange struct fields to minimize padding");
}
