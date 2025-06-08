#[derive(Debug)] //macro 
struct User {
    name: String,
}

fn main() {
    let u = User {
        name: String::from("Sunil Reddy"),
    };
    println!("{:?} {:?}", u.name, u)
}
