extern crate colored;

use colored::*;
use get_shell::*;
use sysinfo::{System, RefreshKind};
use std::process::Command;

fn get_kernel_name() -> Option<String> {
    let result = Command::new("uname")
        .arg("-s").arg("-r").arg("-m").arg("-o").output();

    if let Err(err) = result {
        panic!("Caught an exception: {}", err);
    }

    return Some(String::from_utf8_lossy(&result.unwrap().stdout).to_string().replace("\n", ""))
}

fn get_distribution_name() -> Option<String> {
    let result = Command::new("cat")
        .arg("/etc/os-release").output();

    if let Err(err) = result {
        panic!("Caught an exception: {}", err);
    }

    let binding = String::from_utf8_lossy(&result.unwrap().stdout).to_string();
    let output = binding.split("\n");

    let mut pretty_name: String = "Unknown".to_string();

    for i in output {
        let values: Vec<&str> = i.split("=").collect();

        if values[0] == "PRETTY_NAME" {
            pretty_name = values[1].replace("\n", "").replace("\"", "").to_string(); 
        }
    }

    return Some(pretty_name)
}

fn get_shell() -> Option<String> {
    let shell_name = get_shell_name();

    if let Err(err) = shell_name {
        panic!("Caught an exception {}", err);
    }

    return Some(shell_name.unwrap())
}

fn get_ram_usage(sys: System) -> f32 {
    let total = sys.total_memory();
    let used = sys.used_memory();

    return (used as f32 / total as f32) * 100.0;
}

fn main() {
    control::set_override(true);

    let sys = System::new_with_specifics(RefreshKind::everything());
    let kernel = get_kernel_name().unwrap();
    let distro = get_distribution_name().unwrap();
    let shell = get_shell().unwrap();

    println!("\n{}: {}", "CPU".red().bold(), sys.cpus()[0].brand());
    println!("{}: {}", "Cores".red().bold(), sys.cpus().len());
    println!("{}: {}%", "CPU Usage".red().bold(), sys.global_cpu_info().cpu_usage().round());
    println!("{}: {} MB", "RAM".red().bold(), sys.total_memory() / 1e+6 as u64);
    println!("{}: {}%", "RAM Usage".red().bold(), get_ram_usage(sys) as u64);
    println!("{}: {}", "Distro".red().bold(), distro.white());
    println!("{}: {}", "Shell".red().bold(), shell.white());
    println!("{}: {}\n", "Kernel".red().bold(), kernel.white());
}
