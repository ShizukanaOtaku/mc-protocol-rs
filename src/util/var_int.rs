use crate::packet::{data_types::MCEncode, inbound::MCDeserialize};

#[derive(Debug)]
pub struct VarInt {
    bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum VarIntParseError {
    WrongSize,
    NonTerminated,
}

impl TryFrom<VarInt> for usize {
    type Error = VarIntParseError;

    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        let mut result = 0;
        let mut shift = 0;

        let mut bytes = value.bytes.as_slice();
        for _ in 0..5 {
            let byte = bytes[0];
            bytes = &bytes[1..];

            let data = byte & 0x7F;
            result |= (data as usize) << shift;

            if (byte & 0x80) == 0 {
                return Ok(result);
            }

            shift += 7;
        }

        Ok(result)
    }
}

impl MCDeserialize for VarInt {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let bytes = &bytes[..5];
        for (i, byte) in bytes.iter().enumerate() {
            if byte & 0x80 == 0 {
                return Some((
                    VarInt {
                        bytes: bytes[..i].to_vec(),
                    },
                    i,
                ));
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct VarIntTooSmallError;

impl VarInt {
    pub fn new(mut value: usize) -> Result<Self, VarIntTooSmallError> {
        const VAR_INT_MAX_BYTES: usize = 5;
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

            if bytes.len() >= VAR_INT_MAX_BYTES {
                return Err(VarIntTooSmallError);
            }
        }

        Ok(Self { bytes })
    }

    pub fn bytes(&self) -> usize {
        self.bytes.len()
    }
}

impl MCEncode for VarInt {
    fn into_mc_data(self) -> Vec<u8> {
        self.bytes.clone()
    }
}
