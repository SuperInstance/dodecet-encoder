// crosscheck.rs — Rust cross-check for C accuracy verification
// Run as: cargo run --example crosscheck -- <input.csv> <output.csv>

use std::env;
use std::fs;
use std::io::{BufRead, Write, BufWriter};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: crosscheck <input.csv> <output.csv>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let input = fs::File::open(input_path).expect("Cannot open input file");
    let output = fs::File::create(output_path).expect("Cannot create output file");
    let mut writer = BufWriter::new(output);

    let sqrt3: f64 = 1.7320508075688772;
    let omega_re: f64 = -0.5;
    let omega_im: f64 = sqrt3 / 2.0;

    let weyl_perms: [(usize, usize, usize); 6] = [
        (0, 1, 2), (0, 2, 1), (1, 0, 2), (1, 2, 0), (2, 0, 1), (2, 1, 0),
    ];

    writeln!(writer, "x,y,snap_a,snap_b,error,chamber").unwrap();

    let reader = std::io::BufReader::new(input);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("x,") || line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 { continue; }
        let x: f64 = parts[0].parse().unwrap();
        let y: f64 = parts[1].parse().unwrap();

        let a_f = x - y * omega_re / omega_im;
        let b_f = y / omega_im;
        let a0 = a_f.round() as i32;
        let b0 = b_f.round() as i32;

        let mut best_a = a0;
        let mut best_b = b0;
        let mut best_err = f64::MAX;

        for da in -1..=1_i32 {
            for db in -1..=1_i32 {
                let ca = a0 + da;
                let cb = b0 + db;
                let cx = ca as f64 + cb as f64 * omega_re;
                let cy = cb as f64 * omega_im;
                let err = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
                if err < best_err {
                    best_a = ca;
                    best_b = cb;
                    best_err = err;
                }
            }
        }

        let b1 = x - y * omega_re / omega_im;
        let b2 = y / omega_im;
        let b3 = -(b1 + b2);
        let vals = [b1, b2, b3];
        let mut sorted = [0usize, 1, 2];
        sorted.sort_by(|&a, &b| vals[b].partial_cmp(&vals[a]).unwrap_or(std::cmp::Ordering::Equal));
        let perm = (sorted[0], sorted[1], sorted[2]);
        let chamber = weyl_perms.iter().position(|&p| p == perm).unwrap_or(0);

        writeln!(writer, "{:.15},{:.15},{},{},{:.10},{}",
                 x, y, best_a, best_b, best_err, chamber).unwrap();
    }
    eprintln!("Rust cross-check: wrote results to {}", output_path);
}
