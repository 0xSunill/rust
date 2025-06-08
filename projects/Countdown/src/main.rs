use std::{thread, time::Duration};

fn main() {
    let seconds = 10;

    for i in (1..=seconds).rev() {
        println!("{}...", i);
        thread::sleep(Duration::from_secs(1));
    }

    println!("Time's up!");
}
