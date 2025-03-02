use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

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
                let status_json = "
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
                "
                .to_string();
                let response = OutboundPacket::StatusResponsePacket { status_json };
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

#[derive(Debug)]
struct RawPacket {
    length: usize,
    id: usize,
    data: Vec<u8>,
}

#[derive(Debug)]
enum InboundPacket {
    HandshakePacket {
        protocol_version: usize,
        server_address: String,
        server_port: u16,
        next_state: usize,
    },
}

#[derive(Debug)]
enum OutboundPacket {
    StatusResponsePacket { status_json: String },
}

impl Into<Vec<u8>> for OutboundPacket {
    fn into(self) -> Vec<u8> {
        match self {
            OutboundPacket::StatusResponsePacket { status_json } => {
                let mut buf = Vec::new();
                buf.extend(encode_varint(status_json.len()));
                buf.extend(status_json.bytes());

                let mut final_buf = Vec::new();
                let packet_id = encode_varint(0x00);
                let len = buf.len() + packet_id.len();

                final_buf.extend(encode_varint(len));
                final_buf.extend(packet_id);
                final_buf.extend(buf);

                final_buf
            }
        }
    }
}

enum PacketParseError {
    CorruptPacket,
    UnknownPacket { id: usize },
}

impl TryFrom<RawPacket> for InboundPacket {
    type Error = PacketParseError;

    fn try_from(raw_packet: RawPacket) -> Result<Self, Self::Error> {
        match raw_packet.id {
            0 => {
                let protocol_version = decode_varint(&raw_packet.data[0..5]).unwrap();
                let mut shift = protocol_version.1;

                let len = decode_varint(&raw_packet.data[shift..5]).unwrap();
                shift += len.1;

                let server_address =
                    String::from_utf8(raw_packet.data[shift..shift + len.0].to_vec()).unwrap();

                let server_port = decode_u16_bytes((
                    raw_packet.data[shift + len.0],
                    raw_packet.data[shift + len.0 + 1],
                ));

                let next_state = decode_varint(&raw_packet.data[shift + len.0 + 2..])
                    .unwrap()
                    .0;
                Ok(Self::HandshakePacket {
                    protocol_version: protocol_version.0,
                    server_address,
                    server_port,
                    next_state,
                })
            }
            _ => Err(PacketParseError::UnknownPacket { id: raw_packet.id }),
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

fn encode_varint(mut num: usize) -> Vec<u8> {
    let mut buf = Vec::new();

    loop {
        let mut byte = (num & 0x7F) as u8;

        num >>= 7;

        if num != 0 {
            byte |= 0x80;
        }

        buf.push(byte);

        if num == 0 {
            break;
        }
    }

    buf
}

fn decode_u16_bytes(bytes: (u8, u8)) -> u16 {
    (bytes.0 as u16) << 8 | bytes.1 as u16
}

fn parse_packet(buf: &Vec<u8>) -> RawPacket {
    let length = decode_varint(&buf[0..5]).unwrap();
    let mut shift = length.1;
    let id = decode_varint(&buf[shift..shift + 5]).unwrap();
    shift += id.1;
    let data = &buf[shift..];
    RawPacket {
        length: length.0,
        id: id.0,
        data: data.to_vec(),
    }
}
