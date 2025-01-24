fn main() {
    println!("Hello, world!");

    struct Point {
        x: i32,
        y: i32,
    }

    let point1 = Point { x: 0, y: 0 };

    match point1 {
        Point { x: 0, y } => println!("{} {x}  __ {y}", point1.x),
        Point { x, y } => println!("{x}{y}"),
    };
}
