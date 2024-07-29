
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

fn derive_key(password: &str, key_length: usize) -> Vec<u8> {
    let mut key = vec![0; key_length];
    let password_bytes = password.as_bytes();
    for (i, byte) in key.iter_mut().enumerate() {
        *byte = password_bytes[i % password_bytes.len()] ^ (i as u8);
    }
    for i in 0..10000 {
        for j in 0..key_length {
            key[j] = key[j].wrapping_add(i as u8);
        }
    }
    key
}

fn xor_encrypt_decrypt(data: &mut [u8], key: &[u8]) {
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: <program> <e|d> <file> <password>");
        return Ok(());
    }

    let mode = &args[1];
    let file_name = &args[2];
    let password = &args[3];

    if !["e", "d"].contains(&mode.as_str()) {
        eprintln!("Invalid mode. Use 'e' to encrypt or 'd' to decrypt.");
        return Ok(());
    }

    let path = Path::new(file_name);
    if !path.exists() {
        eprintln!("File does not exist.");
        return Ok(());
    }

    let mut file = OpenOptions::new().read(true).write(true).open(file_name)?;

    let key_length = 256;
    let key = derive_key(password, key_length);

    let mut buffer = vec![0; CHUNK_SIZE];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }
        xor_encrypt_decrypt(&mut buffer[..bytes_read], &key);

        file.seek(SeekFrom::Current(-(bytes_read as i64)))?;
        file.write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}
