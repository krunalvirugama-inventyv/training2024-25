use std::io;
use tokio::time::{sleep, Duration};

// Define a struct for fields

enum GroundType {
    CRICKET,
    FOOTBALL,
    BASKETBALL,
    BADMINTON,
}

enum GroundCondition {
    GOOD,
    AVERAGE,
    BAD,
}

struct Ground {
    name: String,
    length: f64,
    width: f64,
    ground_type: GroundType,
    ground_condition: GroundCondition,
}

impl Ground {
    fn new(name: String, length: f64, width: f64, ground_type: GroundType ,ground_condition: Option<GroundCondition>) -> Self {

        let condition = match ground_condition {
            Some(condition) => condition,
            None => GroundCondition::BAD,
        };
        
        Ground {
            name,
            length,
            width,
            ground_type,
            ground_condition : condition
        }
    }

    fn get_area(&self) -> f64 {
        self.length * self.width
    }

    fn get_ground_type(&self) -> String {
        match self.ground_type {
            GroundType::CRICKET => "Cricket".to_string(),
            GroundType::FOOTBALL => "Football".to_string(),
            GroundType::BASKETBALL => "Basketball".to_string(),
            GroundType::BADMINTON => "Badminton".to_string(),
        }
    }

    fn get_ground_condition(&self) -> String{
        match self.ground_condition {
            GroundCondition::GOOD => "Ground is in good condition, You Can Play On The Ground".to_string(),
            _ => "Ground is Not Good condition, You Can't Play On The Ground".to_string(),
        }
    }

    fn get_details(&self) -> () {
        println!(
            "\n<----------> \nGround Name: {}\nLength: {}m\nWidth: {}m\nGround Type: {}\nArea: {} sq.m \nGround Condition: {}", 
            self.name,
            self.length,
            self.width,
            self.get_ground_type(),
            self.get_area(),
            self.get_ground_condition()
        );
    }
}


pub async fn start_level() {

    println!("\n Loading ... Starting Level 5:   Fields of Structs and Enums \n");
    
    sleep(Duration::from_secs(2)).await;

    println!("--> You are a sports ground manager, responsible for maintaining and managing different sports grounds.");

    sleep(Duration::from_secs(1)).await;
    println!("Allready Ground Details Is There Only Assign A Ground Condition to Check Ground Playing or not ? ");


    sleep(Duration::from_secs(2)).await;
    println!(" \n\n -->Ground No 1 : ");

    let ground1_condition = get_ground_condition().await;
    
  
    let ground1 = Ground::new(
        String::from("Cricket Ground"),
        100.0,
        80.0,
        GroundType::CRICKET,
        ground1_condition
    );

    sleep(Duration::from_secs(2)).await;
    ground1.get_details();


    sleep(Duration::from_secs(2)).await;
    println!("\n\n --> Ground No 2 : ");

    let ground2_condition = get_ground_condition().await;

    let ground2 = Ground::new(
        String::from("Football Ground"),
        120.0,
        100.0,
        GroundType::FOOTBALL,
        ground2_condition
    );

    sleep(Duration::from_secs(2)).await;
    ground2.get_details();

    println!("--> End of Level 5: Fields of Structs and Enums");
    super::_6_level_six::start_level().await;
    
}

async fn get_ground_condition() ->  Option<GroundCondition> {

    let mut input = String::new();
    loop{
        println!("Enter Ground Condition With Number = (1=Good,2=Average,3=Bad)");
        input.clear();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return Some(GroundCondition::GOOD),
            "2" => return Some(GroundCondition::AVERAGE),
            "3" => return Some(GroundCondition::BAD),
            _ => println!("Invalid input! Please enter 1, 2, or 3."),
        }
    }
}
