
use std::fs::{self, File};
use std::io::{self, Read, Write, BufReader, BufWriter, Error, ErrorKind};
use std::path::PathBuf;
use std::time::Instant;
use zeroize::Zeroize;

fn main() -> Result<(), Error> {
    // Ask the user for a file name
    println!("Enter the file name:");
    let file_name = read_input_line()?;

    // Ask the user for a password
    println!("Enter the password:");
    let password = read_input_line()?;

    // Trim any whitespace from the input
    let file_name = file_name.trim();
    let password = password.trim();

    // Check if the file exists and its size
    let file_size = fs::metadata(PathBuf::from(&file_name)).map(|m| m.len()).or_else(|_| Err(Error::new(ErrorKind::NotFound, "File not found")))?;

    // Read the file contents in chunks
    let mut file = BufReader::new(File::open(PathBuf::from(&file_name))?);
    let mut contents = Vec::with_capacity(file_size as usize);
    file.read_to_end(&mut contents)?;

    // Encrypt or decrypt the file contents
    let mut encrypted = Vec::with_capacity(contents.len());
    for (i, byte) in contents.iter().enumerate() {
        encrypted.push(byte ^ password.as_bytes()[i % password.len()]);
    }

    // Write the encrypted or decrypted contents back to the file in chunks
    let mut file = BufWriter::new(File::create(PathBuf::from(&file_name))?);
    file.write_all(&encrypted)?;

    // Zeroize sensitive data
    let mut password_bytes = password.as_bytes().to_vec();
    password_bytes.zeroize();
    contents.zeroize();
    encrypted.zeroize();

    // Print the time taken for the operation
    let duration = Instant::now().elapsed();
    println!("Operation completed in {:.2} seconds.", duration.as_secs_f64());

    Ok(())
}

fn read_input_line() -> Result<String, Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
