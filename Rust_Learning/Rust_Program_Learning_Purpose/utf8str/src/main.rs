fn main() {
    // (1)
    // let mut a = String::from("Krunal Virugama");

    // a.push_str(" Hello");
    // a.push('.');
    // println!("{a}");

    // (2)
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // // both allow refrenace and ownenship reson of 2 is automatically get for the refrence
    // s1.push_str(s2);
    // println!("s2 is {s2}");

    // (3)
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("tic");

    // // Here s1 owenship and reamming for referenace
    // // let s = s1 + "-" + &s2 + "-" + &s3;

    // //Here all for the refrence
    // let s = format!("{s1}-{s2}-{s3}");
    // println!("{s}");


    // (4) NOt Wprking
    // let s1 = "Krunal";
    // // get indexing wiht string
    // let ans = &s1[0];
    // println!("{ans}");


    // (5) Sparde in the Squre breaket is the starting and ending index not a must
    let s1 = "Krunal";
    let s2 = &s1[..];
    println!("{s2}")
}
