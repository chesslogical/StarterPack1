use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::io;

fn main() -> std::io::Result<()> {
    // Ask the user for a file name
    let mut file_name = String::new();
    println!("Enter the file name:");
    io::stdin().read_line(&mut file_name)?;
    
    // Ask the user for a password
    let mut password = String::new();
    println!("Enter the password:");
    io::stdin().read_line(&mut password)?;
    
    // Trim any whitespace from the input
    file_name = file_name.trim().to_string();
    password = password.trim().to_string();
    
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
    
    Ok(())
}
