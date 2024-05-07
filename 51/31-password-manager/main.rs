use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    name: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PasswordManager {
    entries: Vec<Entry>,
}

impl PasswordManager {
    fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    fn save_to_file(&self, path: PathBuf) -> Result<(), io::Error> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load_from_file(path: PathBuf) -> Result<PasswordManager, io::Error> {
        let content = fs::read_to_string(path)?;
        let manager: PasswordManager = serde_json::from_str(&content)?;
        Ok(manager)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = PasswordManager { entries: Vec::new() };

    loop {
        println!("1: Add entry");
        println!("2: List entries");
        println!("3: Save");
        println!("4: Load");
        println!("5: Quit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim().parse::<u32>()?;

        match choice {
            1 => {
                println!("Enter name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name)?;

                println!("Enter username:");
                let mut username = String::new();
                io::stdin().read_line(&mut username)?;

                println!("Enter password:");
                let password = read_password()?;

                manager.add_entry(Entry {
                    name: name.trim().to_string(),
                    username: username.trim().to_string(),
                    password: password.trim().to_string(),
                });
            }
            2 => {
                for entry in &manager.entries {
                    println!("{:?}", entry);
                }
            }
            3 => {
                let home_dir = dirs::home_dir().expect("Could not find home directory");
                let path = home_dir.join(".password_manager.json");
                manager.save_to_file(path)?;
            }
            4 => {
                let home_dir = dirs::home_dir().expect("Could not find home directory");
                let path = home_dir.join(".password_manager.json");
                manager = PasswordManager::load_from_file(path)?;
            }
            5 => break,
            _ => println!("Invalid choice"),
        }
    }

    Ok(())
}

