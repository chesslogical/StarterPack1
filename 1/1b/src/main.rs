use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <E/D> <filename> <keyfile>", args[0]);
        std::process::exit(1);
    }
    let action = &args[1];
    let filename = &args[2];
    let keyfile = &args[3];

    let lock_file_path = format!("{}.lock", filename);
    if Path::new(&lock_file_path).exists() {
        eprintln!("Error: File is currently locked by another process.");
        return Err(io::Error::new(io::ErrorKind::Other, "File is locked"));
    }
    let _lock_file = File::create(&lock_file_path)?;

    let result = match action.as_str() {
        "E" | "D" => {
            let temp_filename = format!("{}.tmp", filename);
            let process_result = process_file(filename, keyfile, &temp_filename, "File processed successfully.");
            if let Err(e) = process_result {
                fs::remove_file(&temp_filename)?;
                return Err(e);
            }
            match fs::rename(&temp_filename, filename) {
                Ok(_) => process_result,
                Err(e) => Err(e),
            }
        }
        _ => {
            eprintln!("Invalid action. Use 'E' for encrypt or 'D' for decrypt.");
            Err(io::Error::new(io::ErrorKind::Other, "Invalid action"))
        }
    };

    fs::remove_file(&lock_file_path)?;
    result
}

fn process_file(data_filename: &str, key_filename: &str, temp_filename: &str, success_message: &str) -> io::Result<()> {
    let data_file = OpenOptions::new().read(true).write(true).open(data_filename)?;
    let key_file = OpenOptions::new().read(true).open(key_filename)?;
    let mut data_reader = BufReader::new(data_file);
    let mut key_reader = BufReader::new(key_file);
    let data_writer = File::create(temp_filename)?;
    let mut data_writer = BufWriter::new(data_writer);

    let mut data_buffer = [0u8; 4096];
    let mut key_buffer = [0u8; 4096];

    while let (Ok(data_len), Ok(key_len)) = (data_reader.read(&mut data_buffer), key_reader.read(&mut key_buffer)) {
        if data_len == 0 || key_len == 0 {
            break;
        }

        if key_len < data_len {
            eprintln!("Error: Key file must be at least as long as the data file.");
            return Err(io::Error::new(io::ErrorKind::Other, "Key file is too short"));
        }

        for i in 0..data_len {
            data_buffer[i] ^= key_buffer[i];
        }

        data_writer.write_all(&data_buffer[..data_len])?;
    }

    data_writer.flush()?;
    println!("{}", success_message);
    Ok(())
}

