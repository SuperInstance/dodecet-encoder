//! Hex editor example
//!
//! Demonstrates how dodecet encoding appears in a hex editor
//!
//! Run with: cargo run --example hex_editor

use dodecet_encoder::{Dodecet, DodecetString, hex};

fn main() {
    println!("=== Dodecet Hex Editor Example ===\n");

    println!("In a hex editor, dodecets appear as 3-digit hex values:\n");

    // Create some example data
    let data = vec![
        Dodecet::from_hex(0x123),
        Dodecet::from_hex(0x456),
        Dodecet::from_hex(0x789),
        Dodecet::from_hex(0xABC),
        Dodecet::from_hex(0xDEF),
    ];

    let hex_string = hex::encode(&data);
    println!("Hex representation:");
    println!("{}\n", hex_string);

    // Show how it would look in a hex editor
    println!("Hex Editor View:");
    println!("Offset  +0 +1 +2 +3 +4 +5 +6 +7  +8 +9 +A +B +C +D +E +F");
    println!("--------+--------------------------------+---------------");

    let bytes: Vec<u8> = hex_string
        .as_bytes()
        .chunks(3)
        .flat_map(|chunk| {
            let val = u16::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap();
            vec![(val >> 4) as u8, ((val & 0x0F) << 4 | (val >> 8)) as u8, (val & 0xFF) as u8]
        })
        .collect();

    for (i, chunk) in bytes.chunks(16).enumerate() {
        let offset = i * 16;
        print!("{:08X}+", offset);

        // Show hex bytes
        for (j, &byte) in chunk.iter().enumerate() {
            if j == 8 {
                print!(" ");
            }
            print!("{:02X} ", byte);
        }

        // Pad if not full row
        for j in chunk.len()..16 {
            if j == 8 {
                print!(" ");
            }
            print!("   ");
        }

        print!(" ");

        // Show ASCII representation
        for &byte in chunk.iter() {
            if byte >= 0x20 && byte <= 0x7E {
                print!("{}", byte as char);
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!();

    // Show the dodecet view
    println!("Dodecet View (3 digits per dodecet):");
    println!("Offset  +0   +1   +2   +3   +4   +5   +6   +7");
    println!("--------+-----+-----+-----+-----+-----+-----+-----+----");

    for (i, chunk) in data.chunks(8).enumerate() {
        let offset = i * 8;
        print!("{:08X}+", offset);

        for d in chunk {
            print!("{:03X} ", d.value());
        }

        println!();
    }

    println!();

    // Show hex view with ASCII
    println!("Hex View with ASCII:");
    println!("{}", hex::hex_view(&hex_string));

    // Demonstrate packing efficiency
    println!("Storage Efficiency:");
    println!("  Traditional 8-bit: {} bytes for 5 values", 5);
    println!("  12-bit dodecet: {} bytes for 5 values (packed)", (5 * 12 + 7) / 8);
    println!("  Space saved: {:.1}%", (1.0 - (5 * 12) as f64 / (5.0 * 16.0)) * 100.0);

    println!("\n=== Hex Editor Example Complete ===");
}
