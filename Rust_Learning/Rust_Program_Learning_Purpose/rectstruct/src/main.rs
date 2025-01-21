#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Reactangle {
        width: 30,
        height: 50,
    };

    println!("rect1 Area is {}", area(&rect1));
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    let scale = 2;
    let rect2 = Reactangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

}

fn area(reactangle: &Reactangle) -> u32 {
    reactangle.width * reactangle.height
}
