use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;
use std::fs::{self, File};
use std::io::{self, Write};
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

fn main() -> io::Result<()> {
    let rt = Runtime::new().unwrap();

    println!("Enter the path of the directory to encrypt:");
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

    let mut files_encrypted = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            println!("Encrypting {:?}", path);
            rt.block_on(encrypt_file(path, &cipher))?;
            files_encrypted += 1;
        }
    }

    if files_encrypted == 0 {
        println!("No files found to encrypt!");
    } else {
        println!("Encryption complete! {} files encrypted.", files_encrypted);
    }

    Ok(())
}

