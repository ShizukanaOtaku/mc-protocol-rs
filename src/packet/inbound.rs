use crate::util::decode::{decode_u16_bytes, decode_varint};

use super::{ConnectionState, RawPacket};

pub trait MCDeserialize<T> {
    /// Tries to deserialize the type from raw byes.
    /// On success, returns the deserialized type, as well as the amount of bytes used to recreate
    /// it.
    ///
    /// # Examples
    /// ```
    /// let bytes = read_packet();
    /// let first_int = VarInt::from_mc_bytes(&bytes);
    ///
    /// match first_int {
    ///     Some(n) => println!("The packet starts with the number {n}"),
    ///     None => println!("The packet does not start with an int."),
    /// }
    /// ```
    fn from_mc_bytes(bytes: &[u8]) -> Option<(T, usize)>;
}

#[derive(Debug)]
pub enum InboundPacket {
    HandshakePacket {
        protocol_version: usize,
        server_address: String,
        server_port: u16,
        next_state: usize,
    },
}

pub enum PacketParseError {
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
