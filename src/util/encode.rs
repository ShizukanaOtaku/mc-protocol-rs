pub fn encode_varint(mut num: usize) -> Vec<u8> {
    let mut buf = Vec::new();

    loop {
        let mut byte = (num & 0x7F) as u8;

        num >>= 7;

        if num != 0 {
            byte |= 0x80;
        }

        buf.push(byte);

        if num == 0 {
            break;
        }
    }

    buf
}
