// (1)

// fn print_labeled_measurement(value: i32 ,label : char )
// {
//     println!("Print Label and Value  {label} : {value} ");
// }

fn main() {
    // println!("Hello, world!");

    // (1)

    // print_labeled_measurement(5, 'h');

    // (2)

    // let x = 5+6;
    // println!("print x {x}");

    // let y = {
    //     let z = 3;
    //     z + 1
    // };

    // println!("The value of y is: {y}");

    // (3)
    // let x = five();
    // println!("Print x is {x}")

    // (4) Control flow here if and else return a same type

    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");

    // (5) Loop
    // Only loop is infinity loop
    // loop {
    //     println!("Again..");
    // }

    // (6) Break Keyword after use value as a return

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter*2;
    //     }
    // };

    // println!("result {:?}",result);



    // (7) Loop Break with label

    // let mut counter = 0;
    // 'counting_up : loop {
    //     println!("Count = {counter}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}" );

    //         if remaining==9 {
    //             break;
    //         }

    //         if counter == 2 {
    //             break 'counting_up;
    //         }

    //         remaining-=1;
    //     }

    //     counter +=1;
    // }



    // (8) While loop

    // let mut number = 3;

    // while number!=0 {
    //     println!("Number = {number}");
    //     number-=1;
    // }


    // (9) array print wiht while loop

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }


    // (10) For loop

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {element}");
    // }

    for number in (1..4).rev() {
        println!("{number}");
    }


}

// (3)

// fn five() -> u32  {
//     5
// }
