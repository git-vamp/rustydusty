#![allow(dead_code)]
#[derive(Debug)]
pub enum IpAddrKind {
     V4(String),
     V6(String),
}


impl IpAddrKind {
    pub fn route(a : IpAddrKind) -> String {
        let a  = match a {
            IpAddrKind::V4(a) => {
                a
            },
            IpAddrKind::V6(a) => {
                a
            }
        };
        a
    }
    
}