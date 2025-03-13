use crate::{
    packet::outbound::{MCEncode, OutboundPacket},
    util::var_int::VarInt,
};

#[test]
fn server_status_response() {
    let packet = OutboundPacket::StatusResponsePacket {
        status_json: "{\"version\":{\"name\":\"1.21.4\",\"protocol\":769}}".to_string(),
    };
    let packet: Vec<u8> = packet.into();
    assert_eq!(
        packet,
        vec![
            46, 0, 44, 123, 34, 118, 101, 114, 115, 105, 111, 110, 34, 58, 123, 34, 110, 97, 109,
            101, 34, 58, 34, 49, 46, 50, 49, 46, 52, 34, 44, 34, 112, 114, 111, 116, 111, 99, 111,
            108, 34, 58, 55, 54, 57, 125, 125
        ]
    )
}

#[test]
fn var_int_serialization() {
    let var_int = VarInt::new(5532).unwrap();
    assert_eq!(var_int.into_mc_data(), vec![156, 43]);
    let var_int = VarInt::new(0).unwrap();
    assert_eq!(var_int.into_mc_data(), vec![0]);
    let var_int = VarInt::new(4).unwrap();
    assert_eq!(var_int.into_mc_data(), vec![4]);
}
