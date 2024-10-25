fn main() {
    let my_str = String::from("hello sunil");
    let get_len = get_string_length(&my_str);
    println!("lenth of the string is {}", get_len);
}

fn get_string_length(s: &str) -> usize {
    return s.chars().count();
}
