
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::io;

fn main() {
    loop {
        println!("Choose an option:");
        println!("1. Encrypt");
        println!("2. Decrypt");
        println!("3. Exit");

        let choice = read_input_number();

        match choice {
            1 => {
                println!("Enter the name of the file to encrypt:");
                let file_name = read_input_string();

                println!("Enter the name of the key file:");
                let key_file_name = read_input_string();

                encrypt_file(&file_name, &key_file_name);
                println!("File encrypted.");
            }
            2 => {
                println!("Enter the name of the file to decrypt:");
                let file_name = read_input_string();

                println!("Enter the name of the key file:");
                let key_file_name = read_input_string();

                encrypt_file(&file_name, &key_file_name);
                println!("File decrypted.");
            }
            3 => {
                break;
            }
            _ => {
                println!("Invalid choice.");
            }
        }
    }
}

fn read_input_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn read_input_number() -> u32 {
    loop {
        let input = read_input_string();
        match input.parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}

fn encrypt_file(file_name: &str, key_file_name: &str) {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            println!("File not found");
            return;
        }
        Err(_) => {
            println!("Failed to open file");
            return;
        }
    };

    let mut key_file = match File::open(key_file_name) {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            println!("Key file not found");
            return;
        }
        Err(_) => {
            println!("Failed to open key file");
            return;
        }
    };

    let mut buffer = Vec::with_capacity(1024 * 1024); // read file in chunks of 1MB
    let mut key_buffer = Vec::with_capacity(1024 * 1024);

    loop {
        buffer.clear();
        key_buffer.clear();

        let bytes_read = file.read_to_end(&mut buffer).expect("Failed to read file");
        let key_bytes_read = key_file.read_to_end(&mut key_buffer).expect("Failed to read key file");

        if bytes_read == 0 || key_bytes_read == 0 {
            break;
        }

        let mut encrypted_buffer = Vec::with_capacity(bytes_read);
        for (i, byte) in buffer.iter().enumerate() {
            let key_byte = key_buffer.get(i % key_bytes_read).unwrap_or(&0);
            encrypted_buffer.push(byte ^ key_byte);
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_name)
            .expect("Failed to open file for writing");

        file.write_all(&encrypted_buffer).expect("Failed to write to file");
    }
}
