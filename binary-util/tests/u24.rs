use binary_util::interfaces::{Reader, Writer};
use binary_util::io::{ByteReader, ByteWriter};
use binary_util::types::{u24, LE};
use binary_util::BinaryIo;

#[derive(BinaryIo)]
struct MyStruct {
    a_value_before: bool,
    test: u24,
    test_le: LE<u24>,
}

#[test]
fn encode_test() {
    let packet = MyStruct {
        a_value_before: true,
        test: 10000.into(),
        test_le: LE(90.into()),
    };
    let mut writer = ByteWriter::new();
    packet.write(&mut writer).unwrap();

    assert_eq!(writer.as_slice(), &[1, 0, 39, 16, 90, 0, 0]);

    // try reading again...
    let mut reader = ByteReader::from(writer.as_slice());

    let packet = MyStruct::read(&mut reader).unwrap();

    assert_eq!(*packet.test, 10000);
    assert_eq!(**packet.test_le, 90);
}

#[test]
fn decode_test() {
    let buf = &[1, 0, 39, 16, 90, 0, 0];
    let mut reader = ByteReader::from(buf.as_slice());
    let packet = MyStruct::read(&mut reader).unwrap();

    assert_eq!(packet.test, 10000.into());
    assert_eq!(packet.test_le, LE(90.into()));
}
