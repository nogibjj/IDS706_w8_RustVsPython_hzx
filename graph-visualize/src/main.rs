// extern crate rasciigraph;

mod lib; // Import the main module
use lib::vis; // 导入 my_function 函数

// use rasciigraph::{plot, Config};
use std::time::Instant;
use std::process::Command;

// cargo run --manifest-path graph-visualize/Cargo.toml

fn main() {
    let start_time = Instant::now();
    vis();
    let end_time = Instant::now();
    let duration_time = end_time - start_time; // Calculate the elapsed time
    // Convert duration to seconds
    // let duration_secs = duration_time.as_secs();
    println!("Total execution time: {:?}", duration_time); // Print the elapsed time

    // Memory usage
    let mem_info = sys_info::mem_info().unwrap();
    println!(
        "Memory Usage: {}%",
        (mem_info.total - mem_info.avail) as f32 / mem_info.total as f32 * 100.0
    );
    // CPU calculation
    let output = Command::new("ps")
        .arg("-o")
        .arg("%cpu")
        .arg("-p")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute ps command");

    // Convert the output to a string
    let usage = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = usage.split('\n').collect();

    // Parse the CPU usage from the output
    if lines.len() >= 2 {
        let usage_str = lines[1].trim();
        let usage_float: Result<f32, _> = usage_str.parse();
        match usage_float {
            Ok(usage) => println!("CPU Usage: {:.2}%", usage),
            Err(_) => println!("Failed to parse CPU usage"),
        }
    } else {
        println!("Failed to get CPU usage");
    }
}
