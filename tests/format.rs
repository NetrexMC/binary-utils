use binary_utils::*;

pub const EXPECTED_DEBUG: &[u8] = &[
    // packet id
    95,
    // u128 as LE
    117, 215, 192, 250, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

#[derive(BinaryStream)]
pub struct TestPacket {
    pub id: u8,
    pub width: LE<u128>
}

#[test]
fn test_varint() {
    let test = TestPacket {
        id: 95,
        width: LE::<u128>(4206942069)
    };
    assert_eq!(&test.parse().unwrap()[..], EXPECTED_DEBUG);
}
