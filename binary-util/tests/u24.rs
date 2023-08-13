use binary_util::interfaces::{Reader, Writer};
use binary_util::io::{ByteReader, ByteWriter};
use binary_util::types::u24;
use binary_util::BinaryIo;

#[derive(BinaryIo)]
struct MyStruct {
    test: u24,
}

#[test]
fn encode_test() {
    let packet = MyStruct { test: 10000.into() };

    assert_eq!(packet.write_to_bytes().unwrap().as_slice(), &[0, 39, 16]);

    let mut writer = ByteWriter::new();
    packet.write(&mut writer).unwrap();

    assert_eq!(writer.as_slice(), &[0, 39, 16]);

    // try reading again...
    let mut reader = ByteReader::from(writer.as_slice());

    let packet = MyStruct::read(&mut reader).unwrap();

    assert_eq!(packet.test, 10000.into());
}
