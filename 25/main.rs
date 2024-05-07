use std::io::{self, Write};
use tiny_keccak::{Hasher, Sha3};
use hex;

const HASH_OUTPUT_SIZE: usize = 64;

fn read_input(prompt: &str) -> Result<String, std::io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?; // Flush stdout to ensure prompt is displayed before input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn validate_input(input: &str) -> Result<(), &'static str> {
    if input.trim().is_empty() {
        return Err("Input is empty. Please enter a word or number.");
    }
    // Add additional input validation rules if needed
    Ok(())
}

fn calculate_sha3_512(input: &str) -> [u8; HASH_OUTPUT_SIZE] {
    let mut hasher = Sha3::v512();
    let mut output = [0u8; HASH_OUTPUT_SIZE];
    hasher.update(input.as_bytes());
    hasher.finalize(&mut output);
    output
}

fn main() {
    println!("SHA3-512 Hash Calculator");

    loop {
        match read_input("Enter a word or number to hash, or 'q' to quit: ") {
            Ok(input) => {
                if input == "q" {
                    println!("Goodbye!");
                    break;
                }

                match validate_input(&input) {
                    Ok(_) => {
                        let output = calculate_sha3_512(&input);

                        let output_hex = hex::encode(output);
                        println!("SHA3-512 Hash: {}", output_hex);
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}
