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
}