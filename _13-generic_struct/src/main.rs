#[derive(Copy, Clone)]
struct Rect<T> {
    height: T,
    weight: T,
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        return self.height * self.weight;
    }
}
// fn Print<T: std::fmt::Display>()

fn main() {
    let r = Rect {
        height: 5,
        weight: 6,
    };
    let r2 = Rect {
        height: 5.5,
        weight: 6.3,
    };

    println!("{}", r.area());
    println!("{}", r2.area());

}
