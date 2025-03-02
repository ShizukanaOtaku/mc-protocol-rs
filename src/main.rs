use std::{io::Read, net::TcpListener, thread};

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
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    let buf = String::from_utf8(buf.to_vec()).unwrap();
    println!(
        "Thread {:?} received a buffer:\n{buf}",
        thread::current().id()
    );
}
