#![allow(unused_imports, dead_code)]
pub mod buffer;
pub mod stream;

pub use buffer::*;
pub use stream::*;

pub trait StreamEncoder {
     fn into_stream(&self) -> BinaryStream;
}

pub trait StreamDecoder {
     fn from_stream(stream: BinaryStream) -> Self;
}

#[cfg(test)]
mod tests {
     use crate::*;

     #[test]
     fn read_write_read_write_varint() {
          let mut stream = stream::BinaryStream::new();
          stream.write_uvar_long(32432);
          dbg!(stream.clone());
          stream.set_offset(0);
          assert_eq!(stream.read_uvar_long(), 32432);
     }

     #[test]
     fn slice_test() {
          let mut stream = stream::BinaryStream::init(&[132, 0, 0, 0, 64, 0, 144, 0, 0, 0, 9, 144, 81, 212, 113, 24, 50, 101, 140, 0, 0, 0, 0, 4, 43, 112, 111, 0].to_vec());
          stream.read_byte();
          stream.read_triad();
          let offset = stream.get_offset();
          let mut clamped = stream.clamp(offset, None);
          assert_eq!(clamped.read_byte(), 64)
     }

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
          assert_eq!(stream.get_offset(), 5);
     }

     #[test]
     fn test_read_triad() {
          let buf = [ 233, 9, 27 ];
          // we need to read the first three bytes
          let mut bin = stream::BinaryStream::init(&buf.to_vec());
          let num = bin.read_triad();

          assert_eq!(1772009, num);
     }

     #[test]
     fn test_read_triad_zero() {
          let buf = [ 0, 0, 0 ];
          let mut bin = stream::BinaryStream::init(&buf.to_vec());
          let num = bin.read_triad();

          assert_eq!(num, 0);
     }

     #[test]
     fn test_read_index_at_1() {
          let buf = [144, 0, 0, 0, 9, 143, 162, 116, 15, 10, 144, 162, 92, 0, 0, 0, 0, 21, 47, 173, 144, 0];
          let mut bin = stream::BinaryStream::init(&buf.to_vec());
          bin.read_byte();
          bin.read_triad();
     }

     #[test]
     fn test_read_var_int() {
          let buf = [236, 189, 203, 118, 242, 202, 214, 247, 247, 126, 189, 36, 151, 241, 166, 155, 253, 14, 73, 128, 183, 73, 207, 128, 132, 193, 72, 24, 161, 3, 82, 70, 198, 30, 128, 216, 6, 36, 48, 182, 49, 167, 140];
          let mut bin = stream::BinaryStream::init(&buf.to_vec());
          let v = bin.read_var_int();
          assert_eq!(v, 0)
     }

     #[test]
     fn test_write_triad() {
          let okay = vec![0, 0, 0];
          let mut bin = stream::BinaryStream::new();
          bin.write_triad(0);
          assert_eq!(okay, bin.get_buffer());
     }

     #[test]
     fn test_read_int() {
          let buf = [ 0, 0, 0, 7 ];
          let mut bin = stream::BinaryStream::init(&buf.to_vec());
          let num = bin.read_int();

          assert_eq!(7, num);
     }
}