use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

const ENCRYPTED_FILE_MARKER: &[u8; 13] = b"ENCRYPTEDFILE";

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), io::Error> {
    println!("Enter the filename:");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    // Check if the file is encrypted or not
    let is_encrypted = is_file_encrypted(filename)?;

    if is_encrypted {
        decrypt_file(filename)?;
        println!("File decrypted successfully!");
    } else {
        encrypt_file(filename)?;
        println!("File encrypted successfully!");
    }

    Ok(())
}

fn is_file_encrypted(filename: &str) -> Result<bool, io::Error> {
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut buffer = [0; 13];
    match file.read_exact(&mut buffer) {
        Ok(_) => Ok(buffer == *ENCRYPTED_FILE_MARKER),
        Err(_) => Ok(false),
    }
}

fn encrypt_file(filename: &str) -> Result<(), io::Error> {
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let key = Key::from(rand::thread_rng().gen::<[u8; 32]>());
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from(rand::thread_rng().gen::<[u8; 12]>());

    let ciphertext = cipher.encrypt(&nonce, content.as_ref()).expect("encryption failure");

    let mut file = File::create(filename)?;
    file.write_all(ENCRYPTED_FILE_MARKER)?;
    file.write_all(key.as_slice())?;
    file.write_all(nonce.as_slice())?;
    file.write_all(&ciphertext)?;

    Ok(())
}

fn decrypt_file(filename: &str) -> Result<(), io::Error> {
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    if buffer.len() < 57 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "File is too small to be decrypted"));
    }

    let key = Key::from_slice(&buffer[13..45]);
    let nonce = Nonce::from_slice(&buffer[45..57]);
    let cipher = Aes256Gcm::new(key);

    let ciphertext = &buffer[57..];
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Decryption failed"))?;

    let mut file = File::create(filename)?;
    file.write_all(&plaintext)?;

    Ok(())
}



