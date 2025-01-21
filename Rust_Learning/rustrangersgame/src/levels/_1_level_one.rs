use std::io;
use tokio::time::{sleep, Duration};

// Cover Datatype String Int Bool
// Cover Topic immutable and mutable

#[tokio::main]
pub async fn start_level() {
    super::_6_level_six::start_level().await;

    println!("\nLoading ... Starting Level 1: Forest of Variables \n");

    sleep(Duration::from_secs(2)).await;

    println!("\n--> Welcome to the Forest of Variables! <---");

    println!(
        "\nOnce upon a time, there was a beautiful forest filled with teak trees, home to countless animals and birds. This forest was under the care and supervision of the Government Authority, which worked tirelessly to protect wildlife and ensure the forest thrived for generations."
    );

    println!(
        "\nThe government knew that teak wood was highly valuable and useful for making ships, boats, and other important tools, so they decided to allow controlled cutting of teak trees only approval of request. However, they also understood that cutting too many trees could harm the forest and the animals that lived there. To ensure balance, they implemented a plan: for every tree cut down, a new tree must be planted."
    );

    println!("\n----> The forest begins with 500 teak trees. <----");

    let teaktree = 500;
    println!("\n (*) Teak tree = {teaktree}");

    sleep(Duration::from_secs(1)).await;

    println!(
        "\nAt first, teaktree = 500 is the count of trees in the forest, but this value is immutable, meaning it cannot change."
    );

    println!(
        "In Rust, when a variable is declared without the mut keyword, it is immutable by default, which means its value cannot be altered."
    );

    println!(
        "However, the count of trees needs to increase or decrease based on actions such as cutting trees or planting new ones. "
    );

    println!(
        "\n To allow this flexibility, we must declare the teaktree variable as mutable using the mut keyword. This will enable the tree count to change each time a tree is cut or a new one is planted."
    );

    let mut teaktree = 500;

    println!("\n (*) Teak tree = {teaktree}");

    planting_tree(&mut teaktree).await;

    cutting_tree(&mut teaktree).await;

    println!("\nFinal number of teak trees in the forest: {}", teaktree);

    println!("\n--> Ending Level 1: Forest of Variables ");
    super::_2_level_two::start_level().await;
}

async fn cutting_tree(teaktree: &mut i32) {
    sleep(Duration::from_secs(1)).await;

    println!(
        "\n\n--> During tree cutting, it is always necessary to obtain permission for the number of Teak trees to be cut. If approval is granted, the trees will be cut; otherwise, they will not be cut."
    );

    sleep(Duration::from_secs(1)).await;

    let mut num_of_trees = String::new();
    println!("\n* How Many Teak Tree Cutting ? (Please Enter Positive Number) : ");
    io::stdin().read_line(&mut num_of_trees).unwrap();
    let num_of_trees: i32 = num_of_trees.trim().parse().unwrap();

    println!("\n* Do you have permission to cut the trees? (true/false): ");
    let mut permission = String::new();
    io::stdin().read_line(&mut permission).unwrap();
    let permission: bool = permission.trim().parse().unwrap();

    sleep(Duration::from_secs(2)).await;

    if permission {
        *teaktree -= num_of_trees;
        println!("---> Approval <---");
        println!("\n\n---> Cut {num_of_trees} trees. Remaining trees: {teaktree}");
    } else {
        println!("---> Denied <---");
        println!("Permission denied. The trees cannot be cut.");
    }
}

async fn planting_tree(teaktree: &mut i32) {
    sleep(Duration::from_secs(1)).await;

    println!(
        "\n\n---> Our planted trees always maintain a record of who planted them and how many trees were planted by each person."
    );

    println!("* Enter Name : ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    sleep(Duration::from_secs(1)).await;

    let mut num_of_trees = String::new();
    println!("\n* Enter How Many Teak Tree Planted (Please Enter Positive Number) : ");
    io::stdin().read_line(&mut num_of_trees).unwrap();
    let num_of_trees: i32 = num_of_trees.trim().parse().unwrap();

    *teaktree += num_of_trees;
    println!("\n\n---> Planted {num_of_trees} trees Person By {name}. Total trees now: {teaktree}");
}
