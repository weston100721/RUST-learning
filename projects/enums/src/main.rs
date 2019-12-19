#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}



fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}",four);
    println!("{:?}",six);

    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("kind = {:?}, address = {}",localhost.kind,localhost.address);



    let home = IpAddr2::V4(127, 0, 0, 1);


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);



    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    };

    let coin = Coin::Quarter;


    let mut count = 0;
    match coin {
        Coin::Quarter => println!("State quarter from !"),
        _ => count += 1,
    }


    let mut count = 0;
    if let Coin::Quarter = coin {
        println!("State quarter from !");
    } else {
        count += 1;
    }
}
