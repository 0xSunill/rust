use std::io;
fn main() {
    let mut input = String::new();
    println!("enter a number \n");
    io::stdin().read_line(&mut input).unwrap();

    let number: i32 = input.trim().parse().unwrap();

    if number % 2 == 0 {
        println!("{} even", number);
    } else {
        println!("{} odd", number);
    }
}
