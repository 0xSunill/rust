use std::time::Instant;
use std::io;

fn main() {
    println!("Press ENTER to start the timer...");
    let _ = io::stdin().read_line(&mut String::new());

    let start = Instant::now();

    println!("Press ENTER to stop the timer...");
    let _ = io::stdin().read_line(&mut String::new());

    let duration = start.elapsed();
    println!("Time elapsed: {:.2?}", duration);
}
