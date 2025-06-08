use std::collections::HashMap;
fn main() {
    let mut users = HashMap::new();
    users.insert(String::from("sunil"), 21);
    users.insert(String::from("sunidhi"), 20);
    
    if let Some(get_roll_number) = users.get("sunil") {
        println!("Roll number for Sunil is: {}", get_roll_number);
    } else {
        println!("User not found.");
    }
}
