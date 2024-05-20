use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn run() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <E/D> <filename> <keyfile>", args[0]);
        std::process::exit(1);
    }

    let action = &args[1];
    let filename = &args[2];
    let keyfile = &args[3];

    // Validate files exist
    if !Path::new(filename).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Data file not found"));
    }
    if !Path::new(keyfile).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Key file not found"));
    }

    // Create a lock file to ensure no concurrent access to the file
    let lock_file_path = format!("{}.lock", filename);
    if Path::new(&lock_file_path).exists() {
        return Err(io::Error::new(io::ErrorKind::Other, "File is currently locked by another process"));
    }
    let _lock_file = File::create(&lock_file_path)?;

    // Ensure lock file is removed when the scope ends
    let _lock_guard = LockFileGuard::new(lock_file_path.clone());

    // Check key length
    let key_len = fs::metadata(keyfile)?.len();
    let file_len = fs::metadata(filename)?.len();
    if key_len < file_len {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Key file must be at least as long as the data file"));
    }

    // Perform encryption or decryption
    match action.as_str() {
        "E" | "D" => {
            let temp_filename = format!("{}.tmp", filename);
            let process_result = process_file(filename, keyfile, &temp_filename);
            if process_result.is_err() {
                fs::remove_file(&temp_filename).ok();
            }
            fs::rename(&temp_filename, filename)?;
            process_result
        }
        _ => {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid action"))
        }
    }
}

fn process_file(data_filename: &str, key_filename: &str, temp_filename: &str) -> io::Result<()> {
    let data_file = OpenOptions::new().read(true).open(data_filename)?;
    let key_file = OpenOptions::new().read(true).open(key_filename)?;
    let mut data_reader = BufReader::new(data_file);
    let mut key_reader = BufReader::new(key_file);
    let data_writer = File::create(temp_filename)?;
    let mut data_writer = BufWriter::new(data_writer);

    let mut data_buffer = [0u8; 4096];
    let mut key_buffer = [0u8; 4096];

    while let Ok(data_len) = data_reader.read(&mut data_buffer) {
        if data_len == 0 {
            break;
        }

        let key_len = key_reader.read(&mut key_buffer)?;
        if key_len < data_len {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Key file must be at least as long as the data file"));
        }

        for i in 0..data_len {
            data_buffer[i] ^= key_buffer[i];
        }

        data_writer.write_all(&data_buffer[..data_len])?;
    }

    data_writer.flush()?;
    println!("File processed successfully.");
    Ok(())
}

struct LockFileGuard {
    path: String,
}

impl LockFileGuard {
    fn new(path: String) -> Self {
        LockFileGuard { path }
    }
}

impl Drop for LockFileGuard {
    fn drop(&mut self) {
        fs::remove_file(&self.path).ok();
    }
}
