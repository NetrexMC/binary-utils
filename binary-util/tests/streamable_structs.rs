#![allow(deprecated)]

use binary_util::{BinaryStream, Streamable};

#[derive(Debug, BinaryStream)]
pub struct TestPacket {
    pub some_int: u8,
    pub some_string: u8,
    // pub unknown_size: VarInt<u32>
}

#[test]
fn construct_struct() {
    let buf = vec![1, 30];
    let pk = TestPacket::compose(&buf, &mut 0).unwrap();
    assert_eq!(buf, pk.parse().unwrap())
}

#[test]
fn write_string() {
    let string = String::from("Hello world!");
    let hello_world_vec = vec![12, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
    assert_eq!(hello_world_vec, string.parse().unwrap());
}

#[test]
fn read_string() {
    let hello_world_vec = vec![12, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
    let string = String::compose(&hello_world_vec[..], &mut 0).unwrap();
    assert_eq!("Hello world!".to_string(), string);
}
