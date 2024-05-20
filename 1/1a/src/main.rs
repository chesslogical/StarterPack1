use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <E/D> <filename>", args[0]);
        std::process::exit(1);
    }
    let action = &args[1];
    let filename = &args[2];

    match action.as_str() {
        "E" => encrypt_file(filename),
        "D" => decrypt_file(filename),
        _ => {
            eprintln!("Invalid action. Use 'E' for encrypt or 'D' for decrypt.");
            std::process::exit(1);
        }
    }
}

fn encrypt_file(filename: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    for byte in buffer.iter_mut() {
        *byte = byte.wrapping_add(1);
    }

    file.set_len(0)?; // Clear the file
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&buffer)?;
    println!("File encrypted successfully.");

    Ok(())
}

fn decrypt_file(filename: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    for byte in buffer.iter_mut() {
        *byte = byte.wrapping_sub(1);
    }

    file.set_len(0)?; // Clear the file
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&buffer)?;
    println!("File decrypted successfully.");

    Ok(())
}
