use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <algorithm_number> <input_string>", args[0]);
        println!("1 - SHA3-224");
        println!("2 - SHA3-256");
        println!("3 - SHA3-384");
        println!("4 - SHA3-512");
        return;
    }

    let algorithm = &args[1];
    let input = &args[2];

    let hash_result = match algorithm.as_str() {
        "1" => format!("{:x}", Sha3_224::digest(input.as_bytes())),
        "2" => format!("{:x}", Sha3_256::digest(input.as_bytes())),
        "3" => format!("{:x}", Sha3_384::digest(input.as_bytes())),
        "4" => format!("{:x}", Sha3_512::digest(input.as_bytes())),
        _ => {
            println!("Invalid algorithm number. Choose 1, 2, 3, or 4.");
            return;
        }
    };

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("a.txt")
        .expect("Unable to open file");

    file.write_all(hash_result.as_bytes())
        .expect("Unable to write data to file");

    println!("Hash written to a.txt");
}

