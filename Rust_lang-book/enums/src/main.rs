#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

fn route(ip: &str, kind: IpAddrKind) {
    println!("routing {:?} addr: {}", kind, ip);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(c: Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Hello, world!");
    route("10.10.10.11", IpAddrKind::v4);

    let lucky_penny = Coin::Penny;
    println!("Value in cents :{}", value_in_cents(lucky_penny));

    let config_max = Some(7u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("The max value is : {max}");
    }
}
