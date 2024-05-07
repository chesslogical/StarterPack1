
use rand::Rng;
use std::fs::{self, File};
use std::io::{self, Write};
use std::time::Instant;

const SHRED_DIR: &str = "./shred";

fn main() -> io::Result<()> {
    // Generate a random key of the same length as the largest file in the directory
    let key_len = get_max_file_size()?;
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..key_len).map(|_| rng.gen()).collect();

    // Start the timer
    let start_time = Instant::now();

    // Encrypt all files in the shred directory
    let files = fs::read_dir(SHRED_DIR)?;
    let mut num_files_encrypted = 0;
    for file in files {
        let file = file?.path();
        if file.is_file() {
            let mut data = fs::read(&file)?;
            bit_shift(&mut data);
            xor(&mut data, &key);
            let mut encrypted_file = File::create(&file)?;
            encrypted_file.write_all(&data)?;
            num_files_encrypted += 1;
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_time = start_time.elapsed();

    // Print the results
    println!("Time taken: {}ms", elapsed_time.as_millis());
    println!("Files encrypted: {}", num_files_encrypted);
    println!("Bytes used in key: {}", key.len());

    // Wait for any key press before exiting
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}

fn bit_shift(data: &mut [u8]) {
    for byte in data {
        *byte = byte.wrapping_shr(1) | byte.wrapping_shl(7);
    }
}

fn xor(data: &mut [u8], key: &[u8]) {
    for i in 0..data.len() {
        data[i] = data[i] ^ key[i % key.len()];
    }
}

fn get_max_file_size() -> io::Result<usize> {
    let mut max_size = 0;
    for entry in fs::read_dir(SHRED_DIR)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let size = entry.metadata()?.len() as usize;
            if size > max_size {
                max_size = size;
            }
        }
    }
    Ok(max_size)
}
