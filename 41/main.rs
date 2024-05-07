use rand::RngCore; // Import the RngCore trait from the rand crate.
use rand::thread_rng; // Import the thread_rng function from the rand crate.
use std::fs::File; // Import the File struct from the std::fs module.
use std::io::prelude::*; // Import the io::prelude module for file I/O functionality.

fn main() {
    let mut key = [0u8; 32]; // Create a 32-byte buffer to hold the AES key.
    thread_rng().fill_bytes(&mut key); // Generate a random AES key and fill the buffer with it.
    
    let mut file = File::create("key.key").expect("Failed to create key file"); // Create a new file called "key.key" in the current directory.
    file.write_all(&key).expect("Failed to write key to file"); // Write the AES key to the file.
    
    println!("Key written to file key.key"); // Print a message to the console to indicate that the key has been written to the file.
}
