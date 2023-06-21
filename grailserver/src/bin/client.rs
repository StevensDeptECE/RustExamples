use std::net::SocketAddr;
use grailserver::tcpclient;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("{}", tcpclient(addr, b"world"));
}
