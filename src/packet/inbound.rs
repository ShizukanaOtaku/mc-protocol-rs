use super::{data_types::var_int::VarInt, ConnectionState, RawPacket};

pub trait MCDecode {
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

pub enum PacketParseError {
    CorruptPacket,
    UnknownPacket { id: isize },
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

            pub fn get_name(&self) -> Option<&str> {
                match (self.get_id(), self.get_state()) {
                    $(
                        ($id, $state) => Some(stringify!($name)),
                    )*
                    _ => None
                }
            }

            #[allow(unused_mut, unused_assignments, unused_variables)]
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
    // Handshaking
    id: 0xFE, state: ConnectionState::Handshaking, LegacyServerListPing {},
    id: 0x00, state: ConnectionState::Handshaking, Handshake {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: VarInt
    },

    // Status
    id: 0x00, state: ConnectionState::Status, StatusRequest {},
    id: 0x01, state: ConnectionState::Status, PingRequest { timestamp: i64 },

    // Login
    id: 0x00, state: ConnectionState::Login, LoginStart {
        player_name: String,
        player_uuid: u128
    }
);
