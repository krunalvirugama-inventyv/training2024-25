fn main() {
    let s = String::from("Hello, World!");
    println!("count in string : {}",str_count(s));
}

fn str_count(s : String) -> usize {
    s.chars().count()
}

