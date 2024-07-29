use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: <program> <file_to_encrypt> <key_file>");
        return;
    }

    let file_name = &args[1];
    let key_file_name = &args[2];

    encrypt_file(file_name, key_file_name);
    println!("Done.");
}

fn encrypt_file(file_name: &str, key_file_name: &str) {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(ref err) => {
            eprintln!("Failed to open file: {}", err);
            return;
        }
    };

    let mut key_file = match File::open(key_file_name) {
        Ok(file) => file,
        Err(ref err) => {
            eprintln!("Failed to open key file: {}", err);
            return;
        }
    };

    let mut buffer = Vec::new();
    let mut key_buffer = Vec::new();

    if file.read_to_end(&mut buffer).is_err() {
        eprintln!("Failed to read file");
        return;
    }
    if key_file.read_to_end(&mut key_buffer).is_err() {
        eprintln!("Failed to read key file");
        return;
    }

    if key_buffer.is_empty() {
        eprintln!("Key file is empty");
        return;
    }

    let encrypted_buffer: Vec<u8> = buffer.iter()
        .zip(key_buffer.iter().cycle())
        .map(|(&b, &k)| b ^ k)
        .collect();

    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open file for writing");
            return;
        }
    };

    if file.write_all(&encrypted_buffer).is_err() {
        eprintln!("Failed to write to file");
    }
}

