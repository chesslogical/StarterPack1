use std::env;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};

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
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            eprintln!("File not found");
            return;
        }
        Err(_) => {
            eprintln!("Failed to open file");
            return;
        }
    };

    let mut key_file = match File::open(key_file_name) {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            eprintln!("Key file not found");
            return;
        }
        Err(_) => {
            eprintln!("Failed to open key file");
            return;
        }
    };

    let mut buffer = Vec::with_capacity(1024 * 1024); // read file in chunks of 1MB
    let mut key_buffer = Vec::with_capacity(1024 * 1024);

    loop {
        buffer.clear();
        key_buffer.clear();

        let bytes_read = file.read_to_end(&mut buffer).expect("Failed to read file");
        let key_bytes_read = key_file.read_to_end(&mut key_buffer).expect("Failed to read key file");

        if bytes_read == 0 || key_bytes_read == 0 {
            break;
        }

        let mut encrypted_buffer = Vec::with_capacity(bytes_read);
        for (i, byte) in buffer.iter().enumerate() {
            let key_byte = key_buffer.get(i % key_bytes_read).unwrap_or(&0);
            encrypted_buffer.push(byte ^ key_byte);
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_name)
            .expect("Failed to open file for writing");

        file.write_all(&encrypted_buffer).expect("Failed to write to file");
    }
}

