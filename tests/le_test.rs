use std::io::Write;

use binary_utils::*;

#[test]
fn read_write_le_mixed() -> Result<(), error::BinaryError> {
    // LE encoded size, but BE data
    // The first 3 bytes are dummy bytes
    let buff_one: Vec<u8> = vec![
        32, 32, 32, 12, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33, 32, 32,
        32, 32,
    ];

    // read first 3 bytes.
    // this makes our offset 3
    let le_header = &buff_one[3..];
    let mut offset: usize = 0;

    // get the length
    let length = LE::<u32>::compose(&le_header, &mut offset)?.0 as usize;
    assert_eq!(12, length);

    // now get the rest of the buffer based on the length
    let decoded = String::from_utf8(le_header[offset..(length + offset)].to_vec()).unwrap();
    assert_eq!(decoded, "Hello World!".to_string());

    // get the rest of the buffer, there should be 4 more bytes
    assert_eq!(&le_header[(length + offset)..], &[32, 32, 32, 32]);

    // Writing test
    let mut buff_two: Vec<u8> = vec![32, 32, 32, 32];
    let to_encode = "Hello World!".to_string();

    // The length of the string in LE:
    let length = LE::<u32>(to_encode.len() as u32);
    buff_two.write_all(&length.parse()?[..])?;
    // we should now have the length of 12 written in LE

    // write the contents of the string now...
    buff_two.write_all(&to_encode.as_bytes())?;

    // Write magic to buffer.
    buff_two.write_all(&[32, 32, 32, 32])?;

    // Now check
    assert_eq!(buff_one, &buff_one[..]);
    Ok(())
}
