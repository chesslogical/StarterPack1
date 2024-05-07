use std::fs;
use std::io;
use std::path::Path;
use std::time::Instant;

mod functions;

const SHRED_DIR: &str = "./shred";

fn main() -> io::Result<()> {
    // Generate a random key of the same length as the largest file in the directory
    let max_file_size = functions::get_max_file_size(Path::new(SHRED_DIR)).unwrap_or(0);
    let key_len = max_file_size as usize;
    let key = functions::generate_key(key_len);

    // Start the timer
    let start_time = Instant::now();

    // Encrypt all files in the shred directory and its subdirectories
    let num_files_encrypted = functions::encrypt_files(Path::new(SHRED_DIR), &key)?;

    // Stop the timer and calculate the elapsed time
    let elapsed_time = start_time.elapsed();

    // Print the results
    println!("Time taken: {}ms", elapsed_time.as_millis());
    println!("Files encrypted: {}", num_files_encrypted);
    println!("Bytes used in key: {}", key.len());

    // Wait for any key press before exiting
    functions::pause()?;

    Ok(())
}
