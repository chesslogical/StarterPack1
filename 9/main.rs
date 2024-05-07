use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::iter::repeat_with;
use std::time::SystemTime;

fn main() -> std::io::Result<()> {
    let start_time = SystemTime::now();

    // Create the "files" directory if it doesn't already exist
    let dir_path = PathBuf::from("files");
    fs::create_dir_all(&dir_path)?;

    // Generate and write 10,000 files
    for i in 1..=10000 {
        let file_path = dir_path.join(format!("{}.txt", i));
        let mut file = File::create(file_path)?;

        // Generate a random string of 100 characters
        let random_chars: String = repeat_with(|| {
            let idx = fastrand::usize(..62);
            let c = if idx < 26 {
                (b'a' + idx as u8) as char
            } else if idx < 52 {
                (b'A' + (idx - 26) as u8) as char
            } else {
                (b'0' + (idx - 52) as u8) as char
            };
            c.to_string()
        }).take(100).collect();

        // Write the random string to the file
        file.write_all(random_chars.as_bytes())?;
    }

    let elapsed = SystemTime::now().duration_since(start_time).unwrap();
    println!("Elapsed time: {:?}", elapsed);

    Ok(())
}
