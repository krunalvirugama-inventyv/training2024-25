use std::vec;

fn main() {
    println!("Hello, world!");

    // (1)
    // let mut vec = Vec::new();

    // vec.push(1);
    // vec.push(2);
    // vec.push(3);
    // vec.push(4);

    // println!("{:?}",vec);

    // (2)
    // let vec = vec![1, 2, 3, 4];

    // let forth_value = &vec[3];

    // println!("The forth value is {}", forth_value);
    // println!("{:?}",vec);

    // (3)

    // Unwrap but not handle some cases
    // let third_value = vec.get(10).unwrap();

    // Unwrap_or handle
    // let third_value = vec.get(10).unwrap_or(&0);

    // handle for using match Options

    // let third_value = match vec.get(10) {
    //     Some(value) => value,
    //     None => &0
    // };

    // println!("The third value is {:?}", third_value);

    // // (4)

    // let mut vec = vec![1, 2, 3, 4, 5];

    // let first_value = &vec[0];

    // // If Passed a Refernace Is Wrong but without Passing a refrance in first_Value is working
    // // vec.push(6);

    // println!("First Value =  {}", first_value);

    // // Print using for loop
    // // Dererence of the i and is multiphicaltion change a value to must be pass a vec in mute a refreranace and mut
    // for i in &mut vec {
    //     println!("{i}");
    //     // *i = *i * 2;
    //     *i *= 2;
    // }

    // println!("vec = {:?}", vec);

    // (4)

    // let vec1 = vec![1, 2, 3];

    // let vec_iter = vec1.iter();

    // let vec_iter2 = vec_iter.map(|x| x + 1);

    // for i in vec_iter2 {
    //     println!("{i}");
    // }

    // println!("vec1 : {vec1:?}");

    // // vec_iter not valid than shadow created
    // let vec_iter = vec1.iter();

    // let vec_iter3 = vec_iter.filter(|x| *x % 2 == 0);

    // for i in vec_iter3 {
    //     println!("{i}");
    // }

    // (4)

    let vec1 = vec![1, 2, 3, 4, 5];

    let vec_odd_double_iter = vec1.iter().filter(|x| *x % 2 != 0).map(|x| x*2);  
    let mut vec_odd = Vec::new();
    for i in vec_odd_double_iter {
        println!("{i}");
        vec_odd.push(i);
    }

    println!("{vec_odd:?}");

    let vec_odd_double_iter2 = vec1.iter().filter(|x| *x % 2 != 0).map(|x| x*2);

    let vec_odd2: Vec<i32> = vec_odd_double_iter2.collect();

    println!("{vec_odd2:?}");


    // (5) Vector Store a Multiplale type using enum

    // let cell: Vec<SpardeSheetCell> = vec![
    //     SpardeSheetCell::Int(10),
    //     SpardeSheetCell::Float(0.32),
    //     SpardeSheetCell::Text(String::from("Krunal"))
    // ];

    // println!("cell = {:?}", cell);
    // println!("cell = {:#?}", cell);

    // match cell.get(1) {
    //     Some(SpardeSheetCell::Int(value)) => println!("This is Int Value = {} " , value),
    //     Some(value) => println!("This is Other Type = {:?} ",value),
    //     None => println!("This is None Value")
    // }
}

#[derive(Debug)]
enum SpardeSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
