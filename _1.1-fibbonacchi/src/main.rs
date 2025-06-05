fn main() {
    let num = fibonacci(10);
    println!("The number is {}", num);
}



fn fibonacci(num: i32) -> i32 {
    if num <= 1 {
        return num;
    }
    let mut a = 0;
    let mut b = 1;
    let mut result = 0;

    for _ in 2..=num {
        result = a + b;
        a = b;
        b = result;
    }
    result
}
