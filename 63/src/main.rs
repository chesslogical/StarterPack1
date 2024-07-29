use std::env;
use std::fs::{File};
use std::io::{self, Write, Read, BufReader};
use std::process;
use rand::Rng;
use rand::distributions::Uniform;

const KEY_FILENAME: &str = "key.key";
const REPORT_FILENAME: &str = "report.txt";
const EXT_ASCII_RANGE: std::ops::RangeInclusive<u8> = 1..=255;
const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: key <size_in_mb> or key scan");
        process::exit(1);
    }

    match args[1].as_str() {
        "scan" => {
            if let Err(e) = scan_key() {
                eprintln!("Error scanning key file: {}", e);
                process::exit(1);
            }
        },
        size_arg => {
            match size_arg.parse::<usize>() {
                Ok(size_in_mb) => {
                    if let Err(e) = create_key(size_in_mb) {
                        eprintln!("Error creating key file: {}", e);
                        process::exit(1);
                    }
                },
                Err(_) => {
                    eprintln!("Invalid size argument. Please provide a number.");
                    process::exit(1);
                }
            }
        }
    }
}

fn create_key(size_in_mb: usize) -> io::Result<()> {
    if std::path::Path::new(KEY_FILENAME).exists() {
        eprintln!("Key file already exists.");
        process::exit(1);
    }

    let size_in_bytes = size_in_mb * 1024 * 1024;
    let mut rng = rand::thread_rng();
    let dist = Uniform::new_inclusive(EXT_ASCII_RANGE.start(), EXT_ASCII_RANGE.end());

    let mut file = File::create(KEY_FILENAME)?;
    let mut buffer = vec![0u8; CHUNK_SIZE];

    let mut bytes_written = 0;
    while bytes_written < size_in_bytes {
        let chunk_size = std::cmp::min(CHUNK_SIZE, size_in_bytes - bytes_written);

        for i in 0..chunk_size {
            buffer[i] = rng.sample(&dist);
        }

        file.write_all(&buffer[..chunk_size])?;
        bytes_written += chunk_size;
    }

    println!("Key file created successfully.");
    Ok(())
}

fn scan_key() -> io::Result<()> {
    let file = File::open(KEY_FILENAME)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut counts = [0usize; 256];

    for &byte in &buffer {
        counts[byte as usize] += 1;
    }

    let mut report = File::create(REPORT_FILENAME)?;
    for (i, &count) in counts.iter().enumerate() {
        if i > 0 {
            writeln!(report, "{}: {}", i as u8 as char, count)?;
        }
    }

    println!("Report generated successfully.");
    Ok(())
}

