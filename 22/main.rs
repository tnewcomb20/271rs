use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    // Get the filename from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: b64_rs <file>");
        std::process::exit(1);
    }

    let path = &args[1];
    let input = fs::read(path).expect("Failed to read file");

    // Base64 table (RFC 4648)
    let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut output = String::new();
    let mut i = 0;

    // Process input in 3-byte chunks
    while i < input.len() {
        let b0 = input[i];
        let b1 = if i + 1 < input.len() { input[i + 1] } else { 0 };
        let b2 = if i + 2 < input.len() { input[i + 2] } else { 0 };

        // Combine bytes into a 24-bit number
        let triple = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);

        // Split into 4 groups of 6 bits
        let c0 = ((triple >> 18) & 0x3F) as u8;
        let c1 = ((triple >> 12) & 0x3F) as u8;
        let c2 = ((triple >> 6) & 0x3F) as u8;
        let c3 = (triple & 0x3F) as u8;

        // Append Base64 characters or '=' padding
        if i + 2 < input.len() {
            output.push(table[c0 as usize] as char);
            output.push(table[c1 as usize] as char);
            output.push(table[c2 as usize] as char);
            output.push(table[c3 as usize] as char);
        } else if i + 1 < input.len() {
            output.push(table[c0 as usize] as char);
            output.push(table[c1 as usize] as char);
            output.push(table[c2 as usize] as char);
            output.push('=');
        } else {
            output.push(table[c0 as usize] as char);
            output.push(table[c1 as usize] as char);
            output.push('=');
            output.push('=');
        }

        i += 3;
    }

    // Line breaks every 76 characters
    for (i, c) in output.chars().enumerate() {
        if i > 0 && i % 76 == 0 {
            println!();
        }
        print!("{}", c);
    }
    io::stdout().flush().unwrap();
}