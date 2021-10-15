#![allow(unused)]
fn main() {
    // Defining Enum
    enum IpAddrKind {
        V4,
        V6,
    }

    // Defining Enum with data
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    route(four);
    route(six);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    fn route(ip_kind: IpAddrKind) {
        println!("ip kind: {:#?}", ip_kind);
    }
}
