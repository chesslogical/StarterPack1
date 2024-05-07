use std::io;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::process::Command;
use crossterm::{
    terminal::{self, ClearType},
    ExecutableCommand,
};

fn main() -> std::result::Result<(), std::io::Error> {  // Corrected Result type here
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;

    loop {
        terminal::disable_raw_mode()?;
        stdout.execute(terminal::Clear(ClearType::All))?;
        let prompt = "Enter the number of minutes for the countdown:";
        let cols = terminal::size()?.0 as usize;
        let spaces = (cols.saturating_sub(prompt.len())) / 2; // Calculate left padding
        let formatted_prompt = format!("{:spaces$}{}", "", prompt, spaces = spaces);

        println!("{}", formatted_prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        terminal::enable_raw_mode()?;
        
        let minutes: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let total_duration = Duration::from_secs(minutes * 60);
        let start_time = Instant::now();
        let mut last_percent = 0;

        while Instant::now() - start_time < total_duration {
            let elapsed = Instant::now() - start_time;
            let new_percent = (elapsed.as_secs_f64() / total_duration.as_secs_f64() * 10.0).floor() as u64;

            if new_percent > last_percent {
                println!("{}% completed", new_percent * 10);
                last_percent = new_percent;
            }

            sleep(Duration::from_secs(1));
        }

        play_sound_times(5); // Play sound five times

        println!("Time's up!");
    }
}

fn play_sound_times(count: u32) {
    for _ in 0..count {
        let _ = Command::new("cmd")
            .args(&["/C", "rundll32 user32.dll,MessageBeep"])
            .status();
        sleep(Duration::from_millis(500));
    }
}

