// (1) Solution using lifetime
// fn main() {
//     let list1 = vec![1, 2, 4, 0, 20, 3];

//     let result1 = largest_find(&list1);

//     println!("Result 1 = {result1}");
// }

// fn largest_find<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// (2)Struct

// #[derive(Debug)]
// struct Point <T ,U>{
//   x : T,
//   y : U
// }

// fn main(){
//     let point1 = Point { x : 4 , y : 3};
//     let point2 = Point {x : 4.5 , y : 3.2};

//     println!("{point1:?}");
//     println!("{point2:?}");
// }

// (3) struct in impl

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn mixup<X, Y>(self, point: Point<X, Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}

fn main() {
    let point1 = Point::new(3, 5);
    let point2 = Point::new(3.4, 4.5);
    println!("{point1:?}");
    println!("{point2:?}");
    let point3 = point1.mixup(point2);
    println!("{point3:?}");

}
