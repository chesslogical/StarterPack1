use std::fs;
use glob::glob;

fn main() {
    match get_cpu_temperatures() {
        Ok(temps) => {
            for (i, temp) in temps.iter().enumerate() {
                println!("Core {}: {}°C", i, temp);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn get_cpu_temperatures() -> Result<Vec<f32>, std::io::Error> {
    let mut temps = Vec::new();
    for entry in glob("/sys/class/thermal/thermal_zone*/temp").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let content = fs::read_to_string(path)?;
                let temp = content.trim().parse::<f32>().unwrap_or(0.0) / 1000.0;
                temps.push(temp);
            }
            Err(e) => eprintln!("Error reading thermal zone: {}", e),
        }
    }
    Ok(temps)
}



