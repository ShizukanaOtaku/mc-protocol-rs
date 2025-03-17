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

impl MCDecode for bool {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if let Some(byte) = bytes.get(0) {
            return Some((*byte == 0x01, 1));
        }
        None
    }
}

impl MCEncode for bool {
    fn into_mc_data(self) -> Vec<u8> {
        vec![if self { 0x01 } else { 0x00 }]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedArray<T> {
    data: Vec<T>,
}

impl<T> PrefixedArray<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> MCEncode for PrefixedArray<T>
where
    T: MCEncode,
{
    fn into_mc_data(self) -> Vec<u8> {
        let mut data = VarInt::new(self.data.len()).unwrap().into_mc_data();
        for item in self.data {
            data.extend(item.into_mc_data());
        }
        data
    }
}

impl<T> MCDecode for PrefixedArray<T>
where
    T: MCDecode,
{
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        let (length, shift) = match VarInt::from_mc_bytes(bytes) {
            Some(length) => length,
            None => return None,
        };
        let mut data = Vec::new();
        let l: usize = length.clone().try_into().unwrap();
        for i in shift..(shift + l) {
            match <T>::from_mc_bytes(&bytes[i..]) {
                Some(item) => data.push(item.0),
                None => return None,
            }
        }
        Some((Self { data }, length.try_into().unwrap()))
    }
}

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
        let (length, offset): (usize, usize) = match VarInt::from_mc_bytes(bytes) {
            Some(data) => (data.0.try_into().unwrap(), data.1),
            None => return None,
        };
        Some((
            String::from_utf8(bytes[offset..offset + length].to_vec()).unwrap(),
            offset + length as usize,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property(pub String, pub String, pub String);

impl MCEncode for Property {
    fn into_mc_data(self) -> Vec<u8> {
        let mut data = self.0.into_mc_data();
        data.extend(self.1.into_mc_data());
        data.extend(self.2.into_mc_data());
        data
    }
}

impl MCDecode for i8 {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if bytes.len() < 1 {
            return None;
        }

        let bytes: [u8; 1] = bytes[..1].try_into().unwrap();
        Some((i8::from_be_bytes(bytes), 1))
    }
}

impl MCDecode for u16 {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if bytes.len() < 2 {
            return None;
        }

        let bytes: [u8; 2] = bytes[..2].try_into().unwrap();
        Some((u16::from_be_bytes(bytes), 2))
    }
}

impl MCDecode for i64 {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if bytes.len() < 8 {
            return None;
        }

        let bytes: [u8; 8] = bytes[..8].try_into().unwrap();
        Some((i64::from_be_bytes(bytes), 8))
    }
}

impl MCDecode for u128 {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if bytes.len() < 16 {
            return None;
        }

        let bytes: [u8; 16] = bytes[..16].try_into().unwrap();
        Some((u128::from_be_bytes(bytes), 16))
    }
}
