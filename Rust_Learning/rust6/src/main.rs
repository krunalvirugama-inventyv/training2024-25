// RUST6
// enum Shape{
//   Circle,
//   Rectangle,
// }

// fn print_shape(shape : Shape){
//     println!("print Shape ");
// }

// fn main() {
//     let my_shape = Shape::Rectangle;
//     print_shape(my_shape);
//     print_shape(Shape::Circle);
// }




// Value type fix with enum shape
enum Shape{
    Circle(f64),
    Rectangle(f64,f64), 
    Square(f64),    
}

fn calculate_area(shape : Shape) -> f64{
    match shape
    {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(w,h) => w * h,
        Shape::Square(s) => s * s,
    }
}

fn main(){
  
    let rect = Shape::Rectangle(10.0,20.0);
    let circle = Shape::Circle(10.0);
    let squre = Shape::Square(10.0);

    println!("Area of Rectangle : {}", calculate_area(rect));
    println!("Area of Rectangle : {}", calculate_area(circle));
    println!("Area of Rectangle : {}", calculate_area(squre));
}
