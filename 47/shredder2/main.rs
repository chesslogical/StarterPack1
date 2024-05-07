
use rand::Rng;
use std::fs::{self, File};
use std::io::{self, Write};
use std::time::Instant;

const SHRED_DIR: &str = "./shred";

fn main() -> io::Result<()> {
    // Generate a random key of the same length as the largest file in the directory
    let key_len = match get_max_file_size() {
        Ok(size) => size,
        Err(e) => {
            println!("Error getting max file size: {}", e);
            pause()?;
            return Err(e);
        }
    };
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..key_len).map(|_| rng.gen()).collect();

    // Start the timer
    let start_time = Instant::now();

    // Encrypt all files in the shred directory
    let files = match fs::read_dir(SHRED_DIR) {
        Ok(dir) => dir,
        Err(e) => {
            println!("Error reading directory: {}", e);
            pause()?;
            return Err(e);
        }
    };
    let mut num_files_encrypted = 0;
    for file in files {
        let file = match file {
            Ok(f) => f.path(),
            Err(e) => {
                println!("Error getting file path: {}", e);
                pause()?;
                continue;
            }
        };
        if file.is_file() {
            let mut data = match fs::read(&file) {
                Ok(d) => d,
                Err(e) => {
                    println!("Error reading file {}: {}", file.display(), e);
                    pause()?;
                    continue;
                }
            };
            bit_shift(&mut data);
            xor(&mut data, &key);
            let mut encrypted_file = match File::create(&file) {
                Ok(f) => f,
                Err(e) => {
                    println!("Error creating encrypted file {}: {}", file.display(), e);
                    pause()?;
                    continue;
                }
            };
            match encrypted_file.write_all(&data) {
                Ok(_) => num_files_encrypted += 1,
                Err(e) => {
                    println!("Error writing encrypted data to file {}: {}", file.display(), e);
                    pause()?;
                    continue;
                }
            }
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_time = start_time.elapsed();

    // Print the results
    println!("Time taken: {}ms", elapsed_time.as_millis());
    println!("Files encrypted: {}", num_files_encrypted);
    println!("Bytes used in key: {}", key.len());

    // Wait for any key press before exiting
    pause()?;

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

fn pause() -> io::Result<()> {
     let mut input = String::new();
    print!("Press any key to continue...");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(())
}
