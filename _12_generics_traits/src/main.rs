fn main() {
    // println!("Hello, world!");

    display_elements(1, 2);
    display_elements(String::from("sunil"), String::from("reddy"));
    // add_elements(5, 10);
}

fn display_elements<T: std::fmt::Display>(a: T, b: T) {
    println!("{}", a);
    println!("{}", b);
}


