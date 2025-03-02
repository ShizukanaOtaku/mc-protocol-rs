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
    let packet = parse_packet(&buf.to_vec());
    match Packet::from(&packet) {
        Some(packet_type) => match packet_type {
            Packet::HandshakePacket {
                server_address,
                server_port,
                next_state,
            } => println!("A handshake was received: {server_address}:{server_port}, next_state: {next_state}"), 
        },
        None => {
            println!("Packet of id {} is not implemented yet.", packet.id);
        }
    }
}

#[derive(Debug)]
struct RawPacket {
    protocol: usize,
    id: usize,
    length: usize,
    data: Vec<u8>,
}

#[derive(Debug)]
enum Packet {
    HandshakePacket {
        server_address: String,
        server_port: u16,
        next_state: usize,
    },
}

impl Packet {
    pub fn from(raw_packet: &RawPacket) -> Option<Self> {
        match raw_packet.id {
            0 => {
                let len = decode_varint(&raw_packet.data[0..5]).unwrap();

                let shift = len.1;
                let server_address =
                    String::from_utf8(raw_packet.data[shift..shift + len.0].to_vec()).unwrap();

                let server_port = decode_u16_bytes((
                    raw_packet.data[shift + len.0],
                    raw_packet.data[shift + len.0 + 1],
                ));

                let next_state = decode_varint(&raw_packet.data[shift + len.0 + 2..])
                    .unwrap()
                    .0;
                Some(Self::HandshakePacket {
                    server_address,
                    server_port,
                    next_state,
                })
            }
            _ => None,
        }
    }
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

fn decode_u16_bytes(bytes: (u8, u8)) -> u16 {
    (bytes.0 as u16) << 8 | bytes.1 as u16
}

fn parse_packet(buf: &Vec<u8>) -> RawPacket {
    let mut shift = 0;
    let length = decode_varint(&buf[0..5]).unwrap();
    shift += length.1;
    let id = decode_varint(&buf[shift..shift + 5]).unwrap();
    shift += id.1;
    let protocol = decode_varint(&buf[shift..shift + 5]).unwrap();
    shift += protocol.1;
    let data = &buf[shift..];
    RawPacket {
        protocol: protocol.0,
        id: id.0,
        length: length.0,
        data: data.to_vec(),
    }
}
