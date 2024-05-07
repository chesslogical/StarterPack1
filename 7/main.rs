
use rand::Rng;
use std::env;
use std::fs;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <num_files> <bytes_per_file>", args[0]);
        std::process::exit(1);
    }

    let num_files = args[1].parse().expect("Invalid input for num_files");
    let bytes_per_file = args[2].parse().expect("Invalid input for bytes_per_file");

    let output_dir = std::env::current_exe()
        .expect("Could not determine executable path")
        .parent()
        .expect("Could not determine parent directory")
        .join("out");

    fs::create_dir_all(&output_dir).expect("Could not create output directory");

    let mut rng = rand::thread_rng();

    for i in 0..num_files {
        let file_name = format!("file{}.txt", i);
        let file_path = output_dir.join(&file_name);

        let file = fs::File::create(file_path).expect("Could not create file");
        let mut file_writer = BufWriter::new(file);

        let data: Vec<u8> = (0..bytes_per_file).map(|_| rng.gen::<u8>()).collect();

        file_writer.write(&data).expect("Could not write to file");
    }
}