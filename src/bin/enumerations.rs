fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home => {:?}", home);

    let home = IpAddrString::V4(String::from("127.0.0.1"));
    println!("home => {:?}", home);

    let home = IpAddr::V4(127, 0, 0, 1);
    println!("home => {:?}", home);

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback => {:?}", loopback);

    let loopback = IpAddrString::V6(String::from("::1"));
    println!("loopback => {:?}", loopback);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("loopback => {:?}", loopback);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrString {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) -> String {
    match ip_kind {
        IpAddrKind::V4 => String::from("0.0.0.0"),
        IpAddrKind::V6 => String::from("::0"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}
