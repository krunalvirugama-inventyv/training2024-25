use std::{thread, time::Duration};

// (1)
// fn main() {

//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap();
// }

// (2)

// fn main(){
//     let v = vec![1,2,3];

//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     // here can not access a v reson is that v is moved to the spawned thread
//     // drop(v);
//     handle.join().unwrap();
// }

// (3)
// use std::sync::mpsc;
// fn main(){

//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });

//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

// (4)

// use std::sync::mpsc;

// fn main(){
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals{
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_millis(1));
//         }

//     });

//     for receiver in rx{
//         println!("Got: {}", receiver);
//     }
// }

// (5)

// use std::sync::mpsc;

// fn main(){
//     let (tx, rx) = mpsc::channel();

//     let tx2 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("(1)hi"),
//             String::from("(1)from"),
//             String::from("(1)the"),
//             String::from("(1)thread"),
//         ];

//         for val in vals{
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_millis(1));
//         }

//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("(2)more"),
//             String::from("(2)message "),
//             String::from("(2)for"),
//             String::from("(2)you"),
//         ];

//         for val in vals{
//             tx2.send(val).unwrap();
//             thread::sleep(Duration::from_millis(1));
//         }

//     });

//     for receiver in rx{
//         println!("Got: {}", receiver);
//     }
// }

// (6)

// use std::sync::Mutex;
// fn main(){

//     let m = Mutex::new(5);

//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }

//     println!("m = {:?}" , m);
// }

// (7)

use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {} ", *counter.lock().unwrap());
}
