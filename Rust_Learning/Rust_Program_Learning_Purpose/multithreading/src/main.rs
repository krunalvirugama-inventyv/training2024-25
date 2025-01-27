use std::thread::{self, spawn};
use std::time::Duration;
// (1)
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 0..10 {
//             println!("hi i am spawn Theread {i}");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     // If run spawn thread first after run the main thread use follwing line to wait for complete
//     handle.join().unwrap();

//     for i in 0..5 {
//         println!("hi i am main Theread {i}");
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// (2)

// fn main() {
//     let x = 1;

//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("{v:?}");
//     });

//     handle.join().unwrap();

//     println!("{x}");
// }

// (3)

// use std::sync::mpsc;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     spawn(move || {
//         // tx.send(String::from("Hello World ")).unwrap();
//         tx.send(String::from("Hello World "));
//     });

//     // let val = rx.recv().unwrap();
//     // println!("Got Value = {val}");

//     match rx.recv(){
//         Ok(value) => println!("got value {value}"),
//         Err(err) => println!("Got A Error while reading {err}")
//     }

// }

// (4)

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();

        spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 10000000..(i + 1 * 100000000) - 1 {
                sum = sum + j;
            }
            producer.send(sum).unwrap();
        });
    }

    drop(tx);

    let mut final_result: u64 = 0;

    for val in rx
    {
        println!("Get the value {} " , val);
        final_result += val;
    }

    println!("Final Result = {final_result}")
}
