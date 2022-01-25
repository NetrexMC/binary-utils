use std::io::Cursor;

use binary_utils::{io::{BinaryWriter, BinaryReader}, VarInt, LE};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

#[test]
fn write_tests() -> std::io::Result<()> {
    let byte_varint = VarInt::<u32>(255);

    // Micro test, tests that the stream is correct
    {
        let mut stream = Cursor::new(Vec::<u8>::new());
        stream.write_u32_varint(byte_varint)?;
        assert_eq!(stream.into_inner(), vec![255, 1]);
    }

    // Test arrays
    {
        let my_array: Vec<u16> = vec![55, 66, 77];
        let mut stream = Cursor::new(Vec::<u8>::new());
        stream.write_array::<BigEndian, u16>(my_array)?;
        assert_eq!(stream.into_inner(), vec![0, 3, 0, 55, 0, 66, 0, 77]);
    }

    Ok(())
}

#[test]
fn read_tests() -> std::io::Result<()> {
    // Varint reading!
    {
        let buffer: Vec<u8> = vec![255, 255, 255, 255, 7, 0, 255, 1, 0, 0];
        let mut cursor = Cursor::new(buffer);
        let v = cursor.read_u32_varint()?;
        assert_eq!(v.0, 2147483647);
        dbg!(cursor.read_u8()?);
        dbg!(cursor.position());
        let v2 = cursor.read_u32_varint()?;
        assert_eq!(v2.0, 255);
    }
    Ok(())
}