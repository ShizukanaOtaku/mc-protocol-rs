use super::data_types::{PrefixedArray, Property};
use crate::packet::data_types::MCEncode;
use crate::packet::ConnectionState;
use crate::packet::VarInt;

pub fn legacy_server_status(
    protocol_version: i32,
    minecraft_version: &str,
    server_name: &str,
    online_players: i32,
    max_players: i32,
) -> Vec<u8> {
    let response = format!("§1\x00{protocol_version}\x00{minecraft_version}\x00{server_name}\x00{online_players}\x00{max_players}");

    let utf16_bytes: Vec<u8> = response
        .encode_utf16()
        .flat_map(|u| u.to_be_bytes())
        .collect();

    let mut packet = vec![0xFF];
    packet.extend_from_slice(&((utf16_bytes.len() / 2) as u16).to_be_bytes());
    packet.extend_from_slice(&utf16_bytes);

    packet
}

#[derive(Debug)]
pub enum PacketSerializationError {
    InvalidState {
        packet: String,
        state: ConnectionState,
    },
}

#[macro_export]
macro_rules! clientbound_packets {
    ($( $name:ident ($($state:ident: $id:literal),*) { $($field:ident: $type:ty),* }),*$(,)?) => {
        #[derive(Debug)]
        pub enum ClientboundPacket {
            $( $name { $( $field : $type ),* }, )*
        }

        impl ClientboundPacket {
            #[allow(unused_mut)]
            pub fn serialize(self, state: ConnectionState) -> Result<Vec<u8>, PacketSerializationError> {
                match self {
                    $( Self::$name { $($field),* } => {
                        let id = match state {
                            $(ConnectionState::$state => $id),*,
                            _ => return Err(PacketSerializationError::InvalidState {packet: stringify!($name).to_string(), state})
                        };
                        let mut encoded_packet: Vec<u8> = Vec::new();
                        let id = VarInt::new(id).unwrap();
                        $(
                            encoded_packet.extend($field.into_mc_data());
                        )*

                        let packet_length = VarInt::new(encoded_packet.len() + id.bytes()).unwrap();
                        let mut final_packet = Vec::new();
                        final_packet.extend(packet_length.into_mc_data());
                        final_packet.extend(id.into_mc_data());
                        final_packet.extend(encoded_packet);
                        Ok(final_packet)
                    }
                    ),*
                }
            }

            pub fn id(&self, state: ConnectionState) -> Option<usize> {
                match (self, state) {
                    $( $((Self::$name { .. }, ConnectionState::$state) => Some($id)),*, )*
                        _ => None
                }
            }
        }
    };
}

clientbound_packets!(
    StatusResponse (Status: 0x00) { json_response: String },
    PongResponse (Status: 0x01) { timestamp: i64 },
    Disconnect (Status: 0x00) { reason: String },
    EncryptionRequest (Status: 0x01) {
        server_id: String,
        public_key: PrefixedArray<i8>,
        verify_token: PrefixedArray<i8>,
        should_authenticate: bool
    },
    LoginSuccess (Status: 0x02) {
        uuid: u128,
        username: String,
        properties: PrefixedArray<Property>
    },
    FinishConfiguration (Configuration: 0x03) {}
);
