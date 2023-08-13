use binary_util::interfaces::Reader;
use binary_util::interfaces::Writer;
use binary_util::io::ByteReader;
use binary_util::BinaryIo;

// A slice of bytes that is used to test the reader.
// this specific slice is encoded as the following:
// - a string with the contents of: "BinaryUtils"
// - a var_u32 with the value of: 2147483647
// - a optional u16 with the value of 34
const SIMPLE_TEST: &[u8] = &[
    0x0B, 0x42, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x55, 0x74, 0x69, 0x6C, 0x73, // String
    0xFF, 0xFF, 0xFF, 0xFF, 0x07, // VarInt
    0x01, 0x00, 0x22, // Option<u16>
];

#[test]
fn read_simple_test() {
    let mut buf = ByteReader::from(&SIMPLE_TEST[..]);
    assert_eq!(buf.read_string().unwrap(), "BinaryUtils");
    assert_eq!(buf.read_var_u32().unwrap(), 2147483647);
    assert_eq!(buf.read_option::<u16>().unwrap(), Some(34));
}

// A more complex test that tests the reader with a struct.
#[derive(BinaryIo)]
pub struct TestPacket {
    pub some_int: u8,
    pub some_string: u8,
    // pub unknown_size: VarInt<u32>
}

#[test]
fn construct_struct() {
    let buf = vec![1, 30];
    // try initializing the struct with the buffer instead of the reader.
    let pk = TestPacket::read_from_slice(&buf).unwrap();
    assert_eq!(buf, pk.write_to_bytes().unwrap().as_slice())
}
