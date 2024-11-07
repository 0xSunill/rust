use std::{fs::read_to_string};

fn main() {
    let result = read_to_string("copy.txt");
    match result {
        Ok(data) => println!("{}", data),
        Err(e) => println!("Error reading file: {}", e),
    }
}
