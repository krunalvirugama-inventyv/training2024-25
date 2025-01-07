fn main() {
    // let ans = is_even(2);
    println!("{}", is_even(-1));
}

fn is_even(num : i32) -> bool {
  if num % 2 == 0{
    return  true;
  }else{
    return false;
  }
}