// This file contains utilities for encoding and decoding streams.
use crate::Streamable;
use std::io::{Cursor, Write};
use byteorder::{BE, ReadBytesExt, WriteBytesExt};

impl Streamable for String {
     fn parse(&self) -> Vec<u8> {
          let mut buffer = Vec::<u8>::new();
          buffer.write_u16::<BE>(self.len() as u16).unwrap();
          buffer.write_all(self.as_bytes()).unwrap();
          buffer
     }

     fn compose(source: &[u8], position: &mut usize) -> Self {
          let mut stream = Cursor::new(source);
          stream.set_position(position.clone() as u64);
          // Maybe do this in the future?
          let len: usize = stream.read_u16::<BE>().unwrap().into();

          unsafe {
               // todo: Remove this nasty hack.
               // todo: The hack being, remove the 2 from indexing on read_short
               // todo: And utilize stream.
               String::from_utf8_unchecked(stream.get_ref()[2..len].to_vec())
          }
     }
}