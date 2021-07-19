#![allow(unused_imports, dead_code)]
pub mod buffer;
pub mod stream;

pub use buffer::*;
pub use stream::*;

#[cfg(test)]
mod tests {
     use crate::*;

     #[test]
     fn test_read_short() {
          let mut bin_stream = stream::BinaryStream::new();
          bin_stream.write_short(12);

          print!("{:?}", bin_stream);
     }

     #[test]
     fn test_write_byte() {
          let okay = vec![10];
          let mut stream = stream::BinaryStream::new();
          stream.write_byte(10);
          assert_eq!(okay, stream.get_buffer());
     }

     #[test]
     fn test_read_byte() {
          let raw = vec![0, 10, 0, 13, 10];
          let mut stream = stream::BinaryStream::init(&raw);
          stream.read_short();
          stream.read_short();
          println!("{}", stream.get_offset());
          let is_ten = stream.read_byte();
          assert_eq!(is_ten, 10);
     }
}