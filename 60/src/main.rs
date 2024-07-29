
use chacha20poly1305::aead::{Aead, KeyInit, OsRng as ChaChaOsRng};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use argon2::{self, password_hash::{PasswordHasher, SaltString}};
use argon2::password_hash::rand_core::OsRng as ArgonOsRng;
use clap::{Arg, Command};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use thiserror::Error;
use rand::RngCore;

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB chunks
const NONCE_SIZE: usize = 12; // Size of the nonce for ChaCha20Poly1305
const SALT_SIZE: usize = 16;  // Size of the salt for Argon2

#[derive(Error, Debug)]
enum CliError {
    #[error("IO Error")]
    Io(#[from] io::Error),
    #[error("Decryption Error")]
    Decryption,
    #[error("Key Error")]
    KeyError,
}

impl From<chacha20poly1305::aead::Error> for CliError {
    fn from(_: chacha20poly1305::aead::Error) -> Self {
        CliError::Decryption
    }
}

fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<Key, CliError> {
    let salt = SaltString::encode_b64(salt).map_err(|_| CliError::KeyError)?;
    let argon2 = argon2::Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| CliError::KeyError)?;

    let hash_value = password_hash.hash.ok_or(CliError::KeyError)?;
    let hash_bytes = hash_value.as_bytes();
    let key_bytes: [u8; 32] = hash_bytes[..32].try_into().map_err(|_| CliError::KeyError)?;

    Ok(*Key::from_slice(&key_bytes))
}

fn encrypt_data_file(input_file: &str, kek: &Key, salt: &[u8]) -> Result<(), CliError> {
    let mut input = File::open(input_file)?;
    let mut temp_output = File::create("temp_encrypted_file")?;
    let cipher = ChaCha20Poly1305::new(kek);

    // Write the salt to the output file
    temp_output.write_all(salt)?;

    let mut buffer = vec![0u8; CHUNK_SIZE];
    loop {
        let read_bytes = input.read(&mut buffer)?;
        if read_bytes == 0 {
            break;
        }

        let mut nonce = [0u8; NONCE_SIZE];
        ChaChaOsRng.fill_bytes(&mut nonce);
        let nonce = Nonce::from_slice(&nonce);

        let ciphertext = cipher.encrypt(nonce, &buffer[..read_bytes])
            .map_err(|_| CliError::Decryption)?;
        temp_output.write_all(nonce)?;
        temp_output.write_all(&ciphertext)?;
    }

    drop(input); // Close the input file
    drop(temp_output); // Close the temporary output file

    // Rename the temporary file to overwrite the original file
    fs::rename("temp_encrypted_file", input_file)?;

    Ok(())
}

fn decrypt_data_file(input_file: &str, password: &str) -> Result<(), CliError> {
    let mut input = File::open(input_file)?;

    // Read the salt from the beginning of the file
    let mut salt = vec![0u8; SALT_SIZE];
    input.read_exact(&mut salt)?;

    // Derive the key from the password and salt
    let kek = derive_key_from_password(password, &salt)?;

    let mut temp_output = File::create("temp_decrypted_file")?;
    let cipher = ChaCha20Poly1305::new(&kek);

    let mut nonce = vec![0u8; NONCE_SIZE];
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut success = true;

    while let Ok(_) = input.read_exact(&mut nonce) {
        let nonce = Nonce::from_slice(&nonce);
        let read_bytes = input.read(&mut buffer)?;
        if read_bytes == 0 {
            break;
        }

        let ciphertext = &buffer[..read_bytes];
        match cipher.decrypt(nonce, ciphertext) {
            Ok(plaintext) => temp_output.write_all(&plaintext)?,
            Err(_) => {
                success = false;
                break;
            }
        }
    }

    drop(input); // Close the input file
    drop(temp_output); // Close the temporary output file

    // Handle failed decryption
    if success {
        // Rename the temporary file to overwrite the original file
        fs::rename("temp_decrypted_file", input_file)?;
    } else {
        // Remove the temporary decrypted file if decryption failed
        fs::remove_file("temp_decrypted_file").ok();
        return Err(CliError::Decryption);
    }

    Ok(())
}

fn main() -> Result<(), CliError> {
    let matches = Command::new("Key Management CLI")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Encrypt and decrypt files using a password-derived key")
        .subcommand(Command::new("e")
            .about("Encrypt a file with a password-derived key")
            .arg(Arg::new("password")
                .required(true)
                .index(1))
            .arg(Arg::new("input_file")
                .required(true)
                .index(2)))
        .subcommand(Command::new("d")
            .about("Decrypt a file using a password-derived key")
            .arg(Arg::new("password")
                .required(true)
                .index(1))
            .arg(Arg::new("input_file")
                .required(true)
                .index(2)))
        .get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("e") {
        let password = sub_matches.get_one::<String>("password").expect("Password is required");
        let input_file = sub_matches.get_one::<String>("input_file").expect("Input file is required");

        let mut salt = vec![0u8; SALT_SIZE];
        ArgonOsRng.fill_bytes(&mut salt);

        let kek = derive_key_from_password(password, &salt)?;
        encrypt_data_file(input_file, &kek, &salt)?;
    } else if let Some(sub_matches) = matches.subcommand_matches("d") {
        let password = sub_matches.get_one::<String>("password").expect("Password is required");
        let input_file = sub_matches.get_one::<String>("input_file").expect("Input file is required");

        decrypt_data_file(input_file, password)?;
    }

    Ok(())
}
