use bin_macro::*;
use binary_utils::{Streamable, varint::VarInt};
#[derive(BinaryStream)]
pub struct TestPacket {
    pub some_int: u8,
    pub some_string: u8,
    pub unknown_size: VarInt::<u32>
}

#[test]
fn construct_struct() {
    let mut buf = vec![1, 30];
    let pk = TestPacket::read(&buf, &mut 0);
    assert_eq!(buf, pk.write())
}
