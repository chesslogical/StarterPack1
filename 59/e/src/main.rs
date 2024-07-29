use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

const BUFFER_SIZE: usize = 1024 * 1024; // 1 MB buffer size

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: <program> <file> <key_file>");
        return;
    }

    let file_name = &args[1];
    let key_file_name = &args[2];

    if let Err(e) = process_file(file_name, key_file_name) {
        eprintln!("Error: {}", e);
    } else {
        println!("Operation complete.");
    }
}

fn process_file(file_name: &str, key_file_name: &str) -> io::Result<()> {
    let mut file = File::open(file_name)?;
    let mut key_file = File::open(key_file_name)?;

    let output_file_name = if file_name.ends_with(".enc") {
        file_name.strip_suffix(".enc").unwrap().to_string()
    } else {
        format!("{}.enc", file_name)
    };

    if OpenOptions::new().read(true).open(&output_file_name).is_ok() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("Output file '{}' already exists. Operation aborted.", output_file_name)));
    }

    let mut output_file = File::create(&output_file_name)?;
    let mut data_buffer = vec![0; BUFFER_SIZE];
    let mut key_buffer = vec![0; BUFFER_SIZE];
    let mut key_index = 0;
    let mut key_bytes_available = 0;

    loop {
        // Read a chunk of the key if needed
        if key_index == key_bytes_available {
            key_bytes_available = key_file.read(&mut key_buffer)?;
            if key_bytes_available == 0 {
                break; // End of key file reached
            }
            key_index = 0;
        }

        // Read a chunk of the file
        let bytes_read = file.read(&mut data_buffer)?;
        if bytes_read == 0 {
            break; // End of file reached
        }

        for i in 0..bytes_read {
            data_buffer[i] ^= key_buffer[key_index];
            key_index = (key_index + 1) % key_bytes_available;
        }

        output_file.write_all(&data_buffer[..bytes_read])?;
    }

    Ok(())
}

