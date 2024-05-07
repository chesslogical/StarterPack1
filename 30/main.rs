use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use tokio::runtime::Runtime;

async fn encrypt_file(path: &Path, cipher: &Aes256Gcm) -> io::Result<()> {
    let content = fs::read(path)?;

    let mut nonce = [0u8; 12];
    rand::thread_rng().fill(&mut nonce);
    let nonce = Nonce::from(nonce);

    let encrypted_content = cipher.encrypt(&nonce, content.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed"))?;

    let mut file = File::create(path)?;
    file.write_all(&nonce)?;
    file.write_all(&encrypted_content)?;

    Ok(())
}

async fn decrypt_file(path: &Path, cipher: &Aes256Gcm) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut nonce = [0u8; 12];
    file.read_exact(&mut nonce)?;
    let nonce = Nonce::from(nonce);

    let mut encrypted_content = Vec::new();
    file.read_to_end(&mut encrypted_content)?;

    let content = cipher.decrypt(&nonce, encrypted_content.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Decryption failed"))?;

    let mut file = File::create(path)?;
    file.write_all(&content)?;

    Ok(())
}

fn process_directory(path: &str, cipher: &Aes256Gcm, encrypt: bool) -> io::Result<()> {
    let rt = Runtime::new().unwrap();

    let mut files_processed = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if encrypt {
                println!("Encrypting {:?}", path);
                rt.block_on(encrypt_file(path, cipher))?;
            } else {
                println!("Decrypting {:?}", path);
                rt.block_on(decrypt_file(path, cipher))?;
            }
            files_processed += 1;
        }
    }

    if files_processed == 0 {
        println!("No files found to process!");
    } else if encrypt {
        println!("Encryption complete! {} files encrypted.", files_processed);
    } else {
        println!("Decryption complete! {} files decrypted.", files_processed);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    println!("Enter the path of the directory to process:");
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    let path = path.trim();

    if !Path::new(path).is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Specified path is not a directory"));
    }

    let mut key = [0u8; 32];
    rand::thread_rng().fill(&mut key);
    let key = Key::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    println!("Do you want to (E)ncrypt or (D)ecrypt?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    match choice.trim().to_uppercase().as_str() {
        "E" => process_directory(path, &cipher, true),
        "D" => process_directory(path, &cipher, false),
        _ => {
            println!("Invalid choice! Please enter 'E' for encryption or 'D' for decryption.");
            Ok(())
        }
    }
}

