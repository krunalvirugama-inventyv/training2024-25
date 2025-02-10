// use std::thread;
// use std::time::Duration;

// fn main() {
//     thread::spawn(|| {  // Start a new thread
//         for i in 1..=5 {
//             println!(" Painting Wall: Step {}", i);
//             thread::sleep(Duration::from_millis(500));
//         }
//     });

//     for i in 1..=5 {
//         println!(" Painting Roof: Step {}", i);
//         thread::sleep(Duration::from_millis(500));
//     }
// }

// (2)
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel(); 

//     thread::spawn(move || {
//         let messages = vec!["(1)Burger", "(2)Pizza", "(3)Drink"];
//         for msg in messages {
//             tx.send(msg).unwrap(); 
//             thread::sleep(Duration::from_millis(500));
//         }
//     });

//     for received in rx {
//         println!("Kitchen received: {}", received);
//     }
// }


// (3)

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let account = Arc::new(Mutex::new(100)); 

    let mut handles = vec![];

    for _ in 0..2 {
        let acc_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            let mut balance = acc_clone.lock().unwrap(); 
            *balance -= 20; 
            println!("Withdrawn! Remaining Balance: {}", balance);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}
