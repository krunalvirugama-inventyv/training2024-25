// // Complete code for the input given for the one at the time

// use std::fs::File;
// use std::io::{self, Write};
// use std::sync::mpsc;
// use std::thread;
// use serde_json::json;

// #[derive(Debug)]
// struct Student {
//     name: String,
//     marks: [u32; 4],
//     total: u32,
//     percentage: f32,
//     rank: usize,
// }

// fn main() {
//     let (tx1, rx1) = mpsc::channel();
//     let (tx2, rx2) = mpsc::channel();
//     let (tx3, rx3) = mpsc::channel();

//     // Thread 1: Take user input
//     let t1 = thread::spawn(move || {
//         for i in 1..=2 {
//             let mut name = String::new();
//             println!("Enter name of Student {}:", i);
//             io::stdin().read_line(&mut name).expect("Failed to read name");

//             let mut marks = [0; 4];
//             for j in 0..4 {
//                 let mut mark = String::new();
//                 println!("Enter marks for subject {}:", j + 1);
//                 io::stdin().read_line(&mut mark).expect("Failed to read mark");
//                 marks[j] = mark.trim().parse().expect("Enter a valid number");
//             }

//             let student = Student {
//                 name: name.trim().to_string(),
//                 marks,
//                 total: 0,
//                 percentage: 0.0,
//                 rank: 0,
//             };

//             tx1.send(student).expect("Failed to send student data");
//         }
//     });

//     // Thread 2: Calculate total marks and percentage
//     let t2 = thread::spawn(move || {
//         let mut received_students = Vec::new();

//         while let Ok(mut student) = rx1.recv() {
//             student.total = student.marks.iter().sum();
//             student.percentage = student.total as f32 / 4.0;
//             received_students.push(student);
//         }

//         for student in received_students {
//             tx2.send(student).expect("Failed to send calculated data");
//         }
//     });

//     // Thread 3: Assign ranks
//     let t3 = thread::spawn(move || {
//         let mut students: Vec<Student> = rx2.iter().collect();
//         students.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());

//         for (rank, mut student) in students.into_iter().enumerate() {
//             student.rank = rank + 1;
//             tx3.send(student).expect("Failed to send ranked data");
//         }
//     });

//     // Thread 4: Store data in JSON files
//     let t4 = thread::spawn(move || {
//         while let Ok(student) = rx3.recv() {
//             let filename = format!("s{}.json", student.rank);
//             let data = json!({
//                 "name": student.name,
//                 "marks": student.marks,
//                 "total": student.total,
//                 "percentage": student.percentage,
//                 "rank": student.rank
//             });

//             let mut file = File::create(&filename).expect("Failed to create file");
//             file.write_all(data.to_string().as_bytes()).expect("Failed to write data");

//             println!("Saved: {}", filename);
//         }
//     });

//     // Wait for all threads to finish
//     t1.join().expect("Thread 1 failed");
//     t2.join().expect("Thread 2 failed");
//     t3.join().expect("Thread 3 failed");
//     t4.join().expect("Thread 4 failed");
// }

use serde_json::json;
use std::fs::File;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::mem;

#[derive(Debug, Default)]
struct Student {
    name: String,
    marks: [u32; 4],
    total: u32,
    percentage: f32,
    rank: usize,
}

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();

    let t1 = thread::spawn(move || {
        for i in 1..=2 {
            let mut name = String::new();
            println!("Enter name of Student {}:", i);
            if io::stdin().read_line(&mut name).is_err() {
                eprintln!("Error reading name");
                continue;
            }

            let mut marks = [0; 4];
            for j in 0..4 {
                let mut mark = String::new();
                println!("Enter marks for subject {}:", j + 1);
                if io::stdin().read_line(&mut mark).is_err() {
                    eprintln!("Error reading marks");
                    continue;
                }
                match mark.trim().parse() {
                    Ok(num) => marks[j] = num,
                    Err(_) => {
                        eprintln!("Invalid input for marks");
                        continue;
                    }
                }
            }

            let student = Student {
                name: name.trim().to_string(),
                marks,
                total: 0,
                percentage: 0.0,
                rank: 0,
            };

            if tx1.send(student).is_err() {
                eprintln!("Failed to send student data");
            }
        }
    });

    // Thread 2: Calculate total marks & percentage
    let t2 = thread::spawn(move || {
        while let Ok(mut student) = rx1.recv() {
            student.total = student.marks.iter().sum();
            student.percentage = student.total as f32 / 4.0;
            if tx2.send(student).is_err() {
                eprintln!("Failed to send calculated data");
            }
        }
    });

    // Thread 3: Assign ranks (Recursive Ranking)
    let mut students = Vec::new();
    let mut last_sent_index = 0; // Tracks the last student that was sent

    let t3 = thread::spawn(move || {
        while let Ok(student) = rx2.recv() {     
            students.push(student);

            println!("📚(1) data {:?}", students);

            // Sort students based on percentage
            students.sort_by(|a, b| b.percentage.total_cmp(&a.percentage));
    
            // Assign ranks
            for (rank, stu) in students.iter_mut().enumerate() {
                stu.rank = rank + 1;
            }

            println!("📚(2) data {:?}", students);

    
            // Send only the newly added students
            while last_sent_index < students.len() {
                let student = students.swap_remove(last_sent_index);  // ✅ Moves the student without defaulting
            
                println!("(3) got rank {:?}", student);
                if tx3.send(student).is_err() {
                    eprintln!("Failed to send ranked data");
                }
            }
        }
    });
    // Thread 4: Store data in JSON files
    let t4 = thread::spawn(move || {
        while let Ok(student) = rx3.recv() {
            let filename = format!("s{}.json", student.rank);
            let data = json!({
                "name": student.name,
                "marks": student.marks,
                "total": student.total,
                "percentage": student.percentage,
                "rank": student.rank
            });

            match File::create(&filename) {
                Ok(mut file) => {
                    if file.write_all(data.to_string().as_bytes()).is_err() {
                        eprintln!("Failed to write data to {}", filename);
                    } else {
                        println!("💾 Saved: {}", filename);
                    }
                }
                Err(_) => eprintln!("Failed to create file {}", filename),
            }
        }
    });

    // let t4 = thread::spawn(move || {
    //     let mut counter = 1;
    //     while let Ok(students) = rx3.recv() { // Receive the whole vector

    //         for student in students { // Loop over each student
    //             let filename = format!("s{}.json", counter);
    //             let data = json!({
    //                 "name": student.name,
    //                 "marks": student.marks,
    //                 "total": student.total,
    //                 "percentage": student.percentage,
    //                 "rank": student.rank
    //             });

    //             match File::create(&filename) {
    //                 Ok(mut file) => {
    //                     if file.write_all(data.to_string().as_bytes()).is_err() {
    //                         eprintln!("Failed to write data to {}", filename);
    //                     } else {
    //                         println!("💾 Saved: {}", filename);
    //                     }
    //                 }
    //                 Err(_) => eprintln!("Failed to create file {}", filename),
    //             }

    //             counter += 1;
    //         }
    //     }
    // });

    // Wait for all threads to finish
    let _ = t1.join();
    let _ = t2.join();
    let _ = t3.join();
    let _ = t4.join();
}
