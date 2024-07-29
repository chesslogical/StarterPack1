use std::env;
use std::fs::{File};
use std::io::{Read, Write, BufReader, BufWriter};
use std::process::exit;
use std::fs;

const KEY_FILE: &str = "key.key";
const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        exit(1);
    }

    let filename = &args[1];

    let key = match read_key() {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Error reading key file: {}", e);
            exit(1);
        }
    };

    if key.is_empty() {
        eprintln!("Error: The key file has zero bytes.");
        exit(1);
    }

    if let Err(e) = process_file(filename, &key) {
        eprintln!("Error processing file: {}", e);
        exit(1);
    }
}

fn read_key() -> Result<Vec<u8>, std::io::Error> {
    let mut key_file = File::open(KEY_FILE)?;
    let mut key = Vec::new();
    key_file.read_to_end(&mut key)?;
    Ok(key)
}

fn process_file(filename: &str, key: &[u8]) -> Result<(), std::io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let temp_filename = format!("{}.tmp", filename);
    let temp_file = File::create(&temp_filename)?;
    let mut writer = BufWriter::new(temp_file);

    let mut buffer = vec![0; CHUNK_SIZE];
    let mut key_pos = 0;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        for i in 0..bytes_read {
            buffer[i] ^= key[key_pos % key.len()];
            key_pos += 1;
        }

        writer.write_all(&buffer[..bytes_read])?;
    }

    writer.flush()?;
    
    fs::rename(&temp_filename, filename)?;

    Ok(())
}


