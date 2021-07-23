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

     #[test]
     fn read_slice_panic() {
          let raw = vec![7, 0, 255, 255, 0, 254, 254, 254, 254, 253, 253, 253, 253, 18, 52, 86, 120, 4, 128, 255, 255, 254, 74, 188, 2, 65, 140, 131, 72, 201, 65, 219, 142, 52];
          let mut stream = stream::BinaryStream::init(&raw);
          stream.read_byte();
          assert_eq!([0, 255, 255, 0].to_vec(), stream.read_slice(Some(4)));
     }

     #[test]
     fn test_read_int() {
          let buf = [ 233, 9, 27, 10 ];
          // we need to read the first three bytes
          let mut bin = stream::BinaryStream::init(&buf.to_vec());
          let num = bin.read_triad();

          assert_eq!(1772009, num);
     }
}