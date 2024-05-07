
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::io::{self, BufRead};
use std::time::Instant;
use zeroize::Zeroize;

fn main() -> std::io::Result<()> {
    // Ask the user for a file name
    println!("Enter the file name:");
    let file_name = read_input_line()?;

    // Ask the user for a password
    println!("Enter the password:");
    let password = read_input_line()?;

    // Trim any whitespace from the input
    let file_name = file_name.trim();
    let password = password.trim();

    // Read the file contents
    let mut file = File::open(PathBuf::from(&file_name))?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Encrypt or decrypt the file contents
    let mut encrypted = Vec::new();
    for (i, byte) in contents.iter().enumerate() {
        encrypted.push(byte ^ password.as_bytes()[i % password.len()]);
    }

    // Write the encrypted or decrypted contents back to the file
    let mut file = File::create(PathBuf::from(&file_name))?;
    file.write_all(&encrypted)?;

    // Zeroize sensitive data
    file_name.to_owned().zeroize();
    password.to_owned().zeroize();
    for byte in contents.iter_mut() {
        byte.zeroize();
    }
    for byte in encrypted.iter_mut() {
        byte.zeroize();
    }

    // Print the time taken for the operation
    let duration = Instant::now().elapsed();
    println!("Operation completed in {:.2} seconds.", duration.as_secs_f64());

    // Prompt the user to hit any key to exit the program
    println!("Press any key to exit...");
    let _ = read_input_char()?;

    Ok(())
}

fn read_input_line() -> io::Result<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    handle.read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn read_input_char() -> io::Result<char> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    handle.read_line(&mut buffer)?;
    Ok(buffer.trim().chars().next().unwrap())
}
