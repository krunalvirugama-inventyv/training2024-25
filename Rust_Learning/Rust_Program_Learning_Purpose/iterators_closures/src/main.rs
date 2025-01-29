// (1)
// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
//         // If user_pref than call a self.most_stocked othwerwice direact return a value for selected Color
//         user_pref.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }

//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };

//     let user_pref1 = Some(ShirtColor::Red);

//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );
// }

// (2)
// fn main() {
//     let example_closure = |x| x;

//     let s = example_closure(String::from("hello"));
//     // let n = example_closure(5);
// }

// (3) Capturing References or Moving Ownership
use std::thread;
// fn main() {
// (i) immutable reference

// let list = vec![1, 2, 3];
// println!("Before defining closure: {list:?}");

// let only_borrows = || println!("From closure: {list:?}");

// println!("Before calling closure: {list:?}");
// only_borrows();
// println!("After calling closure: {list:?}");

// (ii)  mutable reference: here is not a use for immultability refreance to print only one at one time mulatabilty refrance allowed

// let mut list = vec![1, 2, 3];
// println!("Before defining closure: {list:?}");

// let mut borrows_mutably = || list.push(7);

// borrows_mutably();

// println!("After calling closure: {list:?}");

// (iii) Ownership Transafar

// let list = vec![1, 2, 3];
// println!("Before defining closure: {list:?}");

// thread::spawn(move|| println!("From thread: {list:?}"))
//     .join()
//     .unwrap();

// // Here After Defining Closure not allowed to Print reson of move ownership in the clouser
// // println!("After defining closure: {list:?}");
// }

// (4) Moving Captured Values Out of Closures and the Fn Traits
// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }

// (5)
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];

//     list.sort_by_key(|r| r.width);
//     println!("{list:#?}");
// }

// (6)
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     // let mut sort_operations = vec![];
//     // let value = String::from("closure called");

//     let mut num_sort_operations = 0;
//     list.sort_by_key(|r| {
//         // sort_operations.push(value);
//         num_sort_operations += 1;
//         r.width
//     });

//     println!("{list:#?}");
//     println!("{list:#?}, \nsorted in {num_sort_operations} operations");
// }

// (7)Processing a Series of Items with Iterators
// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     for val in v1_iter {
//         println!("Got: {val}");
//     }
// }

// (8)The Iterator Trait and the next Method

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }

// (9)
// fn main() {

//     let v1: Vec<i32> = vec![1, 2, 3];

//     let v2: Vec<i32>  = v1.iter().map(|x| x + 1).collect();

//     println!("{v2:?}");

//     assert_eq!(v2, vec![2, 3, 4]);

// }

// (10)
// Using Closures that Capture Their Environment

fn main(){
    
}
