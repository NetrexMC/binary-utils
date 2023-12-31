use binary_util::interfaces::Reader;
use binary_util::interfaces::Writer;
use binary_util::io::{ByteReader, ByteWriter};
use binary_util::BinaryIo;

// Read bytes exactly to size.

const EXAMPLE_PACKET: &[u8] = &[
    // Packet ID
    2, // Some random short
    0, 0, // Some random int
    0, 0, 0, 0,  // The size of the payload below, (10 bytes)
    10, // The payload
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, // nothing else, (false)
    0,
];

#[test]
fn buffer_read_from_slice() {
    let mut reader = ByteReader::from(EXAMPLE_PACKET);
    let packet_id = reader.read_u8().unwrap();
    assert_eq!(packet_id, 2);
    let some_short = reader.read_u16().unwrap();
    assert_eq!(some_short, 0);
    let some_int = reader.read_u32().unwrap();
    assert_eq!(some_int, 0);
    let payload_size = reader.read_u8().unwrap();
    assert_eq!(payload_size, 10);
    let mut payload = vec![0; payload_size as usize];
    reader.read(&mut payload).unwrap();
    assert_eq!(&payload[..], &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let nothing_else = reader.read_bool().unwrap();
    assert_eq!(nothing_else, false);
}
