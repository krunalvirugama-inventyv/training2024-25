use std::io;
fn main() {
    // (1) Create a Shadow
    // standard way to create let variable name is small case use is use underscrop but not a camel case use

    // let a = 10;
    // println!("Print a : {}" , a );
    // let a = 20;
    // println!("Print a shadow after : {}" , a);

    // (2) constant create must be name is capital otherwice program run but given only warning
    // const must with a size of memory
    // const must be assign time value that not a create a shadow variable

    // const B : i32 = 10;

    // println!("Print const B {}" , B);

    // (3) inside or outside if mut than override value

    // let mut c = 10;
    // println!("Print c : {} ", c);
    // {
    //     c = 20;
    //     println!("Print c Inside a Curly Bracket : {} ", c);
    // }
    // println!("Print c OutSide a Curly Bracket : {} ", c);

    // (4) inside curly bracket shadow varibale create

    // let  d = 10;
    // println!("Print d : {} ", d);
    // {
    //     let d = 20;
    //     println!("Print and Shadow d Inside a Curly Bracket : {} ", d);
    //     // Shadow Varibale after a Curly braket a kill
    // }
    // // if use now outside
    // println!("Print d OutSide a Curly Bracket : {} ", d);

    // (6) Mut conecept but change a type is throw a error  = expected `&str`

    // let mut f = "Krunal Virugama";
    // println!("print f : {}" , f);
    // f = 10;
    // println!("print f : {}" , f);

    // (5) Shadow Crete but diffrent type first a strig secound is number

    // let e = "KrunalVirugama";
    // println!("e print : {}" ,e);

    // let e = 20;
    // println!("e print : {}" ,e);

    // 3.2

    // (6) floting by default f64 anf signed (+,-)

    // let g = 10.0;
    // println!("Print g : {}" , g);

    // let h = -10.0;
    // println!("Print h : {}" , h)

    // (7) char in signle and double both a qoute
    //but if predefined char type must be use a signle qoute

    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = "😻";

    // println!("All Char print here c= {} , z = {} , heart_eyed_cat = {}" , c,z,heart_eyed_cat);

    

    //     Compound Types
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.


    // (8)tuples
    // Assign a type not a must 
    
    // let tup : (i32,f32,u8,()) = (500,6.4,1,());
    // let tup  = (500,6.4,1 ,());
    
    // let (x, y, z, p)  = tup;

    // println!("The value of fully tup is: {:?}",tup);
    // println!("The value of y is: {x}");
    // println!("The value of y is: {y}");
    // println!("The value of y is: {}",z);
    // println!("The value of x using tup.0 is: {}",tup.0);
    // println!("The value of p : {:?}",p);


    // (9)
    
    // let q = {
    //     println!("q is asssign print stament");
    // };

    // println!("print q : {:?}",q)


    // (10)* Array = fix size also same type , defiend type
    // let array = [1,2,3,4,5];
    // println!("Print Array {:?}" , array);
    // println!("Print Array 0 index using array[0] =  {}" , array[0]);


    //(11) if not defiend type only defiend size you also undescope use (_)
    // let array2 : [_; 5]= [1,2,3,4,5];

    // let array2 : [i32; 5]= [1,2,3,4,5];
    // println!("Print array2 : {:?} " , array2)


    // (12) same value length created array  = [value; size_of_array]

    // let array3 = [3; 5];
    // println!("Print array3 = {:?}" , array3);


    // (14)  Array Programas 
    
    let a = [1,2,3,4,5];

    println!("Print Array : {:?}" ,a);
    println!("Please Enter an Array Index : ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index Enter was not a number");

    if index < a.len() {
        let element = a[index];
        println!("the value of the element at index {index } is : {element}");
    
    }
    else {
        println!("Array lenght is  {} Please enter small size(0-4) ",a.len());
    }

}

