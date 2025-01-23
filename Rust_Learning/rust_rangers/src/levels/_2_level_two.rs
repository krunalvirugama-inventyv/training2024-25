use std::io;
use tokio::time::{sleep, Duration};


pub async fn start_level() {
    println!("\nLoading ... Starting Level 2:  Loops of the River \n");
    
    sleep(Duration::from_secs(2)).await;

    println!("->As a river rafting service provider, we offer a unique and thrilling experience for our guests. Our river rafting journey includes 8 exciting checkpoints along the way, where the boat will stop to allow visitors to explore and enjoy special spots near the river.\n->Safety is always our first priority. Before starting the journey, we carefully check the river's flow and conditions to ensure a safe and smooth adventure. At each checkpoint, our team ensures the area is safe for visitors to step out and explore.->\nThe rafting continues after each stop, making it a perfect combination of adventure and relaxation. Guests can enjoy the excitement of rafting while taking breaks to discover beautiful places. Our service is designed to create unforgettable memories while keeping everyone safe and happy.\n->There are always 6 trips scheduled per day for this route, with safety being the top priority.");

    println!("\n--> Each trip begins with a safety check before starting Journey No. 1, ensuring a safe visit to all 8 checkpoints.");

    println!("\n---> Journey No. 1 starts here. Visiting all checkpoints");
    println!("\n To explain how a for loop can be used to visit all the checkpoints in Rust.");

    for number in 1..9 {
        sleep(Duration::from_secs(2)).await;
        println!("-+--> Visiting Place Number {number}");
    }

    println!("\n\n---> In Journey No. 2, the guide will start the river rafting adventure by visiting all the checkpoints. However, the journey has a twist: Checkpoint No. 8 will not be visited at the end, as the guests will be given the option to exit at Checkpoint No. 7 instead.");

    let mut checkpoint_number = 0;

    loop {
        match checkpoint_number == 7 {
            true => {
                println!("----> Exits the Journey No.2 With Place No {checkpoint_number}");
                break;
            }
            false => {
                sleep(Duration::from_secs(2)).await;
                checkpoint_number += 1;

                match checkpoint_number == 2 {
                    true => {
                        let mut subcheckpoint = 1;
                        while subcheckpoint < 3 {
                            sleep(Duration::from_secs(2)).await;
                            println!("-+--> Visiting Place Number {checkpoint_number} In Sub Place {subcheckpoint} ");
                            subcheckpoint += 1;
                        }
                    }
                    false => {
                        sleep(Duration::from_secs(2)).await;
                        println!("-+--> Visiting Place Number {checkpoint_number}");
                    }
                }
            }
        }
    }

    println!("\n\n--->In Journey No. 3, the rafting experience starts like the previous journeys, but there's a new twist: a new guide will now be leading the trip. This guide will visit all the checkpoints and announce each place as the boat moves along.");

    let checkpoint = [
        "Emerald Rapids",
        "Crystal Cove",
        "Sunset Cliff",
        "Misty Falls",
        "Golden Beach",
        "Whispering Pines",
        "Thunder Rock",
        "Serenity Bay",
    ];

    for place in checkpoint {
        sleep(Duration::from_secs(2)).await;
        println!("-+--> Guide: Welcome to {place}!");
        println!("Guide: Enjoy the views and the beauty of {place}.\n");
    }

    println!("\n\n--->In Journey No. 4, the rafting experience becomes more immersive. This time, the guide is more experienced and will not only announce the name of each checkpoint but also mention Number Of Checkpoint and  the number of remaining places to visit. The guide will also tell guests how many places are left before the journey ends.");

    let total_checkpoints = checkpoint.len();
    for (i, place) in checkpoint.iter().enumerate() {
        sleep(Duration::from_secs(2)).await;
        let placeno = i + 1;
        let remaining_places = total_checkpoints - placeno;
        println!("\n -+-->Guide: Now visiting {place}. This is Place No {placeno} and {remaining_places} places left to visit.");
        println!("Guide: Enjoy the beauty of {place}.\n");
    }

    println!("\n\n--->In Journey No. 5 has started, but before we visit each checkpoint, we need to check the safety of each location. If any checkpoint is found unsafe, we will not stop there, and the guide will announce which checkpoint is unsafe. For the unsafe location, the boat will proceed without stopping.");

    let mut danger_place_no = String::new();
    println!("\n -+--> Which Place No is Danger That will Not Visiting ? (Please Write 1-8 In) = ");
    io::stdin().read_line(&mut danger_place_no).unwrap();
    let danger_place_no: usize = danger_place_no.trim().parse().unwrap();

    checkpoint_number = 0;

    while checkpoint_number < total_checkpoints {
        sleep(Duration::from_secs(2)).await;

        match checkpoint_number == danger_place_no - 1 {
            true => {
                println!("\nGuide : Skipp The Place No {}", checkpoint_number + 1);
                checkpoint_number += 1;
            }
            false => {
                println!(
                    "\n -+--> Guide: Now visiting {}. This is Place No {} and {} places left to visit.",
                    checkpoint[checkpoint_number],
                    checkpoint_number + 1,
                    total_checkpoints - checkpoint_number - 1
                );
                checkpoint_number += 1;
            }
        }
    }

    println!("\n\n-->Today, after completing all 5 trips, the boat reached the final exit point. Now, we are heading back to the starting point. During the return journey, the boat stops at each checkpoint in reverse order. At each stop, the checkpoint workers who were stationed there board the boat and travel back to the starting point. Once all stops are made and the workers are back on board, the journey comes to an end..");

    for number in (1..9).rev() {
        sleep(Duration::from_secs(2)).await;
        println!("\n-+--> Now visiting Place No {number} All Worker Seat in Boat.");
    }

  

    println!("\n--> Ending Level 2: Loops of the River");
    super::_3_level_three::start_level().await;
}
