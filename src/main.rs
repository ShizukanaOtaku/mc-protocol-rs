use std::{io::Read, net::TcpListener, thread};

const MAX_PACKET_SIZE: usize = 2097151;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        thread::spawn(move || {
            handle_connection(&mut stream);
        });
    }
}

fn handle_connection(stream: &mut std::net::TcpStream) {
    let mut buf = vec![0; MAX_PACKET_SIZE];
    let bytes_read = stream.read(&mut buf).unwrap();
    let buf = &buf[0..bytes_read];
    println!("Sucessfully read {bytes_read} bytes: {:#?}", buf);
}
