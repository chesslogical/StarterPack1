use sha3::{Digest, Sha3_512};
use std::io::{self, Write};
use base64::{engine::general_purpose, Engine as _};
use zeroize::Zeroize;

const PASSWORD_MIN_LENGTH: usize = 8;
const PASSWORD_MAX_LENGTH: usize = 15;
const SYMBOLS: &[char] = &['!', '#'];

fn main() {
    loop {
        // Prompt the user for input
        print!("Enter a string to generate a password (or type 'exit' to quit): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        // Read the input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Exit the loop if the user types 'exit'
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // Generate the password
        let password = generate_password(input);
        println!("Generated password: {}", password);

        // Zeroize the password after displaying it
        let mut password_vec: Vec<char> = password.chars().collect();
        password_vec.zeroize();
    }
}

fn generate_password(input: &str) -> String {
    // Hash the input string using SHA3-512
    let mut hasher = Sha3_512::new();
    hasher.update(input);
    let hash_result = hasher.finalize();

    // Convert the hash result to a base64 string
    let hash_string = general_purpose::STANDARD.encode(hash_result);

    // Create character sets
    let mut password_chars: Vec<char> = vec![];

    // Add a symbol
    password_chars.push(SYMBOLS[(hash_string.as_bytes()[0] as usize) % SYMBOLS.len()]);

    // Add characters from the hash string
    for c in hash_string.chars() {
        if password_chars.len() >= PASSWORD_MAX_LENGTH {
            break;
        }
        if c.is_alphanumeric() {
            password_chars.push(c);
        }
    }

    // Ensure the password has at least one uppercase, one lowercase, and one digit
    let has_upper = password_chars.iter().any(|c| c.is_uppercase());
    let has_lower = password_chars.iter().any(|c| c.is_lowercase());
    let has_digit = password_chars.iter().any(|c| c.is_digit(10));

    // Insert missing character types if necessary
    if !has_upper {
        password_chars.insert(1, 'A');
    }
    if !has_lower {
        password_chars.insert(2, 'a');
    }
    if !has_digit {
        password_chars.insert(3, '0');
    }

    // Trim the password to the desired length range
    let length = PASSWORD_MIN_LENGTH + (hash_string.as_bytes()[1] as usize % (PASSWORD_MAX_LENGTH - PASSWORD_MIN_LENGTH + 1));
    password_chars.truncate(length);

    password_chars.into_iter().collect()
}
