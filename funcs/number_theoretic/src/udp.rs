struct udpsocket {
}


fn bind(ip : u32, port : u32) -> i32 {
    let connStr = format!("{}.{}.{}.{}:{}",
        ip >> 24, (ip >> 16)&0xFF, (ip >> 8)& 0xFF, ip & 0xFF, port);
    let socket = UdpSocket::bind(connStr)
                           .expect("Could not bind socket");
}

fn server(port : u32) {
    let socket = bind(0, port).expect("Could not bind socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", src);
                    sock.send_to(&buf, &src)
                        .expect("Failed to send a response");
                });
            },
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}

fn client(ip : u32, port : u32) {
    let socket = bind(ip, port)
                           .expect("Could not bind client socket");
    socket.connect("127.0.0.1:8888")
          .expect("Could not connect to server");
    loop {
        let mut input = String::new();
        let mut buffer = [0u8; 1500];
        io::stdin().read_line(&mut input)
                   .expect("Failed to read from stdin");
        socket.send(input.as_bytes())
              .expect("Failed to write to server");

        socket.recv_from(&mut buffer)
              .expect("Could not read into buffer");
        print!("{}", str::from_utf8(&buffer)
                         .expect("Could not write buffer as string"));
    }
}