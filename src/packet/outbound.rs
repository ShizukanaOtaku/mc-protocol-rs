use crate::util::var_int::VarInt;

pub trait IntoMCPacketData {
    fn into_mc_data(&self) -> Vec<u8>;
}

impl IntoMCPacketData for String {
    fn into_mc_data(&self) -> Vec<u8> {
        let length = VarInt::new(self.len());
        let mut data = length.into_mc_data();
        data.extend(self.as_bytes());
        data
    }
}

impl IntoMCPacketData for u8 {
    fn into_mc_data(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl IntoMCPacketData for usize {
    fn into_mc_data(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl IntoMCPacketData for i32 {
    fn into_mc_data(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

macro_rules! implement_packets {
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
                            let packet_id = VarInt::new($packet_id);
                            $(
                                encoded_packet.extend($field.into_mc_data());
                            )*
                            let packet_length = VarInt::new(encoded_packet.len() + &packet_id.bytes());
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
            fn id(&self) -> usize {
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

implement_packets!(0x00 StatusResponsePacket {
    status_json: String
});
