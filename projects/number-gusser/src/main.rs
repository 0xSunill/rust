use rand::Rng;
use std::io;
fn main() {
    let rand_num = rand::rng().random_range(1..=5);
    loop {
        println!("Guess a nunber between 1 to 5:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let gussed_num :i32= match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if rand_num > gussed_num {
            println!("gussed number is lower then the random number \n");
        } else if rand_num < gussed_num {
            println!("gussed number is heigher then the random number\n");
        } else {
            println!("you got it , that is the random number");
            break;
        }
    }
}
