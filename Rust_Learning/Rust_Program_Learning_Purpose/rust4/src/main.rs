struct User{
    first_name : String,
    last_name : String,
    age : i32
}

fn main() {
   
  let user = User {
    first_name: String::from("Krunal"),
    last_name: String::from("Virugama"),
    age : 32,
  };

  println!("{}",user.first_name);   
  println!("{}",user.last_name);
  println!("{}",user.age);     
}
