use std::fs;
use std::fs::File;
use std::io::{self, Read};

// (1)
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn main(){
//     let username = read_username_from_file();
//     match username {
//         Ok(username) => println!("Username: {username}"),
//         Err(e) => panic!("Error reading username: {e}"),
//     }
// }

// (2)
// fn main() -> io::Result<()> {
//     // Open the file
//     let greeting_file = File::open("hello.txt");

//     // Handle the result of opening the file
//     let mut file = match greeting_file {
//         Ok(data) => data,
//         Err(e) => panic!("Problem opening the file: {e}"),
//     };

//     // Read the file content into a String
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     // Display the file contents
//     println!("File Contents:\n{}", contents);

//     Ok(())
// }

// (3)

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// fn read_username_from_file_direct() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

// fn main() {
//     let username = read_username_from_file();

//     match username {
//         Ok(username) => println!("Username: {username}"),
//         Err(e) => panic!("Error reading username: {e}"),
//     }

//     let username_direct = read_username_from_file_direct();

//     match username_direct {
//         Ok(username) => println!("Direct Find Username: {username}"),
//         Err(e) => panic!("Error reading username: {e}"),
//     }
// }

// (4)

// fn main() {
//     // let s = String::from("\nhello");  // s comes into scope
//     let s = String::from("hello");  // s comes into scope

//     let lastchar = last_char_of_first_line(&s);

//     match lastchar {
//         Some(c) => println!("The last character of the first line is: {c}"),
//         None => println!("There is no first line."),
//     }
// }

// fn last_char_of_first_line(text: &str) -> Option<char>{
//     text.lines().next()?.chars().last()
// }

// (5)

// use std::error::Error;
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;
//     Ok(())
// }

// Error message:
// Error: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
// error: process didn't exit successfully: `target\debug\errorhandling.exe` (exit code: 1)

// (6)

use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("Home IP address: {home}");
}
