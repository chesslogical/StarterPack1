use clap::{Arg, Command};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use anyhow::{Context, Result};
use zeroize::Zeroize;

fn main() -> Result<()> {
    let matches = Command::new("Encryption/Decryption App")
        .version("0.1.0")
        .author("Your Name")
        .about("Encrypts or decrypts files using the One Time Pad method.")
        .arg(Arg::new("mode")
            .short('m')
            .long("mode")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Operation mode: 'E' for encryption, 'D' for decryption"))
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Path to the input file"))
        .arg(Arg::new("key")
            .short('k')
            .long("key")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Path to the key file"))
        .get_matches();

    let mode = matches.get_one::<String>("mode").expect("Mode required");
    let filename = matches.get_one::<String>("input").expect("Input file required");
    let key_filename = matches.get_one::<String>("key").expect("Key file required");

    process_file(mode, filename, key_filename, 1024) // Assuming a chunk size of 1024 bytes
}

fn process_file(mode: &str, filename: &str, key_filename: &str, chunk_size: usize) -> Result<()> {
    let input_file = File::open(filename)
        .with_context(|| format!("Failed to open file '{}'", filename))?;
    let mut reader = BufReader::new(input_file);

    let key_file = File::open(key_filename)
        .with_context(|| format!("Failed to open key file '{}'", key_filename))?;
    let mut key_reader = BufReader::new(key_file);

    let mut file_contents = Vec::new();
    let mut key_buffer = vec![0; chunk_size];
    let mut key_pos = 0;

    loop {
        let mut buffer = vec![0; chunk_size];
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        buffer.truncate(bytes_read);

        key_buffer.resize(bytes_read, 0);
        if key_reader.read_exact(&mut key_buffer).is_err() {
            return Err(anyhow::anyhow!("Key file is smaller than the input file or key read error occurred."));
        }

        for i in 0..bytes_read {
            buffer[i] ^= key_buffer[(key_pos + i) % key_buffer.len()];
        }

        file_contents.extend_from_slice(&buffer);

        key_pos = (key_pos + bytes_read) % key_buffer.len(); // Update key position for cycling
    }

    key_buffer.zeroize();

    // Now overwrite the original file
    let mut output_file = File::create(filename)
        .with_context(|| format!("Failed to create output file '{}'", filename))?;
    output_file.write_all(&file_contents)?;

    println!("{} operation completed successfully. File '{}' has been overwritten.", if mode == "E" { "Encryption" } else { "Decryption" }, filename);

    Ok(())
}

