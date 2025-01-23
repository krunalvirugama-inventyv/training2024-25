use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Yello"), 10);
    scores.insert(String::from("Blue"), 20);

    // let score = scores.get(&String::from("Yello")).unwrap_or(&0);

    println!("{scores:?}");

    let scor_yellow = scores.get(&String::from("Yello")).copied().unwrap_or(0);
    println!("{}", scor_yellow);

    scores.insert(String::from("Yello"), 100);

    let scor_yellow = scores.get(&String::from("Yello")).copied().unwrap_or(0);
    println!("{}", scor_yellow);

    scores.entry(String::from("Red")).or_insert(30);

    for (key, value) in &scores {
        println!("{key} : {value}");
    }



    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
