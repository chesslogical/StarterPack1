
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::io::ErrorKind;

fn main() {
    loop {
        println!("Select an option:");
        println!("1. Encrypt");
        println!("2. Decrypt");
        println!("3. Exit");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => {
                let file_name = read_input("Enter file name to encrypt: ");
                let key_file_name = read_input("Enter key file name: ");
                encrypt_file(&file_name, &key_file_name);
            }
            2 => {
                let file_name = read_input("Enter file name to decrypt: ");
                let key_file_name = read_input("Enter key file name: ");
                decrypt_file(&file_name, &key_file_name);
            }
            3 => break,
            _ => continue,
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
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

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let mut key_buffer = Vec::new();
    key_file.read_to_end(&mut key_buffer).expect("Failed to read key file");

    for (i, byte) in buffer.iter_mut().enumerate() {
        let key_byte = key_buffer.get(i);
        match key_byte {
            Some(k) => *byte = *byte ^ *k,
            None => break,
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)
        .expect("Failed to open file for writing");

    file.write_all(&buffer).expect("Failed to write to file");
}

fn decrypt_file(file_name: &str, key_file_name: &str) {
    encrypt_file(file_name, key_file_name);
}
