use std::net::SocketAddr;
use grailserver::tcpserver;

fn main() {
    let addr = SocketAddr::from(([0,0,0,0], 8080));
    tcpserver(addr);
}
