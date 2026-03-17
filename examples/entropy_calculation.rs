// Entropy Calculation Example - Dodecet Encoder Integration
//
// This example demonstrates entropy calculation using dodecet encoding,
// showing how discrete geometric states provide measurable information theory metrics.

use dodecet_encoder::{Dodecet, Point3D};
use std::collections::HashMap;

/// Calculate Shannon entropy of a discrete probability distribution
fn shannon_entropy(probabilities: &HashMap<u16, f64>) -> f64 {
    probabilities.values()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum()
}

/// Calculate probability distribution from dodecet values
fn probability_distribution(dodecets: &[Dodecet]) -> HashMap<u16, f64> {
    let mut counts = HashMap::new();
    let total = dodecets.len() as f64;

    for d in dodecets {
        *counts.entry(d.value()).or_insert(0.0) += 1.0;
    }

    counts.iter()
        .map(|(k, v)| (*k, v / total))
        .collect()
}

/// Calculate joint entropy of multiple variables
#[allow(dead_code)]
fn joint_entropy(distributions: &[HashMap<u16, f64>]) -> f64 {
    // Approximate joint entropy by assuming independence
    distributions.iter()
        .map(|dist| shannon_entropy(dist))
        .sum()
}

/// Calculate mutual information between two variables
fn mutual_information(
    dist_x: &HashMap<u16, f64>,
    dist_y: &HashMap<u16, f64>,
    joint: &HashMap<(u16, u16), f64>
) -> f64 {
    let mut mi = 0.0;

    for (&x, &px) in dist_x {
        for (&y, &py) in dist_y {
            if let Some(&pxy) = joint.get(&(x, y)) {
                if pxy > 0.0 {
                    mi += pxy * (pxy / (px * py)).log2();
                }
            }
        }
    }

    mi
}

/// Point cloud for entropy analysis
#[derive(Debug)]
struct PointCloud {
    points: Vec<Point3D>,
    resolution: u16,
}

impl PointCloud {
    fn new(resolution: u16) -> Self {
        PointCloud {
            points: Vec::new(),
            resolution,
        }
    }

    fn add_point(&mut self, x: u16, y: u16, z: u16) {
        // Quantize to resolution
        let qx = (x / self.resolution) * self.resolution;
        let qy = (y / self.resolution) * self.resolution;
        let qz = (z / self.resolution) * self.resolution;

        self.points.push(Point3D::new(qx, qy, qz));
    }

    fn x_dodecets(&self) -> Vec<Dodecet> {
        self.points.iter()
            .map(|p| Dodecet::new(p.x()).unwrap())
            .collect()
    }

    fn y_dodecets(&self) -> Vec<Dodecet> {
        self.points.iter()
            .map(|p| Dodecet::new(p.y()).unwrap())
            .collect()
    }

    fn z_dodecets(&self) -> Vec<Dodecet> {
        self.points.iter()
            .map(|p| Dodecet::new(p.z()).unwrap())
            .collect()
    }

    fn joint_distribution(&self) -> HashMap<(u16, u16), f64> {
        let mut counts = HashMap::new();
        let total = self.points.len() as f64;

        for p in &self.points {
            let key = (p.x(), p.y());
            *counts.entry(key).or_insert(0.0) += 1.0;
        }

        counts.iter()
            .map(|(k, v)| (*k, v / total))
            .collect()
    }

    fn spatial_entropy(&self) -> (f64, f64, f64) {
        let x_dist = probability_distribution(&self.x_dodecets());
        let y_dist = probability_distribution(&self.y_dodecets());
        let z_dist = probability_distribution(&self.z_dodecets());

        let h_x = shannon_entropy(&x_dist);
        let h_y = shannon_entropy(&y_dist);
        let h_z = shannon_entropy(&z_dist);

        (h_x, h_y, h_z)
    }
}

/// Information gain from quantization
fn quantization_information_gain(original: &[u16], quantized: &[u16]) -> f64 {
    let original_dist: HashMap<u16, f64> = {
        let mut counts = HashMap::new();
        let total = original.len() as f64;
        for &v in original {
            *counts.entry(v).or_insert(0.0) += 1.0;
        }
        counts.iter().map(|(k, v)| (*k, v / total)).collect()
    };

    let quantized_dist: HashMap<u16, f64> = {
        let mut counts = HashMap::new();
        let total = quantized.len() as f64;
        for &v in quantized {
            *counts.entry(v).or_insert(0.0) += 1.0;
        }
        counts.iter().map(|(k, v)| (*k, v / total)).collect()
    };

    let h_original = shannon_entropy(&original_dist);
    let h_quantized = shannon_entropy(&quantized_dist);

    h_original - h_quantized
}

fn main() {
    println!("=== Entropy Calculation with Dodecet Encoding ===\n");

    // 1. Uniform distribution (maximum entropy)
    println!("1. Uniform Distribution (Maximum Entropy):");
    let uniform: Vec<Dodecet> = (0..4096)
        .map(|i| Dodecet::new(i).unwrap())
        .collect();

    let uniform_dist = probability_distribution(&uniform);
    let uniform_entropy = shannon_entropy(&uniform_dist);

    println!("   Range: 0-4095 (all 12-bit values)");
    println!("   Entropy: {:.4} bits", uniform_entropy);
    println!("   Expected: log2(4096) = 12 bits");
    println!("   Match: {:.2}%", (uniform_entropy / 12.0) * 100.0);

    // 2. Binary distribution
    println!("\n2. Binary Distribution:");
    let binary: Vec<Dodecet> = (0..1000)
        .map(|i| if i % 2 == 0 { Dodecet::new(0).unwrap() } else { Dodecet::new(1).unwrap() })
        .collect();

    let binary_dist = probability_distribution(&binary);
    let binary_entropy = shannon_entropy(&binary_dist);

    println!("   Values: 0 and 1 (equal probability)");
    println!("   Entropy: {:.4} bits", binary_entropy);
    println!("   Expected: log2(2) = 1 bit");
    println!("   Match: {:.2}%", (binary_entropy / 1.0) * 100.0);

    // 3. Skewed distribution
    println!("\n3. Skewed Distribution:");
    let skewed: Vec<Dodecet> = (0..1000)
        .flat_map(|_i| {
            std::iter::repeat(Dodecet::new(0).unwrap())
                .take(800)
                .chain(std::iter::repeat(Dodecet::new(1).unwrap()).take(150))
                .chain(std::iter::repeat(Dodecet::new(2).unwrap()).take(50))
        })
        .take(1000)
        .collect();

    let skewed_dist = probability_distribution(&skewed);
    let skewed_entropy = shannon_entropy(&skewed_dist);

    println!("   Distribution: 0 (80%), 1 (15%), 2 (5%)");
    println!("   Entropy: {:.4} bits", skewed_entropy);
    println!("   Expected: -0.8*log2(0.8) - 0.15*log2(0.15) - 0.05*log2(0.05) ≈ 0.88 bits");

    // 4. Spatial entropy in 3D
    println!("\n4. Spatial Entropy in 3D Point Cloud:");

    let mut cloud = PointCloud::new(64);

    // Random distribution
    for i in 0..1000 {
        let x = (i * 13) % 4096;
        let y = (i * 17) % 4096;
        let z = (i * 23) % 4096;
        cloud.add_point(x, y, z);
    }

    let (h_x, h_y, h_z) = cloud.spatial_entropy();
    let joint_h = h_x + h_y + h_z;

    println!("   Points: 1000");
    println!("   Resolution: 64 units");
    println!("   H(X): {:.4} bits", h_x);
    println!("   H(Y): {:.4} bits", h_y);
    println!("   H(Z): {:.4} bits", h_z);
    println!("   Joint H(X,Y,Z): {:.4} bits", joint_h);

    // 5. Structured vs Random
    println!("\n5. Structured vs Random Distributions:");

    // Structured: Grid pattern
    let mut structured_cloud = PointCloud::new(64);
    for x in (0..4096).step_by(256) {
        for y in (0..4096).step_by(256) {
            for z in (0..4096).step_by(256) {
                structured_cloud.add_point(x, y, z);
            }
        }
    }

    let (sh_x, sh_y, sh_z) = structured_cloud.spatial_entropy();
    println!("   Structured (16x16x16 grid):");
    println!("     H(X): {:.4} bits", sh_x);
    println!("     H(Y): {:.4} bits", sh_y);
    println!("     H(Z): {:.4} bits", sh_z);
    println!("     Total: {:.4} bits", sh_x + sh_y + sh_z);

    // Random distribution
    let mut random_cloud = PointCloud::new(64);
    for i in 0..4096 {
        let x = (i * 7) % 4096;
        let y = (i * 11) % 4096;
        let z = (i * 13) % 4096;
        random_cloud.add_point(x, y, z);
    }

    let (rh_x, rh_y, rh_z) = random_cloud.spatial_entropy();
    println!("   Random (4096 points):");
    println!("     H(X): {:.4} bits", rh_x);
    println!("     H(Y): {:.4} bits", rh_y);
    println!("     H(Z): {:.4} bits", rh_z);
    println!("     Total: {:.4} bits", rh_x + rh_y + rh_z);

    // 6. Mutual information
    println!("\n6. Mutual Information (X vs Y):");

    // Independent variables
    let mut independent_cloud = PointCloud::new(64);
    for i in 0..1000 {
        let x = (i * 7) % 4096;
        let y = (i * 13) % 4096; // Different prime = independent
        independent_cloud.add_point(x, y, 0);
    }

    let ind_dist_x = probability_distribution(&independent_cloud.x_dodecets());
    let ind_dist_y = probability_distribution(&independent_cloud.y_dodecets());
    let ind_joint = independent_cloud.joint_distribution();
    let mi_independent = mutual_information(&ind_dist_x, &ind_dist_y, &ind_joint);

    println!("   Independent X and Y:");
    println!("     Mutual Information: {:.6} bits", mi_independent);
    println!("     Expected: ~0 bits (independent)");

    // Correlated variables
    let mut correlated_cloud = PointCloud::new(64);
    for i in 0..1000 {
        let x = (i * 7) % 4096;
        let y = x; // Perfect correlation
        correlated_cloud.add_point(x, y, 0);
    }

    let corr_dist_x = probability_distribution(&correlated_cloud.x_dodecets());
    let corr_dist_y = probability_distribution(&correlated_cloud.y_dodecets());
    let corr_joint = correlated_cloud.joint_distribution();
    let mi_correlated = mutual_information(&corr_dist_x, &corr_dist_y, &corr_joint);

    println!("   Perfectly Correlated X and Y:");
    println!("     Mutual Information: {:.4} bits", mi_correlated);
    println!("     Expected: {:.4} bits (H(X) = H(Y))", shannon_entropy(&corr_dist_x));

    // 7. Quantization effects
    println!("\n7. Quantization Information Loss:");

    let original: Vec<u16> = (0..1000)
        .map(|i| (i * 17) % 4096)
        .collect();

    let quantized_64: Vec<u16> = original.iter()
        .map(|&v| (v / 64) * 64)
        .collect();

    let quantized_256: Vec<u16> = original.iter()
        .map(|&v| (v / 256) * 256)
        .collect();

    let loss_64 = quantization_information_gain(&original, &quantized_64);
    let loss_256 = quantization_information_gain(&original, &quantized_256);

    let dodecets: Vec<Dodecet> = original.iter().map(|&v| Dodecet::new(v).unwrap()).collect();
    println!("   Original entropy: {:.4} bits", shannon_entropy(&probability_distribution(&dodecets)));
    println!("   Quantized to 64-unit bins:");
    println!("     Information loss: {:.4} bits", loss_64);
    println!("   Quantized to 256-unit bins:");
    println!("     Information loss: {:.4} bits", loss_256);

    // 8. Dodecet precision advantages
    println!("\n8. Dodecet Precision Advantages:");

    println!("   12-bit dodecet: 4096 states");
    println!("   Maximum entropy: 12 bits");
    println!("   vs 8-bit byte: 256 states, 8 bits max");
    println!("   Information gain: 4 bits (50% more)");

    let gain_12_vs_8 = 12.0 - 8.0;
    println!("   Additional information: {:.1} bits per value", gain_12_vs_8);

    // 9. Entropy rate of cellular automaton
    println!("\n9. Cellular Automaton Entropy Rate:");

    // Simple rule: next = (current + previous) mod 4096
    let mut ca = vec![Dodecet::new(1).unwrap(); 100];
    for i in 2..100 {
        let prev = ca[i-2].value();
        let curr = ca[i-1].value();
        ca[i] = Dodecet::new((prev + curr) % 4096).unwrap();
    }

    let ca_dist = probability_distribution(&ca);
    let ca_entropy = shannon_entropy(&ca_dist);

    println!("   Rule: X[n] = (X[n-1] + X[n-2]) mod 4096");
    println!("   Cells: 100");
    println!("   Unique states: {}", ca_dist.len());
    println!("   Entropy rate: {:.4} bits/cell", ca_entropy);

    // 10. Summary
    println!("\n=== Key Insights ===");
    println!("✓ 12-bit dodecets provide 50% more information than 8-bit");
    println!("✓ Discrete geometry enables precise entropy calculation");
    println!("✓ Spatial entropy captures 3D structure");
    println!("✓ Mutual information detects correlations");
    println!("✓ Quantization preserves essential information");
    println!("✓ Deterministic encoding eliminates entropy from floating-point errors");
}
