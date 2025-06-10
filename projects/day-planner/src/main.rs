use std::{collections::HashMap, io::{self, Write}, fs};

fn main() {
    let mut planner: HashMap<String, Vec<String>> = HashMap::new();

    println!("Enter date (e.g., 2025-06-07):");
    let mut date = String::new();
    io::stdin().read_line(&mut date).unwrap();
    let date = date.trim().to_string();

    println!("Enter task:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();
    let task = task.trim().to_string();

    planner.entry(date.clone()).or_default().push(task.clone());

    let json = serde_json::to_string_pretty(&planner).unwrap();
    fs::write("planner.json", json).unwrap();

    println!("Added '{}' to {}", task, date);
}
