use std::fs;
use std::io::{self, Write};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let mut extension = String::new();
    println!("Enter the file extension (without the dot): ");
    io::stdin().read_line(&mut extension)?;
    let extension = extension.trim().to_lowercase();

    // Call the user confirmation prompt
    if !prompt_user_confirmation() {
        println!("Operation canceled by user.");
        return Ok(());
    }

    let current_dir = std::env::current_dir()?;
    let mut count = 1;
    let mut processed_files = 0;

    for entry in WalkDir::new(&current_dir).max_depth(1).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext.to_str().unwrap_or("").to_lowercase() == extension {
                while current_dir.join(format!("{}.{}", count, extension)).exists() {
                    count += 1;
                }
                
                let new_filename = format!("{}.{}", count, extension);
                let new_path = current_dir.join(new_filename);

                if path != new_path {
                    match fs::rename(path, &new_path) {
                        Ok(_) => {
                            println!("Renamed {:?} to {:?}", path.file_name().unwrap(), new_path.file_name().unwrap());
                            count += 1;
                            processed_files += 1;
                        },
                        Err(e) => {
                            eprintln!("Failed to rename {:?}: {}", path.file_name().unwrap(), e);
                        }
                    }
                }
            }
        }
    }

    println!("Files processed: {}", processed_files);
    println!("Press 'z' to exit.");

    let mut exit_key = String::new();
    while exit_key.trim() != "z" {
        exit_key.clear();
        io::stdin().read_line(&mut exit_key)?;
    }

    Ok(())
}

fn prompt_user_confirmation() -> bool {
    let mut response = String::new();
    print!("Are you sure you want to proceed with renaming? (y/n): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().to_lowercase() == "y"
}
