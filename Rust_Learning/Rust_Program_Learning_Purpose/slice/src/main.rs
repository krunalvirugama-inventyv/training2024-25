fn main() {
    //  (1)
    // let mut a = String::from("Hello World");
    // let b = first_word(&a);

    // a.clear();

    // println!("(output) {}" , b);

    // (2) [firstindex..lastindex] lastindex not contains in slice

    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("s = {s}");
    // println!("hello = {hello}");
    // println!("world = {world}");

    // (3)
    // let s = String::from("hello");

    // let slice = &s[0..2];
    // println!("slice1 = {slice}");

    // let slice = &s[..2];
    // println!("slice2 = {slice}");

    // (4)
    // let s = String::from("hello");

    // let len = s.len();

    // let slice = &s[3..len];
    // println!("slice1 = {slice}");

    // let slice = &s[3..];
    // println!("slice2 = {slice}");

    // (5)
    // let s = String::from("hello");

    // let len = s.len();

    // let slice = &s[0..len];
    // println!("slice1 = {slice}");

    // let mut slice = &s[..];
    // println!("slice2 = {slice}");

    // (6)
    // let s = String::from("hello world");
    // let s = "Hello, world!";
    // let fword = first_word(&s);
    // let lword = secound_word(&s);
    // println!("word = {s} , fword = {fword} , lword = {lword}");

    // (7)
    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear(); // error!

    // println!("the first word is: {word}");

    // (8)

    // let my_string = String::from("hello world");

    // // `first_word` works on slices of `String`s, whether partial or whole
    // let word = first_word(&my_string[0..6]);
    // println!("first word [0..6] = {word}");
    // let word = first_word(&my_string[..]);
    // println!("first word [..] = {word}");

    // let word = first_word(&my_string);
    // println!("first word &s = {word}");

    // let my_string_literal = "hello world";

    //  // `first_word` works on slices of string literals, whether partial or whole
    //  let word = first_word(&my_string_literal[0..6]);
    // println!("first word [0..6] = {word}");

    //  let word = first_word(&my_string_literal[..]);
    //  println!("first word [..] = {word}");

    //  // Because string literals *are* string slices already,
    //  // this works too, without the slice syntax!
    //  let word = first_word(my_string_literal);
    // println!("first word &str = {word}");

    // (9)

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    println!("slice = {:?}", slice);
    let ans = assert_eq!(slice, &[2, 3]);
    println!("assert_eq ans = {:?}", ans);
}

// (6)(7)
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn secound_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[(i + 1)..];
        }
    }

    &s[..]
}

// (1)
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     println!("(1) {:?}", bytes);

//     for (i, &item) in bytes.iter().enumerate() {
//         // println!("{i}");
//         if item == b' ' {
//             return i;
//         }
//     }

//     println!("(2) {:?}", bytes);

//     s.len()
// }
