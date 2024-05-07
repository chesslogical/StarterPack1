use rand::Rng;
use sha2::{Sha512, Digest};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

fn main() {
    // Measure the time taken by the operation
    let start = Instant::now();

    // Create the "key" directory in the current working directory
    let dir_path = Path::new("key");

    // Check if the "key" directory already exists
    if dir_path.exists() {
        println!("Warning: The 'key' directory already exists.");
    } else {
        // If the directory doesn't exist, create it
        if let Err(e) = fs::create_dir_all(&dir_path) {
            eprintln!("Error creating directory: {}", e);
            return;
        }

        // Generate and write the 1000 files
        let mut rng = rand::thread_rng();
        for i in 1..=1000 {
            let file_path = dir_path.join(format!("{}.key", i));
            let mut file = match File::create(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error creating file {}: {}", file_path.display(), e);
                    return;
                }
            };

            // Generate a random SHA-512 key and convert it to a hexadecimal string
            let mut buffer = [0u8; 64];
            rng.fill(&mut buffer[..]);
            let hash = Sha512::digest(&buffer);
            let hash_hex = format!("{:x}", hash);

            // Write the hexadecimal string to the file
            if let Err(e) = file.write_all(hash_hex.as_bytes()) {
                eprintln!("Error writing to file {}: {}", file_path.display(), e);
                return;
            }
        }

        // Output the result
        let duration = start.elapsed();
        println!("Operation took: {:?}", duration);
        println!("1000 files were written.");
    }

    // Press Enter to exit
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}
