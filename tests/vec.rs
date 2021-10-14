use bin_macro::*;
use binary_utils::varint::VarInt;
use std::io::Cursor;

#[test]
fn test_varint() {
    let v = VarInt::<u32>(25565);
    let val: Vec<u8> = vec![221, 199, 1];
    dbg!(VarInt::<u32>::from_be_bytes(&[255, 255, 255, 1][..]));
    dbg!(&v.to_be_bytes());
}
