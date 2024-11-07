enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main() {
    let my_shape_rect = Shape::Rectangle(3.0, 13.0);
    let final_value_rect = calculate_area(my_shape_rect);
    println!("{}", final_value_rect);
    let my_shape_circle = Shape::Circle(3.0);
    let final_value_circle = calculate_area(my_shape_circle);

    println!("{}", final_value_circle);
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}
