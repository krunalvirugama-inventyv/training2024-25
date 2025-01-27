// Erroe 2 type recoverable and unrecoverable errors.

// In Rust, when something goes very wrong—like trying to open a treasure chest that doesn’t exist—the program doesn’t know how to keep going. So instead of doing something unsafe or unpredictable, it stops everything and says, “I’m panicking! Something is broken!”

// Recoverable errors are those that can be handled and the program can continue to run. does not crash the program.
// ex recoverable error: File not found, network connection lost, invalid user input

// Unrecoverable errors are those that cannot be handled and the program cannot continue to run. crashes the program.
// ex unrecoverable error: trying to access a location in memory that does not exist, calling a method on a type that does not implement it

//  other languages call recoverable errors exceptions and unrecoverable errors panics
//  Rust does not have exceptions. Rust has Result enum to handle recoverable errors and panic! macro to handle unrecoverable errors

// Result enum has two variants: Ok and Err

// panice macro is used to crash the program and print an error message.
// panic! macro is used to handle unrecoverable errors.
// panic! macro to info which line of code caused the panic.

// // (1) explicit panic
// fn main() {
//     println!("Hello, world!");

//    // panic!("crash and burn");

//     println!("This code will not run");
// }

// (2)  taking an action that causes our code to panic if the action fails
// fn main() {
//     let v = vec![1, 2, 3];

//     let ele = v[99];

//     println!("The 100 th element is: {ele}");
// }

// RUST_BACKTRACE=1 cargo run
// It shows the exact spot where the program crashed (the line of code).
// It lists all the steps (functions or calls) that led up to the crash.

// (3) unrecoverable error access index of array to convert in the recoverable error

// fn main() {
//     let v = vec![1, 2, 3];

//     let ele = v.get(99);

//     match ele {
//         Some(ele) => println!("The 100 th element is: {ele}"),
//         None => println!("There is no 100th element."),
//     }
// }

// (4) recoverable error
// Divide Program Panic


// fn main() {
   
//     let result = divided(10,2);

//     println!("The result is: {result}");
// }

// fn divided(x: i32, y: i32) -> i32 {
//     x / y
// }

// (5) recoverable error
// Divide Program Not Panic
// handle with unwrap , unwrap_or, match case
fn main() {
   
    let result = divided(10,0);

    match result {
        Ok(result) => println!("The result is: {result}"),
        Err(e) => println!("Error: {e}"),
    };

}

fn divided(x: i32, y: i32) -> Result<i32, String> {
    if y==0 {   
        return Err("Cannot divide by zero".to_string());
    } 

    Ok(x/y)
}


// expect method is similar to unwrap method, but it allows us to specify the error message.
//  ? operator is used to handle recoverable errors. It is used to return the error value from the current function.