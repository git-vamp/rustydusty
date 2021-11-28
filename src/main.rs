use crate::lib::{Rect};
mod lib;
fn main() {
    let rect = Rect {
        width: 10,
        height: 50,
    };
    
    if rect.width(){
        println!("{:?}", rect.area());
        println!("{}", rect.can_hold(&rect))
    } else {
        println!("width can be Zero[0]")
    }
}

