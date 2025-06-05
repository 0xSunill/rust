fn main() {
    let mut str = String::from("Hello, world! ");
    let str2 = &mut str;
    str2.push_str("sunil reddy");
    let str3 = &str;
    println!("{}", str3)
}
