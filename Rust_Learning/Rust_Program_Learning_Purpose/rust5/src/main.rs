struct Rect {
    width: i32,
    height: i32,    
}

impl Rect{
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn primeter(&self,num : i32) -> i32 { 
        println!("num paramater : {}",num);    
        return 2 * (self.width + self.height)
    }

    fn debug() -> i32{
        return 1;
    }

    
}

fn main() {
    let rect1 = Rect {
        width : 5,
        height : 10,
    };

    println!("Area of Rect : {}",rect1.area());
    println!("Premeter of Rect : {}",rect1.primeter(1));
    println!("Debug of Rect There is not call with object but with struct::  =  {}",Rect::debug());
    
}
