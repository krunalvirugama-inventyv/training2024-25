fn main() {
    // (1)
    // let s1 = "hello";

    // let s1 = String::from("hello");

    // println!("{s1}");

    // {
    //     let s2 = s1;
    //     println!("{s2}");
    // }
    // println!("{s1}");

    // (2)

    // let x = 5;
    // let mut y = x;
    // y=6;

    // println!("x = {x}, y = {y}");

    // (3)
    // let s = String::from("hello");  // s comes into scope

    // takes_ownership(s);             // s's value moves into the function...
    //                                 // ... and so is no longer valid here
    // // println!("{s}"); // not working ownership conecept

    // let x = 5;                      // x comes into scope

    // makes_copy(x);

    // println!("{x}"); // working fixed size complinece time



    // (4)

    // let s1 = gives_ownership(); // gives_ownership moves its return
    //                             // value into s1

    // println!("s1 is  {s1}");

    // let s2 = String::from("hello"); // s2 comes into scope
    // println!("s2 is  {s2}");


    // let s3 = takes_and_gives_back(s2); // s2 is moved into
    //                                    // takes_and_gives_back, which also
    //                                    // moves its return value into s3

    // println!("s3 is  {s3}");




    // (5)
    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);

    // println!("The length of '{s1}' is {len}.");



    // (6) Borrowing

    // let mut s = String::from("hello");

    // println!("s1 = {s}");

    // change(&mut s);

    // println!("after fun run = {s}");



    // (7) Not allow 2 mutable ref create a above signle var using same time

    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);


    // (7) Not allow 2 mutable ref create and use same time solve  use first after use a secound

    // let mut s = String::from("hello");

    // let r1 = &mut s;
    
    // println!("{}", r1);
    // let r2 = &mut s;
    
    // println!("{} ",r2);


    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);


    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{r1} and {r2}");
    // // variables r1 and r2 will not be used after this point

    // let r3 = &mut s; // no problem
    // println!("{r3}");


    
    // Imulatbale use not resticaltion
    // let a =  String::from("hello");
    // let a1 = &a;
    // let a2 = &a;
    // println!("{} {}",a1,a2);


}


// (6)
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
//     println!("{some_string}")
// }


// (5)
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// (4)
// fn gives_ownership() -> String {             // gives_ownership will move its
//     // return value into the function
//     // that calls it

// let some_string = String::from("yours"); // some_string comes into scope

// some_string                              // some_string is returned and
//     // moves out to the calling
//     // function
// }

// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//              // scope

// a_string  // a_string is returned and moves out to the calling function
// }



// (3)

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.
