// fn main() {
//     let s = String::from("Krunal");
//     let index = find_first_a(s);

//     if index == -1 {
//         println!("a is not found");
//     }else{
//         println!("First a is at index : {}" , index)
//     }

// }

// fn find_first_a(s: String) -> i32 {

//     for (index,c) in s.chars().enumerate(){
//         if c == 'a'{
//             return index as i32;
//         }
//     }

//     return -1;
// }

// Options Enum

// fn main() {
//     // let s = String::from("Krunal");
//     let s = String::from("Print");
//     let index = find_first_a(s);

//    match index {
//     Some(index) => println!("first a is at index : {}", index),
//     None => println!("a is not found"),
//    }
// }

// fn find_first_a(s: String) -> Option<i32> {
//     // only charcter given like ["P","r","i","n","t"]
//     for c in s.chars() {
//         println!("Character: {}", c);
//     }

//     // index and character given like [(0, "P"), (1, "r"), (2, "i"), (3, "n"), (4, "t")]
//     for (index,c) in s.chars().enumerate(){
//         if c == 'a'{
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// Result Enum in Main funcation
// use std::fs::read_to_string;

// fn main()
// {
//     let result = read_to_string("./hello.txt");

//     match result{
//         Ok(data) => println!("File content : {}" , data),
//         Err(error) => println!("Error reading file : {}", error)
//     }
// }

// Result Enum with funcation
use std::fs::read_to_string;

fn main() {
    // let result = read_file_string("./hello.txt".to_string());
    let result = read_file_string(String::from("./hello.txt"));
   
    match result {
        Ok(data) => println!("File content : {}", data),
        Err(error) => println!("Error reading file : {}", error),
    }
}

fn read_file_string(filepath: String) -> Result<String, String> {
    let result = read_to_string(filepath);
    match result {
        Ok(data) => Ok(data),
        Err(error) => Err(String::from("File Not Found")),
    }
}
