use super::{data_types::var_int::VarInt, ConnectionState, RawPacket};

pub trait MCDeserialize {
    /// Tries to deserialize the type from raw byes.
    /// On success, returns the deserialized type, as well as the amount of bytes used to recreate
    /// it.
    ///
    /// # Examples
    /// ```ignore
    /// let bytes = read_packet();
    /// let first_int = VarInt::from_mc_bytes(&bytes);
    ///
    /// match first_int {
    ///     Some(n) => println!("The packet starts with the number {n}"),
    ///     None => println!("The packet does not start with an int."),
    /// }
    /// ```
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized;
}

impl MCDeserialize for u16 {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        let bytes: [u8; 2] = bytes[..2].try_into().unwrap();
        Some((u16::from_le_bytes(bytes), 2))
    }
}

impl MCDeserialize for String {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        let (length, offset) = match VarInt::from_mc_bytes(bytes) {
            Some(data) => (usize::try_from(data.0).unwrap(), data.1),
            None => return None,
        };
        Some((
            String::from_utf8(bytes[offset..offset + length].to_vec()).unwrap(),
            offset + length,
        ))
    }
}

pub enum PacketParseError {
    CorruptPacket,
    UnknownPacket { id: usize },
}

macro_rules! inbound_packets {
    ($(id: $id:literal, state: $state:path, $name:ident {$($field:ident: $type:ty),*}),*$(,)?) => {
        pub enum InboundPacket {
            $($name {
                $($field: $type),*
            }),*
        }

        impl InboundPacket {
            pub fn get_state(&self) -> ConnectionState {
                match self {
                    $(Self::$name{..} => {$state})*
                }
            }

            pub fn get_id(&self) -> usize {
                match self {
                    $(Self::$name{..} => {$id})*
                }
            }

            pub fn try_from(state: ConnectionState, raw_packet: RawPacket) -> Result<Self, PacketParseError> {
                match (state, raw_packet.id) {
                    $(($state, $id) => {
                        let mut offset = 0;
                        $(
                            let $field = match <$type>::from_mc_bytes(&raw_packet.data[offset..]) {
                                Some(data) => {
                                    offset += data.1;
                                    data.0
                                },
                                None => {
                                    return Err(PacketParseError::CorruptPacket);
                                }
                            };
                        )*
                        let packet = Self::$name {
                            $($field),*
                        };
                        return Ok(packet);
                    }),*,
                    _ => Err(PacketParseError::UnknownPacket{ id: raw_packet.id })
                }
            }
        }
    };
}

inbound_packets!(
    id: 0, state: ConnectionState::Handshaking, HandshakePacket {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: VarInt
    }
);
