use super::serverbound::MCDecode;
use var_int::VarInt;

pub mod var_int;

pub trait MCEncode {
    fn into_mc_data(self) -> Vec<u8>;
}

macro_rules! impl_size_types {
    ($($type:ty),*) => {
        $(
            impl MCDecode for $type {
                fn from_mc_bytes(bytes: &[u8]) -> Option<($type, usize)> {
                    const TYPE_SIZE: usize = size_of::<$type>();
                    if bytes.len() < TYPE_SIZE {
                        return None;
                    }

                    let bytes: [u8; TYPE_SIZE] = bytes[..TYPE_SIZE].try_into().unwrap();
                    Some((<$type>::from_be_bytes(bytes), TYPE_SIZE))
                }
            }

            impl MCEncode for $type {
                fn into_mc_data(self) -> Vec<u8> {
                    self.to_le_bytes().to_vec()
                }
            }
        )*
    };
}

impl_size_types!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl MCDecode for bool {
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if let Some(byte) = bytes.first() {
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
        let (length, shift) = VarInt::from_mc_bytes(bytes)?;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedOptional<T>(pub Option<T>);

impl<T> MCEncode for PrefixedOptional<T>
where
    T: MCEncode,
{
    fn into_mc_data(self) -> Vec<u8> {
        match self.0 {
            Some(data) => {
                let mut vec = true.into_mc_data();
                vec.extend(data.into_mc_data());
                vec
            }
            None => false.into_mc_data(),
        }
    }
}

impl<T> MCDecode for PrefixedOptional<T>
where
    T: MCDecode,
{
    fn from_mc_bytes(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        let (contains_field, offset) = bool::from_mc_bytes(bytes)?;

        if !contains_field {
            return Some((Self(None), 1));
        }

        let field = T::from_mc_bytes(&bytes[offset..])?;
        Some((Self(Some(field.0)), offset + field.1))
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

        if bytes.len() < offset + length {
            return None;
        }

        Some((
            String::from_utf8(bytes[offset..offset + length].to_vec()).unwrap(),
            offset + length,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property(pub String, pub String, pub PrefixedOptional<String>);

impl MCEncode for Property {
    fn into_mc_data(self) -> Vec<u8> {
        let mut data = self.0.into_mc_data();
        data.extend(self.1.into_mc_data());
        data.extend(self.2.into_mc_data());
        data
    }
}
