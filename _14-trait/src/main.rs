use std::f32::consts::PI;

trait Shape {
    fn area(&self) -> f32;
}

struct Rect {
    height: f32,
    width: f32,
}

struct Circle {
    radius: f32,
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        return self.height * self.width;
    }
}
impl Shape for Circle {
    fn area(&self) -> f32 {
        return PI * self.radius * self.radius;
    }
}

fn print_area<T: Shape>(s: T) {
    println!("{}", s.area())
}

fn main() {
    let r = Rect {
        height: 12.5,
        width: 34.5,
    };

    let c = Circle { radius: 23.5 };
    print_area(r);
    print_area(c);
}
