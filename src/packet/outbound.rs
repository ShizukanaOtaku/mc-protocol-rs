use super::data_types::{var_int::VarInt, MCEncode, PrefixedArray, Property};

pub fn legacy_server_status(
    protocol_version: i32,
    minecraft_version: &str,
    server_name: &str,
    online_players: i32,
    max_players: i32,
) -> Vec<u8> {
    let response = format!("§1\x00{protocol_version}\x00{minecraft_version}\x00{server_name}\x00{online_players}\x00{max_players}",);

    let utf16_bytes: Vec<u8> = response
        .encode_utf16()
        .flat_map(|u| u.to_be_bytes())
        .collect();

    let mut packet = vec![0xFF];
    packet.extend_from_slice(&((utf16_bytes.len() / 2) as u16).to_be_bytes());
    packet.extend_from_slice(&utf16_bytes);

    packet
}

#[macro_export]
macro_rules! outbound_packets {
    ($( $packet_id:literal $variant:ident { $( $field:ident : $ty:ty ),* } ),* ) => {
        #[derive(Debug)]
        pub enum OutboundPacket {
            $( $variant { $( $field : $ty ),* }, )*
        }

        impl Into<Vec<u8>> for OutboundPacket {
            fn into(self) -> Vec<u8> {
                match self {
                    $(
                        OutboundPacket::$variant { $( $field ),* } => {
                            let mut encoded_packet = Vec::new();
                            let packet_id = VarInt::new($packet_id).unwrap();
                            $(
                                encoded_packet.extend($field.into_mc_data());
                            )*
                            let packet_length = VarInt::new(encoded_packet.len() + &packet_id.bytes()).unwrap();
                            let mut final_packet = Vec::new();
                            final_packet.extend(packet_length.into_mc_data());
                            final_packet.extend(packet_id.into_mc_data());
                            final_packet.extend(encoded_packet);
                            final_packet
                        }
                    ),*
                }
            }
        }

        impl OutboundPacket {
            pub fn id(&self) -> usize {
                match self {
                    $(
                        OutboundPacket::$variant { .. } => {
                            $packet_id
                        }
                    ),*
                }
            }
        }
    };
}

outbound_packets!(
    // Status
    0x00 StatusResponse { json_response: String },
    0x01 PongResponse { timestamp: i64 },

    // Login
    0x00 Disconnect { reason: String },
    0x01 EncryptionRequest {
        server_id: String,
        public_key: PrefixedArray<i8>,
        verify_token: PrefixedArray<i8>,
        should_authenticate: bool
    },
    0x02 LoginSuccess {
        uuid: u128,
        username: String,
        properties: PrefixedArray<Property>
    }
);
