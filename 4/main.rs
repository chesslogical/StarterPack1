use std::io::{self, BufRead};

fn main() {
    println!("\x1b[38;2;255;0;0mThis is a bright red text\x1b[0m");
    println!("\x1b[38;2;200;0;0mThis is a dark red text\x1b[0m");
    println!("\x1b[38;2;0;255;0mThis is a bright green text\x1b[0m");
    println!("\x1b[38;2;0;200;0mThis is a dark green text\x1b[0m");
    println!("\x1b[38;2;0;0;255mThis is a bright blue text\x1b[0m");
    println!("\x1b[38;2;0;0;200mThis is a dark blue text\x1b[0m");
    println!("\x1b[38;2;255;255;0mThis is a bright yellow text\x1b[0m");
    println!("\x1b[38;2;200;200;0mThis is a dark yellow text\x1b[0m");

    println!("\x1b[38;2;255;127;80mThis is a coral text\x1b[0m");
    println!("\x1b[38;2;255;20;147mThis is a deep pink text\x1b[0m");
    println!("\x1b[38;2;128;0;128mThis is a purple text\x1b[0m");
    println!("\x1b[38;2;218;165;32mThis is a goldenrod text\x1b[0m");
    println!("\x1b[38;2;255;215;0mThis is a gold text\x1b[0m");

    println!("\x1b[1mThis is a bold text\x1b[0m");
    println!("\x1b[3mThis is an italic text\x1b[0m");
    println!("\x1b[4mThis is an underlined text\x1b[0m");
    println!("\x1b[7mThis is a reverse video text\x1b[0m");

    println!("\nPress Enter to exit...");
    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();
}
