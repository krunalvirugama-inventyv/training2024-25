use std::io;
use tokio::time::{sleep, Duration};


pub async fn start_level() {

    println!("\nLoading ... Starting Level : The Dark Mines of Errors \n");
    
    sleep(Duration::from_secs(2)).await;

    println!("\n-->You are a miner, exploring deep into the coal mines to collect valuable coal and shiny gems. But mining isn’t easy — there are dangers everywhere! The tunnels can collapse, hidden gas pockets might explode, and your tools might break. Some problems can be fixed, so you can keep mining. But others are so dangerous that you have no choice but to leave the mine completely. ");

    println!("\n\n--> The first thing you encounter in the mine is your coal-cutting machine. Sometimes, the machine works perfectly, but other times, it might overheat or fail to start. This is a recoverable error — you can fix the machine or try another tool, and your mining operation can continue.  ");

    println!("\n--> You have five coal-cutting machines to choose from. Each machine has a unique name and condition. You must decide whether to use the machine or repair it.");

    println!("\n--> Recoverable Errors with Result ");

    let machines = vec![
        "Coal Cutter 1",
        "Coal Cutter 2",
        "Coal Cutter 3",
        "Coal Cutter 4",
        "Coal Cutter 5",
    ];

    let mut tool_condition = String::new();

    for (_i, machine) in machines.iter().enumerate() {
        loop {
            println!(
                "\n\n---> Enter the condition for {} (1 for working, 2 for broken):",
                machine
            );
            tool_condition.clear();

            io::stdin()
                .read_line(&mut tool_condition)
                .expect("Failed to read line");

            let tool_condition: i32 = match tool_condition.trim().parse() {
                Ok(num) if num == 1 || num == 2 => num,
                _ => {
                    println!("--> Invalid input! Please enter 1 for working or 2 for broken.");
                    continue;
                }
            };

            match start_coal_cutter(tool_condition).await {
                Ok(success) => {
                    println!("\n---> Machine {}: {}", machine, success);
                    sleep(Duration::from_secs(2)).await;
                    break; // Exit loop after valid input and successful machine check
                }
                Err(error) => {
                    println!("\n---> Machine {}: {}", machine, error);
                    println!(
                        "Machine {} is broken. Try another machine or Reparing a machine.",
                        machine
                    );
                    break;
                }
            }
        }
    }

    println!("\nIn the mine, you find a gas pocket. If you accidentally hit it with your pickaxe, it will explode and cause the mine to collapse. This is a serious problem that you can't fix. When this happens, the program must stop right away because there is no way to continue safely.");
    println!("\n\n---> Unrecoverable Errors with panic!");

    let mut gas_condition = String::new();
    loop {
        println!("\n\n---> Enter gas detection status (1 for detected, 2 for not detected):");
        gas_condition.clear();

        io::stdin()
            .read_line(&mut gas_condition)
            .expect("Failed to read line");

        let gas_detected: i32 = match gas_condition.trim().parse() {
            Ok(num) if num == 1 || num == 2 => num,
            _ => {
                println!("\n--> Invalid input! Please enter 1 for detected or 2 for not detected.");
                continue;
            }
        };

        sleep(Duration::from_secs(2)).await;
        mine_for_coal(gas_detected).await;

        if gas_detected == 1{
           break;   
        }
    }


    println!("\n\n--> Ending Level 4: The Dark Mines of Errors");
    super::_5_level_five::start_level().await;

}

async fn start_coal_cutter(tool_condition: i32) -> Result<&'static str, &'static str> {
    if tool_condition == 2 {
        Err("The coal cutter is broken! Try fixing it or use another tool.")
    } else {
        Ok("The coal cutter is working. You can start mining with this tool.")
    }
}

async fn mine_for_coal(gas_detected: i32) {
    if gas_detected == 1 {
        // panic!("BOOM! You hit a gas pocket, causing an explosion!");
        println!("BOOM! You hit a gas pocket, causing an explosion!");
    }

    println!("You mine safely and collect some valuable coal.");
}
