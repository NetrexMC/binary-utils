pub mod buffer;

pub struct BinaryStream {
     buffer: Vec<u8>,
     offset: usize,
     bounds: (usize, usize)
}

impl BinaryStream {
     /// Increases the offset. If `None` is given in `amount`, 1 will be used.
     fn increase_offset(&self, amount: Option<usize>) -> usize {
          let amnt = match amount {
               None => 1 as usize,
               Some(n) => n
          };

          if (self.offset + amnt) > self.bounds.1 {
               panic!("Offset outside buffer.");
          }

          self.offset = self.offset + amnt;
          self.offset
     }

     /// Changes the offset of the stream to the new given offset.
     /// returns `true` if the offset is in bounds and `false` if the offset is out of bounds.
     fn set_offset(&self, offset: usize) -> bool {
          if offset > self.bounds.1 {
               false
          } else {
               self.offset = offset;
               true
          }
     }

     /// Returns the current offset at the given time when called.
     fn get_offset(&self) -> usize {
          self.offset
     }

     /// Allocates more bytes to the binary stream.
     fn allocate(&self, bytes: usize) {
          self.bounds.1 = self.buffer.len() + bytes;
     }

     /// Create a new Binary Stream from a vector of bytes.
     fn new(buf: Vec<u8>) -> Self {
          Self {
               buffer: buf,
               bounds: (0, buf.len()),
               offset: 0
          }
     }

     /// Similar to slice, clamp, "grips" the buffer from a given offset, and changes the initial bounds.
     /// Meaning that any previous bytes before the given bounds are no longer writable.
     ///
     /// Useful for cloning "part" of a stream, and only allowing certain "bytes" to be read.
     /// Clamps can not be undone.
     ///
     /// **Example:**
     ///
     ///     let stream = BinaryStream::new(vec!(([98,105,110,97,114,121,32,117,116,105,108,115]));
     ///     let shareable_stream = stream.clamp(7); // 32,117,116,105,108,115 are now the only bytes readable externally
     fn clamp(&self, once: usize) -> Self {
          // makes sure that the bound is still possible
          if once < 0 {
               panic!("Bounds not possible");
          } else {
               self.bounds.0 = once;
               *self // Dereferrenced for use by consumer.
          }
     }
}

// impl std::ops::Index<usize> for BinaryStream {

// }

impl buffer::IBufferRead for BinaryStream {
     fn read_byte(&self) -> u8 {

     }

     fn read_bool(&self) -> bool {
          self.read_byte() == 0
     }
}