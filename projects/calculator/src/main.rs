use std::io;
fn main() {
    println!("enter first oparator ( + , - , * ,/)\n");
    let mut op = String::new();
    io::stdin().read_line(&mut op).unwrap();

    println!("enter first number: \n");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).unwrap();

    println!("enter Second number: \n");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();

    println!("{}{}{}", num1, num2, op);

    let a: f64 = num1.trim().parse().unwrap();
    let b: f64 = num2.trim().parse().unwrap();
    let opr: &str = op.trim();

    println!("after trim");
    println!("{} {} {}",opr, a, b);

    match opr {
        "+"=>println!("Result: {} + {} = {}",a,b, a+b),
        "-"=>println!("Result: {} - {} = {}",a,b, a-b),
        "*"=>println!("Result: {} X {} = {}",a,b, a*b),
        "/"=>println!("Result: {} / {} = {}",a,b, a/b),

        _ => println!("Invalid ope  rator"),
    }
}
