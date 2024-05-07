use clap::{Command, Arg};
use ring::{aead, pbkdf2, rand::{SystemRandom, SecureRandom}};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter}};
use std::num::NonZeroU32;
use anyhow::{Result, anyhow};

const ITERATIONS: u32 = 100_000;
const SALT: &[u8] = b"UniqueSalt";

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("AES Encryption App")
        .version("0.1.0")
        .author("Your Company")
        .about("Encrypts and decrypts files using AES-256 GCM")
        .arg(Arg::new("encrypt")
            .long("encrypt")
            .value_name("FILE")
            .help("File to encrypt")
            .required(true))
        .arg(Arg::new("decrypt")
            .long("decrypt")
            .value_name("FILE")
            .help("File to decrypt")
            .required(true))
        .arg(Arg::new("password")
            .long("password")
            .value_name("PASSWORD")
            .help("Password for key derivation")
            .required(true))
        .get_matches();

    let password = matches.get_one::<String>("password").unwrap().as_bytes();
    let mut key = [0u8; 32];
    pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA256, NonZeroU32::new(ITERATIONS).unwrap(), SALT, password, &mut key);

    if let Some(file) = matches.get_one::<String>("encrypt") {
        encrypt_file(file, &key).await?;
    } else if let Some(file) = matches.get_one::<String>("decrypt") {
        decrypt_file(file, &key).await?;
    }

    Ok(())
}

async fn encrypt_file(filename: &str, key: &[u8]) -> Result<()> {
    let file = File::open(filename).await?;
    let mut reader = BufReader::new(file);
    let output_file = File::create(format!("{}.enc", filename)).await?;
    let mut writer = BufWriter::new(output_file);

    let rng = SystemRandom::new();
    let mut nonce = [0u8; 12];
    rng.fill(&mut nonce).map_err(|_| anyhow!("Nonce generation failed"))?;

    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key).map_err(|_| anyhow!("Key setup failed"))?;
    let aead_alg = aead::LessSafeKey::new(unbound_key);
    let mut buffer = vec![0u8; 4096];
    while let Ok(read) = reader.read(&mut buffer).await {
        if read == 0 { break; }
        let tag = aead_alg.seal_in_place_separate_tag(aead::Nonce::assume_unique_for_key(nonce), aead::Aad::empty(), &mut buffer[..read]).map_err(|_| anyhow!("Encryption failed"))?;
        writer.write_all(&buffer[..read]).await?;
        writer.write_all(tag.as_ref()).await?;
    }

    Ok(())
}

async fn decrypt_file(filename: &str, key: &[u8]) -> Result<()> {
    let file = File::open(filename).await?;
    let mut reader = BufReader::new(file);
    let output_file = File::create(format!("{}.dec", filename)).await?;
    let mut writer = BufWriter::new(output_file);

    let nonce = [0u8; 12];
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key).map_err(|_| anyhow!("Key setup failed"))?;
    let aead_alg = aead::LessSafeKey::new(unbound_key);

    let tag_len = aead::AES_256_GCM.tag_len();
    let mut buffer = vec![0u8; 4096 + tag_len];
    while let Ok(read) = reader.read(&mut buffer).await {
        if read == 0 { break; }
        let decrypted_data = aead_alg.open_in_place(aead::Nonce::assume_unique_for_key(nonce), aead::Aad::empty(), &mut buffer[..read]).map_err(|_| anyhow!("Decryption failed"))?;
        writer.write_all(decrypted_data).await?;
    }

    Ok(())
}

