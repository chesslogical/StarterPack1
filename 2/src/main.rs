
use clap::{Arg, Command};
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use winapi::um::winnt::FILE_ATTRIBUTE_READONLY;
use winapi::um::fileapi::{GetFileAttributesW, SetFileAttributesW};
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use anyhow::{Context, Result};
use zeroize::Zeroize;

fn main() -> Result<()> {
    let matches = Command::new("Encryption/Decryption App")
        .version("0.1.0")
        .author("Your Name")
        .about("Encrypts or decrypts files using a repeating key XOR method. Files are given a .enc extension upon encryption.")
        .arg(Arg::new("file")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .index(1)
            .help("Path to the input file"))
        .arg(Arg::new("key")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .index(2)
            .help("Path to the key file"))
        .get_matches();

    let input_filename = matches.get_one::<String>("file").expect("Input file required");
    let key_filename = matches.get_one::<String>("key").expect("Key file required");

    // Determine if we are encrypting or decrypting based on the file extension
    let encrypting = !input_filename.ends_with(".enc");
    let output_filename = if encrypting {
        format!("{}.enc", input_filename)
    } else {
        input_filename.trim_end_matches(".enc").to_string()
    };

    process_file(input_filename, &output_filename, key_filename, 1024, encrypting)
}

fn process_file(input_filename: &str, output_filename: &str, key_filename: &str, chunk_size: usize, encrypting: bool) -> Result<()> {
    // Check file permissions and existence before processing
    check_file_permissions(input_filename, key_filename, output_filename, encrypting)?;

    // Open the key file and read its contents
    let key_file = File::open(key_filename)
        .with_context(|| format!("Failed to open key file '{}'", key_filename))?;
    let mut key_reader = BufReader::new(key_file);

    let mut key_buffer = Vec::new();
    key_reader.read_to_end(&mut key_buffer)
        .with_context(|| "Failed to read the key file.")?;

    if key_buffer.is_empty() {
        return Err(anyhow::anyhow!("Key file cannot be empty."));
    }

    let input_file = File::open(input_filename)
        .with_context(|| format!("Failed to open input file '{}'", input_filename))?;
    let mut reader = BufReader::new(&input_file);

    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_filename)
        .with_context(|| format!("Failed to create output file '{}'", output_filename))?;
    let mut writer = BufWriter::new(output_file);

    let mut buffer = vec![0; chunk_size];
    let mut key_pos = 0;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        buffer.truncate(bytes_read);

        for i in 0..bytes_read {
            buffer[i] ^= key_buffer[key_pos];
            key_pos = (key_pos + 1) % key_buffer.len();
        }

        writer.write_all(&buffer)
            .with_context(|| format!("Failed to write to output file '{}'", output_filename))?;
    }

    writer.flush().context("Failed to flush writer")?;
    key_buffer.zeroize();

    // Set the output file as read-only if encrypting
    if encrypting {
        set_read_only(output_filename)
            .with_context(|| format!("Failed to set read-only permissions on output file '{}'", output_filename))?;
    }

    println!("Operation completed successfully. File '{}' has been processed and saved as '{}'.", input_filename, output_filename);

    Ok(())
}

fn check_file_permissions(input_filename: &str, key_filename: &str, output_filename: &str, encrypting: bool) -> Result<()> {
    // Check if input file exists and is readable
    let input_metadata = fs::metadata(input_filename)
        .with_context(|| format!("Input file '{}' does not exist or is not accessible", input_filename))?;
    if !input_metadata.is_file() {
        return Err(anyhow::anyhow!("Input path '{}' is not a file", input_filename));
    }
    File::open(input_filename)
        .with_context(|| format!("Cannot read input file '{}'", input_filename))?;

    // Check if key file exists and is readable
    let key_metadata = fs::metadata(key_filename)
        .with_context(|| format!("Key file '{}' does not exist or is not accessible", key_filename))?;
    if !key_metadata.is_file() {
        return Err(anyhow::anyhow!("Key path '{}' is not a file", key_filename));
    }
    File::open(key_filename)
        .with_context(|| format!("Cannot read key file '{}'", key_filename))?;

    // Check if we can create or overwrite the output file
    if fs::metadata(output_filename).is_ok() && encrypting {
        return Err(anyhow::anyhow!("Output file '{}' already exists. Cannot overwrite an existing file during encryption.", output_filename));
    }

    // Check if we have write permissions in the directory of the output file
    let output_path = Path::new(output_filename);
    let dir = output_path.parent()
        .ok_or_else(|| anyhow::anyhow!("Could not determine the directory of output file '{}'", output_filename))?;
    let test_file_path = dir.join(".test_write_access");
    if File::create(&test_file_path).is_err() {
        return Err(anyhow::anyhow!("No write permission in the directory of output file '{}'", output_filename));
    }
    // Clean up the test file
    let _ = fs::remove_file(test_file_path);

    Ok(())
}

fn set_read_only(filename: &str) -> std::io::Result<()> {
    let path = Path::new(filename);
    let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
    let attrs = unsafe { GetFileAttributesW(path_wide.as_ptr()) };

    if attrs == u32::MAX {
        return Err(std::io::Error::last_os_error());
    }

    let new_attrs = attrs | FILE_ATTRIBUTE_READONLY;

    let success = unsafe { SetFileAttributesW(path_wide.as_ptr(), new_attrs) };

    if success == 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}
