fn main() {
    let num = 5;
    let fac = fact(num);
    println!("factorial  of {} is {} ", num, fac)
}

fn fact(n: u32) -> u32 {
    if n == 0 {
        return 1;
    } else {
        n * fact(n - 1)
    }
}
