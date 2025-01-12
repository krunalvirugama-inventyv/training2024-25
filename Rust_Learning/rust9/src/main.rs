/// s1 not use after assign
// fn main() {
//     let s1 = String::from("hello");
//     println!("String 1 : {}" , s1);

//     let s2 = s1;
// println!("After assign S2 print a S1 : {}" , s1);
//     // s1 value assign a in s1 than not use s1 moving a ownership kill the s1 and scope of the s1
//     println!("String 2 : {}" , s2);
// }

/// s1 clone clone in s2 than use a s1
fn main() {
    let s1 = String::from("hello");
    println!("String 1 : {}", s1);

    let s2 = s1.clone();
    // s1 value assign a in s1 than not use s1 moving a ownership kill the s1 and scope of the s1
    println!("After assign S2 print a S1 : {}", s1);
    println!("String 1 : {}", s2);
}
