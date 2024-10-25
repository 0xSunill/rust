struct Rect {
    width: i32,
    heigh: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.heigh
    }
}
fn main() {
    let rect = Rect {
        width: 40,
        heigh: 20,
    };

    println!("area of the rectangle is {}", rect.area());
}
