mod lib;
use lib::AveragedCollection;

fn main() {
    let mut collection = AveragedCollection {
        list: Vec::new(),
        average: 0.0,
    };

    collection.add(10);
    collection.add(20);
    collection.add(30);
    
    println!("Current average: {}", collection.average());
    
    collection.remove();
    println!("New average after removal: {}", collection.average());
}
