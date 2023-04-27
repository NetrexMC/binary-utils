use binary_utils::interfaces::{Reader, Writer};
use binary_utils::io::ByteReader;
use binary_utils::BinaryIo;

#[derive(BinaryIo, Debug)]
struct ABC {
    a: u8,
    #[satisfy(self.a == 10)]
    b: Option<u8>,
    c: u8,
}

#[test]
fn abc_derive_write() {
    let b_present = ABC {
        a: 10,
        b: Some(9),
        c: 3,
    };
    assert_eq!(b_present.write_to_bytes().unwrap().as_slice(), &[10, 9, 3]);

    // B is present, but A doesn't satisfy the condition for b to be encoded.
    let a_invalid = ABC {
        a: 4,
        b: Some(99),
        c: 9,
    };
    assert_eq!(a_invalid.write_to_bytes().unwrap().as_slice(), &[4, 9]);

    // B is not present, but A satisfies the condition for b to be encoded.
    // This WILL fail.
    let b_not_present = ABC {
        a: 10,
        b: None,
        c: 9,
    };
    assert_eq!(b_not_present.write_to_bytes().is_err(), true);
}

#[test]
fn abc_derive_read() {
    println!("B_PRESENT");
    const B_PRESENT_BUF: &[u8] = &[10, 9, 3];
    let mut reader = ByteReader::from(B_PRESENT_BUF);
    let b_present = ABC::read(&mut reader).unwrap();

    assert_eq!(b_present.a, 10);
    assert_eq!(b_present.b, Some(9));
    assert_eq!(b_present.c, 3);

    println!("A_NOT_SATISFIED");
    const A_NOT_SATISFIED: &[u8] = &[4, 9];
    let mut reader = ByteReader::from(A_NOT_SATISFIED);
    let a_invalid = ABC::read(&mut reader).unwrap();

    assert_eq!(a_invalid.a, 4);
    assert_eq!(a_invalid.b, None);
    assert_eq!(a_invalid.c, 9);

    const B_NOT_PRESENT_BUF: &[u8] = &[10, 9];
    let mut reader = ByteReader::from(B_NOT_PRESENT_BUF);
    assert_eq!(ABC::read(&mut reader).is_err(), true);
}

#[derive(BinaryIo, Debug)]
struct CompexPacket {
    #[skip]
    is_ack: bool,
    contains_content: bool,
    #[satisfy(self.contains_content == true && self.is_ack == true)]
    content: Option<String>,
    #[if_present(content)]
    content_validated: Option<u32>,
}

#[test]
fn complex_packet_write() {
    // ack is true, but the contents are false.
    let ack_true_content_false = CompexPacket {
        is_ack: true,
        contains_content: false,
        content: None,
        content_validated: None,
    };

    assert_eq!(
        ack_true_content_false.write_to_bytes().unwrap().as_slice(),
        &[0]
    );
}

/// Unnamed structs
#[derive(BinaryIo, Debug, PartialEq)]
struct SpecialStruct(bool, #[skip] Option<u8>);

#[test]
fn special_struct_write() {
    let special_struct = SpecialStruct(true, None);
    assert_eq!(special_struct.write_to_bytes().unwrap().as_slice(), &[1]);
}
