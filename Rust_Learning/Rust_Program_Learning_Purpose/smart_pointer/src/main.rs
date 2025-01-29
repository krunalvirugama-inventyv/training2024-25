// (1)
// enum

// (2) Using Box<T> Like a Reference
// fn main(){
//     let x = 5;
//     let y = Box::new(x);

//     assert_eq!(5,x);
//     assert_eq!(5,*y);
// }

// (3) Defining Our Own Smart Pointer

struct MyBox<T>(T);
use std::ops::Deref;

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.deref()));

    let z = MyBox::new(String::from("KrunalVirugama"));
    // hello(&z);   
    hello(&(*z)[..]);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}