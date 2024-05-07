
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // Get the file name from user input
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Read the file into a byte buffer
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Check if the file has been encrypted before
    let is_encrypted = buffer.iter().any(|&byte| byte == 0);

    // Encrypt or decrypt the byte buffer
    if is_encrypted {
        // Decrypt
        for byte in buffer.iter_mut() {
            if *byte == 0 {
                continue;
            }
            *byte = byte.wrapping_sub(1);
        }
        println!("File decrypted successfully.");
    } else {
        // Encrypt
        for byte in buffer.iter_mut() {
            *byte = byte.wrapping_add(1);
        }
        buffer.push(0);
        println!("File encrypted successfully.");
    }

    // Write the byte buffer back to the file
    let mut file = File::create(filename)?;
    file.write_all(&buffer)?;

    println!("Operation complete.");
    Ok(())
}
