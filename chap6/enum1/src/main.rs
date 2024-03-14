fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);    
    let some_string = Some("a string");
    // let absent_number: Option<i32> = None;//expected `Option<i32>`, found `Option<_>`
    // let absent_number = Option::<i32>::None;
    let absent_number: Option<i32> = Option::None;

    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;//expected `i8`, found enum `Option<i8>`
    // let sum = x + y.unwrap();
}

enum IpAddrKind{
    V4,
    V6,
    }

enum IpAddr2{
    V4(String),
    V6(String),
}

enum IpAddr3{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {
    // do something
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        // do something
    }
}

enum Option<T> {
    Some(T),
    None,
}