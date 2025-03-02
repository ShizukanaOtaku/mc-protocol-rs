use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

use packet::{
    inbound::{InboundPacket, PacketParseError},
    outbound::OutboundPacket,
    parse_packet,
};

const MAX_PACKET_SIZE: usize = 2097151;
const SERVER_STATUS: &str = "
                {
                    \"version\": {
                    \"name\": \"1.21.4\",
                    \"protocol\": 769
                },
                \"players\": {
                    \"max\": 64,
                    \"online\": 8,
                    \"sample\": [
                        {
                            \"name\": \"rustmc\",
                            \"id\": \"4566e69f-c907-48ee-8d71-d7ba5aa00d20\"
                        }
                    ]
                },
                \"description\": {
                    \"text\": \"Rust says hello! :3\"
                },
                    \"favicon\": \"data:image/png;base64,<data>\",
                    \"enforcesSecureChat\": false
                }
";

mod packet;
mod util;

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
    println!("Read {bytes_read} bytes: {buf:?}");
    let packet = parse_packet(&buf.to_vec());
    match InboundPacket::try_from(packet) {
        Ok(packet_type) => match packet_type {
            InboundPacket::HandshakePacket {
                protocol_version,
                server_address,
                server_port,
                next_state,
            } => {
                println!("A handshake was received: protocol: {protocol_version}, {server_address}:{server_port}, next_state: {next_state}");
                let response = OutboundPacket::StatusResponsePacket {
                    status_json: SERVER_STATUS.to_string(),
                };
                let bytes: Vec<u8> = response.into();
                stream.write(bytes.as_slice()).unwrap();
            }
        },
        Err(error) => match error {
            PacketParseError::CorruptPacket => println!("Corrupt packet received."),
            PacketParseError::UnknownPacket { id } => println!("Unknown packet type: {id}"),
        },
    }
}
