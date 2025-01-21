use tokio::time::{sleep, Duration};
pub trait Door {
    fn open_door(&self) -> String;
}

pub trait Display {
    fn get_name(&self) -> &str;

    fn display(&self) -> String {
        format!("{} More Details....", self.get_name())
    }
}

struct OldCaverns {
    name: String,
    establish_owner: String,
    establish_year: i32,
    location: String,
}

impl Display for OldCaverns {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn display(&self) -> String {
        format!("\n<----Display---->\nOldCaverns Name : {} \nEstablish Owner : {} \nEstablish_year : {} \nLocation : {}",self.name,self.establish_owner,self.establish_year,self.location)
    }
}

impl Door for OldCaverns {
    fn open_door(&self) -> String {
        "Open The Door For Mannual".to_string()
    }
}

struct LatestCaverns {
    name: String,
    construction_year: i32,
}

impl Display for LatestCaverns {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn display(&self) -> String {
        format!(
            "\n<----Display---->\n Latest Caverns Name : {} \n Construction Year Owner : {} ",
            self.name, self.construction_year
        )
    }
}

impl Door for LatestCaverns {
    fn open_door(&self) -> String {
        "Open The Door For PIN/Password".to_string()
    }
}

pub async fn start_level() {
    println!("\n-->Loading... Starting Level 6: The Final Battle");

    sleep(Duration::from_secs(2)).await;

    println!("\n---> As a player, I want to explore two distinct types of caverns—the historic Old Caverns from the Maharaja’s time and the modern Latest Caverns—so that I can uncover their unique characteristics and learn how they function. I should be able to view detailed descriptions of each cavern and understand the mechanisms used to open their doors, enhancing my journey through this immersive experience.");

    sleep(Duration::from_secs(2)).await;

    println!("\n---> OldCarvens Is Ajanta Caverns and LatestCaverns Is Magical Caverns");

    let ajanta = OldCaverns {
        name: String::from("Ajanta Caverns"),
        establish_owner: String::from(" Buddhist Community"),
        establish_year: 460,
        location: String::from("Maharashtra"),
    };

    let magical = LatestCaverns {
        name: String::from("Magical Caverns"),
        construction_year: 2024,
    };

    sleep(Duration::from_secs(2)).await;
    println!("\n\n * Display Details of Ajanta Caverns : ");
    display_details(&ajanta);

    sleep(Duration::from_secs(2)).await;
    println!("\n\n * Display Details of Magical Caverns Using Generic : ");
    display_details_using_generic(&magical);

    sleep(Duration::from_secs(2)).await;
    println!("\n\n * Display Details of Ajanta Caverns and How to Open Door : ");
    display_and_door(&ajanta);
}

fn display_details(soruce: &impl Display) {
    println!("\n---> Print The Details : ");
    println!("{}", soruce.display());
}

fn display_details_using_generic<T: Display>(soruce: &T) {
    println!("{}", soruce.display());
}

fn display_and_door<T: Display + Door>(soruce: &T) {
    println!("---> Display Caverns Details and How to Door Open Tell ? <---");
    println!("{}\n {}", soruce.display(), soruce.open_door());
}
