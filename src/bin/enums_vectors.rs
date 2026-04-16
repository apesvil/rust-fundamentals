enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64), // base and height
}

impl Shape {
    fn type_name(&self) -> &str {
        match self {
            Shape::Circle(_) => "Circle",
            Shape::Square(_) => "Square",
            Shape::Triangle(_, _) => "Triangle",
        }
    }
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Triangle(base, height) => 0.5 * base * height,
    }
}

fn largest_shape(shapes: &Vec<Shape>) -> usize {
    let mut max_index: usize = 0;
    let mut max_area: f64 = 0.0;
    shapes.iter().enumerate().for_each(|(i, shape)| {
        let area = area(shape);
        if area > max_area {
            max_area = area;
            max_index = i;
        }
    });
    max_index
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(2.0, 4.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        })
        .sum();

    println!("Total area: {:.2} sq. units", total_area);

    let largest_index = largest_shape(&shapes);
    println!("Largest shape is a {} with area: {:.2} sq. units", shapes[largest_index].type_name(), area(&shapes[largest_index]));
}
