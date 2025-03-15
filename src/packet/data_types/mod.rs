use crate::util::var_int::VarInt;

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
