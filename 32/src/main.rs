
use std::fs::{self, File};
use std::io::{self, Write, Read, Seek, SeekFrom};
use std::time::Instant;
use rand::rngs::OsRng; // Cryptographically secure RNG
use rand::RngCore;

fn main() {
    loop {
        println!("1: Encrypt/Decrypt\n2: Generate Key\n3: Exit");
        match read_line().trim() {
            "1" => process_file(),
            "2" => generate_key(),
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn process_file() {
    println!("Enter the file name:");
    let file_name = read_line().trim().to_string();

    let key = match load_key("key.key") {
        Ok(key) => key,
        Err(e) => {
            println!("Error loading key: {}", e);
            return;
        },
    };

    let start_time = Instant::now();
    if let Err(e) = encrypt_decrypt_file(&file_name, &key) {
        println!("Error processing file: {}", e);
    }

    let duration = start_time.elapsed();
    println!("Operation completed in {:?}", duration);
}

fn encrypt_decrypt_file(file_name: &str, key: &[u8]) -> io::Result<()> {
    let mut file = match File::options().read(true).write(true).open(file_name) {
        Ok(f) => f,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to open file: {}", e))),
    };

    let file_size = match file.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to read file metadata: {}", e))),
    };

    if key.len() < file_size {
        return Err(io::Error::new(io::ErrorKind::Other, "Key is shorter than the file."));
    }

    const CHUNK_SIZE: usize = 4096;
    let mut buffer = vec![0; CHUNK_SIZE];

    let mut processed_size = 0;
    while processed_size < file_size {
        let bytes_to_read = std::cmp::min(CHUNK_SIZE, file_size - processed_size);
        file.read_exact(&mut buffer[..bytes_to_read])?;

        for i in 0..bytes_to_read {
            buffer[i] ^= key[i % key.len()];
        }

        file.seek(SeekFrom::Start(processed_size as u64))?;
        file.write_all(&buffer[..bytes_to_read])?;

        processed_size += bytes_to_read;
    }

    file.set_len(file_size as u64)?;
    Ok(())
}

fn generate_key() {
    let key_file = "key.key";

    if fs::metadata(key_file).is_ok() {
        println!("Error: Key file '{}' already exists. Key generation aborted.", key_file);
        return;
    }

    println!("Enter the desired key length (in bytes):");
    let length = match read_line().trim().parse::<usize>() {
        Ok(l) => l,
        Err(_) => {
            println!("Invalid length");
            return;
        },
    };

    let mut rng = OsRng; // Cryptographically secure RNG
    let mut key = vec![0u8; length];
    rng.fill_bytes(&mut key);

    if let Err(e) = fs::write(key_file, &key) {
        println!("Unable to write key to file: {}", e);
        return;
    }
    println!("Key generated and saved to '{}'", key_file);

    make_key_read_only(key_file).expect("Failed to set key file as read-only");
}

// Platform-specific implementations for making key file read-only
#[cfg(unix)]
fn make_key_read_only(key_file: &str) -> io::Result<()> {
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;
    let permissions = Permissions::from_mode(0o444); // Unix-like read-only permissions
    fs::set_permissions(key_file, permissions)
}

#[cfg(windows)]
fn make_key_read_only(key_file: &str) -> io::Result<()> {
    let mut permissions = fs::metadata(key_file)?.permissions();
    permissions.set_readonly(true);
    fs::set_permissions(key_file, permissions)
}

fn load_key(file_path: &str) -> io::Result<Vec<u8>> {
    fs::read(file_path)
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}
