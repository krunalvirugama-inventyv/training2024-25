// fn main() {
//     let result;

//     {
//         let y = 5;
//         result = &y;
//     }

//     println!("{result}")
// }

// (2)
// fn main() {
//     let result:&str;
//     // let x = String::from("Krunal");
//     // let y = String::from("Krunal Viugama");

//     // result = longest_str(&x,&y);
//     // println!("Result : {}",result);

//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest_str(string1.as_str(), string2.as_str());
//         println!("The longest string is {result}");
//     }

// }

// fn longest_str<'a>(x : &'a str,y: &'a str)-> &'a str   {
//     if x.len() > y.len() {
//         return x;
//     }
//     y
// }


// (3)

// fn main(){
//     let result : &str;
//     let string1 = String::from("long string is long");
//     let string2 = String::from("xyz");
//     result = first_word(&string1);
//     println!("The first word is {result}");
// }
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s;
//         }
//     }

//     &s
// }

// (4)
// fn main() {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r: &i32 = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {r}");   //   |       |
//                           // --+       |
// } 


// (5)

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    // let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: &novel,
    };
    
    println!("{}",novel);

    println!("{}",i.part);
}