use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::process::exit;

const BUFFER_SIZE: usize = 8 * 1024; // 8 KB buffer size

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: <operation> <file> <password>");
        exit(1);
    }

    let operation = &args[1];
    let file_name = &args[2];
    let mut password = args[3].clone(); // Get a mutable clone of the password

    // Open the input file
    let input_file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
            exit(1);
        }
    };
    let mut reader = BufReader::new(input_file);

    // Create the temporary output file
    let temp_file_name = format!("{}.tmp", file_name);
    
    // Remove existing temporary file if it exists
    if let Err(e) = fs::remove_file(&temp_file_name) {
        if e.kind() != std::io::ErrorKind::NotFound {
            eprintln!("Failed to remove existing temporary file: {}", e);
            exit(1);
        }
    }

    let temp_file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_file_name)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create temporary file: {}", e);
            exit(1);
        }
    };
    let mut writer = BufWriter::new(temp_file);

    // Perform operation
    let key = password.as_bytes();
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut pos = 0;
    loop {
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read the file: {}", e);
                exit(1);
            }
        };

        // Encrypt or decrypt the buffer based on the operation
        match operation.as_str() {
            "e" | "d" => {
                for byte in buffer.iter_mut().take(bytes_read) {
                    *byte ^= key[pos % key.len()];
                    pos += 1;
                }
            },
            _ => {
                eprintln!("Invalid operation. Use 'e' for encrypt or 'd' for decrypt.");
                exit(1);
            }
        }

        // Write the buffer to the temporary file
        if let Err(e) = writer.write_all(&buffer[..bytes_read]) {
            eprintln!("Failed to write to the temporary file: {}", e);
            exit(1);
        }
    }

    // Ensure the writer is flushed and file handles are closed before renaming
    drop(writer);
    drop(reader);

    // Rename the temporary file to the original file
    if let Err(e) = std::fs::rename(&temp_file_name, file_name) {
        eprintln!("Failed to overwrite the original file: {}", e);
        // Attempt to remove the temporary file
        if let Err(e) = fs::remove_file(&temp_file_name) {
            eprintln!("Failed to remove temporary file: {}", e);
        }
        exit(1);
    }

    // Securely clear the password from memory
    clear_password(&mut password);

    println!("Operation successful. File has been overwritten.");
}

fn clear_password(password: &mut String) {
    unsafe {
        let vec = password.as_mut_vec();
        for byte in vec {
            *byte = 0;
        }
    }
}

