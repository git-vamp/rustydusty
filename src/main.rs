
use rustydusty::{ IpAddrKind};

// mod lib;

fn main() {
    let x = [
        IpAddrKind::V4(String::from("127.168.0.1")),
        IpAddrKind::V6(String::from("39.168.24.3"))
    ];
    for i in x {
        println!("{}", IpAddrKind::route(i))
    }
}
