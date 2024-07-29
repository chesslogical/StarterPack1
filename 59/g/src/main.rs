use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

// XOR-based encryption with a derived key from the password
fn xor_encrypt_decrypt(data: &mut [u8], key: &[u8]) {
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
}

// Simple function to derive a key from a password
fn derive_key(password: &str, key_length: usize) -> Vec<u8> {
    let mut key = Vec::with_capacity(key_length);
    for (i, &byte) in password.as_bytes().iter().cycle().take(key_length).enumerate() {
        key.push(byte.wrapping_add(i as u8));  // Simple key derivation
    }
    key
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: <program> <e|d> <file> <password>");
        return Ok(());
    }

    let mode = &args[1];  // 'e' for encrypt, 'd' for decrypt
    let file_name = &args[2];
    let password = &args[3];

    if !["e", "d"].contains(&mode.as_str()) {
        eprintln!("Invalid mode. Use 'e' to encrypt or 'd' to decrypt.");
        return Ok(());
    }

    let path = Path::new(file_name);
    if !path.exists() {
        eprintln!("File does not exist.");
        return Ok(());
    }

    // Define output file names
    let output_file_name = if mode == "e" {
        format!("{}.enc", file_name)
    } else {
        file_name.trim_end_matches(".enc").to_string()
    };

    // Check for existing output files to prevent overwriting
    if Path::new(&output_file_name).exists() {
        eprintln!("Output file '{}' already exists. Operation aborted to prevent overwriting.", output_file_name);
        return Ok(());
    }

    let mut file = File::open(file_name)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // Derive a key from the password
    let key_length = 256; // For simplicity, using a fixed key length
    let key = derive_key(password, key_length);

    // Encrypt or Decrypt
    xor_encrypt_decrypt(&mut data, &key);

    let mut output_file = OpenOptions::new().write(true).create(true).truncate(true).open(output_file_name)?;
    output_file.write_all(&data)?;

    Ok(())
}
