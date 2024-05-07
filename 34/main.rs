use std::io::{self, BufRead};

fn main() {
    println!("\x1b[48;2;255;0;0m\x1b[38;2;255;255;255m This is a red text with white background \x1b[0m");
    println!("\x1b[48;2;0;255;0m\x1b[38;2;0;0;0m This is a green text with black background \x1b[0m");
    println!("\x1b[48;2;0;0;255m\x1b[38;2;255;255;255m This is a blue text with white background \x1b[0m");

    println!("\nPress Enter to exit...");
    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();
}
