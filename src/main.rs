use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

use packet::{
    inbound::{InboundPacket, PacketParseError},
    outbound::{IntoMCPacketData, OutboundPacket},
    parse_packet,
};
use util::var_int::VarInt;

const MAX_PACKET_SIZE: usize = 2097151;
const SERVER_STATUS: &str = "{\"version\":{\"name\":\"1.21.4\",\"protocol\":769}}";

mod packet;
mod util;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    let test = parse_packet(&vec![
        44, 123, 34, 118, 101, 114, 115, 105, 111, 110, 34, 58, 123, 34, 110, 97, 109, 101, 34, 58,
        34, 49, 46, 50, 49, 46, 52, 34, 44, 34, 112, 114, 111, 116, 111, 99, 111, 108, 34, 58, 55,
        54, 57, 125, 125,
    ]);
    println!("{test:?}");

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
    let raw_packet = parse_packet(&buf.to_vec());
    match InboundPacket::try_from(raw_packet) {
        Ok(packet) => handle_packet(stream, packet),
        Err(error) => match error {
            PacketParseError::CorruptPacket => println!("Corrupt packet received."),
            PacketParseError::UnknownPacket { id } => println!("Unknown packet type: {id}"),
        },
    }
}

fn handle_packet(stream: &mut std::net::TcpStream, packet: InboundPacket) {
    match packet {
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
            println!("{bytes:?}");
            stream.write(bytes.as_slice()).unwrap();
        }
    }
}
