use binary_utils::io::{ByteReader, ByteWriter};

pub const FIVE_BYTE_VARINT: &[u8] = &[255, 255, 255, 255, 7]; // 2147483647
pub const THREE_BYTE_VARINT: &[u8] = &[255, 255, 127]; // 2097151
pub const TWO_BYTE_VARINT: &[u8] = &[255, 1]; // 255
pub const ONE_BYTE_VARINT: &[u8] = &[127]; // 127

#[test]
fn read_var_u32() {
    let mut buf = ByteReader::from(&FIVE_BYTE_VARINT[..]);
    assert_eq!(buf.read_var_u32().unwrap(), 2147483647);
    let mut buf = ByteReader::from(&THREE_BYTE_VARINT[..]);
    assert_eq!(buf.read_var_u32().unwrap(), 2097151);
    let mut buf = ByteReader::from(&TWO_BYTE_VARINT[..]);
    assert_eq!(buf.read_var_u32().unwrap(), 255);
    let mut buf = ByteReader::from(&ONE_BYTE_VARINT[..]);
    assert_eq!(buf.read_var_u32().unwrap(), 127);
}

pub const NEGATIVE_VARINT: &[u8] = &[253, 255, 255, 255, 15]; // -2147483647
#[test]
fn read_var_i32() {
    let mut buf = ByteReader::from(&NEGATIVE_VARINT[..]);
    assert_eq!(buf.read_var_i32().unwrap(), -2147483647);
    // -12
    let mut buf = ByteReader::from([23].to_vec());
    assert_eq!(buf.read_var_i32().unwrap(), -12);
    // todo: fix this
    // todo: for some reason this doesn't work on very large numbers, we're 1 bit off
    // let mut buf = ByteReader::from([254, 255, 255, 255, 15].to_vec());
    // assert_eq!(buf.read_var_i32().unwrap(), -2147483648);
}

#[test]
fn write_var_u32() {
    let mut buf = ByteWriter::new();
    buf.write_var_u32(2147483647).unwrap();
    assert_eq!(buf.as_slice(), &FIVE_BYTE_VARINT[..]);
    buf.clear();
    buf.write_var_u32(2097151).unwrap();
    assert_eq!(buf.as_slice(), &THREE_BYTE_VARINT[..]);
    buf.clear();
    buf.write_var_u32(255).unwrap();
    assert_eq!(buf.as_slice(), &TWO_BYTE_VARINT[..]);
    buf.clear();
    buf.write_var_u32(127).unwrap();
    assert_eq!(buf.as_slice(), &ONE_BYTE_VARINT[..]);
}

#[test]
fn write_var_i32() {
    let mut buf = ByteWriter::new();
    buf.write_var_i32(-1).unwrap();
    assert_eq!(buf.as_slice(), &[1]);
    buf.clear();
    buf.write_var_i32(-2147483648).unwrap();
    assert_eq!(buf.as_slice(), &[255, 255, 255, 255, 15]);
}

pub const NINE_BYTE_LONG: &[u8] = &[255, 255, 255, 255, 255, 255, 255, 255, 127]; // 9223372036854775807
pub const NEGATIVE_ONE_LONG: &[u8] = &[1]; // -1
pub const NEGATIVE_LONG: &[u8] = &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1]; // -9223372036854775808

#[test]
fn read_var_u64() {
    let mut buf = ByteReader::from(&NINE_BYTE_LONG[..]);
    assert_eq!(buf.read_var_u64().unwrap(), 9223372036854775807);
}

#[test]
fn read_var_i64() {
    let mut buf = ByteReader::from(&NEGATIVE_ONE_LONG[..]);
    assert_eq!(buf.read_var_i64().unwrap(), -1);
    let mut buf = ByteReader::from(&NEGATIVE_LONG[..]);
    assert_eq!(buf.read_var_i64().unwrap(), -9223372036854775808);
}

#[test]
fn write_var_u64() {
    let mut buf = ByteWriter::new();
    buf.write_var_u64(9223372036854775807).unwrap();
    assert_eq!(buf.as_slice(), &NINE_BYTE_LONG[..]);
}

#[test]
fn write_var_i64() {
    let mut buf = ByteWriter::new();
    buf.write_var_i64(-1).unwrap();
    assert_eq!(buf.as_slice(), &NEGATIVE_ONE_LONG[..]);
    buf.clear();
    buf.write_var_i64(-9223372036854775808).unwrap();
    assert_eq!(buf.as_slice(), &NEGATIVE_LONG[..]);
}
