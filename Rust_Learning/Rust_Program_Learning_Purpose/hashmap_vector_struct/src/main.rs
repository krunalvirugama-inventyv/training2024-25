use std::collections::HashMap;

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();

    for (key, value) in vec {
        hm.insert(key, value);
    }

    return hm;
}

fn main() {
    let vec = vec![(String::from("Krunal"), 22), (String::from("Lakhshy"), 23)];

    println!("{vec:?}");

    let hm = group_values_by_keys(vec);

    println!("{hm:?}");
}
