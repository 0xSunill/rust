fn main() {
    let my_string = String::from("mosin");
    match find_s(my_string) {
        Some(index) => println!("the letter 's' is found at index:{}", index),
        None => println!("'s' not found"),
    }
}

fn find_s(st: String) -> Option<i32> {
    for (index, charcater) in st.chars().enumerate() {
        if charcater == 's' {
            return Some(index as i32);
        }
    }
    return None;
}
