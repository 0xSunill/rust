use std::io;
fn main() {
    let mut name = String::new();
    println!("enter you name");
    io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}", name.trim());
}
