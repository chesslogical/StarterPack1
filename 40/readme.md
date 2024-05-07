
Rust Encryption/Decryption otp.


This application is a straightforward, command-line tool designed for basic file encryption and decryption using XOR methodology. It's built with Rust and offers a user-friendly interface for both generating encryption keys and processing files.

Features
The tool features three main functions:

Encrypting or decrypting files using a generated key.
Generating a secure encryption key.
A simple, interactive command-line interface.

How to Use ::
To use this tool, ensure you have Rust installed on your system. Compile the program by navigating to its directory and running cargo build --release. This generates an executable in the target/release directory.

Running the Program --
Execute the program from the command line in the target/release directory. Upon launch, you're presented with a menu offering three options:

Encrypt/Decrypt a file.
Generate an encryption key.
Exit the program.
Encrypting/Decrypting Files
To encrypt or decrypt a file, choose option 1, then enter the file name when prompted. The file should be in the same directory as the program or a relative path should be provided. The program will use the key from "key.key" for this process.

Generating an Encryption Key
Select option 2 to generate a new encryption key. You will be asked to specify the key length. The key is then saved as "key.key" in the program's directory and set to read-only for security.

Exiting the Program
Choose option 3 to exit the program.



//////////

Code details

// Import necessary standard libraries.
use std::fs::{self, File}; // For file operations like reading and writing.
use std::io::{self, Write, Read, Seek, SeekFrom}; // For input/output operations.
use std::time::Instant; // For measuring the duration of operations.
use rand::rngs::OsRng; // Import OsRng for cryptographically secure random number generation.
use rand::RngCore; // Core trait for random number generators.

// The main function, acting as the entry point of the program.
fn main() {
    // Infinite loop to keep the program running until the user decides to exit.
    loop {
        // Display the options to the user.
        println!("1: Encrypt/Decrypt\n2: Generate Key\n3: Exit");

        // Read user input and execute corresponding action.
        match read_line().trim() {
            "1" => process_file(), // If user enters '1', encrypt/decrypt a file.
            "2" => generate_key(), // If user enters '2', generate a new key.
            "3" => break, // If user enters '3', exit the loop and thus the program.
            _ => println!("Invalid choice"), // Handle any other input as invalid.
        }
    }
}

// Function to handle file encryption/decryption.
fn process_file() {
    // Prompt the user to enter the name of the file to be processed.
    println!("Enter the file name:");
    let file_name = read_line().trim().to_string(); // Read and store the file name.

    // Attempt to load the encryption key.
    let key = match load_key("key.key") {
        Ok(key) => key, // If loading is successful, use the key.
        Err(e) => {
            println!("Error loading key: {}", e); // Print an error message if loading fails.
            return;
        },
    };

    // Record the start time of the operation for duration calculation.
    let start_time = Instant::now();

    // Attempt to encrypt or decrypt the file.
    if let Err(e) = encrypt_decrypt_file(&file_name, &key) {
        println!("Error processing file: {}", e); // Print an error message if the operation fails.
    }

    // Calculate and print the duration of the operation.
    let duration = start_time.elapsed();
    println!("Operation completed in {:?}", duration);
}

// Function to encrypt or decrypt a file.
fn encrypt_decrypt_file(file_name: &str, key: &[u8]) -> io::Result<()> {
    // Open the file for reading and writing.
    let mut file = match File::options().read(true).write(true).open(file_name) {
        Ok(f) => f,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to open file: {}", e))),
    };

    // Get the size of the file.
    let file_size = match file.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to read file metadata: {}", e))),
    };

    // Check if the key is long enough for the file.
    if key.len() < file_size {
        return Err(io::Error::new(io::ErrorKind::Other, "Key is shorter than the file."));
    }

    // Define the size of chunks to read from the file.
    const CHUNK_SIZE: usize = 4096;
    // Create a buffer to hold each chunk.
    let mut buffer = vec![0; CHUNK_SIZE];

    // Initialize a variable to keep track of how much of the file has been processed.
    let mut processed_size = 0;
    // Loop until the entire file is processed.
    while processed_size < file_size {
        // Determine the number of bytes to read in the next chunk.
        let bytes_to_read = std::cmp::min(CHUNK_SIZE, file_size - processed_size);
        // Read the chunk into the buffer.
        file.read_exact(&mut buffer[..bytes_to_read])?;

        // XOR each byte in the chunk with the key.
        for i in 0..bytes_to_read {
            buffer[i] ^= key[i % key.len()];
        }

        // Write the processed chunk back to the file.
        file.seek(SeekFrom::Start(processed_size as u64))?;
        file.write_all(&buffer[..bytes_to_read])?;

        // Update the processed size.
        processed_size += bytes_to_read;
    }

    // Truncate the file to its original size (if necessary).
    file.set_len(file_size as u64)?;
    Ok(())
}

// Function to generate a new encryption key.
fn generate_key() {
    // Define the name of the key file.
    let key_file = "key.key";

    // Check if the key file already exists and abort if it does.
    if fs::metadata(key_file).is_ok() {
        println!("Error: Key file '{}' already exists. Key generation aborted.", key_file);
        return;
    }

    // Prompt the user for the desired key length.
    println!("Enter the desired key length (in bytes):");
    // Read and parse the key length.
    let length = match read_line().trim().parse::<usize>() {
        Ok(l) => l,
        Err(_) => {
            println!("Invalid length");
            return;
        },
    };

    // Initialize a cryptographically secure random number generator.
    let mut rng = OsRng;
    // Create a buffer to hold the key.
    let mut key = vec![0u8; length];
    // Fill the buffer with random bytes.
    rng.fill_bytes(&mut key);

    // Write the key to the key file.
    if let Err(e) = fs::write(key_file, &key) {
        println!("Unable to write key to file: {}", e);
        return;
    }
    // Inform the user that the key was successfully generated.
    println!("Key generated and saved to '{}'", key_file);

    // Set the key file to read-only.
    make_key_read_only(key_file).expect("Failed to set key file as read-only");
}

// Platform-specific implementation for making the key file read-only on Unix-like systems.
#[cfg(unix)]
fn make_key_read_only(key_file: &str) -> io::Result<()> {
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;
    // Set Unix-like read-only permissions.
    let permissions = Permissions::from_mode(0o444);
    fs::set_permissions(key_file, permissions)
}

// Platform-specific implementation for making the key file read-only on Windows systems.
#[cfg(windows)]
fn make_key_read_only(key_file: &str) -> io::Result<()> {
    // Retrieve the current file permissions.
    let mut permissions = fs::metadata(key_file)?.permissions();
    // Set the file to be read-only.
    permissions.set_readonly(true);
    fs::set_permissions(key_file, permissions)
}

// Function to load an encryption key from a file.
fn load_key(file_path: &str) -> io::Result<Vec<u8>> {
    // Read and return the key from the specified file.
    fs::read(file_path)
}

// Function to read a line of text from standard input.
fn read_line() -> String {
    // Create a new string to store the input.
    let mut input = String::new();
    // Read a line from standard input and store it in the string.
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // Return the input string.
    input
}
















