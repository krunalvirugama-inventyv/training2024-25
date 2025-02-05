pub trait Draw {
    fn draw(&self);
}
// (1)
// pub struct Screen{
//     pub components: Vec<Box<dyn Draw>>
// }

// impl Screen{
//     pub fn run(&self){
//         for component in self.components.iter(){
//             component.draw();
//         }
//     }
// }

// (2) Here Traint to send same type of the struct in the vector of the Screen struct
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw Button");
    }
}
