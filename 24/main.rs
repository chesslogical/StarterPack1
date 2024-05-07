use sys_info;
use os_info;
use std::env;

fn main() {
    match gather_system_info() {
        Ok(()) => println!("System information gathered successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn gather_system_info() -> Result<(), sys_info::Error> {
    let os_info = os_info::get();
    
    println!("System Information:");
    println!("-------------------");
    println!("OS Type: {}", os_info.os_type());
    println!("OS Version: {}", os_info.version());
    println!("CPU Cores: {}", sys_info::cpu_num()?);
    println!("CPU Speed: {} MHz", sys_info::cpu_speed()?);
    
    let mem_info = sys_info::mem_info()?;
    println!("Total RAM: {} MB", mem_info.total);
    println!("Free RAM: {} MB", mem_info.free);
    
    let load_avg = sys_info::loadavg()?;
    println!("Load Average: {:.2}, {:.2}, {:.2}", load_avg.one, load_avg.five, load_avg.fifteen);
    
    println!("Architecture: {}", env::consts::ARCH);

    Ok(())
}

