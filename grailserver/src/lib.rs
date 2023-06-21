use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    net::ToSocketAddrs,
};

pub fn tcpclient<A: ToSocketAddrs>(addr: A, req: &[u8]) -> String {
    let mut server = TcpStream::connect(addr).unwrap();
    server.write_all(req);
    let mut buf_reader = BufReader::new(&mut server);
    let mut buf = String::new();
    buf_reader.read_line(&mut buf);
    buf
}

pub fn tcpserver<A: ToSocketAddrs>(addr: A) {
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buf = String::new();
    let len = buf_reader.read_line(&mut buf)?;

    let resp = "hello ".to_string() + &buf;
    println!("First line is {len} bytes long");
    stream.write_all(resp.as_bytes()).unwrap();
    println!("Just after server write back");

    Ok(())
}
