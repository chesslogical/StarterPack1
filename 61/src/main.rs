use anyhow::anyhow;
use clap::Parser;
use rand::{rngs::OsRng, RngCore};
use rpassword;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use zeroize::Zeroize;
use ring::aead;
use ring::hmac;

const HMAC_TAG_LEN: usize = 32; // Define the HMAC tag length constant

fn argon2_config<'a>() -> argon2::Config<'a> {
    argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    }
}

fn create_hmac_key(password: &str, salt: &[u8]) -> hmac::Key {
    let argon2_config = argon2_config();
    let key = argon2::hash_raw(password.as_bytes(), salt, &argon2_config).unwrap();
    hmac::Key::new(hmac::HMAC_SHA256, &key)
}

pub fn decrypt_file(enc_file_path: &str, password: &str) -> Result<Vec<u8>, anyhow::Error> {
    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 12];

    let mut encrypted_file = File::open(enc_file_path)?;

    let mut read_count = encrypted_file.read(&mut salt)?;
    if read_count != salt.len() {
        return Err(anyhow!("Error reading salt."));
    }

    read_count = encrypted_file.read(&mut nonce)?;
    if read_count != nonce.len() {
        return Err(anyhow!("Error reading nonce."));
    }

    let file_data = fs::read(enc_file_path)?;
    let (encrypted_data, hmac_tag) = file_data.split_at(file_data.len() - HMAC_TAG_LEN);

    let hmac_key = create_hmac_key(password, &salt);
    hmac::verify(&hmac_key, encrypted_data, hmac_tag).map_err(|_| anyhow!("HMAC verification failed"))?;

    let argon2_config = argon2_config();
    let mut key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;
    let ad = [0u8; 32];
    let mut file_data = encrypted_data[44..].to_vec();

    let ring_key = aead::LessSafeKey::new(aead::UnboundKey::new(&aead::AES_256_GCM, &key).unwrap());
    let ring_nonce = aead::Nonce::assume_unique_for_key(nonce);
    let ring_ad = aead::Aad::from(&ad);

    let plaintext = ring_key
        .open_in_place(ring_nonce, ring_ad, &mut file_data)
        .map_err(|err| anyhow!("ring: decrypting file: {}", err))?;

    salt.zeroize();
    nonce.zeroize();
    key.zeroize();

    Ok(plaintext.to_vec())
}

pub fn encrypt_file(src_file_path: &str, password: &str) -> Result<Vec<u8>, anyhow::Error> {
    let argon2_config = argon2_config();

    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 12];

    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let mut key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;
    let ad = [0u8; 32];
    let mut file_data = fs::read(src_file_path)?;

    let ring_key = aead::LessSafeKey::new(aead::UnboundKey::new(&aead::AES_256_GCM, &key).unwrap());
    let ring_nonce = aead::Nonce::assume_unique_for_key(nonce);
    let ring_ad = aead::Aad::from(&ad);

    ring_key
        .seal_in_place_append_tag(ring_nonce, ring_ad, &mut file_data)
        .map_err(|err| anyhow!("ring: encrypting file: {}", err))?;

    let mut encrypted_data = Vec::new();
    encrypted_data.extend_from_slice(&salt);
    encrypted_data.extend_from_slice(&nonce);
    encrypted_data.extend_from_slice(&file_data);

    let hmac_key = create_hmac_key(password, &salt);
    let hmac_tag = hmac::sign(&hmac_key, &encrypted_data);

    encrypted_data.extend_from_slice(hmac_tag.as_ref());

    salt.zeroize();
    nonce.zeroize();
    key.zeroize();

    Ok(encrypted_data)
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 'encrypt' or 'decrypt'
    action: String,

    /// Path to the input file
    input: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let source_path = args.input;

    if !(args.action == "decrypt" || args.action == "encrypt") {
        return Err(anyhow!("Provide a valid action: 'encrypt' or 'decrypt'"));
    }

    let mut password = rpassword::prompt_password("Enter the password:")?;

    if args.action == "encrypt" {
        let mut confirm_password = rpassword::prompt_password("Confirm the password:")?;
        if password != confirm_password {
            return Err(anyhow!("Passwords do not match."));
        }
        confirm_password.zeroize();

        let encrypted_data = encrypt_file(&source_path, &password)?;

        let mut file = File::create(&source_path)?;
        file.write_all(&encrypted_data)?;
    } else {
        let decrypted_data = decrypt_file(&source_path, &password)?;

        let mut file = File::create(&source_path)?;
        file.write_all(&decrypted_data)?;
    }

    password.zeroize();

    Ok(())
}

