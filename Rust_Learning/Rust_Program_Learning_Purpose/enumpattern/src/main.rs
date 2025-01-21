// (1)

// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
// fn main() {

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     println!("{home:?}");

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };

//     println!("{loopback:?}");

// }

// (2)

// #[derive(Debug)]
// enum IpAddr{
//     V4(String),
//     V6(String),
// }

// (3)

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    //    (2)
    // let home = IpAddr::V4(String::from("127.0.0.0"));

    // let loopback = IpAddr::V6(String::from("::1"));

    // (3)
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?} {:?}", home, loopback);
}
