use data_types::var_int::VarInt;
use serverbound::MCDecode;

pub mod clientbound;
pub mod data_types;
pub mod serverbound;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub id: isize,
    pub data: Vec<u8>,
}

#[allow(clippy::ptr_arg)] // Some packets may be greater than the stack allows, so using the heap
                          // is neccesary
pub fn parse_packet(buf: &Vec<u8>) -> Option<(RawPacket, usize)> {
    if buf.is_empty() {
        return None;
    }

    if buf[0] == 0xFE {
        return Some((
            RawPacket {
                length: 1,
                id: 0xFE,
                data: Vec::new(),
            },
            49,
        ));
    }

    let length = VarInt::from_mc_bytes(buf).unwrap();

    if buf.len() < length.0.clone().try_into().unwrap() {
        return None;
    }

    let mut shift = length.1;
    let length: isize = length.0.try_into().unwrap();
    let id = VarInt::from_mc_bytes(&buf[shift..]).unwrap();
    shift += id.1;
    let data = &buf[shift..shift + length as usize - 1];
    Some((
        RawPacket {
            length,
            id: id.0.try_into().unwrap(),
            data: data.to_vec(),
        },
        length as usize + 1,
    ))
}
