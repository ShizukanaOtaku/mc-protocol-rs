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

impl TryFrom<VarInt> for isize {
    type Error = VarIntParseError;

    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        let mut result = 0;
        let mut shift = 0;

        for (i, &byte) in value.bytes.iter().enumerate() {
            result |= ((byte & 0x7F) as isize) << shift;
            shift += 7;

            if byte & 0x80 == 0 {
                if i > 4 {
                    return Err(VarIntParseError::WrongSize);
                }
                return Ok(result);
            }
        }

        Err(VarIntParseError::NonTerminated)
    }
}

impl MCDeserialize for VarInt {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut var_int_bytes = Vec::new();

        for (i, &byte) in bytes.iter().enumerate() {
            var_int_bytes.push(byte);

            if byte & 0x80 == 0 {
                if var_int_bytes.len() > 5 {
                    return None;
                }

                return Some((
                    VarInt {
                        bytes: var_int_bytes,
                    },
                    i + 1,
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
