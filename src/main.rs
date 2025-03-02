use std::{io::Read, net::TcpListener, thread};

const MAX_PACKET_SIZE: usize = 2097151;

#[derive(Debug)]
struct Packet {
    protocol: usize,
    id: usize,
    length: usize,
    data: Vec<u8>,
}

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
    println!("Sucessfully read {bytes_read} bytes: {buf:?}");
    let packet = parse_packet(&buf.to_vec());
    println!("Parsed a packet: {packet:?}");
}

fn decode_varint(mut buf: &[u8]) -> Result<(usize, usize), ()> {
    let mut result = 0;
    let mut shift = 0;

    for byte_count in 0..5 {
        let byte = buf[0];
        buf = &buf[1..];

        let data = byte & 0x7F; // Get only the 7 data bits
        result |= (data as usize) << shift; // Shift and add the bits

        if (byte & 0x80) == 0 {
            return Ok((result, byte_count + 1));
        }

        shift += 7;
    }

    Err(()) // Invalid VarInt (too many bytes)
}

fn parse_packet(buf: &Vec<u8>) -> Packet {
    let mut shift = 0;
    let length = decode_varint(&buf[0..5]).unwrap();
    shift += length.1;
    let id = decode_varint(&buf[shift..shift + 5]).unwrap();
    shift += id.1;
    let protocol = decode_varint(&buf[shift..shift + 5]).unwrap();
    shift += protocol.1;
    let data = &buf[shift..];
    Packet {
        protocol: protocol.0,
        id: id.0,
        length: length.0,
        data: data.to_vec(),
    }
}
