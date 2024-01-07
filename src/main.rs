extern crate colored;

// ======== IMPORTS START HERE ========
use colored::*;
use get_shell::*;
use sysinfo::{System, RefreshKind};
use std::process::Command;
// ========  IMPORTS END HERE  ========

// =================================================
// Here we're getting kernel's name
// by executing `uname -s -r -m -o`
// in the shell.
//
// @return std::option::Option<std::string::String>
// =================================================
fn get_kernel_name() -> Option<String> {
    let result = Command::new("uname")
        .arg("-s").arg("-r").arg("-m").arg("-o").output();

    // Handling any potential errors by panicking
    if let Err(err) = result {
        panic!("Caught an exception: {}", err);
    }

    return Some(String::from_utf8_lossy(&result.unwrap().stdout).to_string().replace("\n", ""))
}

// =================================================
// Here we're getting the Linux
// distribution's name by executing
// `cat /etc/os-release` in the shell.
//
// @return std::option::Option<std::string::String>
// =================================================
fn get_distribution_name() -> Option<String> {
    let result = Command::new("cat")
        .arg("/etc/os-release").output();

    // Handing any potential errors by panicking
    if let Err(err) = result {
        panic!("Caught an exception: {}", err);
    }

    // Creating a binding because Rust drops this value afterwards
    let binding = String::from_utf8_lossy(&result.unwrap().stdout).to_string();
    let output = binding.split("\n");

    let mut pretty_name: String = "Unknown".to_string();

    // Processing the output
    // Output example:
    //
    // VALUE1="value"
    // VALUE2="another value"
    // 
    // Where: 
    // VALUE1 is `values[0]`
    // "?"    is `values[1]`
    for i in output {
        let values: Vec<&str> = i.split("=").collect();

        if values[0] == "PRETTY_NAME" {
            pretty_name = values[1].replace("\n", "").replace("\"", "").to_string(); 
        }
    }

    return Some(pretty_name)
}

// =================================================
// Getting the shell in which
// the `fetch` binary was
// executed.
//
// @return std::option::Option<std::string::String>
// =================================================
fn get_shell() -> Option<String> {
    let shell_name = get_shell_name();

    // Handling any potential errors by panicking
    if let Err(err) = shell_name {
        panic!("Caught an exception {}", err);
    }

    return Some(shell_name.unwrap())
}

// ===========================================
// Getting RAM usage percentage by
// getting `total` and `used` amounts
// and processing them through a mathemetical
// formula: (used / total) * 100
//
// @return f32
// ===========================================
fn get_ram_usage(sys: System) -> f32 {
    let total = sys.total_memory();
    let used = sys.used_memory();

    return (used as f32 / total as f32) * 100.0;
}

// Main function
fn main() {
    // Telling the library to always print out colors
    // do not respect the `no-color` value.
    control::set_override(true);

    // [VARIABLES BEGIN]
    let sys = System::new_with_specifics(RefreshKind::everything());
    let kernel = get_kernel_name().unwrap();
    let distro = get_distribution_name().unwrap();
    let shell = get_shell().unwrap();
    // [VARIABLES END]

    // [PRINTS BEGIN]
    println!("\n{}: {}", "CPU".red().bold(), sys.cpus()[0].brand());
    println!("{}: {}", "Cores".red().bold(), sys.cpus().len());
    println!("{}: {}%", "CPU Usage".red().bold(), sys.global_cpu_info().cpu_usage().round());
    println!("{}: {} MB", "RAM".red().bold(), sys.total_memory() / 1e+6 as u64); // because
                                                                                 // `sys.total_memory()`
                                                                                 // returns value
                                                                                 // in bytes, we
                                                                                 // convert it into
                                                                                 // megabytes by
                                                                                 // dividing it by
                                                                                 // 1e+6 and
                                                                                 // converting it
                                                                                 // to u64.
    println!("{}: {}%", "RAM Usage".red().bold(), get_ram_usage(sys) as u64); // here we convert it
                                                                              // to u64 because we
                                                                              // don't want any
                                                                              // remainders.
    println!("{}: {}", "Distro".red().bold(), distro);
    println!("{}: {}", "Shell".red().bold(), shell);
    println!("{}: {}\n", "Kernel".red().bold(), kernel);
    // [PRINTS END]
}

// [TESTS BEGIN HERE]
#[cfg(test)]
mod tests {
    use std::process::Command;
    use get_shell::get_shell_name;

    #[test]
    fn check_kernel_name() {
        let result = Command::new("uname")
            .arg("-s").arg("-r").arg("-m").arg("-o").output();

        if let Err(err) = result {
            panic!("Command doesn't execute: {}", err)
        } 
    }

    #[test]
    fn check_distribution_name() {
        let result = Command::new("cat")
            .arg("/etc/os-release").output();

        if let Err(err) = result {
            panic!("Command doesn't execute: {}", err);
        }
    }

    #[test]
    fn check_shell_name() {
        let shell_name = get_shell_name();

        if let Err(err) = shell_name {
            panic!("Unable to get the shell: {}", err);
        }
    }
}
// [TESTS END HERE]
