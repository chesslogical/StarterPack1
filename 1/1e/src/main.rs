use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::process;

fn main() -> io::Result<()> {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
    Ok(())
}

fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <E/D> <filename> <keyfile>", args[0]);
        process::exit(1);
    }

    let action = &args[1];
    let filename = &args[2];
    let keyfile = &args[3];

    if !Path::new(filename).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Data file '{}' not found", filename)));
    }
    if !Path::new(keyfile).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Key file '{}' not found", keyfile)));
    }

    let lock_file_path = format!("{}.lock", filename);
    if Path::new(&lock_file_path).exists() {
        return Err(io::Error::new(io::ErrorKind::Other, "File is currently locked by another process"));
    }
    let _lock_file = File::create(&lock_file_path)?;
    let _lock_guard = LockFileGuard::new(lock_file_path.clone());

    let key_len = fs::metadata(keyfile)?.len();
    let file_len = fs::metadata(filename)?.len();
    if key_len < file_len {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Key file must be at least as long as the data file"));
    }

    match action.as_str() {
        "E" | "D" => process_file(action, filename, keyfile),
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid action. Use 'E' for encryption or 'D' for decryption.")),
    }
}

fn process_file(action: &str, data_filename: &str, key_filename: &str) -> io::Result<()> {
    let temp_filename = format!("{}.tmp", data_filename);
    let checksum_filename = format!("{}.checksum", data_filename);

    println!("Opening files...");
    let data_file = OpenOptions::new().read(true).open(data_filename)?;
    let key_file = OpenOptions::new().read(true).open(key_filename)?;
    let mut data_reader = BufReader::new(data_file);
    let mut key_reader = BufReader::new(key_file);

    let temp_file = File::create(&temp_filename)?;
    let mut data_writer = BufWriter::new(temp_file);

    let mut data_buffer = [0u8; 4096];
    let mut key_buffer = [0u8; 4096];
    let mut hasher = SimpleHasher::new();
    let mut key_index = 0;
    let mut key_buffer_len = 0;

    loop {
        let data_len = data_reader.read(&mut data_buffer)?;
        if data_len == 0 {
            break;
        }

        if key_buffer_len - key_index < data_len {
            let read_len = key_reader.read(&mut key_buffer[key_buffer_len..])?;
            key_buffer_len += read_len;

            if key_buffer_len < data_len {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Key file must be at least as long as the data file"));
            }
        }

        for i in 0..data_len {
            data_buffer[i] ^= key_buffer[key_index];
            key_index += 1;
            if key_index == key_buffer_len {
                key_index = 0;
                key_buffer_len = 0;
            }
        }

        data_writer.write_all(&data_buffer[..data_len])?;
        hasher.update(&data_buffer[..data_len]);

        // Zero out buffers after use
        data_buffer.iter_mut().for_each(|byte| *byte = 0);
    }

    data_writer.flush()?;
    let new_checksum = hasher.finalize();

    if action == "E" {
        fs::write(&checksum_filename, new_checksum.to_string())?;
        println!("Encryption completed successfully.");
    } else {
        let original_checksum_str = fs::read_to_string(&checksum_filename)?;
        let original_checksum = original_checksum_str.trim().parse::<u32>().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid checksum in checksum file")
        })?;

        if original_checksum != new_checksum {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Checksum mismatch! Data integrity verification failed."));
        }
        fs::remove_file(&checksum_filename)?;
        println!("Decryption completed successfully.");
    }

    fs::rename(&temp_filename, data_filename)?;
    println!("File processed successfully.");
    Ok(())
}

struct SimpleHasher {
    checksum: u32,
}

impl SimpleHasher {
    fn new() -> Self {
        Self { checksum: 0 }
    }

    fn update(&mut self, data: &[u8]) {
        for &byte in data {
            self.checksum = self.checksum.wrapping_add(byte as u32);
        }
    }

    fn finalize(self) -> u32 {
        self.checksum
    }
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

