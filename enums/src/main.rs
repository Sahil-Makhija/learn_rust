#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

fn main() {
    println!("Hello, world!");
    route("10.10.10.11", IpAddrKind::v4);
}

fn route(ip: &str, kind: IpAddrKind) {
    println!("routing {:?} addr: {}", kind, ip);
}
