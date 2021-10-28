use binary_utils::*;
use std::{io::Write};


// Extracted from protocol.
#[derive(Debug, Clone)]
pub struct LString32(pub String);

impl Streamable for LString32 {
     fn parse(&self) -> Vec<u8> {
         // get the length
         let mut buffer: Vec<u8> = Vec::new();
         buffer.write_all(&LE::<u32>(self.0.len() as u32).parse()[..]).unwrap();
         // now we write string buffer.
         buffer.write_all(&self.0.clone().into_bytes()[..]).unwrap();
         buffer
     }

     fn compose(source: &[u8], position: &mut usize) -> Self {
         let length = LE::<u32>::compose(&source[..], position);
         let bytes = &source[*position..(*position + length.0 as usize)];

         *position += bytes.len();

         Self(unsafe { String::from_utf8_unchecked(bytes.to_vec()) })
     }
}


pub const HW_TEST_DATA: &[u8] = &[
     // Length of the string in Little Endian Format
     12, 0, 0, 0,
     // Contents of string
     72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33
];

#[test]
fn write_l32string() {
     let hello_world = "Hello World!".to_string();
     let data = LString32(hello_world).parse();

     assert_eq!(HW_TEST_DATA, &data[..]);
}

#[test]
fn read_l32string() {
     let hello_world = "Hello World!".to_string();
     let data = LString32::compose(HW_TEST_DATA, &mut 0);
     assert_eq!(data.0, hello_world);
}

#[test]
fn read_twice() {
     let hello_world = "Hello World!".to_string();
     let mut stream = Vec::<u8>::new();
     stream.write_all(&LString32(hello_world.clone()).parse()[..]).unwrap();
     stream.write_all(&LString32(hello_world).parse()[..]).unwrap();
     // ok read it.
     let mut pos: usize = 0;
     let one = LString32::compose(&stream[..], &mut pos).0;
     dbg!(&one);
     dbg!(&pos);
     let two = LString32::compose(&stream[..], &mut pos).0;

     assert_eq!(one, two);
}