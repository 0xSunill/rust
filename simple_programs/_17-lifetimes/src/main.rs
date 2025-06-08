fn main() {
    let str1 = String::from("sunil");
    let ans;

    {
        let str2 = String::from("Red");
        ans = find_len(&str1, &str2);
        println!("largest string is {}", ans);
    };
}
fn find_len<'a>(str1: &'a String, str2: &'a String) -> &'a String {
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}
