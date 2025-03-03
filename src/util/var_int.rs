use crate::packet::outbound::IntoMCPacketData;

#[derive(Debug)]
pub struct VarInt {
    bytes: Vec<u8>,
}

impl VarInt {
    pub fn new(mut value: usize) -> Self {
        let mut bytes = Vec::new();

        loop {
            let mut byte = (value & 0x7F) as u8;

            value >>= 7;

            if value != 0 {
                byte |= 0x80;
            }

            bytes.push(byte);

            if value == 0 {
                break;
            }
        }

        Self { bytes }
    }

    pub fn value(&self) -> usize {
        let mut result = 0;
        let mut shift = 0;

        let mut bytes = self.bytes.as_slice();
        for _ in 0..5 {
            let byte = self.bytes[0];
            bytes = &bytes[1..];

            let data = byte & 0x7F;
            result |= (data as usize) << shift;

            if (byte & 0x80) == 0 {
                return result;
            }

            shift += 7;
        }

        result
    }

    pub fn bytes(&self) -> usize {
        self.bytes.len()
    }
}

impl IntoMCPacketData for VarInt {
    fn into_mc_data(self) -> Vec<u8> {
        self.bytes.clone()
    }
}
