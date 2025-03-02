use crate::util::encode::encode_varint;

#[derive(Debug)]
pub enum OutboundPacket {
    StatusResponsePacket { status_json: String },
}

impl Into<Vec<u8>> for OutboundPacket {
    fn into(self) -> Vec<u8> {
        match self {
            OutboundPacket::StatusResponsePacket { status_json } => {
                let mut buf = Vec::new();
                buf.extend(encode_varint(status_json.len()));
                buf.extend(status_json.bytes());

                let mut final_buf = Vec::new();
                let packet_id = encode_varint(0x00);
                let len = buf.len() + packet_id.len();

                final_buf.extend(encode_varint(len));
                final_buf.extend(packet_id);
                final_buf.extend(buf);

                final_buf
            }
        }
    }
}
