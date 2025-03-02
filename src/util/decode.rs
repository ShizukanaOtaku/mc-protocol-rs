pub fn decode_varint(mut buf: &[u8]) -> Result<(usize, usize), ()> {
    let mut result = 0;
    let mut shift = 0;

    for byte_count in 0..5 {
        let byte = buf[0];
        buf = &buf[1..];

        let data = byte & 0x7F; // Get only the 7 data bits
        result |= (data as usize) << shift; // Shift and add the bits

        if (byte & 0x80) == 0 {
            return Ok((result, byte_count + 1));
        }

        shift += 7;
    }

    Err(()) // Invalid VarInt (too many bytes)
}

pub fn decode_u16_bytes(bytes: (u8, u8)) -> u16 {
    (bytes.0 as u16) << 8 | bytes.1 as u16
}
