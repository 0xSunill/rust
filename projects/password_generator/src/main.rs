use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    println!("Generated password: {}", password);
}
