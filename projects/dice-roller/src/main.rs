use rand::Rng;

fn main() {
    let roll = rand::rng().gen_range(1..=6);
    println!("🎲 You rolled: {}", roll);
}
