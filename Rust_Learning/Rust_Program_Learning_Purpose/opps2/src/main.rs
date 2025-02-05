mod lib;
use lib::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw SelectBox");
    }
}

fn main() {
    // (1)
    // let screen = Screen {
    //     components: vec![
    //         Box::new(SelectBox {
    //             width: 75,
    //             height: 20,
    //             options: vec![String::from("Yes"), String::from("No"), String::from("Maybe")],
    //         }),
    //         Box::new(Button {
    //             width: 50,
    //             height: 10,
    //             label: String::from("OK"),
    //         }),
    //     ],
    // };

    // (2)
    // Here Traint to send same type of the struct in the vector of the Screen struct 
    let screen = Screen {
        components: vec![
            
            Button {    
                width: 50,
                height: 10,
                label: String::from("OK"),
            },
            Button {    
                width: 40,
                height: 35,
                label: String::from("OKOK"),
            },

        ],
    };
    screen.run();
}
