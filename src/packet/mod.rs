use crate::util::decode::decode_varint;

pub mod data_types;
pub mod inbound;
pub mod outbound;

pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

#[derive(Debug)]
pub struct RawPacket {
    length: usize,
    id: usize,
    data: Vec<u8>,
}

#[allow(clippy::ptr_arg)] // Some packets may be greater than the stack allows, so using the heap
                          // is neccesary
pub fn parse_packet(buf: &Vec<u8>) -> RawPacket {
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
