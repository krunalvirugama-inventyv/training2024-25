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


fn main() {
    // let s = String::from("Krunal");
    let s = String::from("Print");
    let index = find_first_a(s);    

   match index {
      Some(index) => println!("first a is at index : {}", index),
        None => println!("a is not found"),
   }
}

fn find_first_a(s: String) -> Option<i32> {
    
    for (index,c) in s.chars().enumerate(){
        if c == 'a'{
            return Some(index as i32);
        }
    }
    return None;
}

