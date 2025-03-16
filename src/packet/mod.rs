use data_types::var_int::VarInt;
use inbound::MCDeserialize;

pub mod data_types;
pub mod inbound;
pub mod outbound;

#[derive(Debug)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

#[derive(Debug)]
pub struct RawPacket {
    pub length: isize,
    id: isize,
    data: Vec<u8>,
}

#[allow(clippy::ptr_arg)] // Some packets may be greater than the stack allows, so using the heap
                          // is neccesary
pub fn parse_packet(buf: &Vec<u8>) -> RawPacket {
    let length = VarInt::from_mc_bytes(&buf).unwrap();
    let mut shift = length.1;
    let length: isize = length.0.try_into().unwrap();
    let id = VarInt::from_mc_bytes(&buf[shift..]).unwrap();
    shift += id.1;
    let data = &buf[shift..];
    RawPacket {
        length,
        id: id.0.try_into().unwrap(),
        data: data.to_vec(),
    }
}
