pub mod calc;
use calc::largest::Largest;

fn main() {
    let a = Largest { v1: 3, v2: 4 };
    let b = Largest { v1: 3.32, v2: 4.31 };
    let c = Largest {
        v1: String::from("hello"),
        v2: String::from("yeas"),
    };
    println!("max: {:?} min: {:?}", a.bigger(), a.smaller());
    println!("max: {:?} min: {:?}", b.bigger(), b.smaller());
    println!("max: {:?} min: {:?}", c.bigger(), c.smaller());
}
// nail cutter
