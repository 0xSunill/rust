struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let user = User {
        first_name: String::from("Sunil"),
        last_name: String::from("Reddy"),
        age: 21,
    };

    println!("name is {} {} ", user.first_name, user.last_name);
    println!("and the age is {}", user.age);
}
