
// // // (1)
// use rand::random;
// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use std::fs;
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// #[derive(Debug, Serialize, Deserialize)]
// enum TireType {
//     Sports,
//     OffRoad,
//     Regular,
// }

// static TIRE_TYPES: [TireType; 3] = [TireType::Sports, TireType::OffRoad, TireType::Regular];

// #[derive(Debug)]
// struct Car<'a> {
//     model_name: String,
//     parts_value: u32,
//     tire: Option<&'a TireType>,
// }

// fn main() {

//     let (tx1, rx1) = mpsc::channel(); // Thread 1 -> Thread 2
//     let (tx2, rx2) = mpsc::channel(); // Thread 2 -> Thread 3


//     let producer_thread = thread::spawn(move || {
//         for i in 1..=25 {
//             let car = Car {
//                 model_name: format!("Car_{}", i),
//                 parts_value: random::<u32>() % 40000 + 30000,
//                 tire: None,
//             };

//             println!("Thread 1: Created {}", car.model_name);
//             tx1.send(car).expect("Failed to send car");
//             thread::sleep(Duration::from_millis(200)); 
//         }
//     });

   
//     let tire_assigner_thread = thread::spawn(move || {
//         while let Ok(mut car) = rx1.recv() {
        
//             let random_index = random::<u8>() % TIRE_TYPES.len() as u8;
//             println!("Thread 2: Assigning Tire to {}", random_index);
//             car.tire = Some(&TIRE_TYPES[random_index as usize]);

//             println!("Thread 2: Assigned Tire to {}", car.model_name);
//             tx2.send(car).expect("Failed to send car with tire");
//             thread::sleep(Duration::from_millis(200));
//         }
//     });

  
//     let file_saver_thread = thread::spawn(move || {
//         while let Ok(car) = rx2.recv() {
//             let filename = format!("{}.json", car.model_name);
//             let json_data = json!({
//                 "model_name": car.model_name,
//                 "parts_value": car.parts_value,
//                 "tire": match car.tire {
//                     Some(TireType::Sports) => "Sports",
//                     Some(TireType::OffRoad) => "OffRoad",
//                     Some(TireType::Regular) => "Regular",
//                     None => "None"
//                 }
//             });

//             match fs::write(&filename, json_data.to_string()) {
//                 Ok(_) => println!("Thread 3: Saved {}", filename),
//                 Err(e) => println!("Thread 3: Failed to save {}: {}", filename, e),
//             }
//             thread::sleep(Duration::from_millis(200));
//         }
//     });

//     let _ = producer_thread.join();
//     let _ = tire_assigner_thread.join();
//     let _ = file_saver_thread.join();

//     println!("All cars processed successfully!");
// }


// (2)
use rand::random;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
enum TireType {
    Sports,
    OffRoad,
    Regular,
}

static TIRE_TYPES: [TireType; 3] = [TireType::Sports, TireType::OffRoad, TireType::Regular];

#[derive(Debug)]
struct Car<'a> {
    model_name: String,
    parts_value: u32,
    tire: Option<&'a TireType>,
}

fn main() {
    let car_data = Arc::new(Mutex::new(None));

    for i in 1..=25 {
        let car_data_clone1 = Arc::clone(&car_data);
        let car_data_clone2 = Arc::clone(&car_data);
        let car_data_clone3 = Arc::clone(&car_data);

        let thread1 = thread::spawn(move || {
            match car_data_clone1.lock() {
                Ok(mut car) => {
                    *car = Some(Car {
                        model_name: format!("Car_{}", i),
                        parts_value : random::<u32>() % 40000 + 30000,
                        tire: None,
                    });
                    println!("Thread 1: Assigned Model - Car_{}", i);
                }
                Err(e) => println!("Thread 1: Failed to acquire lock: {}", e),
            }
            thread::sleep(Duration::from_millis(500));
        });

        let _ = thread1.join();

        let thread2 = thread::spawn(move || {
            match car_data_clone2.lock() {
                Ok(mut car) => {
                    if let Some(ref mut car_data) = *car {
                        let random_index = random::<u8>() % TIRE_TYPES.len() as u8;
                        car_data.tire = Some(&TIRE_TYPES[random_index as usize]);
                        println!("Thread 2: Assigned Tire for Car_{}", i);
                    }
                }
                Err(e) => println!("Thread 2: Failed to acquire lock: {}", e),
            }
            thread::sleep(Duration::from_millis(500));
        });

        let _ = thread2.join();

        let thread3 = thread::spawn(move || {
            match car_data_clone3.lock() {
                Ok(car) => {
                    if let Some(ref car_data) = *car {
                        let filename = format!("{}.json", car_data.model_name);
                        let json_data = json!({
                            "model_name": car_data.model_name,
                            "parts_value": car_data.parts_value,
                            "tire": car_data.tire.map_or("None".to_string(), |t| match t {
                                TireType::Sports => "Sports".to_string(),
                                TireType::OffRoad => "OffRoad".to_string(),
                                TireType::Regular => "Regular".to_string(),
                            })
                        });

                        match fs::write(&filename, json_data.to_string()) {
                            Ok(_) => println!("Thread 3: Saved {}", filename),
                            Err(e) => println!("Thread 3: Failed to save {}: {}", filename, e),
                        }
                    }
                }
                Err(e) => println!("Thread 3: Failed to acquire lock: {}", e),
            }
        });

        let _ = thread3.join();
    }

    println!("All cars processed!");
}


// (3) Car Thread Created
// use rand::random;
// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use std::fs;
// use std::thread;
// use std::time::Duration;

// #[derive(Debug, Serialize, Deserialize)]
// enum TireType {
//     Sports,
//     OffRoad,
//     Regular,
// }

// static TIRE_TYPES: [TireType; 3] = [TireType::Sports, TireType::OffRoad, TireType::Regular];

// #[derive(Debug)]
// struct Car<'a> {
//     model_name: String,
//     parts_value: u32,
//     tire: &'a TireType,
// }
// fn main() {
//     let num_cars = 25;
//     let mut handles = vec![];

//     for i in 1..=num_cars {
//         let handle = thread::spawn(move || {
           
//             let parts_value = random::<u32>() % 40000 + 30000;
//             let random_index = random::<u8>() % TIRE_TYPES.len() as u8;
//             let tire = &TIRE_TYPES[random_index as usize];

//             let car = Car {
//                 model_name: format!("Car_{}", i),
//                 parts_value,
//                 tire,
//             };

//             println!("Processing Car {} in Thread {:?}", i, thread::current().id());
//             thread::sleep(Duration::from_millis(500));

//             let json_data = json!({
//                 "model_name": car.model_name,
//                 "parts_value": car.parts_value,
//                 "tire": match car.tire {
//                     TireType::Sports => "Sports",
//                     TireType::OffRoad => "OffRoad",
//                     TireType::Regular => "Regular",
//                 }
//             });

//             let filename = format!("Car_{}.json", i);
//             match fs::write(&filename, json_data.to_string()) {
//                 Ok(_) => println!("Saved {}", filename),
//                 Err(e) => println!("Failed to save {}: {}", filename, e),
//             }
//         });

//         handles.push(handle);
//     }

//     for handle in handles {
//         let _ = handle.join();
//     }

//     println!("All cars processed!");
// }


// (4)
// use rand::random;
// use rayon::prelude::*;
// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use std::fs::File;
// use std::io::Write;
// use std::sync::{Arc, Mutex};

// #[derive(Debug, Serialize, Deserialize, Copy, Clone)]
// enum TireType {
//     Sports,
//     OffRoad,
//     Regular,
// }

// const TIRE_TYPES: [TireType; 3] = [TireType::Sports, TireType::OffRoad, TireType::Regular];

// #[derive(Debug, Serialize)]
// struct Car {
//     model_name: String,
//     parts_value: u32,
//     tire: TireType,
// }

// fn main() {
//     let num_cars = 25;
    
//     // Shared Mutex for File Writing
//     let file_mutex = Arc::new(Mutex::new(()));

//     // Rayon Parallel Processing
//     (1..=num_cars).into_par_iter().for_each(|i| {
     
//         let parts_value = random::<u32>() % 40000 + 30000;
//         let random_index = random::<u8>() % TIRE_TYPES.len() as u8;
//         let tire = TIRE_TYPES[random_index as usize];
//         let car = Car {
//             model_name: format!("Car_{}", i),
//             parts_value,
//             tire,
//         };

//         println!("Processing {}", car.model_name);

//         let json_data = json!({
//             "model_name": car.model_name,
//             "parts_value": car.parts_value,
//             "tire": match car.tire {
//                 TireType::Sports => "Sports",
//                 TireType::OffRoad => "OffRoad",
//                 TireType::Regular => "Regular",
//             }
//         });

//         let filename = format!("Car_{}.json", i);

//         {
//             let _lock = file_mutex.lock().unwrap();
//             match File::create(&filename).and_then(|mut file| file.write_all(json_data.to_string().as_bytes())) {
//                 Ok(_) => println!("Saved {}", filename),
//                 Err(e) => println!("Failed to save {}: {}", filename, e),
//             }
//         }
//     });

//     println!("All cars processed successfully!");
// }
