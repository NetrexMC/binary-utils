use bin_macro::*;
use binary_utils::Streamable;

#[derive(Debug, BinaryStream, PartialEq)]
#[repr(u8)]
pub enum Test {
     Apple = 0,
     Pair = 1
}

#[test]
fn read_test_from_buffer() {
     let buffer: &[u8] = &[0];
     let result = Test::compose(buffer, &mut 0);
     assert_eq!(Test::Apple, result);
}

#[test]
fn write_read_buffer() {
     // write first
     let variant = Test::Pair;
     let buffer = variant.parse();

     assert_eq!(buffer, vec![1]);

     // read now
     let compose = Test::compose(&buffer[..], &mut 0);

     assert!(match compose { Test::Pair => true, _ => false }, "Reconstruction was not equivelant to Test::Pair");
}