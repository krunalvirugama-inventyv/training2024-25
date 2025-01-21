use std::io;
use tokio::time::{sleep, Duration};

pub async fn start_level() {

    println!("\nLoading ... Starting Level 3:   Mountains of Memory \n");
    
    sleep(Duration::from_secs(2)).await;

    println!("\n--> As an adventurer, I want a step-by-step guide to climb the mountain so that I can safely and confidently reach the top. The guide will ensure I know the proper techniques and the best route to follow. This will enhance my mountain riding experience while prioritizing safety and enjoyment.");

    println!("\n--> As a climber, I want an oxygen bottle stall midway up the mountain so that I can refill or purchase an oxygen bottle when needed. This ensures I am well-prepared to continue my journey safely to the top, with the help of the stall owner providing support");

    let stall_owner_ravibhaii = String::from("oxygen");
    println!("\n---> Stall Owner Ravibhaii owns the first oxygen bottle: {stall_owner_ravibhaii}");

    println!("\n--> Purchase an oxygen bottle for Krunal, and the ownership of the oxygen bottle will be transferred Ravibhaii to Krunal.");

    sleep(Duration::from_secs(2)).await;

    let krunal = stall_owner_ravibhaii;
    println!("\n---> Krunal now owns: {krunal}");

    println!("\n--> This oxygen bottle has a small display to indicate how much oxygen is available in the bottle.");

    sleep(Duration::from_secs(2)).await;

    let mut oxyzon_bottle = 0;
    check_oxyzon_bottle_liter(oxyzon_bottle).await;
    println!("\n--> Refill the oxygen bottle if required in the middle of the mountain.");
    refill_oxyzon_bottle(&mut oxyzon_bottle).await;

    check_oxyzon_bottle_liter(oxyzon_bottle).await;

    // Borrowing: Sharing the oxygen bottle within the group
    println!("\n--> Krunal's group of 5 people will take turns using the bottle.");

    sleep(Duration::from_secs(2)).await;
    let jay = borrow_oxygen_bottle(krunal).await;

    println!("\n---> Jay is using the oxygen bottle. (Borrowed): {jay}");

    sleep(Duration::from_secs(2)).await;
    check_oxyzon_bottle_liter(oxyzon_bottle).await;

    let mitanshu = borrow_oxygen_bottle(jay).await;

    println!("\n---> Mitanshu is now using the oxygen bottle. (Borrowed): {mitanshu}");

    sleep(Duration::from_secs(2)).await;
    println!("\n--> The group decides to refill the bottle again.");

    refill_oxyzon_bottle(&mut oxyzon_bottle).await;

    sleep(Duration::from_secs(2)).await;
    check_oxyzon_bottle_liter(oxyzon_bottle).await;


    
    println!("\n--> Ending Level 3:  Mountains of Memory");
    super::_4_level_four::start_level().await;
}

async fn check_oxyzon_bottle_liter(oxyzon_bottle: i32) {
    println!("\n--> Oxyzon_bottle in Available Oxyzon {oxyzon_bottle} liter ");
}

async fn borrow_oxygen_bottle(name: String) -> String {
    name
}

async fn refill_oxyzon_bottle(oxyzon_bottle: &mut i32) {
    println!("\n\n * Enter the amount of oxygen to refill the bottle (positive number): ");
    let mut refill_liter = String::new();
    io::stdin().read_line(&mut refill_liter).unwrap();
    let refill_liter: i32 = refill_liter.trim().parse().unwrap();

    sleep(Duration::from_secs(2)).await;



    *oxyzon_bottle += refill_liter;
    println!("\n--> Oxygen bottle refilled by {refill_liter} liters.");
}
