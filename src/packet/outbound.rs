use crate::packet::data_types::MCEncode;
use crate::packet::VarInt;

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
    0x00 StatusResponse {
        json_response: String
    },
    0x01 PongResponse {
        timestamp: i64
    }
);
