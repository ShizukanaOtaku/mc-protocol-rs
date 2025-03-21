use crate::packet::data_types::PrefixedArray;
use crate::packet::ConnectionState;
use crate::packet::RawPacket;
use crate::packet::VarInt;

use super::data_types::PrefixedOptional;

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
    CorruptPacket { problematic_field: String },
    UnknownPacket { id: isize },
}

macro_rules! serverbound_packets {
    ($( $name:ident ($($state:ident: $id:literal),*) {$($field:ident: $type:ty),*})*) => {
        pub enum ServerboundPacket {
            $($name {
                $($field: $type),*
            }),*
        }

        impl ServerboundPacket {
            pub fn get_id(&self, state: ConnectionState) -> Option<usize> {
                match (self, state) {
                    $( $((Self::$name { .. }, ConnectionState::$state) => Some($id)),*, )*
                        _ => None
                }
            }

            pub fn get_name(&self) -> Option<&str> {
                match (self) {
                    $( Self::$name { .. } => Some(stringify!($name)),)*
                }
            }

            #[allow(unused_mut, unused_assignments, unused_variables)]
            pub fn try_from(state: ConnectionState, raw_packet: RawPacket) -> Result<Self, PacketParseError> {
                match (state, raw_packet.id) {
                    $( $( (ConnectionState::$state, $id) )|* => {
                        let mut offset = 0;
                        $(
                            let $field = match <$type>::from_mc_bytes(&raw_packet.data[offset..]) {
                                Some(data) => {
                                    offset += data.1;
                                    data.0
                                },
                                None => {
                                    return Err(PacketParseError::CorruptPacket{ problematic_field: stringify!($field).to_string() });
                                }
                            };
                        )*
                            let packet = Self::$name {
                                $($field),*
                            };
                        Ok(packet)
                    }, )*
                    _ => Err(PacketParseError::UnknownPacket{ id: raw_packet.id })}
            }
        }
    };
}

serverbound_packets!(
    LegacyServerListPing (Handshaking: 0xFE) {}
    Handshake (Handshaking: 0x00) {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: VarInt
    }
    StatusRequest (Status: 0x00) {}
    PingRequest (Status: 0x01, Play: 0x24) { timestamp: i64 }
    LoginStart (Login: 0x00) {
        player_name: String,
        player_uuid: u128
    }
    EncryptionResponse (Login: 0x01) {
        shared_secret: PrefixedArray<i8>,
        verify_token: PrefixedArray<i8>
    }
    LoginPluginResponse (Login: 0x02) {
        message_id: VarInt,
        data: PrefixedOptional<Vec<i8>>
    }
    LoginAcknowledged (Login: 0x03) {}
    CookieResponse (Login: 0x04) {
        key: String,
        payload: PrefixedOptional<PrefixedArray<i8>>
    }
    ClientInformation (Configuration: 0x00, Play: 0x0C) {
        locale: String,
        view_distance: i8,
        chat_mode: VarInt,
        chat_colors: bool,
        displayed_skin_parts: u8,
        main_hand: VarInt,
        enable_text_filtering: bool,
        allow_server_listings: bool,
        particle_status: VarInt
    }
    FinishConfiguration (Configuration: 0x03) {}
);
