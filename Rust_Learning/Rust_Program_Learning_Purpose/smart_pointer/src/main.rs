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

// struct MyBox<T>(T);
// use std::ops::Deref;

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *(y.deref()));

//     let z = MyBox::new(String::from("KrunalVirugama"));
//     // hello(&z);   
//     hello(&(*z)[..]);
// }

// fn hello(name: &str) {
//     println!("Hello, {name}!");
// }



// (4) Drop

// struct CustomSmartPointer{
//     data :String
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self){
//         println!("Dropping CustomeSmartPointer with Data {}",self.data);
//     }
// }
// fn main(){
//     let c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };

//     println!("Before Drop");

//     // if drop the strcut for call as a funcation drop parameter as struct
//     drop(c);

//     println!("After Drop");

// }


// (5) rc

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    let  a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }   
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}