use super::inbound::MCDecode;
use var_int::VarInt;

pub mod var_int;

pub trait MCEncode {
    fn into_mc_data(self) -> Vec<u8>;
}

macro_rules! impl_size_types_encoding {
    ($($t:ty),*) => {
        $(impl MCEncode for $t {
            fn into_mc_data(self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        })*
    };
}

impl_size_types_encoding!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl MCEncode for String {
    fn into_mc_data(self) -> Vec<u8> {
        let length = VarInt::new(self.len()).unwrap();
        let mut data = length.into_mc_data();
        data.extend(self.as_bytes());
        data
    }
}

impl MCDecode for String {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        let (length, offset) = match VarInt::from_mc_bytes(bytes) {
            Some(data) => (isize::try_from(data.0).unwrap(), data.1),
            None => return None,
        };
        Some((
            String::from_utf8(bytes[offset..offset + length as usize].to_vec()).unwrap(),
            offset + length as usize,
        ))
    }
}

impl MCDecode for u16 {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        let bytes: [u8; 2] = bytes[..2].try_into().unwrap();
        Some((u16::from_be_bytes(bytes), 2))
    }
}

impl MCDecode for i64 {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        let bytes: [u8; 8] = bytes[..8].try_into().unwrap();
        Some((i64::from_be_bytes(bytes), 8))
    }
}

impl MCDecode for u128 {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        let bytes: [u8; 16] = bytes[..16].try_into().unwrap();
        Some((u128::from_be_bytes(bytes), 16))
    }
}
