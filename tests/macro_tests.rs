use bin_macro::*;
use binary_utils::Streamable;
#[derive(BinaryStream)]
pub struct TestPacket {
     pub some_int: u8,
     pub some_string: u8
}

#[test]
fn construct_struct() {
     let mut buf = vec![1, 30];
     let pk = TestPacket::read(&buf, &mut 0);
}