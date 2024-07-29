use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 1024 * 1024; // 1 MB buffer size, adjust based on requirements

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

fn process_file(file_name: &str, key_file_name: &str) -> std::io::Result<()> {
    let mut file = File::open(file_name)?;
    let mut key_file = File::open(key_file_name)?;

    let mut key_buffer = Vec::new();
    key_file.read_to_end(&mut key_buffer)?;

    if key_buffer.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Key file is empty"));
    }

    let output_file_name = if file_name.ends_with(".enc") {
        file_name.strip_suffix(".enc").unwrap().to_string()
    } else {
        format!("{}.enc", file_name)
    };

    // Check if the output file already exists
    if OpenOptions::new().read(true).open(&output_file_name).is_ok() {
        return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, format!("Output file '{}' already exists. Operation aborted.", output_file_name)));
    }

    let mut output_file = File::create(&output_file_name)?;
    let mut buffer = vec![0; BUFFER_SIZE];
    let mut key_index = 0;

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file reached
        }

        for i in 0..bytes_read {
            buffer[i] ^= key_buffer[key_index];
            key_index = (key_index + 1) % key_buffer.len();
        }

        output_file.write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}

