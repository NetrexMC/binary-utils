use binary_utils::*;

#[derive(BinaryStream)]
#[repr(u8)]
pub enum PacketField {
    Cats = 10,
    Dogs = 12,
    Apple,
}

#[test]
pub fn field_test_enum() {
    assert_eq!(PacketField::Apple.parse().unwrap()[0], 13);
}
