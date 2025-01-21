fn main() {
    
    let user1 = User {
        username: String::from("virugamacoder"),
        email: String::from("virugamacoder@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1 = {:?}", user1);

    let user2 = build_user(String::from("coder"),"virugama1@gmail.com".to_string());
    println!("user2 = {:?}", user2);
 


    let user3 = User {
        username  : user1.username,
        email : user1.email,
        active: false,
        sign_in_count: 0,
    };

    println!("user3 = {:?}", user3);


    let user4 = User {
        active : true,
        ..user3
    };

    println!("user4 = {:?}", user4);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
