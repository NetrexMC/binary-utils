use binary_utils::interfaces::{Reader, Writer};
use binary_utils::io::ByteReader;
use binary_utils::BinaryIo;

#[derive(BinaryIo, Debug)]
#[repr(u8)]
pub enum TestPacket {
    A,
    B(u8, u8),
    C(u8, u8, u8),
}

#[test]
fn encode_test() {
    let packet = TestPacket::B(1, 2);

    assert_eq!(packet.write_to_bytes().unwrap().as_slice(), &[1, 1, 2]);
}

#[test]
fn decode_test() {
    let buf: &[u8] = &[1, 1, 2];
    let mut reader = ByteReader::from(buf);

    match TestPacket::read(&mut reader).unwrap() {
        TestPacket::B(a, b) => {
            assert_eq!(a, 1);
            assert_eq!(b, 2);
        }
        _ => panic!("Wrong packet type"),
    }
}
