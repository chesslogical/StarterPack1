use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    // get the file name from the command line argument
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    // open the file
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);

    // count the occurrences of each extended ASCII character
    let mut char_counts = [0u64; 256];
    let mut buf = [0u8; 1024];
    loop {
        let num_bytes = buf_reader.read(&mut buf)?;
        if num_bytes == 0 {
            break;
        }
        for byte in &buf[0..num_bytes] {
            char_counts[*byte as usize] += 1;
        }
    }

    // write the report to a file
    let mut report_file = File::create("report.txt")?;
    for i in 0..256 {
        writeln!(
            report_file,
            "Character {}: {} occurrences",
            i,
            char_counts[i]
        )?;
    }

    Ok(())
}