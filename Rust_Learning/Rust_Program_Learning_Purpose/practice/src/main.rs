// use std::any::Any;

// trait Shape {
//     fn area(&self) -> f64;
// }

// // Define the structs for Rectangle, Circle, and Square
// struct Rectangle {
//     width: f64,
//     height: f64,
// }

// struct Circle {
//     radius: f64,
// }

// struct Square {
//     side: f64,
// }

// // Implement the trait for any type that can be 'Any'
// impl Shape for dyn Any {
//     fn area(&self) -> f64 {
//         // Pattern match to figure out which concrete type we're dealing with
//         if let Some(rect) = self.downcast_ref::<Rectangle>() {
//             return rect.width * rect.height;
//         }
//         if let Some(circle) = self.downcast_ref::<Circle>() {
//             return std::f64::consts::PI * circle.radius * circle.radius;
//         }
//         if let Some(square) = self.downcast_ref::<Square>() {
//             return square.side * square.side;
//         }
//         0.0 // If none of the types match, return 0.0
//     }
// }

// fn main() {
//     // Create instances of each shape
//     let rect = Rectangle { width: 5.0, height: 10.0 };
//     let circle = Circle { radius: 7.0 };
//     let square = Square { side: 4.0 };

//     // Create a vector of shapes
//     let shapes: Vec<Box<dyn Any>> = vec![
//         Box::new(rect),
//         Box::new(circle),
//         Box::new(square),
//     ];

//     // Loop over the shapes and print the area of each
//     for shape in shapes {
//         println!("Area: {}", shape.area());
//     }
// }

macro_rules! impl_shape {
    ($t:ty) => {
        impl Shape for $t {
            fn area(&self) -> f64 {
                0.0 // Placeholder area, could be customized
            }
        }
    };
}

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl_shape!(Rectangle);

fn main() {
    let rect = Rectangle { width: 5.0, height: 10.0 };
    println!("Area: {}", rect.area());
}
