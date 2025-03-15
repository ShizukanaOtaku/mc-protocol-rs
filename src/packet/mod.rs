use inbound::MCDeserialize;

use crate::util::var_int::VarInt;

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
    let length: (VarInt, usize) = VarInt::from_mc_bytes(&buf).unwrap();
    let mut shift = length.1;
    let id: (VarInt, usize) = VarInt::from_mc_bytes(&buf[shift..shift + 5]).unwrap();
    shift += id.1;
    let data = &buf[shift..];
    RawPacket {
        length: length.0.try_into().unwrap(),
        id: id.0.try_into().unwrap(),
        data: data.to_vec(),
    }
}
