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
          self.buffer.resize(self.bounds.1, 0)
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
     fn clamp(&self, offset: usize) -> Self {
          // makes sure that the bound is still possible
          if offset < 0 {
               panic!("Bounds not possible");
          } else {
               self.bounds.0 = offset;
               *self // Dereferrenced for use by consumer.
          }
     }

     /// Checks whether or not the given offset is in between the streams bounds and if the offset is valid.
     fn is_within_bounds(&self, offset: usize) -> bool {
          !(offset > self.bounds.1 || offset < self.bounds.0 || offset > self.buffer.len())
     }

     /// Reads a byte, updates the offset, clamps to last offset.
     fn read(&self) -> u8 {
          let byte = self[self.offset];
          self.clamp(self.offset);
          self.increase_offset(None);
          byte
     }
}

impl std::ops::Index<usize> for BinaryStream {
     type Output = u8;
     fn index(&self, idx: usize) -> &u8 {
          if self.is_within_bounds(idx) {
               if self.bounds.0 == 0 && self.bounds.1 == self.buffer.len() {
                    panic!("Index is out of bounds due to clamp.");
               } else {
                    panic!("Index is out of bounds.");
               }
          }

          self.buffer.get(idx).unwrap()
     }
}

impl std::ops::IndexMut<usize> for BinaryStream {
     fn index_mut(&mut self, offset: usize) -> &mut u8 {
          if self.is_within_bounds(offset) {
               self.buffer.get(offset).as_mut().unwrap()
          } else {
               panic!("Offset: {} is out of bounds.", offset);
          }
     }
}

impl buffer::IBufferRead for BinaryStream {
     fn read_byte(&self) -> u8 {

     }

     fn read_bool(&self) -> bool {
          self.read_byte() == 0
     }
}